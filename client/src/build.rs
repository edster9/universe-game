//! Build mode, a designer's tool (M, or `/build`): rearranging the world by
//! hand. Hover over a thing to see its bounding box and axes; hold the left button to
//! drag it along the ground, or the middle button to lift or lower it; let go
//! and it stays where it is. Dragged, it follows the ground's rise and fall:
//! its height is above the ground wherever it is. Nothing falls yet:
//! something lifted stays in the air. Each move goes through the engine as the designer (as `/place`
//! does), so `/save` keeps the arrangement. The islander still walks. See
//! "Shortcuts for development" in docs/ideas/tools.md.

use bevy::camera::primitives::MeshAabb;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use engine::world::{EntityId, World as EngineWorld};

use crate::camera::pointed_at;
use crate::draw::{Named, point};
use crate::menu::ground;
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
    id: EntityId,
    lifting: bool,
    /// From the point on the ground under the pointer to the thing's spot.
    offset: Vec2,
    /// Metres above the ground.
    height: f32,
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

/// Something the designer can move: lying in a place, and not alive.
fn movable(world: &EngineWorld, id: EntityId) -> bool {
    world.location(id).is_some_and(|l| world.is_place(l)) && !world.is_agent(id)
}

/// Hovering, grabbing, moving, and letting go.
#[allow(clippy::too_many_arguments)]
pub fn drag(
    mut build: ResMut<Build>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    named: Query<(&Named, &GlobalTransform)>,
    pieces: Query<(Entity, &Mesh3d, &GlobalTransform)>,
    owners: Query<&Named>,
    parents: Query<&ChildOf>,
    meshes: Res<Assets<Mesh>>,
    land: Res<Land>,
    mut sim: ResMut<Sim>,
    mut console: ResMut<Console>,
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
    let mut outline = |id: EntityId, colour: Color| {
        if let Some((low, high)) = bounds(id, &pieces, &owners, &parents, &meshes) {
            let size = (high - low).max(Vec3::splat(0.05));
            let middle = (low + high) / 2.0;
            gizmos.cube(Transform::from_translation(middle).with_scale(size), colour);
            gizmos.axes(
                Transform::from_translation(middle),
                size.max_element() * 0.75 + 0.3,
            );
        }
    };
    let Some(grab) = &mut build.grab else {
        // Nothing held: show what's under the pointer, and pick it up.
        let world = sim.world();
        let Some(id) = pointed_at(cursor, camera, eye, &named).filter(|&id| movable(world, id))
        else {
            return;
        };
        outline(id, HOVERED);
        let lifting = buttons.just_pressed(MouseButton::Middle);
        if !(lifting || buttons.just_pressed(MouseButton::Left)) {
            return;
        }
        let at = world.spot(id).map(point).unwrap_or_default();
        let under = camera
            .viewport_to_world(eye, cursor)
            .ok()
            .and_then(|ray| ground(ray, &land))
            .unwrap_or(at);
        build.grab = Some(Grab {
            id,
            lifting,
            offset: Vec2::new(at.x - under.x, at.z - under.z),
            height: world.raised(id) as f32 / 1e6,
        });
        return;
    };
    outline(grab.id, HELD);
    let button = if grab.lifting {
        MouseButton::Middle
    } else {
        MouseButton::Left
    };
    let Play::Live(session) = &mut sim.play else {
        return;
    };
    if !buttons.pressed(button) {
        // Let go: it stays.
        let world = session.world();
        let middle = world
            .place_of(grab.id)
            .and_then(|p| world.position(p))
            .unwrap_or((0, 0));
        let at = world.spot(grab.id).unwrap_or(middle);
        let text = format!(
            "Moved {} to {:.1} {:.1}, {:.1} m up.",
            engine::sight::label(world, session.player(), grab.id),
            (at.0 - middle.0) as f32 / 1e6,
            (at.1 - middle.1) as f32 / 1e6,
            grab.height
        );
        console.say(Said::Debug, &text);
        build.grab = None;
        return;
    }
    let world = session.world();
    let mut at = world.spot(grab.id).unwrap_or((0, 0));
    if grab.lifting {
        grab.height = (grab.height - motion.delta.y * LIFT).max(0.0);
    } else if let Some(under) = camera
        .viewport_to_world(eye, cursor)
        .ok()
        .and_then(|ray| ground(ray, &land))
    {
        let x = under.x + grab.offset.x;
        let z = under.z + grab.offset.y;
        at = ((x * 1e6) as i64, (-z * 1e6) as i64);
    }
    // Beyond the edge of the place, it stays where it was.
    let _ = session.place(grab.id, at, (grab.height * 1e6) as u64);
}

/// The box round everything drawn for a thing, in the scene: its shapes,
/// or the meshes inside its model.
fn bounds(
    id: EntityId,
    pieces: &Query<(Entity, &Mesh3d, &GlobalTransform)>,
    owners: &Query<&Named>,
    parents: &Query<&ChildOf>,
    meshes: &Assets<Mesh>,
) -> Option<(Vec3, Vec3)> {
    let mut found: Option<(Vec3, Vec3)> = None;
    for (entity, mesh, transform) in pieces {
        let owner = owners.get(entity).ok().or_else(|| {
            parents
                .iter_ancestors(entity)
                .find_map(|a| owners.get(a).ok())
        });
        if owner.is_none_or(|&Named(thing)| thing != id) {
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
