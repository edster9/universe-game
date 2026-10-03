//! Build mode, a designer's tool (M, or `/build`): rearranging the world by
//! hand. Hover over a thing (anywhere on it, as drawn) to see its bounding
//! box and axes; hold the left button to
//! drag it along the ground, or the middle button to lift or lower it; let go
//! and it stays where it is. Dragged, it follows the ground's rise and fall:
//! its height is above the ground wherever it is. Nothing falls yet:
//! something lifted stays in the air. Each move goes through the engine as the designer (as `/place`
//! does), so `/save` keeps the arrangement. Models in the showcase move too,
//! on screen only. The islander still walks. See
//! "Shortcuts for development" in docs/ideas/tools.md.

use bevy::camera::primitives::MeshAabb;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use engine::world::{EntityId, World as EngineWorld};

use crate::built::Built;
use crate::camera::{Picked, Pointing};
use crate::draw::{Named, on_ground, point};
use crate::showcase::Shown;
use crate::terminal::{Console, Said};
use crate::terrain::Land;
use crate::{Play, Sim};

/// Whether build mode is on, and what's held.
#[derive(Resource, Default)]
pub struct Build {
    pub on: bool,
    grab: Option<Grab>,
}

/// Something being moved: by the left button along the ground, or by the
/// middle button up and down.
struct Grab {
    what: Picked,
    lifting: bool,
    /// The height it was taken hold of at: the pointer moves it across
    /// that level, so what's taken by its top follows the pointer as well
    /// as what's taken by its foot.
    level: f32,
    /// From where it was taken hold of to its spot, across.
    offset: Vec2,
    /// Metres above the ground.
    height: f32,
}

impl Build {
    /// Whether something is held, so the camera leaves the mouse alone.
    pub fn holding(&self) -> bool {
        self.grab.is_some()
    }
}

/// How many metres a pixel of the mouse's movement lifts something.
const LIFT: f32 = 0.01;

const HOVERED: Color = Color::srgb(1.0, 0.9, 0.2);
const HELD: Color = Color::srgb(1.0, 0.5, 0.1);

/// M turns build mode on and off.
pub fn toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut console: ResMut<Console>,
    mut build: ResMut<Build>,
) {
    if console.typing || !keys.just_pressed(KeyCode::KeyM) {
        return;
    }
    build.on = !build.on;
    build.grab = None;
    say(&mut console, build.on);
}

/// What build mode says when it's turned on or off.
pub fn say(console: &mut Console, on: bool) {
    console.say(
        Said::Debug,
        if on {
            "Build mode: hover over something to see it; hold the left button to drag it along \
             the ground, the middle button to lift or lower it. M again to leave; /save keeps it."
        } else {
            "Build mode off."
        },
    );
}

/// Something the designer can move: lying in a place, and not alive, nor a
/// source placed in Blender (sources move in Blender, the owner, 2026-10-02).
fn movable(world: &EngineWorld, built: &Built, id: EntityId) -> bool {
    world.location(id).is_some_and(|l| world.is_place(l))
        && !world.is_agent(id)
        && !built.sources.contains(&id)
}

