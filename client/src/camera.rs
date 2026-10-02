//! The islander's camera: above and behind them, following wherever they
//! go. The mouse turns it around them (hold the right button and drag), as
//! do the arrow keys, and the angle stays while they walk; the wheel brings
//! it closer or further. F breaks out into free flying, a developer's tool,
//! with the arrows (WASD still walks the islander), and F again snaps back.
//! Pointing at something names it, in the islander's words.

use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraState};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::draw::{Named, spot};
use crate::terminal::Console;
use crate::terrain::Land;
use crate::{Options, Sim};

/// Where the camera looks from, around the islander.
#[derive(Resource)]
pub struct Eye {
    /// Around them, in radians, and how far above the level, looking down.
    pub yaw: f32,
    pub pitch: f32,
    /// How far from them, in metres.
    pub distance: f32,
    /// The point it looks at: just above the islander, following smoothly.
    pub target: Option<Vec3>,
    pub flying: bool,
}

/// The name shown by the pointer.
#[derive(Component)]
pub struct Pointer;

/// Head height, where the camera looks.
const HEAD: f32 = 1.5;
const NEAREST: f32 = 2.0;
const FURTHEST: f32 = 800.0;

pub fn setup(mut commands: Commands, options: Res<Options>, sim: Res<Sim>, land: Res<Land>) {
    let flying = options.from.is_some();
    // Behind the islander as seen from the middle of the land: looking
    // inland, unless told otherwise.
    let world = sim.world();
    let me = spot(world, &land, sim.me(), sim.now());
    let middle = land.middle();
    let away = Vec2::new(me.x - middle.x, me.z - middle.y);
    let yaw = options
        .yaw
        .map_or(away.x.atan2(away.y), |d: f32| d.to_radians());
    commands.insert_resource(Eye {
        yaw,
        pitch: options.pitch.map_or(0.35, |d: f32| d.to_radians()),
        distance: options.zoom.unwrap_or(12.0).clamp(NEAREST, FURTHEST),
        target: None,
        flying,
    });
    let from = options.from.unwrap_or(me + Vec3::new(0.0, 10.0, 10.0));
    let look = options.look.unwrap_or(me);
    let mut state = FreeCameraState::default();
    state.enabled = flying;
    commands.spawn((
        Camera3d::default(),
        // Flames glow past their edges.
        bevy::post_process::bloom::Bloom::NATURAL,
        Transform::from_translation(from).looking_at(look, Vec3::Y),
        DistanceFog {
            color: Color::srgb(0.62, 0.74, 0.88),
            falloff: FogFalloff::Linear {
                start: 3_000.0,
                end: 40_000.0,
            },
            ..default()
        },
        FreeCamera {
            walk_speed: 20.0,
            run_speed: 200.0,
            // The arrows fly the camera; WASD still walks the islander.
            key_forward: KeyCode::ArrowUp,
            key_back: KeyCode::ArrowDown,
            key_left: KeyCode::ArrowLeft,
            key_right: KeyCode::ArrowRight,
            ..default()
        },
        state,
    ));
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: bevy::text::FontSize::Px(16.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.97, 0.85)),
        Node {
            position_type: PositionType::Absolute,
            padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        Pointer,
    ));
}

