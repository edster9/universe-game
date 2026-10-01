//! One person at the console, acting in one world.

use engine::datasheet::{self, Datasheet};
use engine::gate::{Cause, Change, Holder};
use engine::intent::{self, Command, Intent};
use engine::laws::{self, ActError};
use engine::matter;
use engine::nature;
use engine::units::{self, Credits, Energy, Mass};
use engine::view::{self, Thing};
use engine::world::{Claim, EntityId, News, Requirement, World};

mod queue;

pub const HELP: &str = "\
Commands:
  look                              describe where you are
  inventory                         what you carry, and your credits
  backpack                          what you carry, a line each
  body [all]                        how your body is: its vitals, or everything measured
  go <place>                        walk somewhere
  walk to <thing>                   walk up to something in this place, within reach (\"go to\" too)
  walk to <east> <north>            walk to a spot, in metres from the middle of this place
  stop                              stop what you're doing; a walk stops where you've got to
  take <thing> [from <container>]   pick something up
  drop <thing>                      put something down; \"drop all\" puts down everything
  put <thing> in <container>        put something inside something
  give <thing> to <person>          hand something over
  pay <person> <amount>             pay credits
  dig <source> with <tool>          dig material out of the ground
  light <thing>                     light a fire in something that burns fuel
  pour <liquid> into <container>    pour something molten
  work <thing> into <shape> with <tool>
                                    shape something with a tool
  rub <thing> against <thing>       rub two parts together to make both finer (10 minutes)
  assemble <design>                 put carried parts together to a design (or \"make\")
  disassemble <thing>               take something apart into its parts
  wear <thing> [on your feet]       wear something soft you carry; \"take off <thing>\"
  join <thing> and <thing>          put things together without a design
  call <thing> a <word>             name something in your own words (\"call it a …\")
  tell <person> that <thing> is a <word>
                                    teach someone your word for something
  wait [seconds]                    let time pass; \"wait until free\", \"wait until 08:30\"
  start <command>                   start something without waiting for it
  <command> x3                      do it three times (\"gather sticks x3\")
  <command> 500 g                   do it until you carry that much more, or less (\"gather wood 500 g\")
  <command>; <command>              one after another (\"go to sticks; gather sticks x2\")
  as <person>                       act as someone else, hearing how what they started came out
  sleep [for <time>] [in <shelter>]  sleep, if your body needs it; a player can rest for a time
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
    /// What each person last started without waiting, to tell them how it
    /// came out.
    started: std::collections::BTreeMap<EntityId, Intent>,
    announced_death: Option<String>,
    /// How long the last action took.
    spent: u64,
    /// With the clock running on its own: actions start, and how they came
    /// out is told when they're over (`catch_up`).
    real_time: bool,
    /// Whether the player was asleep when last told the news.
    was_asleep: bool,
    /// What the player asked to do more than once, or one after another.
    queue: Option<queue::Queue>,
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
                started: std::collections::BTreeMap::new(),
                announced_death: None,
                spent: 0,
                real_time: false,
                was_asleep: false,
                queue: None,
            }),
            _ => Err(format!(
                "there's no person with the id {player:?} in this world"
            )),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// Runs with the clock going on its own, as a game client does: a
    /// command that takes time starts, as if with `start`, rather than
    /// moving the clock on until it's done.
    pub fn with_real_time(mut self) -> Self {
        self.real_time = true;
        self
    }

    /// What the player hasn't heard yet: how what they started came out,
    /// who went for them, falling asleep, and their own death. Empty if
    /// nothing. For a client, after the clock moves.
    pub fn catch_up(&mut self) -> String {
        let text = if self.world.pending(self.player).is_some() {
            String::new()
        } else if let Some(text) = self.queue_catch_up() {
            text
        } else {
            self.with_outcome(String::new())
        };
        let was_asleep = self.was_asleep;
        let mut text = self.with_collapse(text, was_asleep);
        self.was_asleep = self.world.is_asleep(self.player);
        let died = self.world.life(self.player).and_then(|l| l.died_of.clone());
        if let Some(cause) = died.filter(|_| self.announced_death.is_none()) {
            text = format!("{text}\nYou have died of {cause}.");
            self.announced_death = Some(cause);
        }
        text.trim().to_string()
    }

    pub fn handle(&mut self, line: &str) -> Reply {
        let line = line.trim();
        if let Some(reply) = self.queued(line) {
            return reply;
        }
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
            "as" => self.act_as(rest),
            "start" => self.start(rest),
            "stop" => self.stop(),
            "wait" | "z" => self.wait(rest),
            "datasheet" | "ds" => Reply::say(self.datasheet(rest)),
            "designs" => Reply::say(self.designs()),
            "recall" | "memory" => Reply::say(self.recall()),
            "body" => Reply::say(body(&self.world, self.player, rest == "all").join("\n")),
            "backpack" | "pack" => Reply::say(backpack(&self.world, self.player).join("\n")),
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
            Ok(Command::Act(_)) if self.real_time => self.start(line),
            Ok(Command::Act(intent)) => {
                let started = self.world.tick();
                let was_asleep = self.world.is_asleep(self.player);
                match laws::perform(&mut self.world, self.player, intent.clone()) {
                    Ok(changes) => {
                        let spent = self.world.tick() - started;
                        self.spent = spent;
                        let text = self.describe(&intent, &changes);
                        let text = if spent > 0 && !matches!(intent, Intent::Rub { .. }) {
                            format!("{text} (That took {}.)", units::show_duration(spent))
                        } else {
                            text
                        };
                        let news = self.memory_news(&changes);
                        let text = if news.is_empty() {
                            text
                        } else {
                            format!("{text}\n{}", news.join("\n"))
                        };
                        let text = if matches!(intent, Intent::Sleep { .. }) {
                            text
                        } else {
                            self.with_collapse(text, was_asleep)
                        };
                        Reply::say(text)
                    }
                    Err(ActError::Refused(refusal)) => Reply::refuse(self.refusal(&refusal)),
                    Err(fault @ ActError::Fault(_)) => Reply::refuse(format!("!! {fault}")),
                }
            }
        }
    }

    /// Starts something without waiting for it: the clock doesn't move, and
    /// how it comes out is told when it's over.
    fn start(&mut self, line: &str) -> Reply {
        let intent = match intent::parse(line) {
            Ok(Command::Act(intent)) => intent,
            Ok(_) => return Reply::refuse("Only an action can be started."),
            Err(error) => return Reply::refuse(sentence(&error.to_string())),
        };
        match laws::start(&mut self.world, self.player, intent.clone()) {
            Ok(laws::Started::Due(until)) => {
                self.started.insert(self.player, intent.clone());
                Reply::say(format!(
                    "You start to {intent}. You'll be done at {}.",
                    self.clock(until)
                ))
            }
            Ok(laws::Started::Now { changes, .. }) => Reply::say(self.describe(&intent, &changes)),
            Err(ActError::Refused(refusal)) => Reply::refuse(self.refusal(&refusal)),
            Err(fault @ ActError::Fault(_)) => Reply::refuse(format!("!! {fault}")),
        }
    }

    /// Stops what the player is doing.
    fn stop(&mut self) -> Reply {
        match laws::stop(&mut self.world, self.player) {
            Ok(_) => {
                // They know they stopped: no news of being cut short.
                self.world.take_outcome(self.player);
                self.started.remove(&self.player);
                match self.stop_queue() {
                    Some(done) if !done.is_empty() => Reply::say(format!("You stop.\n{done}")),
                    _ => Reply::say("You stop."),
                }
            }
            // Between one time and the next of something asked for more
            // than once.
            Err(ActError::Refused(_)) if self.queue_busy() => {
                let done = self.stop_queue().unwrap_or_default();
                Reply::say(format!("You stop.\n{done}").trim().to_string())
            }
            Err(ActError::Refused(refusal)) => Reply::refuse(self.refusal(&refusal)),
            Err(fault @ ActError::Fault(_)) => Reply::refuse(format!("!! {fault}")),
        }
    }

    /// Acts as someone else from now on, hearing how anything they'd
    /// started came out.
    fn act_as(&mut self, name: &str) -> Reply {
        let wanted = name.trim().to_lowercase();
        let found = self.world.entities().find(|&id| {
            self.world.is_agent(id)
                && (self.world.key(id).to_lowercase() == wanted
                    || self.world.label(id).to_lowercase() == wanted)
        });
        match found {
            Some(id) => {
                self.player = id;
                let text = format!("You are {}.", self.world.label(id));
                Reply::say(self.with_outcome(text))
            }
            None => Reply::refuse(format!("There's no person called {name:?}.")),
        }
    }

    /// Adds how the player's started action came out, if it's over and they
    /// haven't heard.
    fn with_outcome(&mut self, text: String) -> String {
        let Some(outcome) = self.world.take_outcome(self.player) else {
            return text;
        };
        let intent = self.started.remove(&self.player);
        let news = match (outcome, intent) {
            (engine::world::Outcome::Done(changes), Some(intent)) => {
                self.describe(&intent, &changes)
            }
            (engine::world::Outcome::Done(_), None) => "You finish.".into(),
            (engine::world::Outcome::Failed(why), _) => {
                format!("You couldn't finish: {}", sentence(&why))
            }
            (engine::world::Outcome::Interrupted, _) => {
                "You were cut short, and didn't finish.".into()
            }
        };
        format!("{text}\n{news}")
    }

    /// Lets `seconds` of the world's time pass, as the wall clock does in a
    /// live session.
    pub fn advance(&mut self, seconds: u64) {
        if nature::run(&mut self.world, seconds).is_err() {
            // A fault is a bug in a law; the live channel carries on, and the
            // next reply will show the state as it stands.
        }
    }

    /// A tick as a time of day, in a world with days, or as seconds.
    pub fn clock(&self, tick: u64) -> String {
        let day = self.world.settings().day;
        if day == 0 {
            return format!("{tick} s");
        }
        let t = (tick + self.world.settings().starts_at) % day;
        format!("{:02}:{:02}", t / 3_600, t / 60 % 60)
    }

    fn refusal(&self, refusal: &laws::Refusal) -> String {
        match refusal {
            laws::Refusal::Busy(until) => {
                format!("You're busy until {}.", self.clock(*until))
            }
            other => sentence(&other.to_string()),
        }
    }

    /// Waits a number of seconds ("wait 60"), or a time with a unit ("wait 2
    /// h", "wait 3 day"); until the player's action is over ("wait until
    /// free"); or until a time of day ("wait until 08:30").
    fn wait(&mut self, time: &str) -> Reply {
        if let Some(until) = time.strip_prefix("until ").map(str::trim) {
            return self.wait_until(until);
        }
        let seconds = if time.is_empty() {
            Some(1)
        } else if let Ok(n) = time.parse::<u64>() {
            Some(n)
        } else {
            units::parse_quantity(&short_units(time), units::property::DURATION, "a time").ok()
        };
        let was_asleep = self.world.is_asleep(self.player);
        match seconds {
            Some(n) if (1..=MAX_WAIT).contains(&n) => match nature::run(&mut self.world, n) {
                Ok(()) if n == 1 => {
                    let text = self.with_outcome("A second passes.".into());
                    Reply::say(text)
                }
                Ok(()) => {
                    let text = self
                        .with_collapse(format!("{} passes.", units::show_duration(n)), was_asleep);
                    Reply::say(self.with_outcome(text))
                }
                Err(fault) => Reply::refuse(format!("!! engine fault: {fault}")),
            },
            _ => Reply::refuse("Try \"wait\", \"wait 60\", or \"wait 2 h\" (up to 30 days)."),
        }
    }

    /// Lets time pass until the player is free, or until a time of day.
    fn wait_until(&mut self, until: &str) -> Reply {
        let now = self.world.tick();
        let seconds = if until == "free" || until == "done" {
            match self.world.pending(self.player) {
                Some(pending) => pending.until.saturating_sub(now),
                None => return Reply::say("You aren't busy."),
            }
        } else {
            let day = self.world.settings().day;
            let parsed = until.split_once(':').and_then(|(h, m)| {
                Some(h.trim().parse::<u64>().ok()? * 3_600 + m.trim().parse::<u64>().ok()? * 60)
            });
            match (parsed, day) {
                (Some(at), day) if day > 0 && at < day => {
                    let of_day = (now + self.world.settings().starts_at) % day;
                    (at + day - of_day) % day
                }
                _ => {
                    return Reply::refuse(
                        "Try \"wait until free\", or a time of day like \"wait until 08:30\".",
                    );
                }
            }
        };
        if seconds == 0 {
            return Reply::say(self.with_outcome("No time passes.".into()));
        }
        let was_asleep = self.world.is_asleep(self.player);
        match nature::run(&mut self.world, seconds) {
            Ok(()) => {
                let text = self.with_collapse(
                    format!("{} passes.", units::show_duration(seconds)),
                    was_asleep,
                );
                Reply::say(self.with_outcome(text))
            }
            Err(fault) => Reply::refuse(format!("!! engine fault: {fault}")),
        }
    }

    /// Adds news of the player having dropped asleep from exhaustion, if
    /// they were awake before (`was_asleep` is false), and of anyone who went
    /// for them since they last heard.
    fn with_collapse(&mut self, text: String, was_asleep: bool) -> String {
        let mut text = text;
        for news in self.world.take_news(self.player) {
            let who = |by| sentence_case(&self.world.label_for(self.player, by));
            text = match news {
                News::Attacked {
                    by,
                    wound: Some(rate),
                } => format!(
                    "{text}\n{} goes for you, and wounds you: you're bleeding {} a second.",
                    who(by),
                    Mass::from_mg(rate)
                ),
                News::Attacked { by, wound: None } => {
                    format!("{text}\n{} goes for you, and misses.", who(by))
                }
                News::Took { by, thing } => format!(
                    "{text}\n{} takes your {}.",
                    who(by),
                    self.world.label_for(self.player, thing)
                ),
            };
        }
        if !was_asleep && self.world.is_asleep(self.player) {
            format!("{text}\nYou're exhausted, and fall asleep where you are.")
        } else {
            text
        }
    }

    /// What the player's memory learned from what they just did: things gone
    /// from where they arrived, and what they were told holding or not.
    fn memory_news(&self, changes: &[Change]) -> Vec<String> {
        let w = &self.world;
        let mut news = Vec::new();
        for change in changes {
            match change {
                Change::Sight { gone, .. } if !gone.is_empty() => {
                    let labels: Vec<String> =
                        gone.iter().map(|&g| w.label_for(self.player, g)).collect();
                    news.push(format!(
                        "Gone since you were last here: {}.",
                        labels.join(", ")
                    ));
                }
                Change::Settle {
                    claim: Claim::Thing(_, name),
                    held: true,
                    source,
                    ..
                } => news.push(format!(
                    "{} is here, as {source} showed.",
                    sentence_case(name)
                )),
                Change::Settle {
                    claim: Claim::Thing(_, name),
                    held: false,
                    source,
                    ..
                } => news.push(format!(
                    "{} showed {name} here, but there's none. You correct your memory.",
                    sentence_case(source)
                )),
                _ => {}
            }
        }
        news
    }

    /// What the player remembers: what's certain, what's only possible and
    /// where it came from, and how often they've had to correct themselves.
    fn recall(&self) -> String {
        let w = &self.world;
        let Some(memory) = w.memory(self.player) else {
            return "You don't remember anything.".into();
        };
        let places: Vec<String> = memory
            .places
            .iter()
            .map(|&p| w.label_for(self.player, p))
            .collect();
        let mut lines = vec![format!("Places you know: {}.", list_or(&places, "none"))];
        if memory.finds_ways {
            let ways: Vec<String> = memory
                .ways
                .iter()
                .map(|&(a, b)| {
                    format!(
                        "{} to {}",
                        w.label_for(self.player, a),
                        w.label_for(self.player, b)
                    )
                })
                .collect();
            lines.push(format!("Ways you know: {}.", list_or(&ways, "none")));
        }
        if memory.possible.is_empty() {
            lines.push("Nothing you've been told is still unconfirmed.".into());
        } else {
            lines.push("Possible, but not yet seen for yourself:".into());
            for (claim, source) in &memory.possible {
                lines.push(format!("  {}, from {source}", claim_text(w, claim)));
            }
        }
        lines.push(format!(
            "Confirmed with your own eyes: {}. Corrected: {}.",
            memory.confirmed, memory.corrected
        ));
        lines.join("\n")
    }

    pub fn player(&self) -> EntityId {
        self.player
    }

    fn look(&self) -> String {
        let Some(look) = view::look(&self.world, self.player) else {
            return "You aren't anywhere.".into();
        };
        let mut lines = vec![sentence_case(&look.place)];
        if let Some((day, time)) = look.time {
            let light = if look.dark {
                "night, and dark"
            } else if look.night {
                "night, lit by fire"
            } else {
                "daylight"
            };
            lines.push(format!(
                "Day {day}, {:02}:{:02}, {light}.",
                time / 3_600,
                time / 60 % 60
            ));
        }
        let ways = if look.finding_ways {
            "Ways out you know"
        } else {
            "Ways out"
        };
        lines.push(format!("{ways}: {}", list_or(&look.exits, "none")));
        if !look.people.is_empty() {
            lines.push(format!("Also here: {}", look.people.join(", ")));
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
            let mut lines = vec![format!(
                "You're carrying {}, {} in all.",
                things(&inv.things),
                inv.carried
            )];
            for container in inv.things.iter().filter(|t| !t.contents.is_empty()) {
                lines.push(format!(
                    "In {}: {}",
                    the(&container.label),
                    things(&container.contents)
                ));
            }
            lines.join("\n")
        };
        format!("{carrying}\nYou have {}.", inv.credits)
    }

    fn describe(&self, intent: &Intent, changes: &[Change]) -> String {
        let w = &self.world;
        let name = |id: EntityId| laws::named(w, self.player, id);
        // A walk within the place: "go to the grass", "walk to 12 -3".
        let walked_to = match intent {
            Intent::Walk { to } | Intent::Go { place: to, .. } => {
                Some(to.strip_prefix("to ").unwrap_or(to))
            }
            _ => None,
        };
        match (walked_to, changes.first()) {
            (Some(to), Some(&Change::Spot { entity, .. })) if entity == self.player => {
                return format!("You walk to {to}.");
            }
            (Some(_), None) => return "You're already there.".into(),
            _ => {}
        }
        match (intent, changes.first()) {
            (
                Intent::Go {
                    aboard: Some(vessel),
                    ..
                },
                Some(&Change::Move { to, .. }),
            ) => {
                let vessel = changes
                    .get(1)
                    .and_then(|c| match c {
                        &Change::Move { entity, .. } => Some(name(entity)),
                        _ => None,
                    })
                    .unwrap_or_else(|| vessel.clone());
                format!("You cross to {} on {vessel}.", w.label_for(self.player, to))
            }
            (Intent::Go { .. }, Some(&Change::Move { to, .. })) => {
                let mut text = format!("You go to {}.", w.label_for(self.player, to));
                // What the ground did to your feet, or to what's on them.
                for change in changes {
                    match *change {
                        Change::Wound { agent, rate } if agent == self.player => {
                            text = format!(
                                "{text} The rough ground cuts your feet: you're bleeding {} a second.",
                                Mass::from_mg(rate)
                            );
                        }
                        Change::Split { from, .. } => {
                            let shoe = w.underfoot(self.player).filter(|&s| w.sole(s) == from);
                            if let Some(shoe) = shoe.filter(|&s| w.worn_through(s)) {
                                text = format!(
                                    "{text} Your {} wore through.",
                                    name(shoe).trim_start_matches("the ")
                                );
                            }
                        }
                        _ => {}
                    }
                }
                text
            }
            (
                Intent::Go { .. },
                Some(Change::Settle {
                    claim: Claim::Way(_, to),
                    source,
                    ..
                }),
            ) => format!(
                "You look for the way to {} that {source} showed, but there's none. You correct your memory.",
                w.label_for(self.player, *to)
            ),
            (Intent::Attack { with, .. }, Some(&Change::Wound { agent, rate })) => {
                let with = with
                    .as_ref()
                    .map(|t| format!(" with the {t}"))
                    .unwrap_or_default();
                // People are "them"; creatures, "it".
                let (them, they) = if w.memory(agent).is_some() {
                    ("them", "they're")
                } else {
                    ("it", "it's")
                };
                format!(
                    "You strike {}{with}, and wound {them}: {they} bleeding {} a second.",
                    name(agent),
                    Mass::from_mg(rate)
                )
            }
            (Intent::Attack { .. }, _) => "You strike, and miss.".into(),
            (Intent::Butcher { .. }, _) => {
                let pieces: Vec<String> = changes
                    .iter()
                    .filter_map(|c| match c {
                        Change::Split { take, .. } => Some(format!(
                            "{} ({})",
                            w.describe_composition_for(self.player, take),
                            Mass::from_mg(
                                u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)
                            )
                        )),
                        _ => None,
                    })
                    .collect();
                format!("You cut it up: {}.", pieces.join(", "))
            }
            (Intent::Read { .. }, _) => {
                let shown: Vec<String> = changes
                    .iter()
                    .filter_map(|c| match c {
                        Change::Hear { claim, .. } => Some(claim_text(w, claim)),
                        _ => None,
                    })
                    .collect();
                if shown.is_empty() {
                    "You study it, but it shows nothing you don't already know.".into()
                } else {
                    format!(
                        "You study it. It shows {}. You can't be sure of any of it until you see it.",
                        shown.join("; ")
                    )
                }
            }
            (Intent::Take { .. } | Intent::TakeFrom { .. }, Some(&Change::Move { entity, .. })) => {
                // Anyone who saw it taken from them.
                let seen: Vec<String> = changes
                    .iter()
                    .filter_map(|c| match c {
                        Change::Notice {
                            agent,
                            news: News::Took { .. },
                        } => Some(sentence_case(&name(*agent))),
                        _ => None,
                    })
                    .collect();
                let mut text = format!("You take {}.", name(entity));
                for owner in seen {
                    text.push_str(&format!(" {owner} sees you take it: it's theirs."));
                }
                text
            }
            (Intent::Offer { .. }, Some(&Change::Move { entity, to })) => {
                let got = changes.get(1).and_then(|c| match *c {
                    Change::Move { entity, .. } => Some(entity),
                    _ => None,
                });
                format!(
                    "{} takes {} and gives you {}.",
                    sentence_case(&name(to)),
                    name(entity),
                    got.map_or("nothing".into(), name)
                )
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
            (Intent::Fill { .. }, Some(Change::Split { take, at, .. })) => format!(
                "You fill {} with {} of {}.",
                name(*at),
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition_for(self.player, take)
            ),
            (Intent::Dig { .. }, Some(Change::Split { take, .. })) => format!(
                "You dig out {} of {}.",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition_for(self.player, take)
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
                let mut text = format!(
                    "You rub them together for {}.",
                    units::show_duration(self.spent)
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
                        w.label_for(self.player, dust),
                        w.temperature(dust).unwrap_or_default()
                    );
                }
                text
            }
            (Intent::Assemble { .. }, Some(Change::Assemble { at, .. })) => {
                let made = w
                    .contents(*at)
                    .into_iter()
                    .max()
                    .map(name)
                    .unwrap_or_default();
                let left = if *at == self.player {
                    ""
                } else {
                    " It's too heavy to carry, so it stays here."
                };
                format!(
                    "You put together {made}.{left} Type \"datasheet {}\" to see how it measures up.",
                    made.trim_start_matches("the ")
                )
            }
            (Intent::Join { .. }, Some(Change::Assemble { at, .. })) => {
                let made = w.contents(*at).into_iter().max();
                let left = if *at == self.player {
                    ""
                } else {
                    " It's too heavy to carry, so it stays here."
                };
                match made {
                    Some(made)
                        if w.has_words(self.player) && w.recognise(self.player, made).is_none() =>
                    {
                        format!(
                            "You've made something new: {}.{left} What do you call it? (\"call it a …\")",
                            w.label_for(self.player, made)
                        )
                    }
                    Some(made) => format!("You put together {}.{left}", name(made)),
                    None => "You put them together.".into(),
                }
            }
            (Intent::Call { word, .. }, _) => {
                let recipe = if changes.iter().any(|c| matches!(c, Change::Recipe { .. })) {
                    format!(" You remember how you made it: \"make {word}\" will make another.")
                } else {
                    String::new()
                };
                format!("You call it {word}.{recipe}")
            }
            (Intent::Ask { request, .. }, Some(&Change::Request { agent, .. })) => {
                format!("{} agrees to {request}.", sentence_case(&name(agent)))
            }
            (Intent::Tell { word, .. }, Some(&Change::Word { agent, .. })) => {
                format!("You tell {} it's a {word}.", name(agent))
            }
            (
                Intent::Wear { .. },
                Some(Change::Wear {
                    item,
                    worn: Some(worn),
                    ..
                }),
            ) => match worn.on {
                engine::world::Covering::Feet => {
                    format!("You wear {} on your feet.", name(*item))
                }
                engine::world::Covering::Body => format!("You put on {}.", name(*item)),
            },
            (Intent::TakeOff { .. }, Some(Change::Wear { item, .. })) => {
                format!("You take off {}.", name(*item))
            }
            (Intent::Disassemble { .. }, Some(&Change::Disassemble { .. })) => {
                "You take it apart.".into()
            }
            (Intent::Eat { item }, _) => format!("You eat the {item}."),
            (Intent::Survey, _) => {
                let sights = laws::in_sight(w, w.location(self.player).expect("somewhere"));
                if sights.is_empty() {
                    "You take in the view, but nothing stands out in the distance.".into()
                } else {
                    let lines: Vec<String> = sights
                        .iter()
                        .map(|s| {
                            let km = (s.distance + 500_000_000) / 1_000_000_000;
                            let far = if km == 0 {
                                "under a kilometre away".to_string()
                            } else {
                                format!("about {km} km away")
                            };
                            format!(
                                "To the {}, {far}: {}.",
                                s.direction,
                                w.from_afar(s.place).unwrap_or_default()
                            )
                        })
                        .collect();
                    format!("You take in the view.\n{}", lines.join("\n"))
                }
            }
            (Intent::Explore, Some(&Change::Learn { to, .. })) => {
                format!("You find a way to {}.", w.label_for(self.player, to))
            }
            (Intent::Explore, _) => "You search around, but find no new way out.".into(),
            (Intent::Sleep { .. }, None) => {
                let time = w
                    .time_of_day()
                    .map(|t| format!(" until {:02}:{:02}", t / 3_600, t / 60 % 60))
                    .unwrap_or_default();
                format!("You rest{time}.")
            }
            (Intent::Sleep { .. }, first) => {
                let time = w
                    .time_of_day()
                    .map(|t| format!(" at {:02}:{:02}", t / 3_600, t / 60 % 60))
                    .unwrap_or_default();
                let inside = match first {
                    Some(&Change::Sleep {
                        shelter: Some(shelter),
                        ..
                    }) => format!(" in {}", name(shelter)),
                    _ => String::new(),
                };
                format!("You sleep{inside}, and wake{time}.")
            }
            (Intent::Drink { .. }, Some(Change::Shift { take, .. })) => format!(
                "You drink {} of {}.",
                Mass::from_mg(u64::try_from(matter::total_mass(take)).unwrap_or(u64::MAX)),
                w.describe_composition_for(self.player, take)
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
                w.describe_composition_for(self.player, take)
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
            &Change::Spot { entity, at } => format!(
                "{} steps to {:.1} m east, {:.1} m north",
                w.label(entity),
                at.0 as f64 / 1e6,
                at.1 as f64 / 1e6
            ),
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
            Change::Assemble { design, parts, .. } => match design {
                Some(design) => format!("{} parts assembled into a {design}", parts.len()),
                None => format!("{} parts joined into something new", parts.len()),
            },
            Change::Word { agent, word, .. } => {
                format!("{} learns the word {word:?}", w.label(*agent))
            }
            Change::Recipe { agent, word, .. } => {
                format!("{} learns a way to make a {word}", w.label(*agent))
            }
            &Change::Made { agent, thing } => {
                format!("{} made {}", w.label(agent), w.key(thing))
            }
            Change::Begin {
                agent,
                intent,
                until,
            } => format!("{} starts to {intent}, due at {until} s", w.label(*agent)),
            Change::End { agent, outcome } => match outcome {
                engine::world::Outcome::Done(changes) => {
                    format!(
                        "{} finishes, with {} changes",
                        w.label(*agent),
                        changes.len()
                    )
                }
                engine::world::Outcome::Failed(why) => {
                    format!("{} finishes, but can't: {why}", w.label(*agent))
                }
                engine::world::Outcome::Interrupted => format!("{} is cut short", w.label(*agent)),
            },
            &Change::Vitality {
                entity,
                inflow,
                drawn,
                fluid,
                ..
            } => format!(
                "{} takes in {} of vitality, draws {} of stamina, and gets back {fluid}",
                w.label(entity),
                Energy::from_uj(inflow),
                Energy::from_uj(drawn)
            ),
            Change::Wear { agent, item, worn } => match worn {
                Some(_) => format!("{} wears {}", w.label(*agent), w.label(*item)),
                None => format!("{} takes off {}", w.label(*agent), w.label(*item)),
            },
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
            &Change::Store {
                entity,
                from,
                mass,
                into,
                ..
            } => format!(
                "{} stores {mass} of {} as {}",
                w.label(entity),
                w.materials()[&from].label,
                w.materials()[&into].label
            ),
            Change::Hear {
                agent,
                claim,
                source,
            } => format!(
                "{} hears of {} from {source}",
                w.label(*agent),
                claim_text(w, claim)
            ),
            Change::Sight {
                agent, place, gone, ..
            } => format!(
                "{} sees what's at {} ({} gone)",
                w.label(*agent),
                w.label(*place),
                gone.len()
            ),
            Change::Settle {
                agent, claim, held, ..
            } => format!(
                "{} finds {} {}",
                w.label(*agent),
                claim_text(w, claim),
                if *held { "true" } else { "false" }
            ),
            &Change::Avoid {
                agent,
                place,
                until,
            } => format!(
                "{} keeps away from {} until {until} s",
                w.label(agent),
                w.label(place)
            ),
            &Change::Wound { agent, rate } => format!(
                "{} is wounded, bleeding {} a second",
                w.label(agent),
                Mass::from_mg(rate)
            ),
            Change::Notice {
                agent,
                news: News::Attacked { by, .. },
            } => format!("{} knows {} went for them", w.label(*agent), w.label(*by)),
            Change::Notice {
                agent,
                news: News::Took { by, thing },
            } => format!(
                "{} sees {} take {}",
                w.label(*agent),
                w.label(*by),
                w.label(*thing)
            ),
            &Change::Struck { agent, at } => {
                format!("{} remembers going for {}", w.label(agent), w.label(at))
            }
            Change::Request { agent, intent } => {
                format!("{} takes on: {intent}", w.label(*agent))
            }
            &Change::TakeUp { agent } => format!("{} takes up a request", w.label(agent)),
            &Change::Occupy { agent, until } => {
                format!("{} is busy until {until} s", w.label(agent))
            }
            &Change::See { agent, place } => {
                format!("{} sees {}", w.label(agent), w.label(place))
            }
            &Change::Learn { agent, to, .. } => {
                format!("{} learns the way to {}", w.label(agent), w.label(to))
            }
            &Change::Warm { entity, amount, .. } => {
                format!("{} warms by {amount} from the air", w.label(entity))
            }
            &Change::Sleep { agent, until, .. } => {
                format!("{} sleeps until {until} s", w.label(agent))
            }
            Change::Die { agent, cause } => format!("{} dies of {cause}", w.label(*agent)),
            &Change::StartActivity { agent, .. } => format!("{} starts rubbing", w.label(agent)),
            &Change::EndActivity { agent } => format!("{} stops", w.label(agent)),
            &Change::Grow { entity, mass, .. } => format!("{} grows by {mass}", w.label(entity)),
            &Change::Transform {
                entity,
                from,
                to,
                mass,
            } => format!(
                "{mass} of {} in {} becomes {}",
                w.materials()[&from].label,
                w.label(entity),
                w.materials()[&to].label
            ),
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
/// The vitals, in the order a body window shows them: water, food,
/// strength, rest, warmth, and wounds.
const VITALS: &[datasheet::Property] = &[
    datasheet::Property::DiedOf,
    datasheet::Property::BodyFluid,
    datasheet::Property::StoredEnergy,
    datasheet::Property::Stamina,
    datasheet::Property::Awake,
    datasheet::Property::Temperature,
    datasheet::Property::Bleeding,
    datasheet::Property::Working,
];

/// How a person's body is: what the engine measures about it, the same
/// numbers `datasheet me` shows, never written. Just the vitals, or with
/// `all`, everything measured; and what they're wearing.
pub fn body(world: &World, who: EntityId, all: bool) -> Vec<String> {
    let sheet = datasheet::measure(world, who);
    let mut lines: Vec<String> = if all {
        format_datasheet(&sheet)
            .into_iter()
            .map(|l| l.trim().to_string())
            .collect()
    } else {
        VITALS
            .iter()
            .filter_map(|&p| sheet.get(p).map(|v| format!("{}: {v}", p.name())))
            .collect()
    };
    let worn: Vec<String> = view::inventory(world, who)
        .things
        .iter()
        .filter(|t| t.notes.iter().any(|n| n == "worn" || n == "on your feet"))
        .map(|t| t.label.clone())
        .collect();
    lines.push(format!("wearing: {}", list_or(&worn, "nothing")));
    lines
}

/// What a person carries, a line each: alike things together, with how
/// many and how much, and what's inside anything; then the load.
pub fn backpack(world: &World, who: EntityId) -> Vec<String> {
    let inv = view::inventory(world, who);
    let mut lines = Vec::new();
    // Label, how many, their mass in all, the first one's mass, and what's
    // inside.
    let mut shown: Vec<(String, u64, Mass, Mass, Vec<Thing>)> = Vec::new();
    for thing in &inv.things {
        let label = if thing.notes.is_empty() {
            thing.label.clone()
        } else {
            format!("{} ({})", thing.label, thing.notes.join(", "))
        };
        // Alike means the same name and about the same size: a stick and a
        // twig may both be a lump of wood, but they aren't alike.
        let about = |a: Mass, b: Mass| a.mg() * 4 <= b.mg() * 5 && b.mg() * 4 <= a.mg() * 5;
        match shown.iter_mut().find(|(l, _, _, first, inner)| {
            *l == label
                && about(*first, thing.mass)
                && inner.is_empty()
                && thing.contents.is_empty()
        }) {
            Some((_, n, mass, _, _)) => {
                *n += 1;
                *mass = Mass::from_mg(mass.mg() + thing.mass.mg());
            }
            None => shown.push((label, 1, thing.mass, thing.mass, thing.contents.clone())),
        }
    }
    for (label, n, mass, _, inner) in shown {
        if n > 1 {
            let each = Mass::from_mg(mass.mg() / n);
            lines.push(format!("{label} x{n}: {mass}, about {each} each"));
        } else {
            lines.push(format!("{label}: {mass}"));
        }
        for thing in inner {
            lines.push(format!("  {}: {}", thing.label, thing.mass));
        }
    }
    if lines.is_empty() {
        lines.push("nothing".into());
    }
    let load = match datasheet::measure(world, who).get(datasheet::Property::Carrying) {
        Some(v) => format!("carrying {v}"),
        None => format!("carrying {}", inv.carried),
    };
    lines.push(load);
    if inv.credits > Credits::ZERO {
        lines.push(format!("{}", inv.credits));
    }
    lines
}

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
            details.extend(
                t.away
                    .map(|(gap, way)| format!("{} {way}", laws::metres(gap))),
            );
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

/// Times as people say them, in the units the console reads: "10
/// minutes" is "10 min", "2 hours" is "2 h".
fn short_units(time: &str) -> String {
    time.split_whitespace()
        .map(|word| match word {
            "second" | "seconds" | "sec" | "secs" => "s",
            "minute" | "minutes" | "mins" => "min",
            "hour" | "hours" => "h",
            "days" => "day",
            other => other,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Turns an engine message into a sentence: capitalised, with a full stop.
fn sentence(text: &str) -> String {
    let text = sentence_case(text);
    if text.ends_with(['.', '!', '?']) || text.ends_with(".\"") {
        text
    } else {
        format!("{text}.")
    }
}

/// Something a person was told, in words.
fn claim_text(world: &engine::world::World, claim: &Claim) -> String {
    match claim {
        Claim::Place(p) => world.label(*p),
        Claim::Way(a, b) => format!("a way from {} to {}", world.label(*a), world.label(*b)),
        Claim::Thing(at, name) => format!("{name} at {}", world.label(*at)),
    }
}
