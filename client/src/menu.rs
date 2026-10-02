//! Clicking on things. A left click on something drawn opens a small menu
//! beside it of what the islander could do with it now: the laws' own
//! choices (`console::menu`), checked as if done, so only what would work is
//! offered; something out of reach is walked up to first. Choosing one sends
//! its command to the console, as if typed, and the laws decide again. A
//! left click on open ground walks there. Esc, or a click elsewhere, closes
//! the menu. The console is for processes; clicking is for things. See
//! "Clicking on things" in docs/requirements.md.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::pointed_at;
use crate::draw::Named;
use crate::terminal::{Console, send_as};
use crate::terrain::Land;
use crate::{Play, Sim};

/// The open menu.
#[derive(Component)]
pub struct Menu;

/// One of its choices: the command it sends, and how it reads.
#[derive(Component)]
pub struct Choice {
    line: String,
    said: String,
}

const IDLE: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);
const LIT: Color = Color::srgba(0.4, 0.6, 0.9, 0.45);

#[allow(clippy::too_many_arguments)]
pub fn click(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    named: Query<(&Named, &GlobalTransform)>,
    land: Res<Land>,
    mut choices: Query<(&Interaction, &Choice, &mut BackgroundColor)>,
    ui: Query<&Interaction, Without<Choice>>,
    open: Query<Entity, With<Menu>>,
    mut console: ResMut<Console>,
    mut sim: ResMut<Sim>,
    mut exit: MessageWriter<AppExit>,
    build: Res<crate::build::Build>,
) {
    // In build mode, clicks move things instead.
    if build.on {
        return;
    }
    let close = |commands: &mut Commands| {
        for menu in &open {
            commands.entity(menu).despawn();
        }
    };
    // A choice clicked: send it.
    let mut chosen = None;
    for (interaction, choice, mut background) in &mut choices {
        match interaction {
            Interaction::Pressed => chosen = Some((choice.line.clone(), choice.said.clone())),
            Interaction::Hovered => background.0 = LIT,
            Interaction::None => background.0 = IDLE,
        }
    }
    if let Some((line, said)) = chosen {
        close(&mut commands);
        send_as(&mut console, &mut sim, &line, &said, &mut exit);
        return;
    }
    if keys.just_pressed(KeyCode::Escape) && !console.typing {
        close(&mut commands);
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    close(&mut commands);
    // On the console or a window, not the world.
    if ui.iter().any(|i| *i != Interaction::None) {
        return;
    }
    let (Ok(window), Ok((camera, eye))) = (window.single(), camera.single()) else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Play::Live(session) = &sim.play else {
        return;
    };
    let (world, me) = (session.world(), session.player());
    match pointed_at(cursor, camera, eye, &named).filter(|&id| id != me) {
        Some(thing) => {
            let offered = console::menu::menu(world, me, thing);
            let name = engine::sight::label(world, me, thing);
            open_menu(&mut commands, window.size(), cursor, &name, offered);
        }
        None => {
            // Open ground: walk there, within this place.
            let Ok(ray) = camera.viewport_to_world(eye, cursor) else {
                return;
            };
            let Some(at) = ground(ray, &land) else {
                return;
            };
            let Some(here) = world.place_of(me) else {
                return;
            };
            let middle = world.position(here).unwrap_or((0, 0));
            let east = at.x - middle.0 as f32 / 1e6;
            let north = -at.z - middle.1 as f32 / 1e6;
            let line = format!("walk to {east:.2} {north:.2}");
            send_as(&mut console, &mut sim, &line, "walk there", &mut exit);
        }
    }
}

/// Where a ray from the camera meets the ground, within a few hundred metres.
pub(crate) fn ground(ray: Ray3d, land: &Land) -> Option<Vec3> {
    (1..=1_000)
        .map(|i| ray.get_point(i as f32 * 0.5))
        .find(|p| p.y <= land.height(Vec2::new(p.x, p.z)))
}

fn open_menu(
    commands: &mut Commands,
    window: Vec2,
    cursor: Vec2,
    name: &str,
    offered: Vec<console::menu::Choice>,
) {
    let text = |text: String, colour: Color| {
        (
            Text::new(text),
            TextFont {
                font_size: bevy::text::FontSize::Px(15.0),
                ..default()
            },
            TextColor(colour),
        )
    };
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                // Beside the pointer, towards the middle of the window, so
                // it never runs off the edge.
                left: if cursor.x < window.x / 2.0 {
                    Val::Px(cursor.x + 12.0)
                } else {
                    Val::Auto
                },
                right: if cursor.x < window.x / 2.0 {
                    Val::Auto
                } else {
                    Val::Px(window.x - cursor.x + 12.0)
                },
                top: if cursor.y < window.y / 2.0 {
                    Val::Px(cursor.y)
                } else {
                    Val::Auto
                },
                bottom: if cursor.y < window.y / 2.0 {
                    Val::Auto
                } else {
                    Val::Px(window.y - cursor.y)
                },
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(6.0)),
                row_gap: Val::Px(1.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            GlobalZIndex(10),
            Interaction::default(),
            Menu,
        ))
        .with_children(|menu| {
            menu.spawn(text(name.to_string(), Color::srgb(1.0, 0.88, 0.4)));
            if offered.is_empty() {
                menu.spawn(text(
                    "nothing you can do with it now".into(),
                    Color::srgb(0.7, 0.7, 0.7),
                ));
            }
            for choice in offered {
                menu.spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(IDLE),
                    Choice {
                        line: choice.line,
                        said: choice.said,
                    },
                ))
                .with_child(text(choice.label, Color::srgb(0.95, 0.95, 0.92)));
            }
        });
}
