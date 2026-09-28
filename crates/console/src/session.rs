//! One person at the console, acting in one world.

use engine::gate::{Cause, Change, Holder};
use engine::intent::{self, Command, Intent};
use engine::laws::{self, ActError};
use engine::matter;
use engine::nature;
use engine::units::{Credits, Energy, Mass};
use engine::view::{self, Thing};
use engine::world::{EntityId, World};

pub const HELP: &str = "\
Commands:
  look                              describe where you are
  inventory                         what you carry, and your credits
  go <place>                        walk somewhere
  take <thing> [from <container>]   pick something up
  drop <thing>                      put something down
  put <thing> in <container>        put something inside something
  give <thing> to <person>          hand something over
  pay <person> <amount>             pay credits
  dig <source> with <tool>          dig material out of the ground
  light <thing>                     light a fire in something that burns fuel
  pour <liquid> into <container>    pour something molten
  work <thing> into <shape> with <tool>
                                    shape something with a tool
  wait [seconds]                    let time pass
Testing tools:
  totals                            the world's total mass, energy, and credits (these never change)
  time                              how long the world has been running
  log [n]                           the last n entries that passed the gate
  become <person>                   act as someone else
  quit
Things can be named by part of their description (\"lump\"), or by id (\"#12\").";

/// The longest wait allowed in one command: one game day.
const MAX_WAIT: u64 = 86_400;

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
            "time" => Reply::say(format!(
                "The world has been running for {} seconds.",
                self.world.tick()
            )),
            "log" => Reply::say(self.log(rest)),
            "become" => Reply::say(self.become_person(rest)),
            "wait" | "z" => Reply::say(self.wait(rest)),
            _ => Reply::say(self.command(line)),
        }
    }

    fn command(&mut self, line: &str) -> String {
        match intent::parse(line) {
            Err(error) => sentence(&error.to_string()),
            Ok(Command::Look) => self.look(),
            Ok(Command::Inventory) => self.inventory(),
            Ok(Command::Act(intent)) => {
                match laws::act(&mut self.world, self.player, intent.clone()) {
                    Ok(changes) => self.describe(&intent, &changes),
                    Err(ActError::Refused(refusal)) => sentence(&refusal.to_string()),
                    Err(fault @ ActError::Fault(_)) => format!("!! {fault}"),
                }
            }
        }
    }

    fn wait(&mut self, seconds: &str) -> String {
        let seconds = if seconds.is_empty() {
            Ok(1)
        } else {
            seconds.parse::<u64>()
        };
        match seconds {
            Ok(n) if (1..=MAX_WAIT).contains(&n) => match nature::run(&mut self.world, n) {
                Ok(()) if n == 1 => "A second passes.".into(),
                Ok(()) => format!("{n} seconds pass."),
                Err(fault) => format!("!! engine fault: {fault}"),
            },
            _ => format!("Try \"wait\" or \"wait 60\" (up to {MAX_WAIT} seconds)."),
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
        for container in look.things.iter().filter(|t| !t.contents.is_empty()) {
            lines.push(format!(
                "In {}: {}",
                the(&container.label),
                things(&container.contents)
            ));
        }
        if !look.air.is_empty() {
            lines.push(format!("In the air: {}", things(&look.air)));
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

    fn describe(&self, intent: &Intent, changes: &[Change]) -> String {
        let w = &self.world;
        let name = |id: EntityId| laws::named(w, id);
        match (intent, changes.first()) {
            (Intent::Go { .. }, Some(&Change::Move { to, .. })) => {
                format!("You go to {}.", w.label(to))
            }
            (Intent::Take { .. } | Intent::TakeFrom { .. }, Some(&Change::Move { entity, .. })) => {
                format!("You take {}.", name(entity))
            }
            (Intent::Drop { .. }, Some(&Change::Move { entity, .. })) => {
                format!("You drop {}.", name(entity))
            }
            (Intent::Put { .. }, Some(&Change::Move { entity, to })) => {
                format!("You put {} in {}.", name(entity), name(to))
            }
            (Intent::Give { .. }, Some(&Change::Move { entity, to })) => {
                format!("You give {} to {}.", name(entity), name(to))
            }
            (Intent::Pay { .. }, Some(&Change::Transfer { to, amount, .. })) => {
                format!("You pay {} {amount}.", name(to))
            }
            (Intent::Dig { .. }, Some(Change::Split { take, .. })) => format!(
                "You dig out {} of {}.",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition(take)
            ),
            (Intent::Light { .. }, Some(&Change::Light { chamber, .. })) => {
                format!("You light {}.", name(chamber))
            }
            (Intent::Pour { .. }, Some(&Change::Move { entity, to })) => {
                format!("You pour {} into {}.", name(entity), name(to))
            }
            (Intent::Work { .. }, Some(&Change::Shape { entity, .. })) => {
                format!("You work it into {}.", name(entity))
            }
            _ => "Done.".into(),
        }
    }

    fn totals(&self) -> String {
        let w = &self.world;
        let mass = u64::try_from(w.total_mass()).map(Mass::from_mg);
        let energy = u64::try_from(w.total_energy()).map(Energy::from_uj);
        let credits = u64::try_from(w.total_credits()).map(Credits::new);
        match (mass, energy, credits) {
            (Ok(mass), Ok(energy), Ok(credits)) => {
                format!("The world holds {mass}, {energy} of energy, and {credits} in total.")
            }
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
        log[log.len().saturating_sub(count)..]
            .iter()
            .map(|entry| {
                let changes = entry
                    .changes
                    .iter()
                    .map(|c| self.change_summary(c))
                    .collect::<Vec<_>>()
                    .join("; ");
                match &entry.cause {
                    Cause::Action { actor, intent } => {
                        format!(
                            "#{} {}: {intent} ({changes})",
                            entry.seq,
                            self.world.label(*actor)
                        )
                    }
                    Cause::Nature { tick } => {
                        format!("#{} nature at {tick} s: {changes}", entry.seq)
                    }
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn change_summary(&self, change: &Change) -> String {
        let w = &self.world;
        let holder = |h: &Holder| match *h {
            Holder::Thing(id) => w.label(id),
            Holder::Surroundings(place) => format!("surroundings of {}", w.label(place)),
        };
        match change {
            &Change::Move { entity, to } => format!("{} -> {}", w.label(entity), w.label(to)),
            &Change::Transfer { from, to, amount } => {
                format!("{amount}: {} -> {}", w.label(from), w.label(to))
            }
            Change::Heat { from, to, amount } => {
                format!("{amount} heat: {} -> {}", holder(from), holder(to))
            }
            &Change::Burn {
                entity,
                material,
                mass,
            } => {
                format!(
                    "burn {mass} of {} in {}",
                    w.materials()[&material].label,
                    w.label(entity)
                )
            }
            Change::Split { from, take, .. } => format!(
                "{} of {} comes out of {}",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition(take),
                w.label(*from)
            ),
            &Change::Merge { into, .. } => format!("merges into {}", w.label(into)),
            &Change::Shape { entity, .. } => format!("{} takes shape", w.label(entity)),
            &Change::Light { chamber, lit } => {
                format!(
                    "{} {}",
                    w.label(chamber),
                    if lit { "is lit" } else { "goes out" }
                )
            }
        }
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
        .map(|t| {
            let mut details = vec![t.mass.to_string()];
            details.extend(t.temperature.map(|temperature| temperature.to_string()));
            details.extend(t.notes.iter().cloned());
            format!("{} ({})", t.label, details.join(", "))
        })
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

/// "the hearth" for "hearth", unchanged for names that already start with
/// "the" or a capital.
fn the(label: &str) -> String {
    if label.starts_with("the ") || label.chars().next().is_some_and(char::is_uppercase) {
        label.to_string()
    } else {
        format!("the {label}")
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
