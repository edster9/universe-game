//! Tools: what the person at the keyboard does, outside the world, as
//! opposed to commands, which the actor does in it. Tools start with a
//! slash. For now they're settings: `/settings` lists them, `/set <name>
//! <value>` (or `/<name> <value>`) changes one, and `/<name>` alone flips
//! one that's on or off. Keys change the same settings. `/save`, `/load`
//! and `/saves` (single player), and `/make` and `/light` (the server's
//! choice), are the session's own. Each tool has a layer, which says who
//! may use it. See docs/ideas/tools.md.

use bevy::prelude::*;

use crate::camera::Eye;
use crate::graphics::{self, Graphics, Quality};
use crate::grid::Grid;
use crate::panels::Panels;
use crate::terminal::{Console, Said};
use crate::terrain::Land;
use crate::{Play, Sim};

/// Settings that aren't kept anywhere else.
#[derive(Resource)]
pub struct Settings {
    /// Back to real speed when what the actor is doing is done.
    pub snap: bool,
    /// A circle on the ground showing how far the islander can reach.
    pub reach: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            snap: true,
            reach: false,
        }
    }
}

/// Who may use a tool (docs/ideas/tools.md, decided 2026-09-30). In single
/// player, all of them; there's only single player so far.
#[derive(Clone, Copy)]
enum Layer {
    /// Utility: changes how you see and use the game, never the world. Any
    /// server.
    Always,
    /// Changes the world or yourself, not anyone's time: allowed or not by
    /// the server's administrator.
    ServersChoice,
    /// Changes the shared clock: never on a server.
    SinglePlayer,
}

impl Layer {
    fn name(self) -> &'static str {
        match self {
            Layer::Always => "always",
            Layer::ServersChoice => "server's choice",
            Layer::SinglePlayer => "single player",
        }
    }
}

/// Every setting: its name, its key, its layer, and what it does.
const SETTINGS: &[(&str, &str, Layer, &str)] = &[
    (
        "speed",
        "[ ]",
        Layer::SinglePlayer,
        "game seconds a second; 1 is real time",
    ),
    ("pause", "Space", Layer::SinglePlayer, "the world stops"),
    (
        "snap",
        "",
        Layer::SinglePlayer,
        "back to real speed when what you're doing is done",
    ),
    ("grid", "G", Layer::Always, "a grid on the ground"),
    (
        "reach",
        "",
        Layer::Always,
        "a circle showing how far you can reach",
    ),
    ("fly", "F", Layer::Always, "the camera flies free"),
    ("backpack", "B", Layer::Always, "the backpack window"),
    ("body", "V", Layer::Always, "the body window"),
    (
        "build",
        "M",
        Layer::ServersChoice,
        "build mode: drag things with the left button, lift them with the middle one",
    ),
    (
        "showcase",
        "",
        Layer::Always,
        "models from an asset pack set out to look at, not in the world",
    ),
];

/// Every tool's name, with its slash, for Tab to complete.
pub fn names() -> Vec<String> {
    let mut names: Vec<String> = [
        "help", "settings", "set", "save", "load", "saves", "make", "light", "place",
    ]
    .into_iter()
    .chain(SETTINGS.iter().map(|(name, ..)| *name))
    .chain(graphics::SETTINGS.iter().map(|(name, ..)| *name))
    .map(|n| format!("/{n}"))
    .collect();
    names.sort();
    names
}

/// The values a setting takes, for Tab to complete after its name (or after
/// `/set <name>`).
pub fn values(tool: &str) -> Vec<String> {
    let name = tool.trim_start_matches('/');
    if name == "set" {
        return SETTINGS
            .iter()
            .map(|(name, ..)| *name)
            .chain(graphics::SETTINGS.iter().map(|(name, ..)| *name))
            .map(str::to_string)
            .collect();
    }
    if let Some((_, values, _)) = graphics::SETTINGS.iter().find(|(n, ..)| *n == name) {
        return values
            .split(", ")
            .filter(|v| !v.contains(' ') || *v == "or off")
            .map(|v| v.trim_start_matches("or ").to_string())
            .filter(|v| v != "metres")
            .collect();
    }
    if SETTINGS.iter().any(|(n, ..)| *n == name) && name != "speed" {
        return vec!["on".into(), "off".into()];
    }
    Vec::new()
}

