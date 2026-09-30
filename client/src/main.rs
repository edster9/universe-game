//! The game's native client, first scene: the companion's island, drawn live
//! from the engine. The engine runs here, as it does in the tests; the client
//! only draws what the world holds, from simple shapes and colours (see
//! `style.toml`), and moves walkers along their way between the moments they
//! set off and arrive. An overview of the whole world, for development: not
//! yet one character's view. See docs/ideas/the-client.md.
//!
//! Controls: WASD to fly, E/Q up and down, Shift faster, hold the left mouse
//! button to look around; Space pauses the world, [ and ] slow it down and
//! speed it up.
//!
//! `client --shot <file.png> [--after <seconds>] [--speed <x>]` saves one
//! frame after a few seconds and exits, so the scene can be checked without
//! a screen watcher. `--from x,y,z` and `--look x,y,z` place the camera.

use std::collections::{BTreeMap, BTreeSet};
use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use engine::world::{EntityId, World as EngineWorld};
use serde::Deserialize;

mod terrain;
use terrain::Land;

const WORLD: &str = include_str!("../../data/companion.toml");
const THINGS: &str = include_str!("../../data/island-things.toml");
const FAR_FOLK: &str = include_str!("../../data/far-folk.toml");
const STYLE: &str = include_str!("../style.toml");

/// How far from a place's centre its things are scattered, in metres.
const PATCH: f32 = 90.0;

#[derive(Deserialize, Clone)]
struct Look {
    form: String,
    #[serde(default = "grey")]
    colour: String,
    #[serde(default = "one")]
    size: f32,
}

fn grey() -> String {
    "#909090".into()
}

fn one() -> f32 {
    1.0
}

#[derive(Deserialize)]
struct Style {
    #[serde(default)]
    kind: BTreeMap<String, Look>,
    #[serde(default)]
    material: BTreeMap<String, Look>,
    fallback: Look,
}

impl Style {
    /// How to draw a thing: by its kind if it has one, else by what it's
    /// mostly made of.
    fn look(&self, world: &EngineWorld, id: EntityId) -> Look {
        if let Some(look) = world.kind_of(id).and_then(|k| self.kind.get(k)) {
            return look.clone();
        }
        world
            .composition(id)
            .and_then(engine::matter::dominant)
            .map(|m| &world.materials()[&m].key)
            .and_then(|key| self.material.get(key))
            .unwrap_or(&self.fallback)
            .clone()
    }
}

/// The engine's world, and how fast it runs.
#[derive(Resource)]
struct Sim {
    world: EngineWorld,
    /// Game seconds per real second.
    speed: f32,
    paused: bool,
    /// Game time not yet run, in seconds.
    owed: f32,
}

#[derive(Resource)]
struct Styles(Style);

/// Meshes and materials made once and shared.
#[derive(Resource, Default)]
struct Kit {
    meshes: BTreeMap<String, Handle<Mesh>>,
    materials: BTreeMap<String, Handle<StandardMaterial>>,
}

/// A drawn thing that moves: a creature, a person, or a loose piece.
#[derive(Component)]
struct Drawn(EntityId);

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct Sun;

/// What the command line asked for.
#[derive(Resource, Default)]
struct Options {
    shot: Option<String>,
    after: f32,
    from: Vec3,
    look: Vec3,
}

fn point(text: Option<&String>, or: Vec3) -> Vec3 {
    let v: Vec<f32> = text
        .map(|t| t.split(',').filter_map(|n| n.trim().parse().ok()).collect())
        .unwrap_or_default();
    if v.len() == 3 {
        Vec3::new(v[0], v[1], v[2])
    } else {
        or
    }
}

