//! The console: translucent, along the bottom of the window. An input
//! line, and a log where replies, news, and grey debugging messages scroll.
//! It works as games' chat lines and consoles do (researched 2026-10-06):
//! Enter to type (or `/`, which starts a tool), Enter again to send (or Esc
//! to stop), and the keys go back to the game; while typing, the keys are
//! the line's: Left, Right, Home, End, Delete, Ctrl+Backspace, Ctrl+V to
//! paste, Up and Down for earlier commands, and Tab to complete a command,
//! a tool, or the name of something the islander can make out (`line.rs`).
//! The mouse wheel over the console, or Page Up and Page Down, scroll back
//! through the log; sending, or End when not typing, comes back to the
//! newest. The key left of 1 (`) makes it bigger, smaller, or hidden.
//! Commands go through the console's own session, so every reply and
//! refusal is the one the scripts prove.
//!
//! With `--script`, the client plays a script file through the console
//! instead, a line at a time, and saves a screenshot at each expectation.

use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use console::script::Step;

use crate::line::{self, Line, Slot};
use crate::{Options, Play, Sim};

/// What a log line is, which sets its colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    input: Line,
    log: Vec<(Said, String)>,
    /// How many lines back from the newest the log is scrolled.
    scroll: usize,
    /// The pointer is over the console: the wheel scrolls it, not the
    /// camera.
    pub over: bool,
    /// Bumped whenever the log changes, to redraw it.
    changed: bool,
    size: Size,
    history: Vec<String>,
    /// How far back in the history, while going through it.
    back: usize,
    /// Tools typed or spoken, for `tools::run_typed`.
    pub tools: Vec<String>,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            typing: false,
            input: Line::default(),
            log: Vec::new(),
            scroll: 0,
            over: false,
            changed: true,
            size: Size::Small,
            history: Vec::new(),
            back: 0,
            tools: Vec::new(),
        }
    }
}

/// How many lines the log keeps.
const KEPT: usize = 1_000;

impl Console {
    pub fn say(&mut self, said: Said, text: &str) {
        let before = self.log.len();
        for line in text.lines() {
            self.log.push((said, line.to_string()));
        }
        // Scrolled back, what's being read stays put as lines arrive.
        if self.scroll > 0 {
            self.scroll += self.log.len() - before;
        }
        if self.log.len() > KEPT {
            self.log.drain(..self.log.len() - KEPT);
        }
        self.changed = true;
    }

    /// How many lines show at the console's size.
    fn shown(&self) -> usize {
        match self.size {
            Size::Small => 9,
            Size::Large => 34,
            Size::Hidden => 0,
        }
    }

    /// A reply longer than the console shows from its top, with the command
    /// that asked for it, and "newer lines below" under it: read on by
    /// scrolling, as a pager would, without one.
    fn show_from_top(&mut self, lines: usize) {
        let shown = self.shown();
        if shown > 0 && lines + 1 > shown {
            self.scroll = lines + 2 - shown;
            self.scroll_by(0);
        }
    }

    /// Scrolls back (up) or on (down) by `lines`, within the log.
    fn scroll_by(&mut self, lines: isize) {
        // Back as far as the oldest line, with the "newer lines below" line
        // taking the last place.
        let most = self.log.len().saturating_sub(self.shown().max(1) - 1);
        self.scroll = (self.scroll as isize + lines).clamp(0, most as isize) as usize;
        self.changed = true;
    }
}

#[derive(Component)]
pub struct Panel;

#[derive(Component)]
pub struct LogLines;

#[derive(Component)]
pub struct InputLine;

/// The line saying newer lines are below, which jumps to them when clicked.
#[derive(Component)]
pub struct Newer;

/// A click on "newer lines below" comes back to the newest.
pub fn jump(
    mut console: ResMut<Console>,
    clicked: Query<&Interaction, (Changed<Interaction>, With<Newer>)>,
) {
    if clicked.iter().any(|i| *i == Interaction::Pressed) {
        console.scroll = 0;
        console.changed = true;
    }
}

pub fn setup(mut commands: Commands, mut console: ResMut<Console>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                bottom: Val::Px(12.0),
                // The whole width of the window.
                right: Val::Px(12.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            Panel,
            // A click on the console isn't a click on the world.
            Interaction::default(),
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
        "Keys: WASD walk; right- or middle-drag, or the arrows, to look; the wheel to zoom; \
         F fly; B backpack; V body; T speak; G grid; M build mode; Space pause; [ ] slower \
         or faster.",
    );
    console.say(
        Said::Debug,
        "Enter to type a command, Enter to send it; \"help\" lists them. / starts a tool. Tab \
         completes. The wheel over this, or Page Up and Down, scrolls back. Hold T to speak a \
         command. ` resizes this.",
    );
}

