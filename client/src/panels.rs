//! The backpack and the body: two small windows on the right. B opens and
//! closes the backpack: what the islander carries, a line each, with masses.
//! V opens and closes the body: its vitals, as the engine measures them;
//! click it to see everything measured, and again for just the vitals. Both
//! are the console's own `backpack` and `body`, so they show the same
//! numbers as `datasheet me`.

use bevy::prelude::*;

use crate::terminal::Console;
use crate::{Options, Sim};

#[derive(Resource)]
pub struct Panels {
    pub backpack: bool,
    pub body: bool,
    /// Everything measured, not just the vitals.
    pub all: bool,
}

#[derive(Component)]
pub struct Backpack;

#[derive(Component)]
pub struct Body;

const WIDTH: f32 = 380.0;

fn panel(top: f32) -> (Node, BackgroundColor) {
    (
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(12.0),
            top: Val::Px(top),
            width: Val::Px(WIDTH),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            row_gap: Val::Px(2.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
    )
}

pub fn setup(mut commands: Commands, options: Res<Options>) {
    let open = |name: &str| options.open.split(',').any(|o| o.trim() == name);
    commands.insert_resource(Panels {
        backpack: open("backpack"),
        body: open("body") || open("body-all"),
        all: open("body-all"),
    });
    commands.spawn((panel(12.0), Backpack));
    // Clicking the body shows everything measured.
    commands.spawn((panel(0.0), Button, Body));
}

/// B and V open and close them; a click on the body expands it.
pub fn keys(
    keys: Res<ButtonInput<KeyCode>>,
    console: Res<Console>,
    mut panels: ResMut<Panels>,
    clicked: Query<&Interaction, (Changed<Interaction>, With<Body>)>,
) {
    if !console.typing {
        if keys.just_pressed(KeyCode::KeyB) {
            panels.backpack = !panels.backpack;
        }
        if keys.just_pressed(KeyCode::KeyV) {
            panels.body = !panels.body;
        }
    }
    for interaction in &clicked {
        if *interaction == Interaction::Pressed {
            panels.all = !panels.all;
        }
    }
}

fn line(text: String, colour: Color, size: f32) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: bevy::text::FontSize::Px(size),
            ..default()
        },
        TextColor(colour),
    )
}

/// Fills both from the world, as it is now.
#[allow(clippy::type_complexity)]
pub fn show(
    mut commands: Commands,
    sim: Res<Sim>,
    panels: Res<Panels>,
    mut backpack: Query<(Entity, &mut Visibility), (With<Backpack>, Without<Body>)>,
    mut body: Query<(Entity, &mut Visibility, &mut Node), (With<Body>, Without<Backpack>)>,
    computed: Query<&ComputedNode, With<Backpack>>,
    mut last: Local<Vec<String>>,
) {
    let (Ok((pack, mut pack_shown)), Ok((body, mut body_shown, mut body_node))) =
        (backpack.single_mut(), body.single_mut())
    else {
        return;
    };
    let world = sim.world();
    let me = sim.me();
    let pack_lines = console::session::backpack(world, me);
    let body_lines = console::session::body(world, me, panels.all);
    // The body sits under the backpack when both are open.
    let below = if panels.backpack {
        computed
            .single()
            .map_or(0.0, |c| c.size().y * c.inverse_scale_factor())
            + 12.0
    } else {
        0.0
    };
    body_node.top = Val::Px(12.0 + below);
    *pack_shown = if panels.backpack {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    *body_shown = if panels.body {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    let mut now = pack_lines.clone();
    now.push(String::new());
    now.extend(body_lines.iter().cloned());
    now.push(panels.all.to_string());
    if *last == now {
        return;
    }
    *last = now;
    let title = Color::srgb(1.0, 0.88, 0.4);
    let text = Color::srgb(0.95, 0.95, 0.92);
    let quiet = Color::srgb(0.7, 0.7, 0.7);
    commands.entity(pack).despawn_children();
    commands.entity(pack).with_children(|p| {
        p.spawn(line("Backpack (B)".into(), title, 16.0));
        let n = pack_lines.len();
        for (i, l) in pack_lines.into_iter().enumerate() {
            p.spawn(line(l, if i + 1 == n { quiet } else { text }, 15.0));
        }
    });
    commands.entity(body).despawn_children();
    commands.entity(body).with_children(|p| {
        let more = if panels.all {
            "click for less"
        } else {
            "click for all"
        };
        p.spawn(line(format!("Body (V), {more}"), title, 16.0));
        for l in body_lines {
            p.spawn(line(l, text, 15.0));
        }
    });
}
