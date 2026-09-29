//! One person at the console, acting in one world.

use engine::datasheet::{self, Datasheet};
use engine::gate::{Cause, Change, Holder};
use engine::intent::{self, Command, Intent};
use engine::laws::{self, ActError};
use engine::matter;
use engine::nature;
use engine::units::{self, Credits, Energy, Mass};
use engine::view::{self, Thing};
use engine::world::{EntityId, Requirement, World};

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
  rub <thing> against <thing>       rub two parts together to make both finer (10 minutes)
  assemble <design>                 put carried parts together to a design
  disassemble <thing>               take something apart into its parts
  wait [seconds]                    let time pass
Testing tools:
  totals                            the world's total mass, energy, and credits (these never change)
  datasheet <thing|here|me>         everything the engine measures about something
  designs                           the designs in this world and what they need
  time                              how long the world has been running
  log [n]                           the last n entries that passed the gate
  become <person>                   act as someone else
  quit
Things can be named by part of their description (\"lump\"), or by id (\"#12\").";

/// The longest wait allowed in one command: one game day.
const MAX_WAIT: u64 = 30 * 86_400;

pub struct Session {
    world: World,
    player: EntityId,
    announced_death: Option<String>,
}

pub struct Reply {
    pub text: String,
    pub quit: bool,
    /// The command wasn't understood, or the laws refused it.
    pub refused: bool,
}

impl Reply {
    fn say(text: impl Into<String>) -> Self {
        Reply {
            text: text.into(),
            quit: false,
            refused: false,
        }
    }

    fn refuse(text: impl Into<String>) -> Self {
        Reply {
            text: text.into(),
            quit: false,
            refused: true,
        }
    }
}

impl Session {
    /// Starts a session acting as the agent whose data ID is `player`.
    pub fn new(world: World, player: &str) -> Result<Self, String> {
        match world.find_by_key(player) {
            Some(id) if world.is_agent(id) => Ok(Session {
                world,
                player: id,
                announced_death: None,
            }),
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
        let mut reply = match first.to_lowercase().as_str() {
            "" => Reply::say(""),
            "help" | "?" => Reply::say(HELP),
            "quit" | "exit" => Reply {
                text: "Goodbye.".into(),
                quit: true,
                refused: false,
            },
            "totals" => Reply::say(self.totals()),
            "time" => Reply::say(format!(
                "The world has been running for {} seconds.",
                self.world.tick()
            )),
            "log" => Reply::say(self.log(rest)),
            "become" => Reply::say(self.become_person(rest)),
            "wait" | "z" => self.wait(rest),
            "datasheet" | "ds" => Reply::say(self.datasheet(rest)),
            "designs" => Reply::say(self.designs()),
            _ => self.command(line),
        };
        // News of the player's own death comes with whatever they were doing.
        let died = self.world.life(self.player).and_then(|l| l.died_of.clone());
        match died {
            Some(cause) if self.announced_death.is_none() => {
                self.announced_death = Some(cause.clone());
                reply.text = format!("{}\nYou have died of {cause}.", reply.text)
                    .trim()
                    .to_string();
                reply
            }
            _ => reply,
        }
    }

    fn command(&mut self, line: &str) -> Reply {
        match intent::parse(line) {
            Err(error) => Reply::refuse(sentence(&error.to_string())),
            Ok(Command::Look) => Reply::say(self.look()),
            Ok(Command::Inventory) => Reply::say(self.inventory()),
            Ok(Command::Act(intent)) => {
                let started = self.world.tick();
                match laws::perform(&mut self.world, self.player, intent.clone()) {
                    Ok(changes) => {
                        let text = self.describe(&intent, &changes);
                        let spent = self.world.tick() - started;
                        Reply::say(if spent > 0 && !matches!(intent, Intent::Rub { .. }) {
                            format!("{text} (That took {}.)", units::show_duration(spent))
                        } else {
                            text
                        })
                    }
                    Err(ActError::Refused(refusal)) => {
                        Reply::refuse(sentence(&refusal.to_string()))
                    }
                    Err(fault @ ActError::Fault(_)) => Reply::refuse(format!("!! {fault}")),
                }
            }
        }
    }

    /// Waits a number of seconds ("wait 60"), or a time with a unit ("wait 2
    /// h", "wait 3 day").
    fn wait(&mut self, time: &str) -> Reply {
        let seconds = if time.is_empty() {
            Some(1)
        } else if let Ok(n) = time.parse::<u64>() {
            Some(n)
        } else {
            units::parse_quantity(time, units::property::DURATION, "a time").ok()
        };
        match seconds {
            Some(n) if (1..=MAX_WAIT).contains(&n) => match nature::run(&mut self.world, n) {
                Ok(()) if n == 1 => Reply::say("A second passes."),
                Ok(()) => Reply::say(format!("{} passes.", units::show_duration(n))),
                Err(fault) => Reply::refuse(format!("!! engine fault: {fault}")),
            },
            _ => Reply::refuse("Try \"wait\", \"wait 60\", or \"wait 2 h\" (up to 30 days)."),
        }
    }

    pub fn player(&self) -> EntityId {
        self.player
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
            (Intent::Rub { .. }, _) => {
                let parts: Vec<String> = changes
                    .iter()
                    .filter_map(|c| match *c {
                        Change::Refine { entity, tolerance } => Some(format!(
                            "{} is now within {}",
                            name(entity),
                            units::show(u128::from(tolerance), units::show::LENGTH, 3)
                        )),
                        _ => None,
                    })
                    .collect();
                let session = match intent {
                    Intent::Rub {
                        seconds: Some(s), ..
                    } => *s,
                    _ => w.settings().rubbing_time,
                };
                let mut text = format!(
                    "You rub them together for {}.",
                    units::show_duration(session)
                );
                if !parts.is_empty() {
                    text = format!("{text} {}.", sentence_case(&parts.join(", and ")));
                }
                // What the rubbing wore off, and how hot it got.
                let dust = changes.iter().find_map(|c| match c {
                    Change::StartActivity {
                        activity: engine::world::Activity::Rubbing { dust, .. },
                        ..
                    } => Some(*dust),
                    _ => None,
                });
                if let Some(dust) = dust.filter(|&d| w.exists(d)) {
                    let state = if w.is_burning(dust) {
                        ", and it's smouldering"
                    } else {
                        ""
                    };
                    text = format!(
                        "{text} {} of {} wears off, at {}{state}.",
                        w.mass(dust),
                        w.label(dust),
                        w.temperature(dust).unwrap_or_default()
                    );
                }
                text
            }
            (Intent::Assemble { .. }, Some(Change::Assemble { .. })) => {
                let made = w
                    .contents(self.player)
                    .into_iter()
                    .max()
                    .map(name)
                    .unwrap_or_default();
                format!(
                    "You put together {made}. Type \"datasheet {}\" to see how it measures up.",
                    made.trim_start_matches("the ")
                )
            }
            (Intent::Disassemble { .. }, Some(&Change::Disassemble { .. })) => {
                "You take it apart.".into()
            }
            (Intent::Eat { item }, _) => format!("You eat the {item}."),
            (Intent::Drink { .. }, Some(Change::Shift { take, .. })) => format!(
                "You drink {} of {}.",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition(take)
            ),
            (Intent::Gather { .. }, Some(Change::Exert { .. }) | None) => {
                "You search but find nothing.".into()
            }
            (Intent::Divide { .. }, Some(Change::Split { take, .. })) => format!(
                "You pull it apart, and now hold {} of it in each hand.",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX))
            ),
            (Intent::Gather { .. }, Some(Change::Split { take, .. })) => format!(
                "You find {} of {}.",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition(take)
            ),
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
            &Change::Refine { entity, tolerance } => format!(
                "{} refined to {}",
                w.label(entity),
                units::show(u128::from(tolerance), units::show::LENGTH, 3)
            ),
            Change::Assemble { design, parts, .. } => {
                format!("{} parts assembled into a {design}", parts.len())
            }
            &Change::Disassemble { assembly } => format!("{} taken apart", w.key(assembly)),
            Change::Shift { from, to, take } => format!(
                "{} of {} from {} into {}",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition(take),
                w.label(*from),
                w.label(*to)
            ),
            Change::Release { from, take, place } => format!(
                "{} of {} from {} into the surroundings of {}",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition(take),
                w.label(*from),
                w.label(*place)
            ),
            &Change::Exert { agent, until } => {
                format!("{} works hard until {until} s", w.label(agent))
            }
            Change::Die { agent, cause } => format!("{} dies of {cause}", w.label(*agent)),
            &Change::StartActivity { agent, .. } => format!("{} starts rubbing", w.label(agent)),
            &Change::EndActivity { agent } => format!("{} stops", w.label(agent)),
        }
    }

    fn datasheet(&self, name: &str) -> String {
        let name = if name.is_empty() { "here" } else { name };
        let Some(id) = laws::find_reachable(&self.world, self.player, name) else {
            return format!("You don't see {name} here.");
        };
        let mut lines = vec![format!(
            "Datasheet: {} ({})",
            self.world.label(id),
            self.world.key(id)
        )];
        lines.extend(format_datasheet(&datasheet::measure(&self.world, id)));
        lines.join("\n")
    }

    fn designs(&self) -> String {
        let w = &self.world;
        if w.designs().is_empty() {
            return "There are no designs in this world.".into();
        }
        w.designs()
            .values()
            .map(|d| {
                let slots: Vec<String> = d
                    .slots
                    .iter()
                    .map(|(slot, requirement)| {
                        let needs = match requirement {
                            Requirement::Shape(shape) => w
                                .shapes()
                                .get(shape)
                                .map_or(shape.clone(), |s| s.label.clone()),
                            Requirement::Design(design) => w
                                .designs()
                                .get(design)
                                .map_or(design.clone(), |x| x.label.clone()),
                            Requirement::Material(material) => {
                                format!("any piece of {}", w.materials()[material].label)
                            }
                        };
                        format!("{slot}: {needs}")
                    })
                    .collect();
                format!("{}: {}", d.label, slots.join(", "))
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

/// A datasheet as indented lines.
pub fn format_datasheet(sheet: &Datasheet) -> Vec<String> {
    let mut lines = Vec::new();
    if !sheet.made_of.is_empty() {
        let parts: Vec<String> = sheet
            .made_of
            .iter()
            .map(|(label, share)| {
                format!(
                    "{label} {}",
                    units::show(u128::from(*share), &[("%", 100)], 2).replace(" %", "%")
                )
            })
            .collect();
        lines.push(format!("  made of: {}", parts.join(", ")));
    }
    if !sheet.parts.is_empty() {
        let parts: Vec<String> = sheet
            .parts
            .iter()
            .map(|(slot, label)| format!("{slot}: {label}"))
            .collect();
        lines.push(format!("  parts: {}", parts.join("; ")));
    }
    for (property, value) in &sheet.entries {
        lines.push(format!("  {}: {value}", property.name()));
    }
    lines
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
