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
];

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
}

impl Changeable<'_> {
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
            _ => String::new(),
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
        let flag: &mut bool = match name {
            "pause" => &mut self.sim.paused,
            "snap" => &mut self.settings.snap,
            "reach" => &mut self.settings.reach,
            "grid" => &mut self.grid.shown,
            "fly" => &mut self.eye.flying,
            "backpack" => &mut self.panels.backpack,
            "body" => &mut self.panels.body,
            "build" => &mut self.build.on,
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
                 changes one; /<name> alone flips one that's on or off. /save <name> saves the \
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
    if !reply.refused && Land::of(session.world()) != *things.land {
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
}