fn main() {
    let mut options = Options {
        shot: None,
        after: 4.0,
        // Above the beach, looking inland over the forest to the hillside.
        from: Vec3::new(120.0, 60.0, 180.0),
        look: Vec3::new(-300.0, 20.0, -20.0),
    };
    let mut speed = 60.0;
    // "--from x,y,z" and "--from=x,y,z" alike.
    let args: Vec<String> = std::env::args()
        .flat_map(|a| match a.split_once('=') {
            Some((k, v)) if a.starts_with("--") => vec![k.to_string(), v.to_string()],
            _ => vec![a],
        })
        .collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--shot" => {
                options.shot = args.get(i + 1).cloned();
                i += 1;
            }
            "--after" => {
                options.after = args.get(i + 1).and_then(|a| a.parse().ok()).unwrap_or(4.0);
                i += 1;
            }
            "--from" => {
                options.from = point(args.get(i + 1), options.from);
                i += 1;
            }
            "--look" => {
                options.look = point(args.get(i + 1), options.look);
                i += 1;
            }
            "--speed" => {
                speed = args.get(i + 1).and_then(|a| a.parse().ok()).unwrap_or(60.0);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    let world = engine::data::load_world_with(WORLD, &[THINGS, FAR_FOLK])
        .expect("the companion's world")
        .with_player_rules("survivor")
        .expect("the islander");
    let style: Style = toml::from_str(STYLE).expect("the style file");
    let land = Land::of(&world);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Universe game: the companion's island".into(),
                resolution: (1600, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FreeCameraPlugin)
        .insert_resource(ClearColor(Color::srgb(0.55, 0.72, 0.9)))
        .insert_resource(Sim {
            world,
            speed,
            paused: false,
            owed: 0.0,
        })
        .insert_resource(Styles(style))
        .insert_resource(land)
        .insert_resource(options)
        .init_resource::<Kit>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (controls, run_world, draw_movers, day_and_night, hud, shot).chain(),
        )
        .run();
}

/// Metres east, up, and south in the scene for a place: the engine keeps
/// micrometres east and north, and heights.
fn place_at(world: &EngineWorld, place: EntityId) -> Vec3 {
    let (east, north) = world.position(place).unwrap_or((0, 0));
    Vec3::new(
        east as f32 / 1e6,
        world.height(place) as f32 / 1e6,
        -(north as f32 / 1e6),
    )
}

/// A point set down on the ground.
fn on_ground(land: &Land, p: Vec3) -> Vec3 {
    Vec3::new(p.x, land.height(Vec2::new(p.x, p.z)), p.z)
}

/// A steady scatter within a place's patch, the same for the same thing
/// every time: an angle and a distance from its id.
fn scatter(seed: u64, n: u64, reach: f32) -> Vec3 {
    let mut x = seed
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(n.wrapping_mul(0xBF58_476D_1CE4_E5B9));
    x ^= x >> 31;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 29;
    let angle = (x % 10_000) as f32 / 10_000.0 * TAU;
    let distance = ((x >> 20) % 10_000) as f32 / 10_000.0;
    Vec3::new(angle.cos(), 0.0, angle.sin()) * reach * distance.sqrt()
}

fn id_seed(world: &EngineWorld, id: EntityId) -> u64 {
    // Keys are stable ids from data, or "#n" for things made in play.
    world
        .key(id)
        .bytes()
        .fold(0xCBF2_9CE4_8422_2325u64, |h, b| {
            (h ^ b as u64).wrapping_mul(0x100_0000_01B3)
        })
}

fn colour(hex: &str) -> Color {
    Srgba::hex(hex)
        .map(Color::from)
        .unwrap_or(Color::srgb(0.6, 0.6, 0.6))
}

