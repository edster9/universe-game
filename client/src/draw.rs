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
    let along = |from: (i64, i64), to: (i64, i64), since: u64, until: u64| {
        let t = ((now - since as f32) / (until - since).max(1) as f32).clamp(0.0, 1.0);
        point(from).lerp(point(to), t)
    };
    let here = world.spot(id).unwrap_or((0, 0));
    let at = if let Some((_, to, since, until)) = engine::laws::journey(world, id) {
        // On a path: from where they stood to where they'll arrive.
        along(here, world.arrival(id, to), since, until)
    } else if let Some((from, to, since, until)) = engine::laws::stride(world, id) {
        // Walking within the place.
        along(from, to, since, until)
    } else {
        point(here)
    };
    on_ground(land, at)
}

/// A spot from the engine (µm east and north) in the scene's metres.
pub fn point((east, north): (i64, i64)) -> Vec3 {
    Vec3::new(east as f32 / 1e6, 0.0, -(north as f32 / 1e6))
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

    /// A material that gives its own light, as flames do: `glow` is how
    /// bright, in the renderer's light values.
    fn glowing(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        name: &str,
        glow: LinearRgba,
    ) -> Handle<StandardMaterial> {
        let key = format!("glow {name}");
        if let Some(handle) = self.materials.get(&key) {
            return handle.clone();
        }
        // Brighter than white, so it glows past its edges.
        let handle = materials.add(StandardMaterial {
            base_color: Color::LinearRgba(glow),
            emissive: glow,
            unlit: true,
            ..default()
        });
        self.materials.insert(key, handle.clone());
        handle
    }

    /// Smoke, see-through: `thin` from 0 (thickest) to 3 (nearly gone).
    fn smoke(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        thin: usize,
    ) -> Handle<StandardMaterial> {
        let key = format!("smoke {thin}");
        if let Some(handle) = self.materials.get(&key) {
            return handle.clone();
        }
        let handle = materials.add(StandardMaterial {
            base_color: Color::srgba(0.4, 0.4, 0.42, 0.24 - thin as f32 * 0.055),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 1.0,
            ..default()
        });
        self.materials.insert(key, handle.clone());
        handle
    }
}

/// How much is burning in or on a thing, in grams: itself, if it's
/// burning, and everything burning inside it.
fn burning(world: &EngineWorld, id: EntityId) -> f32 {
    let own = if world.is_burning(id) {
        world.mass(id).mg() as f32 / 1_000.0
    } else {
        0.0
    };
    own + world
        .contents(id)
        .into_iter()
        .map(|inner| burning(world, inner))
        .sum::<f32>()
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
    /// A fire at `at`: flames as tall as the burning mass makes them, a
    /// warm light that flickers on everything around, and smoke rising and
    /// drifting. `t` is the clock the flicker follows, `seed` keeps each
    /// fire's flicker its own.
    fn fire(&mut self, at: Vec3, grams: f32, t: f32, seed: u64) {
        // About a metre of flame for a few hundred grams alight.
        let h = (0.25 + 0.3 * (grams + 1.0).log10()).clamp(0.2, 3.0);
        let phase = (seed % 100) as f32;
        let cone = self.kit.mesh(self.meshes, "tree");
        let ball = self.kit.mesh(self.meshes, "rock");
        let outer = self
            .kit
            .glowing(self.materials, "outer", LinearRgba::rgb(6.0, 1.3, 0.12));
        let inner = self
            .kit
            .glowing(self.materials, "inner", LinearRgba::rgb(7.0, 3.2, 0.5));
        for i in 0..5 {
            let k = i as f32;
            let flicker = 0.75 + 0.25 * (t * 9.0 + k * 2.1 + phase).sin();
            let (material, width, height, offset) = if i < 3 {
                let a = k * 2.1 + phase;
                (
                    outer.clone(),
                    0.55,
                    1.0,
                    Vec3::new(a.cos(), 0.0, a.sin()) * h * 0.18,
                )
            } else {
                (inner.clone(), 0.35, 0.7, Vec3::ZERO)
            };
            self.commands.spawn((
                Mesh3d(cone.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(at + offset + Vec3::Y * h * height * flicker * 0.5)
                    .with_rotation(Quat::from_rotation_y(t * 0.7 + k))
                    .with_scale(Vec3::new(h * width, h * height * flicker, h * width)),
                bevy::light::NotShadowCaster,
                Mover,
            ));
        }
        let flicker = 0.85 + 0.1 * (t * 13.0 + phase).sin() + 0.05 * (t * 31.0).sin();
        self.commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.6, 0.28),
                intensity: 400_000.0 * h * h * flicker,
                range: 40.0 * h.sqrt(),
                shadow_maps_enabled: true,
                ..default()
            },
            Transform::from_translation(at + Vec3::Y * h * 0.8),
            Mover,
        ));
        for i in 0..14 {
            // Each puff rises, grows, drifts downwind, and thins, then
            // starts again.
            let rise = (t * 0.12 + i as f32 / 14.0 + phase * 0.01).fract();
            let sway = (i as f32 * 2.4 + phase).sin() * 0.6 * rise;
            let thin = ((rise * 4.0) as usize).min(3);
            let material = self.kit.smoke(self.materials, thin);
            self.commands.spawn((
                Mesh3d(ball.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(
                    at + Vec3::new(
                        rise * rise * 3.0 + sway,
                        h + rise * 9.0 * h.sqrt(),
                        rise + sway * 0.5,
                    ),
                )
                .with_scale(Vec3::splat(h * (0.4 + rise * 2.2))),
                bevy::light::NotShadowCaster,
                Mover,
            ));
        }
    }

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
        // Around its spot, over as far as it spreads.
        let at = world
            .spot(thing)
            .map_or_else(|| place_at(world, place), point);
        let spread = world.spread(thing) as f32 / 1e6;
        for i in 0..n {
            let offset = scatter(seed, i, spread * 0.9);
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
    time: Res<Time>,
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
    let t = time.elapsed_secs();
    for &id in &scene.in_sight {
        let moves = world.is_agent(id) || world.is_portable(id);
        let grams = burning(world, id);
        if grams > 0.0 {
            // Fixed things burn where they stand, at their first piece.
            let seed = id_seed(world, id);
            let at = if moves {
                spot(world, &land, id, now)
            } else {
                let spread = world.spread(id) as f32 / 1e6;
                let middle = world.spot(id).map_or(Vec3::ZERO, point);
                on_ground(&land, middle + scatter(seed, 0, spread * 0.9))
            };
            brush.fire(at + Vec3::Y * 0.2, grams, t, seed);
        }
        if !moves {
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
