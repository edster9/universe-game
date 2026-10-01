//! Walking the islander with WASD, relative to where the camera looks, as in
//! any third-person game. To the engine, each step is the same as typing
//! "walk to <east> <north>": a short walk within the place, renewed while a
//! key is held. A key pressed during a commanded walk within the place takes
//! over from where the islander has got to. Walking into the edge of a place
//! towards a way out sets off along it. See docs/ideas/nearness.md.

use bevy::prelude::*;
use engine::intent::Intent;

use crate::camera::Eye;
use crate::terminal::Console;
use crate::tools::Settings;
use crate::{Play, Sim};

/// How far one step goes, in metres: about a second's walk.
const STEP: f32 = 1.3;
/// A way out within this angle of where you're walking is the one you take.
const TOWARDS: f32 = 0.8;

/// Walks the islander while WASD is held.
pub fn walk_keys(
    keys: Res<ButtonInput<KeyCode>>,
    console: Res<Console>,
    eye: Res<Eye>,
    mut sim: ResMut<Sim>,
    mut by_keys: Local<bool>,
) {
    if console.typing {
        return;
    }
    // Where the camera looks, along the ground: east and north.
    let forward = Vec2::new(-eye.yaw.sin(), eye.yaw.cos());
    let right = Vec2::new(forward.y, -forward.x);
    let mut heading = Vec2::ZERO;
    for (key, way) in [
        (KeyCode::KeyW, forward),
        (KeyCode::KeyS, -forward),
        (KeyCode::KeyD, right),
        (KeyCode::KeyA, -right),
    ] {
        if keys.pressed(key) {
            heading += way;
        }
    }
    let Play::Live(session) = &mut sim.play else {
        return;
    };
    if heading == Vec2::ZERO {
        *by_keys = false;
        return;
    }
    let heading = heading.normalize();
    // What they're doing now, and where the step would take them: a walk
    // within the place, or, at its edge, a way out that way.
    let (walking, line) = {
        let world = session.world();
        let me = session.player();
        let walking = match world.pending(me).map(|p| &p.intent) {
            None => None,
            Some(Intent::Walk { .. }) => Some(true),
            // On a path between places, or busy with something else.
            Some(_) => return,
        };
        let (Some(here), Some(spot)) = (world.place_of(me), world.spot(me)) else {
            return;
        };
        let middle = world.position(here).unwrap_or((0, 0));
        let to = Vec2::new(
            (spot.0 - middle.0) as f32 / 1e6,
            (spot.1 - middle.1) as f32 / 1e6,
        ) + heading * STEP;
        let size = world.size(here) as f32 / 1e6;
        let line = if to.length() > size {
            let way_out = world
                .known_exits(me, here)
                .into_iter()
                .filter_map(|exit| {
                    let at = world.position(exit)?;
                    let towards = Vec2::new((at.0 - middle.0) as f32, (at.1 - middle.1) as f32);
                    Some((exit, towards.normalize_or_zero().angle_to(heading).abs()))
                })
                .filter(|&(_, angle)| angle < TOWARDS)
                .min_by(|a, b| a.1.total_cmp(&b.1));
            match way_out {
                Some((exit, _)) => format!("go {}", engine::laws::pointer(exit)),
                None => return,
            }
        } else {
            format!("walk to {:.2} {:.2}", to.x, to.y)
        };
        (walking, line)
    };
    match walking {
        // A walk the keys started: let it finish, then the next step.
        Some(true) if *by_keys => return,
        // A walk someone typed or spoke: the keys take over from where
        // they've got to, stepping on from there next frame.
        Some(_) => {
            session.handle("stop");
            return;
        }
        None => {}
    }
    let reply = session.handle(&line);
    *by_keys = !reply.refused && line.starts_with("walk");
}

/// With the `reach` setting, a circle on the ground around the islander:
/// how far they can reach.
pub fn draw_reach(
    settings: Res<Settings>,
    sim: Res<Sim>,
    land: Res<crate::terrain::Land>,
    mut gizmos: Gizmos,
) {
    if !settings.reach {
        return;
    }
    let world = sim.world();
    let me = sim.me();
    let Some(reach) = world.reach(me) else {
        return;
    };
    let at = crate::draw::spot(world, &land, me, sim.now());
    let radius = reach as f32 / 1e6;
    let points = (0..=48).map(|i| {
        let angle = i as f32 / 48.0 * std::f32::consts::TAU;
        let p = Vec2::new(at.x + radius * angle.cos(), at.z + radius * angle.sin());
        Vec3::new(p.x, land.height(p) + 0.1, p.y)
    });
    gizmos.linestrip(points, Color::srgba(0.4, 0.9, 1.0, 0.9));
}