/// The game's own tools, a line each, for "help tools" (the session's
/// tools come before them).
pub fn listing() -> String {
    let mut lines = vec![" In the game".to_string()];
    lines.push(format!(
        "  {:<14}{}",
        "/settings", "every setting, and what it's set to now"
    ));
    lines.push(format!(
        "  {:<14}{}",
        "/set", "change a setting: /set <name> <value>, or /<name> <value>"
    ));
    for (name, key, _, help) in SETTINGS {
        let key = if key.is_empty() {
            String::new()
        } else {
            format!(" (key {key})")
        };
        lines.push(format!("  {:<14}{help}{key}", format!("/{name}")));
    }
    for (name, _, help) in graphics::SETTINGS {
        lines.push(format!("  {:<14}{help}", format!("/{name}")));
    }
    lines.join("\n")
}

/// Help on one of the game's own tools ("help /grid", "help quality"), if
/// that's what's asked and it's not one the session knows.
pub fn help_on(line: &str) -> Option<String> {
    let mut words = line.split_whitespace();
    if !matches!(words.next(), Some("help" | "?")) {
        return None;
    }
    let name = words.next()?.trim_start_matches('/').to_lowercase();
    if console::help::entry(&name).is_some() {
        return None;
    }
    let ways = |values: &[String]| {
        if values.is_empty() {
            String::new()
        } else {
            format!("\n  /{name} <value>: {}", values.join(", "))
        }
    };
    if let Some((_, key, layer, help)) = SETTINGS.iter().find(|(n, ..)| *n == name) {
        let key = if key.is_empty() {
            String::new()
        } else {
            format!("\nKey: {key}.")
        };
        let flips = if name == "speed" {
            "\n  /speed <x>: from 1 (real time) to 16384".to_string()
        } else {
            format!("\n  /{name}: flips it on or off{}", ways(&values(&name)))
        };
        return Some(format!(
            "/{name}: {help}{flips}{key}\nWho may use it: {}.",
            layer.name()
        ));
    }
    if let Some((_, _, help)) = graphics::SETTINGS.iter().find(|(n, ..)| *n == name) {
        let mut text = format!("/{name}: {help}{}", ways(&values(&name)));
        if name == "quality" {
            for quality in [Quality::Low, Quality::Medium, Quality::High, Quality::Ultra] {
                let preset = Graphics::preset(quality)
                    .value("quality")
                    .unwrap_or_default();
                text += &format!("\n  {preset}: {}", Graphics::describe(quality));
            }
        }
        text += "\nKept between games. Who may use it: always (it's your own machine's).";
        return Some(text);
    }
    match name.as_str() {
        "settings" => Some("/settings: lists every setting and what it's set to now.".into()),
        "help" => Some("/help: the tools in short; \"help tools\" lists them all, a line each.".into()),
        "set" => Some(
            "/set <name> <value> changes a setting (/<name> <value> does too); /<name> alone flips one that's on or off.".into(),
        ),
        _ => None,
    }
}

/// Everything a tool can change.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Changeable<'w> {
    sim: ResMut<'w, Sim>,
    settings: ResMut<'w, Settings>,
    grid: ResMut<'w, Grid>,
    eye: ResMut<'w, Eye>,
    panels: ResMut<'w, Panels>,
    land: Res<'w, Land>,
    build: ResMut<'w, crate::build::Build>,
    showcase: ResMut<'w, crate::showcase::Showcase>,
    graphics: ResMut<'w, Graphics>,
}