/// Hovering, grabbing, moving, and letting go.
#[allow(clippy::too_many_arguments)]
pub fn drag(
    mut build: ResMut<Build>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut pointing: Pointing,
    pieces: Query<(Entity, &Mesh3d, &GlobalTransform)>,
    owners: Query<&Named>,
    parents: Query<&ChildOf>,
    mut shown: Query<(&Shown, &mut Transform)>,
    meshes: Res<Assets<Mesh>>,
    land: Res<Land>,
    mut sim: ResMut<Sim>,
    mut console: ResMut<Console>,
    blender: Res<Built>,
    mut gizmos: Gizmos,
) {
    if !build.on {
        return;
    }
    let (Ok(window), Ok((camera, eye))) = (window.single(), camera.single()) else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    // The box round it, and its axes from the middle: red east and west,
    // green up and down, blue north and south. For turning things, later.
    let mut outline = |what: Picked, colour: Color| {
        if let Some((low, high)) = bounds(what, &pieces, &owners, &parents, &meshes) {
            let size = (high - low).max(Vec3::splat(0.05));
            let middle = (low + high) / 2.0;
            gizmos.cube(Transform::from_translation(middle).with_scale(size), colour);
            gizmos.axes(
                Transform::from_translation(middle),
                size.max_element() * 0.75 + 0.3,
            );
        }
    };
    // Where the pointer is at a level: its ray meeting that height.
    let across = |level: f32| {
        let ray = camera.viewport_to_world(eye, cursor).ok()?;
        let t = (level - ray.origin.y) / ray.direction.y;
        (t > 0.0 && t < 500.0).then(|| ray.get_point(t))
    };
    let Some(grab) = &mut build.grab else {
        // Nothing held: show what's under the pointer, and pick it up.
        let world = sim.world();
        let Some((what, held_at)) = pointing.hit(cursor, camera, eye, |p| match p {
            Picked::Thing(id) => movable(world, &blender, id),
            Picked::Shown(_) => true,
        }) else {
            return;
        };
        outline(what, HOVERED);
        let lifting = buttons.just_pressed(MouseButton::Middle);
        if !(lifting || buttons.just_pressed(MouseButton::Left)) {
            return;
        }
        let (at, height) = match what {
            Picked::Thing(id) => (
                world.spot(id).map(point).unwrap_or_default(),
                world.raised(id) as f32 / 1e6,
            ),
            Picked::Shown(e) => {
                let Ok((_, transform)) = shown.get(e) else {
                    return;
                };
                let at = transform.translation;
                (at, at.y - land.height(Vec2::new(at.x, at.z)))
            }
        };
        build.grab = Some(Grab {
            what,
            lifting,
            level: held_at.y,
            offset: Vec2::new(at.x - held_at.x, at.z - held_at.z),
            height,
        });
        return;
    };
    outline(grab.what, HELD);
    let button = if grab.lifting {
        MouseButton::Middle
    } else {
        MouseButton::Left
    };
    let held = buttons.pressed(button);
    if grab.lifting {
        grab.height = (grab.height - motion.delta.y * LIFT).max(0.0);
    }
    // Where it goes along the ground: under the pointer, as it was taken.
    let along = (!grab.lifting)
        .then(|| across(grab.level))
        .flatten()
        .map(|u| Vec2::new(u.x + grab.offset.x, u.z + grab.offset.y));
    match grab.what {
        Picked::Shown(e) => {
            let Ok((name, mut transform)) = shown.get_mut(e) else {
                build.grab = None;
                return;
            };
            let flat = along.unwrap_or(Vec2::new(transform.translation.x, transform.translation.z));
            transform.translation =
                on_ground(&land, Vec3::new(flat.x, 0.0, flat.y)) + Vec3::Y * grab.height;
            if !held {
                let text = format!(
                    "Moved {} to {:.1} {:.1}, {:.1} m up (the showcase: not kept by /save).",
                    name.0, flat.x, -flat.y, grab.height
                );
                console.say(Said::Debug, &text);
                build.grab = None;
            }
        }
        Picked::Thing(id) => {
            let Play::Live(session) = &mut sim.play else {
                return;
            };
            if !held {
                // Let go: it stays.
                let world = session.world();
                let middle = world
                    .place_of(id)
                    .and_then(|p| world.position(p))
                    .unwrap_or((0, 0));
                let at = world.spot(id).unwrap_or(middle);
                let text = format!(
                    "Moved {} to {:.1} {:.1}, {:.1} m up.",
                    engine::sight::label(world, session.player(), id),
                    (at.0 - middle.0) as f32 / 1e6,
                    (at.1 - middle.1) as f32 / 1e6,
                    grab.height
                );
                console.say(Said::Debug, &text);
                build.grab = None;
                return;
            }
            let at = along.map_or_else(
                || session.world().spot(id).unwrap_or((0, 0)),
                |p| ((p.x * 1e6) as i64, (-p.y * 1e6) as i64),
            );
            // Beyond the edge of the place, it stays where it was.
            let _ = session.place(id, at, (grab.height * 1e6) as u64);
        }
    }
}

/// The box round everything drawn for a thing, in the scene: its shapes,
/// or the meshes inside its model.
fn bounds(
    what: Picked,
    pieces: &Query<(Entity, &Mesh3d, &GlobalTransform)>,
    owners: &Query<&Named>,
    parents: &Query<&ChildOf>,
    meshes: &Assets<Mesh>,
) -> Option<(Vec3, Vec3)> {
    let mut found: Option<(Vec3, Vec3)> = None;
    for (entity, mesh, transform) in pieces {
        let mine = match what {
            Picked::Thing(id) => std::iter::once(entity)
                .chain(parents.iter_ancestors(entity))
                .find_map(|e| owners.get(e).ok())
                .is_some_and(|&Named(thing)| thing == id),
            Picked::Shown(root) => {
                entity == root || parents.iter_ancestors(entity).any(|e| e == root)
            }
        };
        if !mine {
            continue;
        }
        let Some(aabb) = meshes.get(&mesh.0).and_then(|m| m.compute_aabb()) else {
            continue;
        };
        let (c, h) = (Vec3::from(aabb.center), Vec3::from(aabb.half_extents));
        for corner in 0..8 {
            let sign = Vec3::new(
                if corner & 1 == 0 { -1.0 } else { 1.0 },
                if corner & 2 == 0 { -1.0 } else { 1.0 },
                if corner & 4 == 0 { -1.0 } else { 1.0 },
            );
            let p = transform.transform_point(c + h * sign);
            found = Some(match found {
                Some((low, high)) => (low.min(p), high.max(p)),
                None => (p, p),
            });
        }
    }
    found
}
