//! One person at the console, acting in one world.

use engine::gate::Change;
use engine::intent::{self, Command};
use engine::laws::{self, ActError};
use engine::units::{Credits, Mass};
use engine::view::{self, Thing};
use engine::world::{EntityId, World};

pub const HELP: &str = "\
Commands:
  look                       describe where you are
  inventory                  what you carry, and your credits
  go <place>                 walk somewhere
  take <thing>               pick something up
  drop <thing>               put something down
  give <thing> to <person>   hand something over
  pay <person> <amount>      pay credits
Testing tools:
  totals                     the world's total mass and credits (these never change)
  log [n]                    the last n actions that passed the gate
  become <person>            act as someone else
  quit";

pub struct Session {
    world: World,
    player: EntityId,
}

pub struct Reply {
    pub text: String,
    pub quit: bool,
}

impl Reply {
    fn say(text: impl Into<String>) -> Self {
        Reply {
            text: text.into(),
            quit: false,
        }
    }
}

impl Session {
    /// Starts a session acting as the agent whose data ID is `player`.
    pub fn new(world: World, player: &str) -> Result<Self, String> {
        match world.find_by_key(player) {
            Some(id) if world.is_agent(id) => Ok(Session { world, player: id }),
            _ => Err(format!(
                "there's no person with the id {player:?} in this world"
            )),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn handle(&mut self, line: &str) -> Reply {
        let line = line.trim();
        let (first, rest) = line
            .split_once(' ')
            .map_or((line, ""), |(f, r)| (f, r.trim()));
        match first.to_lowercase().as_str() {
            "" => Reply::say(""),
            "help" | "?" => Reply::say(HELP),
            "quit" | "exit" => Reply {
                text: "Goodbye.".into(),
                quit: true,
            },
            "totals" => Reply::say(self.totals()),
            "log" => Reply::say(self.log(rest)),
            "become" => Reply::say(self.become_person(rest)),
            _ => Reply::say(self.command(line)),
        }
    }

    fn command(&mut self, line: &str) -> String {
        match intent::parse(line) {
            Err(error) => sentence(&error.to_string()),
            Ok(Command::Look) => self.look(),
            Ok(Command::Inventory) => self.inventory(),
            Ok(Command::Act(intent)) => match laws::act(&mut self.world, self.player, intent) {
                Ok(changes) => changes
                    .iter()
                    .map(|c| self.describe(c))
                    .collect::<Vec<_>>()
                    .join("\n"),
                Err(ActError::Refused(refusal)) => sentence(&refusal.to_string()),
                Err(fault @ ActError::Fault(_)) => format!("!! {fault}"),
            },
        }
    }

    fn look(&self) -> String {
        let Some(look) = view::look(&self.world, self.player) else {
            return "You aren't anywhere.".into();
        };
        let mut lines = vec![sentence_case(&look.place)];
        lines.push(format!("Ways out: {}", list_or(&look.exits, "none")));
        if !look.people.is_empty() {
            lines.push(format!("People here: {}", look.people.join(", ")));
        }
        if !look.things.is_empty() {
            lines.push(format!("Things here: {}", things(&look.things)));
        }
        lines.join("\n")
    }

    fn inventory(&self) -> String {
        let inv = view::inventory(&self.world, self.player);
        let carrying = if inv.things.is_empty() {
            "You aren't carrying anything.".to_string()
        } else {
            format!(
                "You're carrying {}, {} in all.",
                things(&inv.things),
                inv.carried
            )
        };
        format!("{carrying}\nYou have {}.", inv.credits)
    }

    fn describe(&self, change: &Change) -> String {
        let w = &self.world;
        match *change {
            Change::Move { entity, to } if entity == self.player => {
                format!("You go to {}.", w.label(to))
            }
            Change::Move { entity, to } if to == self.player => {
                format!("You take the {}.", w.label(entity))
            }
            Change::Move { entity, to } if w.is_place(to) => {
                format!("You drop the {}.", w.label(entity))
            }
            Change::Move { entity, to } => {
                format!("You give the {} to {}.", w.label(entity), w.label(to))
            }
            Change::Transfer { to, amount, .. } => format!("You pay {} {amount}.", w.label(to)),
        }
    }

    fn totals(&self) -> String {
        let mass = u64::try_from(self.world.total_mass()).map(Mass::from_mg);
        let credits = u64::try_from(self.world.total_credits()).map(Credits::new);
        match (mass, credits) {
            (Ok(mass), Ok(credits)) => format!("The world holds {mass} and {credits} in total."),
            _ => "The world's totals are too large to show.".into(),
        }
    }

    fn log(&self, count: &str) -> String {
        let count = if count.is_empty() {
            10
        } else {
            match count.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return "Try \"log\" or \"log 20\".".into(),
            }
        };
        let log = self.world.log();
        if log.is_empty() {
            return "Nothing has happened yet.".into();
        }
        let w = &self.world;
        log[log.len().saturating_sub(count)..]
            .iter()
            .map(|entry| {
                let changes = entry
                    .changes
                    .iter()
                    .map(|change| match *change {
                        Change::Move { entity, to } => {
                            format!("{} -> {}", w.label(entity), w.label(to))
                        }
                        Change::Transfer { from, to, amount } => {
                            format!("{amount}: {} -> {}", w.label(from), w.label(to))
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                format!(
                    "#{} {}: {} ({changes})",
                    entry.seq,
                    w.label(entry.actor),
                    entry.intent
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn become_person(&mut self, name: &str) -> String {
        let wanted = name.trim().to_lowercase();
        let found = self.world.entities().find(|&id| {
            self.world.is_agent(id)
                && (self.world.key(id).to_lowercase() == wanted
                    || self.world.label(id).to_lowercase() == wanted)
        });
        match found {
            Some(id) => {
                self.player = id;
                format!("You are now {}.\n{}", self.world.label(id), self.look())
            }
            None => format!("There's no person called {name:?}."),
        }
    }
}

fn things(things: &[Thing]) -> String {
    things
        .iter()
        .map(|t| format!("{} ({})", t.label, t.mass))
        .collect::<Vec<_>>()
        .join(", ")
}

fn list_or(items: &[String], empty: &str) -> String {
    if items.is_empty() {
        empty.to_string()
    } else {
        items.join(", ")
    }
}

fn sentence_case(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Turns an engine message into a sentence: capitalised, with a full stop.
fn sentence(text: &str) -> String {
    let text = sentence_case(text);
    if text.ends_with(['.', '!', '?']) {
        text
    } else {
        format!("{text}.")
    }
}