impl Changeable<'_> {
    fn graphics_name(&self, quality: Quality) -> String {
        Graphics::preset(quality)
            .value("quality")
            .unwrap_or_default()
    }

    fn value(&self, name: &str) -> String {
        let on = |b: bool| if b { "on" } else { "off" }.to_string();
        match name {
            "speed" => format!("{}", self.sim.speed),
            "pause" => on(self.sim.paused),
            "snap" => on(self.settings.snap),
            "reach" => on(self.settings.reach),
            "grid" => on(self.grid.shown),
            "fly" => on(self.eye.flying),
            "backpack" => on(self.panels.backpack),
            "body" => on(self.panels.body),
            "build" => on(self.build.on),
            "showcase" => on(self.showcase.shown),
            _ => self.graphics.value(name).unwrap_or_default(),
        }
    }

    /// Sets `name` to `value`, or flips it if no value is given.
    fn set(&mut self, name: &str, value: Option<&str>) -> Result<(), String> {
        if name == "speed" {
            let speed: f32 = value
                .and_then(|v| v.trim_start_matches('x').parse().ok())
                .filter(|s| (1.0..=16_384.0).contains(s))
                .ok_or("speed is a number from 1 to 16384, like /speed 64")?;
            self.sim.speed = speed;
            return Ok(());
        }
        // The drawing settings: kept for the next game when changed.
        if let Some(result) = self.graphics.set(name, value) {
            if result.is_ok() {
                self.graphics.keep();
            }
            return result;
        }
        let flag: &mut bool = match name {
            "pause" => &mut self.sim.paused,
            "snap" => &mut self.settings.snap,
            "reach" => &mut self.settings.reach,
            "grid" => &mut self.grid.shown,
            "fly" => &mut self.eye.flying,
            "backpack" => &mut self.panels.backpack,
            "body" => &mut self.panels.body,
            "build" => &mut self.build.on,
            "showcase" => &mut self.showcase.shown,
            _ => {
                return Err(format!(
                    "there's no setting \"{name}\"; /settings lists them"
                ));
            }
        };
        *flag = match value {
            None => !*flag,
            Some("on" | "yes" | "true" | "1") => true,
            Some("off" | "no" | "false" | "0") => false,
            Some(other) => return Err(format!("{name} is on or off, not \"{other}\"")),
        };
        Ok(())
    }
}

/// A tool, as typed.
#[derive(Debug, PartialEq)]
enum Tool<'a> {
    Help,
    Settings,
    /// A setting, and its new value: flipped if none.
    Set(&'a str, Option<&'a str>),
    Unknown,
}

fn parse(line: &str) -> Tool<'_> {
    let words: Vec<&str> = line.trim_start_matches('/').split_whitespace().collect();
    match words.as_slice() {
        [] | ["help"] => Tool::Help,
        ["settings"] => Tool::Settings,
        ["set", name, value] => Tool::Set(name, Some(value)),
        ["set", name] => Tool::Set(name, None),
        [name, value] => Tool::Set(name, Some(value)),
        [name] => Tool::Set(name, None),
        _ => Tool::Unknown,
    }
}

/// Runs a tool, and says what came of it.
pub fn run(line: &str, console: &mut Console, things: &mut Changeable) {
    let first = line.split_whitespace().next().unwrap_or("");
    if ["/save", "/load", "/saves", "/make", "/light"].contains(&first) {
        session_tool(line, console, things);
        return;
    }
    let (name, value) = match parse(line) {
        Tool::Help => {
            console.say(
                Said::Debug,
                "Tools: /settings lists the settings; /set <name> <value>, or /<name> <value>, \
                 changes one; /<name> alone flips one that's on or off; /quality low, medium, \
                 high or ultra sets how the scene is drawn all at once. /save <name> saves the \
                 whole world, /load [name] picks a save up (\"last\" is made when the game ends), \
                 and /saves lists them (single player). /make <thing> puts something in front of \
                 you (\"/make fire\", \"/make 2 kg wood\", \"/make fire ring\"), and /light <thing> \
                 lights it.",
            );
            return;
        }
        Tool::Settings => {
            for (name, key, layer, help) in SETTINGS {
                let key = if key.is_empty() {
                    String::new()
                } else {
                    format!(", key {key}")
                };
                let value = things.value(name);
                let layer = layer.name();
                console.say(
                    Said::Debug,
                    &format!("{name} = {value} ({layer}{key}): {help}"),
                );
            }
            for (name, values, help) in graphics::SETTINGS {
                let value = things.value(name);
                console.say(
                    Said::Debug,
                    &format!("{name} = {value} (always; {values}): {help}"),
                );
            }
            return;
        }
        // What the presets set, rather than flipping anything.
        Tool::Set("quality", None) => {
            let now = things.value("quality");
            console.say(
                Said::Debug,
                &format!("quality = {now}; /quality <preset> sets:"),
            );
            for quality in [Quality::Low, Quality::Medium, Quality::High, Quality::Ultra] {
                let name = things.graphics_name(quality);
                let sets = Graphics::describe(quality);
                console.say(Said::Debug, &format!("  {name}: {sets}"));
            }
            return;
        }
        Tool::Set(name, value) => (name, value),
        Tool::Unknown => {
            console.say(Said::Refused, "Try /settings, or /set <name> <value>.");
            return;
        }
    };
    match things.set(name, value) {
        Ok(()) => {
            let value = things.value(name);
            console.say(Said::Debug, &format!("{name} = {value}"));
        }
        Err(why) => console.say(Said::Refused, &format!("{why}.")),
    }
}