#[allow(clippy::too_many_arguments)]
pub fn follow(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    console: Res<Console>,
    sim: Res<Sim>,
    land: Res<Land>,
    mut eye: ResMut<Eye>,
    mut camera: Query<(&mut Transform, &mut FreeCameraState), With<Camera3d>>,
) {
    let Ok((mut transform, mut free)) = camera.single_mut() else {
        return;
    };
    if keys.just_pressed(KeyCode::KeyF) && !console.typing {
        eye.flying = !eye.flying;
    }
    if eye.flying && !free.enabled && !console.typing {
        // Fly off from where the camera is, by key or by /fly.
        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        free.yaw = yaw;
        free.pitch = pitch;
    }
    // No flying about while typing.
    free.enabled = eye.flying && !console.typing;
    if eye.flying {
        return;
    }

    if buttons.pressed(MouseButton::Right) {
        eye.yaw -= motion.delta.x * 0.005;
        eye.pitch = (eye.pitch + motion.delta.y * 0.005).clamp(-0.3, 1.45);
    }
    // The arrows turn the camera around the islander too.
    if !console.typing {
        let turn = time.delta_secs() * 1.6;
        let axis = |plus: KeyCode, minus: KeyCode| {
            f32::from(u8::from(keys.pressed(plus))) - f32::from(u8::from(keys.pressed(minus)))
        };
        eye.yaw += axis(KeyCode::ArrowLeft, KeyCode::ArrowRight) * turn;
        eye.pitch =
            (eye.pitch + axis(KeyCode::ArrowUp, KeyCode::ArrowDown) * turn * 0.6).clamp(-0.3, 1.45);
    }
    let lines = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / MouseScrollUnit::SCROLL_UNIT_CONVERSION_FACTOR,
    };
    eye.distance = (eye.distance * (-lines * 0.15).exp()).clamp(NEAREST, FURTHEST);

    let world = sim.world();
    let me = spot(world, &land, sim.me(), sim.now()) + Vec3::Y * HEAD;
    // Follow smoothly, but jump when they've moved far at once.
    let target = match eye.target {
        Some(t) if t.distance(me) < 50.0 => t.lerp(me, 1.0 - (-time.delta_secs() * 8.0).exp()),
        _ => me,
    };
    eye.target = Some(target);
    let round = Vec3::new(
        eye.yaw.sin() * eye.pitch.cos(),
        eye.pitch.sin(),
        eye.yaw.cos() * eye.pitch.cos(),
    );
    let mut at = target + round * eye.distance;
    // Never under the ground.
    at.y = at.y.max(land.height(Vec2::new(at.x, at.z)) + 0.5);
    *transform = Transform::from_translation(at).looking_at(target, Vec3::Y);
}

/// The drawn thing nearest the pointer on screen, in front, within a
/// small reach of it.
pub fn pointed_at(
    cursor: Vec2,
    camera: &Camera,
    eye: &GlobalTransform,
    named: &Query<(&Named, &GlobalTransform)>,
) -> Option<engine::world::EntityId> {
    let mut best: Option<(f32, Named)> = None;
    for (&thing, at) in named {
        let Ok(screen) = camera.world_to_viewport_with_depth(eye, at.translation()) else {
            continue;
        };
        let d = screen.truncate().distance(cursor);
        if screen.z > 0.0 && d < 28.0 && best.is_none_or(|(b, _)| d < b) {
            best = Some((d, thing));
        }
    }
    best.map(|(_, Named(id))| id)
}

/// Names what the pointer is on: the nearest drawn thing to it on screen.
pub fn point(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    named: Query<(&Named, &GlobalTransform)>,
    sim: Res<Sim>,
    menu: Query<(), With<crate::menu::Menu>>,
    mut pointer: Query<(&mut Text, &mut Node, &mut Visibility), With<Pointer>>,
) {
    let Ok((mut text, mut node, mut visible)) = pointer.single_mut() else {
        return;
    };
    text.0.clear();
    *visible = Visibility::Hidden;
    let (Ok(window), Ok((camera, eye))) = (window.single(), camera.single()) else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    // An open menu names the thing already.
    if !menu.is_empty() {
        return;
    }
    if let Some(id) = pointed_at(cursor, camera, eye, &named) {
        let world = sim.world();
        text.0 = if id == sim.me() {
            "you".into()
        } else {
            engine::sight::label(world, sim.me(), id)
        };
        *visible = Visibility::Inherited;
        node.left = Val::Px(cursor.x + 16.0);
        node.top = Val::Px(cursor.y - 8.0);
    }
}