impl Kit {
    fn mesh(&mut self, meshes: &mut Assets<Mesh>, form: &str) -> Handle<Mesh> {
        if let Some(handle) = self.meshes.get(form) {
            return handle.clone();
        }
        // Each at unit size; scaled when placed.
        let mesh: Mesh = match form {
            "log" => Cylinder::new(0.12, 1.0).into(),
            "tree" => Cone::new(0.35, 1.0).into(),
            "trunk" => Cylinder::new(0.06, 1.0).into(),
            "bush" => Sphere::new(0.5).mesh().ico(2).expect("a bush"),
            "tuft" => Cone::new(0.25, 1.0).into(),
            "pool" => Cylinder::new(0.5, 0.02).into(),
            "mound" => Sphere::new(0.5).mesh().ico(3).expect("a mound"),
            "person" => Capsule3d::new(0.17, 0.66).into(),
            "beast" => Capsule3d::new(0.35, 0.8).into(),
            "patch" => Cylinder::new(1.0, 1.0).into(),
            _ => Sphere::new(0.5).mesh().ico(1).expect("a rock"),
        };
        let handle = meshes.add(mesh);
        self.meshes.insert(form.to_string(), handle.clone());
        handle
    }

    fn material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        hex: &str,
    ) -> Handle<StandardMaterial> {
        if let Some(handle) = self.materials.get(hex) {
            return handle.clone();
        }
        let handle = materials.add(StandardMaterial {
            base_color: colour(hex),
            perceptual_roughness: 0.9,
            ..default()
        });
        self.materials.insert(hex.to_string(), handle.clone());
        handle
    }
}

/// How many pieces show a fixed source of `kg`: more for more, within reason.
fn pieces(kg: f32) -> u64 {
    (kg.max(1.0).log10() * 3.0).round().clamp(1.0, 16.0) as u64
}

/// Draws one piece of a thing, at `at` on the ground, as its look says.
#[allow(clippy::too_many_arguments)]
fn draw_piece(
    commands: &mut Commands,
    kit: &mut Kit,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    look: &Look,
    at: Vec3,
    turn: f32,
    marker: Option<Drawn>,
) {
    let s = look.size;
    let parts: Vec<(&str, &str, Transform)> = match look.form.as_str() {
        "none" => Vec::new(),
        "log" => vec![(
            "log",
            look.colour.as_str(),
            Transform::from_translation(at + Vec3::Y * 0.12)
                .with_rotation(Quat::from_rotation_y(turn) * Quat::from_rotation_z(FRAC_PI_2))
                .with_scale(Vec3::new(1.0, s, 1.0)),
        )],
        "tree" => vec![
            (
                "trunk",
                "#5b3a1e",
                Transform::from_translation(at + Vec3::Y * s * 0.2).with_scale(Vec3::new(
                    s * 0.5,
                    s * 0.4,
                    s * 0.5,
                )),
            ),
            (
                "tree",
                look.colour.as_str(),
                Transform::from_translation(at + Vec3::Y * s * 0.65).with_scale(Vec3::splat(s)),
            ),
        ],
        "pool" => vec![(
            "pool",
            look.colour.as_str(),
            Transform::from_translation(at + Vec3::Y * 0.02).with_scale(Vec3::new(s, 1.0, s)),
        )],
        "mound" => vec![(
            "mound",
            look.colour.as_str(),
            Transform::from_translation(at).with_scale(Vec3::new(s, s * 0.08, s)),
        )],
        "person" => vec![(
            "person",
            look.colour.as_str(),
            Transform::from_translation(at + Vec3::Y * s * 0.5).with_scale(Vec3::splat(s / 1.0)),
        )],
        "beast" => vec![(
            "beast",
            look.colour.as_str(),
            Transform::from_translation(at + Vec3::Y * 0.4)
                .with_rotation(Quat::from_rotation_y(turn) * Quat::from_rotation_z(FRAC_PI_2))
                .with_scale(Vec3::splat(s)),
        )],
        form => vec![(
            form,
            look.colour.as_str(),
            Transform::from_translation(at + Vec3::Y * s * 0.3)
                .with_rotation(Quat::from_rotation_y(turn))
                .with_scale(Vec3::new(s, s * 0.7, s * 0.9)),
        )],
    };
    for (form, hex, transform) in parts {
        let mut entity = commands.spawn((
            Mesh3d(kit.mesh(meshes, form)),
            MeshMaterial3d(kit.material(materials, hex)),
            transform,
        ));
        if let Some(Drawn(id)) = marker {
            entity.insert(Drawn(id));
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn setup(
    mut commands: Commands,
    sim: Res<Sim>,
    styles: Res<Styles>,
    land: Res<Land>,
    options: Res<Options>,
    mut kit: ResMut<Kit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let world = &sim.world;

    // The sea, all round, and the land.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200_000.0, 200_000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.13, 0.33, 0.52),
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::default(),
    ));
    let ground = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.95,
        ..default()
    });
    for mesh in land.mesh() {
        commands.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(ground.clone())));
    }

    let places: Vec<EntityId> = world.entities().filter(|&e| world.is_place(e)).collect();
    for &place in &places {
        let at = place_at(world, place);
        // What's fixed there, as clusters of pieces.
        for thing in world.contents(place) {
            if world.is_agent(thing) || world.is_portable(thing) {
                continue;
            }
            let look = styles.0.look(world, thing);
            let kg = world.mass(thing).mg() as f32 / 1e6;
            let n = if matches!(look.form.as_str(), "pool" | "mound") {
                1
            } else {
                pieces(kg)
            };
            let seed = id_seed(world, thing);
            for i in 0..n {
                let offset = scatter(seed, i, PATCH * 0.85);
                let turn = (seed.wrapping_add(i) % 628) as f32 / 100.0;
                draw_piece(
                    &mut commands,
                    &mut kit,
                    &mut meshes,
                    &mut materials,
                    &look,
                    on_ground(&land, at + offset),
                    turn,
                    None,
                );
            }
        }
    }

    // The sun, turned by `day_and_night`.
    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::default(),
        Sun,
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        ..default()
    });

    // Looking over the beach towards the forest and the hillside.
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(options.from).looking_at(options.look, Vec3::Y),
        DistanceFog {
            color: Color::srgb(0.62, 0.74, 0.88),
            falloff: FogFalloff::Linear {
                start: 3_000.0,
                end: 40_000.0,
            },
            ..default()
        },
        FreeCamera {
            walk_speed: 60.0,
            run_speed: 600.0,
            ..default()
        },
    ));

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: bevy::text::FontSize::Px(18.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(12.0),
            ..default()
        },
        Hud,
    ));
}