/// The session's own tools: saving and loading the whole world (single
/// player), and the designer making things (the server's choice).
fn session_tool(line: &str, console: &mut Console, things: &mut Changeable) {
    let Play::Live(session) = &mut things.sim.play else {
        console.say(Said::Refused, "That's only for a game played live.");
        return;
    };
    let reply = session.handle(line);
    let said = if reply.refused {
        Said::Refused
    } else {
        Said::Reply
    };
    console.say(said, &reply.text);
    // The land is drawn once, at the start.
    if !reply.refused && Land::of(session.world()).flat_if(things.land.is_flat()) != *things.land {
        console.say(
            Said::Refused,
            "This save is of another world, so the land drawn is still the old one: \
             start the game with --load <name> to see it.",
        );
    }
}

/// Runs the tools typed or spoken since last frame.
pub fn run_typed(mut console: ResMut<Console>, mut things: Changeable) {
    for line in std::mem::take(&mut console.tools) {
        run(&line, &mut console, &mut things);
    }
}

#[cfg(test)]
mod tests {
    use super::{Tool, parse};

    #[test]
    fn tools_are_read_as_typed() {
        assert_eq!(parse("/"), Tool::Help);
        assert_eq!(parse("/settings"), Tool::Settings);
        assert_eq!(parse("/set grid"), Tool::Set("grid", None));
        assert_eq!(parse("/set speed 64"), Tool::Set("speed", Some("64")));
        assert_eq!(parse("/grid off"), Tool::Set("grid", Some("off")));
        assert_eq!(parse("/grid"), Tool::Set("grid", None));
        assert_eq!(parse("/set speed 64 now"), Tool::Unknown);
    }

    #[test]
    fn tab_knows_every_tool_and_each_settings_values() {
        let names = super::names();
        assert!(names.contains(&"/quality".to_string()));
        assert!(names.contains(&"/grid".to_string()));
        assert!(names.contains(&"/save".to_string()));
        assert_eq!(
            super::values("/quality"),
            ["low", "medium", "high", "ultra"]
        );
        assert_eq!(super::values("/grid"), ["on", "off"]);
        assert_eq!(super::values("/fps-cap"), ["off"]);
        assert!(super::values("/set").contains(&"shadows".to_string()));
    }

    #[test]
    fn the_games_own_tools_have_their_help() {
        for name in super::names() {
            let asked = format!("help {name}");
            let session = console::help::entry(name.trim_start_matches('/')).is_some();
            assert!(
                session || super::help_on(&asked).is_some(),
                "no help for {name}"
            );
        }
        let quality = super::help_on("help /quality").unwrap();
        assert!(quality.contains("low, medium, high, ultra") && quality.contains("shadows"));
        assert!(super::help_on("help grid").unwrap().contains("Key: G"));
        // The session's own tools and commands are left to the session.
        assert!(super::help_on("help /save").is_none());
        assert!(super::help_on("help gather").is_none());
    }
}
