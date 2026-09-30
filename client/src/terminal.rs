//! The console: translucent, at the bottom left, where most online games
//! keep their chat. An input line, and a log where replies, news, and grey
//! debugging messages scroll. Enter to type (and to send), Esc to stop, the
//! key left of 1 (`) to make it bigger, smaller, or hidden; up and down
//! bring back earlier commands. Commands go through the console's own
//! session, so every reply and refusal is the one the scripts prove.
//!
//! With `--script`, the client plays a script file through the console
//! instead, a line at a time, and saves a screenshot at each expectation.

use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use console::script::Step;

use crate::{Options, Play, Sim};

/// What a log line is, which sets its colour.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Said {
    /// What the player typed.
    Typed,
    Reply,
    Refused,
    /// What happened to them, told as it happens.
    News,
    /// For development: how the client itself is doing.
    Debug,
}

impl Said {
    fn colour(self) -> Color {
        match self {
            Said::Typed => Color::srgb(0.6, 0.8, 1.0),
            Said::Reply => Color::srgb(0.95, 0.95, 0.92),
            Said::Refused => Color::srgb(1.0, 0.65, 0.35),
            Said::News => Color::srgb(1.0, 0.88, 0.4),
            Said::Debug => Color::srgb(0.7, 0.7, 0.7),
        }
    }
}

/// How much of the log shows.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Size {
    Small,
    Large,
    Hidden,
}

#[derive(Resource)]
pub struct Console {
    /// Keys go to the input line, not the game.
    pub typing: bool,
    input: String,
    log: Vec<(Said, String)>,
    /// Bumped whenever the log changes, to redraw it.
    changed: bool,
    size: Size,
    history: Vec<String>,
    /// How far back in the history, while going through it.
    back: usize,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            typing: false,
            input: String::new(),
            log: Vec::new(),
            changed: true,
            size: Size::Small,
            history: Vec::new(),
            back: 0,
        }
    }
}

const KEPT: usize = 500;

impl Console {
    pub fn say(&mut self, said: Said, text: &str) {
        for line in text.lines() {
            self.log.push((said, line.to_string()));
        }
        if self.log.len() > KEPT {
            self.log.drain(..self.log.len() - KEPT);
        }
        self.changed = true;
    }
}

#[derive(Component)]
pub struct Panel;

#[derive(Component)]
pub struct LogLines;

#[derive(Component)]
pub struct InputLine;

pub fn setup(mut commands: Commands, mut console: ResMut<Console>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                bottom: Val::Px(12.0),
                width: Val::Percent(46.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            Panel,
        ))
        .with_children(|panel| {
            panel.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    overflow: Overflow::clip(),
                    ..default()
                },
                LogLines,
            ));
            panel.spawn((
                Text::new(""),
                TextFont {
                    font_size: bevy::text::FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Said::Typed.colour()),
                InputLine,
            ));
        });
    console.say(
        Said::Debug,
        "Enter to type a command, Esc to stop; \"help\" lists them. ` resizes this.",
    );
}

/// Sends a line to the session, as the player typing it.
fn send(console: &mut Console, sim: &mut Sim, line: &str, exit: &mut MessageWriter<AppExit>) {
    console.history.push(line.to_string());
    console.say(Said::Typed, &format!("> {line}"));
    match &mut sim.play {
        Play::Live(session) => {
            let reply = session.handle(line);
            let said = if reply.refused {
                Said::Refused
            } else {
                Said::Reply
            };
            console.say(said, &reply.text);
            if reply.quit {
                exit.write(AppExit::Success);
            }
        }
        Play::Script(_) => console.say(Said::Debug, "A script is playing; it takes no commands."),
    }
}

/// Takes keys while typing: letters into the input line, Enter to send.
/// First, anything `--type` asked for, as if typed.
pub fn type_in(
    mut keys: MessageReader<KeyboardInput>,
    options: Res<Options>,
    mut console: ResMut<Console>,
    mut sim: ResMut<Sim>,
    mut exit: MessageWriter<AppExit>,
    mut started: Local<bool>,
) {
    if !*started {
        *started = true;
        for line in options
            .typed
            .split(';')
            .map(str::trim)
            .filter(|l| !l.is_empty())
        {
            send(&mut console, &mut sim, line, &mut exit);
        }
    }
    for key in keys.read() {
        if key.state != ButtonState::Pressed {
            continue;
        }
        if !console.typing {
            match key.logical_key {
                Key::Enter => console.typing = true,
                Key::Character(ref c) if c.as_str() == "`" => {
                    console.size = match console.size {
                        Size::Small => Size::Large,
                        Size::Large => Size::Hidden,
                        Size::Hidden => Size::Small,
                    };
                    console.changed = true;
                }
                _ => {}
            }
            continue;
        }
        match &key.logical_key {
            Key::Escape => console.typing = false,
            Key::Enter => {
                let line = std::mem::take(&mut console.input);
                console.back = 0;
                if !line.trim().is_empty() {
                    send(&mut console, &mut sim, line.trim(), &mut exit);
                }
            }
            Key::Backspace => {
                console.input.pop();
            }
            Key::ArrowUp | Key::ArrowDown => {
                let n = console.history.len();
                console.back = if key.logical_key == Key::ArrowUp {
                    (console.back + 1).min(n)
                } else {
                    console.back.saturating_sub(1)
                };
                console.input = if console.back == 0 {
                    String::new()
                } else {
                    console.history[n - console.back].clone()
                };
            }
            Key::Space => console.input.push(' '),
            Key::Character(c) => {
                console
                    .input
                    .extend(c.chars().filter(|ch| !ch.is_control()));
            }
            _ => {}
        }
    }
}

