//! A debugging grid over the ground, G to show and hide it: fine lines every
//! 5 m and bright ones every 50 m, fixed to the world and draped over the
//! land, so movement shows on ground with nothing on it. It covers the
//! ground around whatever the camera is looking at.

use bevy::prelude::*;

use crate::camera::Eye;
use crate::terminal::Console;
use crate::terrain::Land;

/// Fine lines this far apart, in metres; every tenth is bright.
const FINE: f32 = 5.0;
const BRIGHT_EVERY: i32 = 10;
/// How far out from the middle the grid reaches.
const REACH: f32 = 150.0;
/// How often a line bends to follow the ground.
const STEP: f32 = 2.5;
/// Just above the ground: the drawn land is coarser than its shape, and
/// would hide lines laid exactly on it.
const LIFT: f32 = 0.3;

#[derive(Resource)]
pub struct Grid {
    pub shown: bool,
}

pub fn setup(
    mut commands: Commands,
    options: Res<crate::Options>,
    mut store: ResMut<GizmoConfigStore>,
) {
    commands.insert_resource(Grid {
        shown: options.open.split(',').any(|o| o.trim() == "grid"),
    });
    let (config, _) = store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 1.5;
    // Drawn a little in front of what's at the same depth: lines on the
    // ground stay visible on it.
    config.depth_bias = -0.02;
}

pub fn toggle(keys: Res<ButtonInput<KeyCode>>, console: Res<Console>, mut grid: ResMut<Grid>) {
    if !console.typing && keys.just_pressed(KeyCode::KeyG) {
        grid.shown = !grid.shown;
    }
}

pub fn draw(
    grid: Res<Grid>,
    eye: Res<Eye>,
    land: Res<Land>,
    camera: Query<&Transform, With<Camera3d>>,
    mut gizmos: Gizmos,
) {
    if !grid.shown {
        return;
    }
    // Around the islander, or, flying, the ground under the camera.
    let middle = match (eye.flying, eye.target, camera.single()) {
        (false, Some(target), _) => target,
        (_, _, Ok(camera)) => camera.translation,
        _ => return,
    };
    // Dark fine lines show on sand and grass alike; the 50 m lines stand
    // out in yellow.
    let fine = Color::srgba(0.0, 0.0, 0.0, 0.45);
    let bright = Color::srgba(1.0, 0.8, 0.1, 0.95);
    let lines = (REACH / FINE) as i32;
    let (cx, cz) = (
        (middle.x / FINE).round() as i32,
        (middle.z / FINE).round() as i32,
    );
    let ground = |x: f32, z: f32| Vec3::new(x, land.height(Vec2::new(x, z)) + LIFT, z);
    let points = (2.0 * REACH / STEP) as i32;
    for n in -lines..=lines {
        for along_x in [true, false] {
            let index = if along_x { cz + n } else { cx + n };
            let colour = if index.rem_euclid(BRIGHT_EVERY) == 0 {
                bright
            } else {
                fine
            };
            let fixed = index as f32 * FINE;
            let start = if along_x { cx } else { cz } as f32 * FINE - REACH;
            gizmos.linestrip(
                (0..=points).map(|i| {
                    let moving = start + i as f32 * STEP;
                    if along_x {
                        ground(moving, fixed)
                    } else {
                        ground(fixed, moving)
                    }
                }),
                colour,
            );
        }
    }
}