fn controls(keys: Res<ButtonInput<KeyCode>>, mut sim: ResMut<Sim>) {
    if keys.just_pressed(KeyCode::Space) {
        sim.paused = !sim.paused;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        sim.speed = (sim.speed * 4.0).min(14_400.0);
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        sim.speed = (sim.speed / 4.0).max(1.0);
    }
}

/// Runs the engine's world on, a whole number of game seconds at a time.
fn run_world(time: Res<Time>, mut sim: ResMut<Sim>) {
    if sim.paused {
        return;
    }
    sim.owed += time.delta_secs() * sim.speed;
    let whole = sim.owed.floor();
    if whole >= 1.0 {
        sim.owed -= whole;
        let seconds = whole as u64;
        if let Err(fault) = engine::nature::run(&mut sim.world, seconds) {
            eprintln!("engine fault: {fault}");
            sim.paused = true;
        }
    }
}

/// Draws everything that moves or is carried off: creatures and people, at
/// their place or on their way, and loose pieces lying at a place. Redrawn
/// from the world each frame.
#[allow(clippy::too_many_arguments)]
fn draw_movers(
    mut commands: Commands,
    sim: Res<Sim>,
    styles: Res<Styles>,
    land: Res<Land>,
    mut kit: ResMut<Kit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    drawn: Query<Entity, With<Drawn>>,
) {
    for entity in &drawn {
        commands.entity(entity).despawn();
    }
    let world = &sim.world;
    let now = world.tick() as f32 + sim.owed;
    let mut seen = BTreeSet::new();
    for id in world.entities().collect::<Vec<_>>() {
        let Some(place) = world.location(id).filter(|&p| world.is_place(p)) else {
            continue;
        };
        if !(world.is_agent(id) || world.is_portable(id)) || !seen.insert(id) {
            continue;
        }
        let look = styles.0.look(world, id);
        let seed = id_seed(world, id);
        let spot = |p: EntityId| place_at(world, p) + scatter(seed, 7, PATCH * 0.6);
        // On their way: between where they set off and where they're going.
        let at = match engine::laws::journey(world, id) {
            Some((from, to, since, until)) if until > since => {
                let t = ((now - since as f32) / (until - since) as f32).clamp(0.0, 1.0);
                spot(from).lerp(spot(to), t)
            }
            _ => spot(place),
        };
        let lying = world.life(id).is_some_and(|l| l.died_of.is_some()) || world.is_asleep(id);
        let mut look = look;
        if world.life(id).is_some_and(|l| l.died_of.is_some()) {
            look.colour = "#3a3a3a".into();
        }
        let turn = (seed % 628) as f32 / 100.0;
        if lying && look.form == "person" {
            look.form = "beast".into();
            look.size *= 0.5;
        }
        draw_piece(
            &mut commands,
            &mut kit,
            &mut meshes,
            &mut materials,
            &look,
            on_ground(&land, at),
            turn,
            Some(Drawn(id)),
        );
    }
}