/// Redraws the log when it changes, and the input line always.
pub fn show(
    mut commands: Commands,
    time: Res<Time>,
    mut console: ResMut<Console>,
    log: Query<Entity, With<LogLines>>,
    mut panel: Query<(&mut Node, &mut Visibility), With<Panel>>,
    mut input: Query<&mut Text, With<InputLine>>,
) {
    if let Ok(mut text) = input.single_mut() {
        let caret = if ((time.elapsed_secs() * 2.0) as u64).is_multiple_of(2) {
            "_"
        } else {
            " "
        };
        text.0 = if console.typing {
            format!("> {}{caret}", console.input)
        } else {
            String::new()
        };
    }
    if let Ok((mut node, mut visible)) = panel.single_mut() {
        *visible = if console.size == Size::Hidden && !console.typing {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        node.width = Val::Percent(if console.size == Size::Large {
            60.0
        } else {
            46.0
        });
    }
    if !console.changed {
        return;
    }
    console.changed = false;
    let Ok(log) = log.single() else { return };
    let shown = match console.size {
        Size::Small => 9,
        Size::Large => 34,
        Size::Hidden => 0,
    };
    let from = console.log.len().saturating_sub(shown);
    commands.entity(log).despawn_children();
    commands.entity(log).with_children(|lines| {
        for (said, line) in &console.log[from..] {
            lines.spawn((
                Text::new(line.clone()),
                TextFont {
                    font_size: bevy::text::FontSize::Px(15.0),
                    ..default()
                },
                TextColor(said.colour()),
            ));
        }
    });
}

/// Plays a script a line at a time, showing each command and reply in the
/// console, and asking for a screenshot at each expectation. When it's
/// over, the client exits: successfully if every line passed.
pub fn play_script(
    time: Res<Time>,
    options: Res<Options>,
    mut sim: ResMut<Sim>,
    mut console: ResMut<Console>,
    mut shots: ResMut<Shots>,
    mut next: Local<f32>,
) {
    let Play::Script(playing) = &mut sim.play else {
        return;
    };
    let now = time.elapsed_secs();
    // Give the scene a moment to settle before the first line.
    if *next == 0.0 {
        *next = now + 1.5;
    }
    if now < *next || shots.exit.is_some() {
        return;
    }
    match playing.step() {
        Some(Ok(Step::Command {
            command,
            reply,
            refused,
            ..
        })) => {
            console.say(Said::Typed, &format!("> {command}"));
            console.say(if refused { Said::Refused } else { Said::Reply }, &reply);
            *next = now + options.step;
        }
        Some(Ok(Step::Expected { line, expectation })) => {
            console.say(Said::Debug, &format!("expected {expectation}: yes"));
            shots.waiting.push(line);
            shots.taken += 1;
            *next = now + 0.8;
        }
        Some(Err(why)) => {
            console.say(Said::Refused, &format!("The script failed: {why}"));
            println!("FAILED: {why}");
            shots.waiting.push(9999);
            shots.exit = Some((now + 3.0, false));
        }
        None => {
            let commands = playing.transcript().len();
            println!(
                "PASSED: {commands} commands, {} expectations, each with a screenshot",
                shots.taken
            );
            console.say(Said::Debug, "The script is over, and passed.");
            shots.exit = Some((now + 2.0, true));
        }
    }
}

/// Screenshots a script asked for, waiting for the frame to show them, and
/// when to exit, and whether all went well.
#[derive(Resource, Default)]
pub struct Shots {
    waiting: Vec<usize>,
    taken: usize,
    exit: Option<(f32, bool)>,
}

/// Takes the screenshots a script asked for, once the frame shows the line
/// they're for, and exits when the script is over.
pub fn take_shots(
    mut commands: Commands,
    time: Res<Time>,
    options: Res<Options>,
    mut shots: ResMut<Shots>,
    mut frames: Local<u32>,
    mut exit: MessageWriter<AppExit>,
) {
    if !shots.waiting.is_empty() {
        // A few frames for the log to be laid out and drawn.
        *frames += 1;
        if *frames < 4 {
            return;
        }
        *frames = 0;
        let name = options
            .script
            .as_deref()
            .and_then(|s| std::path::Path::new(s).file_stem())
            .map_or("script".into(), |s| s.to_string_lossy().to_string());
        for line in shots.waiting.drain(..) {
            let path = format!("{}/{name}-{line:04}.png", options.shots);
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));
        }
    }
    match shots.exit {
        Some((at, passed)) if time.elapsed_secs() >= at => {
            exit.write(if passed {
                AppExit::Success
            } else {
                AppExit::from_code(1)
            });
        }
        _ => {}
    }
}