/// Sends a line to the session, as the player typing it.
pub fn send(console: &mut Console, sim: &mut Sim, line: &str, exit: &mut MessageWriter<AppExit>) {
    send_as(console, sim, line, line, exit);
}

/// What Tab can complete, for the word being typed: commands (from the
/// console's own help) and tools first; after a tool, its values; after a
/// command, the names of what the islander can make out.
fn completions(sim: &Sim, slot: Slot) -> Vec<String> {
    match slot {
        Slot::First => {
            let mut words: Vec<String> = console::help::commands()
                .iter()
                .map(|c| c.name.clone())
                .chain(["help".to_string()])
                .chain(crate::tools::names())
                .collect();
            words.sort();
            words.dedup();
            words
        }
        // What there's help on: every command, topic, and tool.
        Slot::After("help" | "?") => {
            let mut words: Vec<String> = console::help::names()
                .into_iter()
                .map(str::to_string)
                .chain(crate::tools::names())
                .collect();
            words.sort();
            words.dedup();
            words
        }
        Slot::After(tool) if tool.starts_with('/') => crate::tools::values(tool),
        Slot::After(_) => match &sim.play {
            Play::Live(session) => session.names(),
            Play::Script(_) => Vec::new(),
        },
    }
}

/// Sends a line to the session, shown in the log as `shown`: a command
/// chosen from a menu, pointing at things, shown in the player's words.
pub fn send_as(
    console: &mut Console,
    sim: &mut Sim,
    line: &str,
    shown: &str,
    exit: &mut MessageWriter<AppExit>,
) {
    console.history.push(line.to_string());
    // Back to the newest, to see the reply.
    console.scroll = 0;
    console.say(Said::Typed, &format!("> {shown}"));
    // Tools, for the person at the keyboard, not the actor.
    if line.starts_with('/') {
        console.tools.push(line.to_string());
        return;
    }
    // Help on the game's own settings, which the session doesn't know.
    if let Some(text) = crate::tools::help_on(line) {
        console.say(Said::Reply, &text);
        console.show_from_top(text.lines().count());
        return;
    }
    match &mut sim.play {
        Play::Live(session) => {
            let mut reply = session.handle(line);
            let said = if reply.refused {
                Said::Refused
            } else {
                Said::Reply
            };
            if line.trim() == "help tools" {
                reply.text = format!("{}\n{}", reply.text, crate::tools::listing());
            }
            console.say(said, &reply.text);
            console.show_from_top(reply.text.lines().count());
            if reply.quit {
                exit.write(AppExit::Success);
            }
        }
        Play::Script(_) => console.say(Said::Debug, "A script is playing; it takes no commands."),
    }
}