/// The sun's angle and strength from the world's time of day.
fn day_and_night(
    sim: Res<Sim>,
    mut sun: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut sky: ResMut<ClearColor>,
) {
    let settings = sim.world.settings();
    let (Some(of_day), day) = (sim.world.time_of_day(), settings.day) else {
        return;
    };
    let (rise, set) = (settings.sunrise as f32, settings.sunset as f32);
    let t = of_day as f32;
    // 0 at sunrise, 1 at sunset; outside that, night.
    let arc = (t - rise) / (set - rise);
    let up = if (0.0..=1.0).contains(&arc) {
        (arc * std::f32::consts::PI).sin()
    } else {
        0.0
    };
    let _ = day;
    for (mut transform, mut light) in &mut sun {
        let angle = arc.clamp(0.0, 1.0) * std::f32::consts::PI;
        let direction = Vec3::new(-angle.cos(), -angle.sin().max(0.05), -0.3);
        *transform = Transform::default().looking_to(direction, Vec3::Y);
        light.illuminance = 15_000.0 * up.sqrt();
    }
    ambient.brightness = 80.0 + 900.0 * up.sqrt();
    // Night's dark blue to a clear day's pale blue, quickly after dawn.
    let day = up.sqrt();
    sky.0 = Color::srgb(0.03 + 0.52 * day, 0.05 + 0.68 * day, 0.12 + 0.8 * day);
}

fn hud(sim: Res<Sim>, mut text: Query<&mut Text, With<Hud>>) {
    let world = &sim.world;
    let clock = world.time_of_day().map_or(String::new(), |t| {
        format!(
            "Day {}, {:02}:{:02}",
            world.tick() / 86_400 + 1,
            t / 3600,
            t / 60 % 60
        )
    });
    let living = world
        .entities()
        .filter(|&e| world.is_living(e) && world.is_agent(e))
        .count();
    let state = if sim.paused {
        "paused".to_string()
    } else {
        format!("x{}", sim.speed)
    };
    for mut text in &mut text {
        text.0 = format!(
            "{clock}   {state}   {living} living\nWASD fly, E/Q up/down, Shift faster, hold right mouse to look; Space pause, [ ] slower/faster"
        );
    }
}

/// With `--shot`, saves a frame after a few seconds, then exits.
fn shot(
    mut commands: Commands,
    time: Res<Time>,
    options: Res<Options>,
    mut done: Local<Option<f32>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = &options.shot else { return };
    let now = time.elapsed_secs();
    match *done {
        None if now >= options.after => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path.clone()));
            *done = Some(now);
        }
        Some(at) if now > at + 2.0 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
