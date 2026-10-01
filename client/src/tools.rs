//! Tools: what the person at the keyboard does, outside the world, as
//! opposed to commands, which the actor does in it. Tools start with a
//! slash. For now they're settings: `/settings` lists them, `/set <name>
//! <value>` (or `/<name> <value>`) changes one, and `/<name>` alone flips
//! one that's on or off. Keys change the same settings. See
//! docs/ideas/tools.md.

use bevy::prelude::*;

use crate::Sim;
use crate::camera::Eye;
use crate::grid::Grid;
use crate::panels::Panels;
use crate::terminal::{Console, Said};

/// Settings that aren't kept anywhere else.
#[derive(Resource)]
pub struct Settings {
    /// Back to real speed when what the actor is doing is done.
    pub snap: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { snap: true }
    }
}

/// Every setting: its name, its key, and what it does.
const SETTINGS: &[(&str, &str, &str)] = &[
    ("speed", "[ ]", "game seconds a second; 1 is real time"),
    ("pause", "Space", "the world stops"),
    (
        "snap",
        "",
        "back to real speed when what you're doing is done",
    ),
    ("grid", "G", "a grid on the ground"),
    ("fly", "F", "the camera flies free"),
    ("backpack", "B", "the backpack window"),
    ("body", "V", "the body window"),
];

/// Everything a tool can change.
#[derive(bevy::ecs::system::SystemParam)]
pub struct Changeable<'w> {
    sim: ResMut<'w, Sim>,
    settings: ResMut<'w, Settings>,
    grid: ResMut<'w, Grid>,
    eye: ResMut<'w, Eye>,
    panels: ResMut<'w, Panels>,
}

impl Changeable<'_> {
    fn value(&self, name: &str) -> String {
        let on = |b: bool| if b { "on" } else { "off" }.to_string();
        match name {
            "speed" => format!("{}", self.sim.speed),
            "pause" => on(self.sim.paused),
            "snap" => on(self.settings.snap),
            "grid" => on(self.grid.shown),
            "fly" => on(self.eye.flying),
            "backpack" => on(self.panels.backpack),
            "body" => on(self.panels.body),
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
            "grid" => &mut self.grid.shown,
            "fly" => &mut self.eye.flying,
            "backpack" => &mut self.panels.backpack,
            "body" => &mut self.panels.body,
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
    let (name, value) = match parse(line) {
        Tool::Help => {
            console.say(
                Said::Debug,
                "Tools: /settings lists the settings; /set <name> <value>, or /<name> <value>, \
                 changes one; /<name> alone flips one that's on or off.",
            );
            return;
        }
        Tool::Settings => {
            for (name, key, help) in SETTINGS {
                let key = if key.is_empty() {
                    String::new()
                } else {
                    format!(" (key {key})")
                };
                let value = things.value(name);
                console.say(Said::Debug, &format!("{name} = {value}{key}: {help}"));
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