/// Takes keys while typing: letters into the input line, Enter to send.
/// First, anything `--type` asked for, as if typed.
#[allow(clippy::too_many_arguments)]
pub fn type_in(
    mut keys: MessageReader<KeyboardInput>,
    held: Res<ButtonInput<KeyCode>>,
    options: Res<Options>,
    mut console: ResMut<Console>,
    mut sim: ResMut<Sim>,
    mut clipboard: Option<ResMut<bevy::clipboard::Clipboard>>,
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
    let ctrl = held.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let page = console.shown().saturating_sub(1).max(1) as isize;
    for key in keys.read() {
        if key.state != ButtonState::Pressed {
            continue;
        }
        // Reading back through the log, typing or not: Ctrl+End to the
        // newest, Ctrl+Home to the oldest.
        match key.logical_key {
            Key::End if ctrl => {
                console.scroll = 0;
                console.changed = true;
                continue;
            }
            Key::Home if ctrl => {
                console.scroll_by(isize::MAX / 2);
                continue;
            }
            Key::PageUp => {
                console.scroll_by(page);
                continue;
            }
            Key::PageDown => {
                console.scroll_by(-page);
                continue;
            }
            _ => {}
        }
        if !console.typing {
            match &key.logical_key {
                Key::Enter => console.typing = true,
                // A tool, straight away.
                Key::Character(c) if c.as_str() == "/" => {
                    console.typing = true;
                    console.input.set("/");
                }
                Key::End => {
                    console.scroll = 0;
                    console.changed = true;
                }
                Key::Character(c) if c.as_str() == "`" => {
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
            // Closed without sending, the line thrown away, as games do.
            Key::Escape => {
                console.typing = false;
                console.input.take();
                console.back = 0;
            }
            Key::Enter => {
                // Sent, and the keys go back to the game: [ and ] work at once.
                console.typing = false;
                let line = console.input.take();
                console.back = 0;
                if !line.trim().is_empty() {
                    send(&mut console, &mut sim, line.trim(), &mut exit);
                }
            }
            Key::Backspace if ctrl => console.input.delete_word(),
            Key::Backspace => console.input.backspace(),
            Key::Delete => console.input.delete(),
            Key::ArrowLeft => console.input.left(),
            Key::ArrowRight => console.input.right(),
            Key::Home => console.input.home(),
            Key::End => console.input.end(),
            Key::Tab => {
                let Console { input, .. } = &mut *console;
                let listed = line::complete(input, |slot| completions(&sim, slot));
                if !listed.is_empty() {
                    console.say(Said::Debug, &listed.join(", "));
                }
            }
            Key::ArrowUp | Key::ArrowDown => {
                let n = console.history.len();
                console.back = if key.logical_key == Key::ArrowUp {
                    (console.back + 1).min(n)
                } else {
                    console.back.saturating_sub(1)
                };
                let earlier = if console.back == 0 {
                    String::new()
                } else {
                    console.history[n - console.back].clone()
                };
                console.input.set(&earlier);
            }
            Key::Character(c) if ctrl => {
                if c.eq_ignore_ascii_case("v")
                    && let Some(clipboard) = clipboard.as_deref_mut()
                    && let Some(Ok(text)) = clipboard.fetch_text().poll_result()
                {
                    console.input.insert(&text);
                }
            }
            Key::Space => console.input.insert(" "),
            Key::Character(c) => console.input.insert(c),
            _ => {}
        }
    }
}

/// The mouse wheel over the console scrolls its log (the camera's zoom
/// leaves it alone then: `Console::over`).
pub fn wheel(
    mut console: ResMut<Console>,
    panel: Query<(&Interaction, &Visibility), With<Panel>>,
    scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    console.over = panel
        .single()
        .is_ok_and(|(i, v)| *i != Interaction::None && *v != Visibility::Hidden);
    if !console.over || scroll.delta.y == 0.0 {
        return;
    }
    let notches = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / MouseScrollUnit::SCROLL_UNIT_CONVERSION_FACTOR,
    };
    // Three lines a notch; up is back.
    console.scroll_by((notches * 3.0).round() as isize);
}

/// Redraws the log when it changes, and the input line always.
pub fn show(
    mut commands: Commands,
    time: Res<Time>,
    mut console: ResMut<Console>,
    log: Query<Entity, With<LogLines>>,
    mut panel: Query<(&mut Node, &mut Visibility), With<Panel>>,
    mut input: Query<&mut Text, With<InputLine>>,
    voice: Res<crate::voice::Voice>,
) {
    if let Ok(mut text) = input.single_mut() {
        let caret = if ((time.elapsed_secs() * 2.0) as u64).is_multiple_of(2) {
            "_"
        } else {
            " "
        };
        text.0 = if console.typing {
            let caret = if caret == "_" { '|' } else { ' ' };
            format!("> {}", console.input.shown(caret))
        } else if voice.listening() {
            format!("(listening{caret})")
        } else if voice.busy {
            "(hearing...)".into()
        } else {
            String::new()
        };
    }
    if let Ok((_, mut visible)) = panel.single_mut() {
        *visible = if console.size == Size::Hidden && !console.typing && !voice.listening() {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
    if !console.changed {
        return;
    }
    console.changed = false;
    let Ok(log) = log.single() else { return };
    let shown = console.shown();
    // Scrolled back: a line saying so takes the last place.
    let scrolled = console.scroll.min(console.log.len().saturating_sub(shown));
    let end = console.log.len() - scrolled;
    let from = end.saturating_sub(if scrolled > 0 {
        shown.saturating_sub(1)
    } else {
        shown
    });
    commands.entity(log).despawn_children();
    commands.entity(log).with_children(|lines| {
        let below = (scrolled > 0).then(|| {
            (
                Said::Debug,
                format!(
                    "--- {scrolled} newer lines below: click here, scroll down, Page Down, or Ctrl+End ---"
                ),
            )
        });
        for (said, line) in console.log[from..end].iter() {
            lines.spawn((
                Text::new(line.clone()),
                TextFont {
                    font_size: bevy::text::FontSize::Px(15.0),
                    ..default()
                },
                TextColor(said.colour()),
            ));
        }
        // A click on it comes back to the newest, as chat windows' "jump
        // to present" does.
        if let Some((said, line)) = below {
            lines.spawn((
                Text::new(line),
                TextFont {
                    font_size: bevy::text::FontSize::Px(15.0),
                    ..default()
                },
                TextColor(said.colour()),
                Interaction::default(),
                Newer,
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

#[cfg(test)]
mod tests {
    use bevy::input::keyboard::{Key, KeyCode, KeyboardInput};
    use bevy::input::{ButtonState, InputPlugin};
    use bevy::prelude::*;
    use console::session::Session;

    use super::{Console, type_in};
    use crate::{Options, Play, Sim};

    /// The console with the skill yard's islander, and keys pressed into
    /// it as the window would, with nothing sent to Windows.
    fn app() -> App {
        let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../data");
        let world = console::load_world_file(&data.join("skill-yard.toml")).unwrap();
        let session = Session::new(world, "player").unwrap().with_real_time();
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin))
            .insert_resource(Sim {
                play: Play::Live(Box::new(session)),
                speed: 1.0,
                paused: false,
                owed: 0.0,
            })
            .insert_resource(Options {
                shot: None,
                after: 0.0,
                from: None,
                look: None,
                yaw: None,
                pitch: None,
                zoom: None,
                script: None,
                shots: String::new(),
                step: 0.0,
                typed: String::new(),
                open: String::new(),
            })
            .init_resource::<Console>()
            .add_systems(Update, type_in);
        app.update();
        app
    }

    fn key(app: &mut App, logical: Key, code: KeyCode, state: ButtonState) {
        app.world_mut().write_message(KeyboardInput {
            key_code: code,
            logical_key: logical,
            state,
            text: None,
            repeat: false,
            window: Entity::PLACEHOLDER,
        });
    }

    /// A key pressed and let go, in one frame.
    fn press(app: &mut App, logical: Key, code: KeyCode) {
        key(app, logical.clone(), code, ButtonState::Pressed);
        key(app, logical, code, ButtonState::Released);
        app.update();
    }

    fn typed(app: &mut App, text: &str) {
        for c in text.chars() {
            let logical = if c == ' ' {
                Key::Space
            } else {
                Key::Character(c.to_string().into())
            };
            key(app, logical.clone(), KeyCode::KeyA, ButtonState::Pressed);
            key(app, logical, KeyCode::KeyA, ButtonState::Released);
        }
        app.update();
    }

    fn console(app: &App) -> &Console {
        app.world().resource::<Console>()
    }

    #[test]
    fn a_slash_starts_a_tool_and_tab_completes_it_and_its_value() {
        let mut app = app();
        press(&mut app, Key::Character("/".into()), KeyCode::Slash);
        assert!(console(&app).typing);
        typed(&mut app, "qua");
        press(&mut app, Key::Tab, KeyCode::Tab);
        assert_eq!(console(&app).input.text, "/quality ");
        typed(&mut app, "me");
        press(&mut app, Key::Tab, KeyCode::Tab);
        assert_eq!(console(&app).input.text, "/quality medium ");
        press(&mut app, Key::Enter, KeyCode::Enter);
        assert!(!console(&app).typing);
        assert_eq!(app.world().resource::<Console>().tools, ["/quality medium"]);
    }

    #[test]
    fn tab_completes_a_command_and_what_the_islander_sees() {
        let mut app = app();
        let names = match &app.world().resource::<Sim>().play {
            Play::Live(session) => session.names(),
            Play::Script(_) => unreachable!(),
        };
        let name = names
            .iter()
            .find(|n| n.len() > 4)
            .expect("something in sight")
            .clone();
        press(&mut app, Key::Enter, KeyCode::Enter);
        typed(&mut app, "gath");
        press(&mut app, Key::Tab, KeyCode::Tab);
        assert_eq!(console(&app).input.text, "gather ");
        typed(&mut app, &name[..3]);
        press(&mut app, Key::Tab, KeyCode::Tab);
        let line = console(&app).input.text.clone();
        // Completed, or the names it could be listed in the log.
        let listed = console(&app)
            .log
            .last()
            .map(|(_, l)| l.clone())
            .unwrap_or_default();
        assert!(
            line.starts_with(&format!("gather {name}")) || listed.contains(&name),
            "{line:?} / {listed:?} / {names:?}"
        );
    }

    #[test]
    fn the_line_edits_where_the_cursor_is_and_the_log_scrolls_back() {
        let mut app = app();
        press(&mut app, Key::Enter, KeyCode::Enter);
        typed(&mut app, "lok around");
        press(&mut app, Key::Home, KeyCode::Home);
        press(&mut app, Key::ArrowRight, KeyCode::ArrowRight);
        press(&mut app, Key::ArrowRight, KeyCode::ArrowRight);
        typed(&mut app, "o");
        assert_eq!(console(&app).input.text, "look around");
        // Ctrl+Backspace takes a word.
        press(&mut app, Key::End, KeyCode::End);
        key(
            &mut app,
            Key::Control,
            KeyCode::ControlLeft,
            ButtonState::Pressed,
        );
        app.update();
        press(&mut app, Key::Backspace, KeyCode::Backspace);
        key(
            &mut app,
            Key::Control,
            KeyCode::ControlLeft,
            ButtonState::Released,
        );
        app.update();
        assert_eq!(console(&app).input.text, "look ");
        press(&mut app, Key::Escape, KeyCode::Escape);

        // Help is longer than the console: Page Up goes back, End returns.
        press(&mut app, Key::Enter, KeyCode::Enter);
        typed(&mut app, "help");
        press(&mut app, Key::Enter, KeyCode::Enter);
        // It's longer than the console: shown from its top. End comes to
        // the newest; Page Up goes back from there.
        assert!(console(&app).scroll > 0);
        press(&mut app, Key::End, KeyCode::End);
        assert_eq!(console(&app).scroll, 0);
        press(&mut app, Key::PageUp, KeyCode::PageUp);
        let back = console(&app).scroll;
        assert!(
            back > 0,
            "log {} lines, last {:?}",
            console(&app).log.len(),
            console(&app).log.last().map(|l| &l.1)
        );
        // New lines don't move what's being read.
        app.world_mut()
            .resource_mut::<Console>()
            .say(super::Said::News, "a boar grunts");
        assert_eq!(console(&app).scroll, back + 1);
        press(&mut app, Key::End, KeyCode::End);
        assert_eq!(console(&app).scroll, 0);
    }

    #[test]
    fn a_long_reply_shows_from_its_top_and_ctrl_end_jumps_to_the_newest() {
        let mut app = app();
        press(&mut app, Key::Enter, KeyCode::Enter);
        typed(&mut app, "help");
        press(&mut app, Key::Enter, KeyCode::Enter);
        // The window starts at the command that asked, the rest below it.
        let c = console(&app);
        let shown = c.shown();
        assert!(c.scroll > 0);
        let top = c.log.len() - c.scroll - (shown - 1);
        let help_at = c.log.iter().rposition(|(_, l)| l == "> help").unwrap();
        assert_eq!(c.log[top].1, "> help", "help at {help_at}");
        // Ctrl+End, even while typing, comes back to the newest.
        press(&mut app, Key::Enter, KeyCode::Enter);
        key(
            &mut app,
            Key::Control,
            KeyCode::ControlLeft,
            ButtonState::Pressed,
        );
        app.update();
        press(&mut app, Key::End, KeyCode::End);
        assert_eq!(console(&app).scroll, 0);
        // Ctrl+Home goes to the oldest.
        press(&mut app, Key::Home, KeyCode::Home);
        let c = console(&app);
        assert_eq!(c.scroll, c.log.len() - (c.shown() - 1));
    }

    #[test]
    fn tab_after_help_completes_what_there_is_help_on() {
        let mut app = app();
        press(&mut app, Key::Enter, KeyCode::Enter);
        typed(&mut app, "help gat");
        press(&mut app, Key::Tab, KeyCode::Tab);
        assert_eq!(console(&app).input.text, "help gather ");
        press(&mut app, Key::Escape, KeyCode::Escape);
        press(&mut app, Key::Enter, KeyCode::Enter);
        typed(&mut app, "help /qual");
        press(&mut app, Key::Tab, KeyCode::Tab);
        assert_eq!(console(&app).input.text, "help /quality ");
        press(&mut app, Key::Enter, KeyCode::Enter);
        let said = &console(&app).log;
        assert!(
            said.iter().any(|(_, l)| l.starts_with("/quality: ")),
            "{said:?}"
        );
    }
}
