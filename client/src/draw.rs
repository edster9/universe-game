//! Drawing what the islander pictures (`engine::view::scene`): everything
//! where they stand, and the fixed things they remember elsewhere, from
//! simple shapes and colours (see `style.toml`). Walkers move along their way
//! between the moments they set off and arrive.

use std::collections::BTreeMap;
use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::prelude::*;
use engine::world::{EntityId, World as EngineWorld};
use serde::Deserialize;

use crate::Sim;
use crate::terrain::Land;

pub const STYLE: &str = include_str!("../style.toml");

/// How far from a place's centre its things are scattered, in metres.
const PATCH: f32 = 90.0;

#[derive(Deserialize, Clone)]
pub struct Look {
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

#[derive(Deserialize, Resource)]
pub struct Style {
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

/// Meshes and materials made once and shared.
#[derive(Resource, Default)]
pub struct Kit {
    meshes: BTreeMap<String, Handle<Mesh>>,
    materials: BTreeMap<String, Handle<StandardMaterial>>,
}

/// A drawn piece of something in the world: pointing at it names it.
#[derive(Component, Clone, Copy)]
pub struct Named(pub EntityId);

/// Something that moves or can be carried off, redrawn every frame.
#[derive(Component, Clone)]
pub struct Mover;

/// A fixed thing, redrawn only when what's pictured changes.
#[derive(Component, Clone)]
pub struct Scenery;

/// Metres east, up, and south in the scene for a place: the engine keeps
/// micrometres east and north, and heights.
pub fn place_at(world: &EngineWorld, place: EntityId) -> Vec3 {
    let (east, north) = world.position(place).unwrap_or((0, 0));
    Vec3::new(
        east as f32 / 1e6,
        world.height(place) as f32 / 1e6,
        -(north as f32 / 1e6),
    )
}

/// A point set down on the ground.
pub fn on_ground(land: &Land, p: Vec3) -> Vec3 {
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

/// Where a person, creature, or loose thing stands, on the ground: its spot
/// at its place, or on its way between two places. `now` is the world's
/// time, with the fraction of a second not yet run.
pub fn spot(world: &EngineWorld, land: &Land, id: EntityId, now: f32) -> Vec3 {
    let seed = id_seed(world, id);
    let at = |p: EntityId| place_at(world, p) + scatter(seed, 7, PATCH * 0.6);
    let point = match engine::laws::journey(world, id) {
        Some((from, to, since, until)) if until > since => {
            let t = ((now - since as f32) / (until - since) as f32).clamp(0.0, 1.0);
            at(from).lerp(at(to), t)
        }
        _ => world.place_of(id).map_or(Vec3::ZERO, |p| {
            place_at(world, p) + scatter(seed, 7, PATCH * 0.6)
        }),
    };
    on_ground(land, point)
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

/// What drawing needs, in one place.
pub struct Brush<'a, 'w> {
    pub commands: Commands<'w, 'a>,
    pub kit: &'a mut Kit,
    pub meshes: &'a mut Assets<Mesh>,
    pub materials: &'a mut Assets<StandardMaterial>,
}

impl Brush<'_, '_> {
    /// Draws one piece of a thing, at `at` on the ground, as its look says,
    /// marked with `marker`.
    fn piece(
        &mut self,
        look: &Look,
        at: Vec3,
        turn: f32,
        named: Named,
        marker: impl Bundle + Clone,
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
                Transform::from_translation(at + Vec3::Y * s * 0.5).with_scale(Vec3::splat(s)),
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
            let mesh = self.kit.mesh(self.meshes, form);
            let material = self.kit.material(self.materials, hex);
            self.commands.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                transform,
                named,
                marker.clone(),
            ));
        }
    }
}

/// The fixed things the islander pictures, as clusters of pieces around
/// their places: redrawn when what's pictured, or how much of it there is,
/// changes.
#[allow(clippy::too_many_arguments)]
pub fn draw_scenery(
    commands: Commands,
    sim: Res<Sim>,
    style: Res<Style>,
    land: Res<Land>,
    mut kit: ResMut<Kit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    drawn: Query<Entity, With<Scenery>>,
    mut last: Local<Vec<(EntityId, EntityId, u64)>>,
) {
    let world = sim.world();
    let scene = engine::view::scene(world, sim.me());
    let mut fixed: Vec<(EntityId, EntityId, u64)> = scene
        .remembered
        .iter()
        .copied()
        .chain(scene.here.into_iter().flat_map(|here| {
            scene
                .in_sight
                .iter()
                .filter(|&&t| !world.is_agent(t) && !world.is_portable(t))
                .map(move |&t| (here, t))
        }))
        .map(|(place, t)| (place, t, pieces(world.mass(t).mg() as f32 / 1e6)))
        .collect();
    fixed.sort();
    if *last == fixed {
        return;
    }
    let mut brush = Brush {
        commands,
        kit: &mut kit,
        meshes: &mut meshes,
        materials: &mut materials,
    };
    for entity in &drawn {
        brush.commands.entity(entity).despawn();
    }
    for &(place, thing, n) in &fixed {
        let look = style.look(world, thing);
        let n = if matches!(look.form.as_str(), "pool" | "mound") {
            1
        } else {
            n
        };
        let seed = id_seed(world, thing);
        let at = place_at(world, place);
        for i in 0..n {
            let offset = scatter(seed, i, PATCH * 0.85);
            let turn = (seed.wrapping_add(i) % 628) as f32 / 100.0;
            brush.piece(
                &look,
                on_ground(&land, at + offset),
                turn,
                Named(thing),
                Scenery,
            );
        }
    }
    *last = fixed;
}

/// Everything that moves or can be carried off, where the islander stands:
/// people and creatures, at their spot or on their way, and loose pieces.
/// Redrawn from the world each frame.
#[allow(clippy::too_many_arguments)]
pub fn draw_movers(
    commands: Commands,
    sim: Res<Sim>,
    style: Res<Style>,
    land: Res<Land>,
    mut kit: ResMut<Kit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    drawn: Query<Entity, With<Mover>>,
) {
    let mut brush = Brush {
        commands,
        kit: &mut kit,
        meshes: &mut meshes,
        materials: &mut materials,
    };
    for entity in &drawn {
        brush.commands.entity(entity).despawn();
    }
    let world = sim.world();
    let now = sim.now();
    let scene = engine::view::scene(world, sim.me());
    for &id in &scene.in_sight {
        if !(world.is_agent(id) || world.is_portable(id)) {
            continue;
        }
        let mut look = style.look(world, id);
        let dead = world.life(id).is_some_and(|l| l.died_of.is_some());
        if dead {
            look.colour = "#3a3a3a".into();
        }
        if (dead || world.is_asleep(id)) && look.form == "person" {
            look.form = "beast".into();
            look.size *= 0.5;
        }
        let turn = (id_seed(world, id) % 628) as f32 / 100.0;
        brush.piece(&look, spot(world, &land, id, now), turn, Named(id), Mover);
    }
}
