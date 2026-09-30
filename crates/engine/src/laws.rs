//! The laws decide whether an intent is allowed and which changes it makes.
//! They never change the world themselves: they hand changes to the gate.
//!
//! Names only resolve among what the actor can perceive. Nothing here asks
//! what something *is* anywhere else in the world (docs/slices.md, "No
//! oracles").

use std::collections::BTreeSet;
use std::fmt;

use crate::datasheet::{self, Datasheet, Property, Value};
use crate::gate::{Cause, Change, Fault};
use crate::intent::Intent;
use crate::matter::{self, Composition, MaterialId, State};
use crate::nature;
use crate::units::{Credits, Mass};
use crate::words::Recipe;
use crate::world::{Claim, Covering, EntityId, Outcome, Requirement, World, Worn};

/// How long it takes to take in what a map shows, in seconds.
const READING_TIME: u64 = 300;

/// How long a blow takes, in seconds.
const STRIKING_TIME: u64 = 5;

/// How long it takes to butcher a body, in seconds.
const BUTCHERING_TIME: u64 = 1_800;

/// Processes a person must know how to do. Knowledge becomes real in slice 4
/// (docs/ideas/knowledge.md). Until then everyone knows everything, but every
/// process already asks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Process {
    Dig,
    Light,
    Pour,
    Work,
    Assemble,
    Gather,
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Process::Dig => "dig",
            Process::Light => "light a fire",
            Process::Pour => "pour",
            Process::Work => "shape things",
            Process::Assemble => "put that together or take it apart",
            Process::Gather => "gather",
        })
    }
}

/// Why the laws refused an intent. A refused intent changes nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    NotAnAgent,
    Nowhere,
    NoSuchExit(String),
    NotHere(String),
    InContainer {
        item: String,
        container: String,
    },
    NotInside {
        item: String,
        container: String,
    },
    AlreadyCarrying(String),
    /// A name fits several things that look different.
    Which {
        name: String,
        options: Vec<String>,
    },
    /// Nothing has just been made for "it" to mean.
    NothingMade,
    /// Busy with something else until this tick.
    Busy(u64),
    /// Couldn't be carried out when it was due: why.
    Later(String),
    /// Cut short, by a wound.
    Interrupted,
    /// Too stiff to wear.
    TooStiff(String),
    /// What's in it is held higher than the actor can get.
    OutOfReach(String),
    /// Something else is already on the feet.
    FeetCovered(String),
    NotWearing(String),
    AlreadyWearing(String),
    CannotWear,
    /// Only someone with words of their own can learn one.
    NoWordsToLearn(String),
    /// Someone asked is asleep, and doesn't hear.
    TheyreAsleep(String),
    /// Someone without a mind of their own decides for themselves.
    OwnMind(String),
    /// Someone asked, whose mind is confined, doesn't do that.
    NotTheirWork(String),
    /// Someone asked can't do it, and says why, in their own words.
    TheySay {
        who: String,
        why: String,
    },
    NotCarrying(String),
    CannotCarry(String),
    NotAContainer(String),
    IntoItself,
    NoOneHere(String),
    NotYourself,
    ZeroAmount,
    NotEnough {
        have: Credits,
        want: Credits,
    },
    TooMuch,
    DoesntKnowHow(Process),
    NotDiggable(String),
    NotATool(String),
    TooHard {
        tool: String,
        target: String,
    },
    Exhausted(String),
    NotAChamber(String),
    AlreadyLit(String),
    NoFuel(String),
    NotLiquid(String),
    WouldMelt(String),
    UnknownShape(String),
    NotSolid(String),
    TooHot(String),
    CannotWork(String),
    UnknownDesign(String),
    MissingPart {
        slot: String,
        needs: String,
    },
    NotAnAssembly(String),
    NotAPart(String),
    AsFineAsItGets(String),
    CannotEat(String),
    NotDrinkable(String),
    NotThirsty,
    NotGatherable(String),
    NeedsTool {
        source: String,
        tool: String,
    },
    NoFlame(String),
    TooHeavy {
        carrying: Mass,
        limit: Mass,
    },
    TooWeak {
        part: String,
        holds: Mass,
        load: Mass,
    },
    MustCross {
        place: String,
        liquid: String,
    },
    NothingToCross(String),
    NotAVessel(String),
    NothingToRead(String),
    NoEdge(Option<String>),
    NotDead(String),
    TooDarkToSee,
    NotAShelter(String),
    PutItDown(String),
    DontKnowWay(String),
    CannotFill(String),
    WouldSoften {
        container: String,
        liquid: String,
    },
    Full(String),
    Asleep,
    TooDark,
    NotTired,
    NeverSleeps,
    Sinks {
        vessel: String,
        liquid: String,
    },
    WouldSink {
        vessel: String,
        carries: Mass,
        load: Mass,
    },
    TheyCantCarry(String),
    WouldSpoil(String),
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::NotAnAgent => write!(f, "only a person can do that"),
            Refusal::Nowhere => write!(f, "you aren't standing anywhere"),
            Refusal::NoSuchExit(name) => write!(f, "there's no way to {name} from here"),
            Refusal::NotHere(name) => write!(f, "you don't see {name} here"),
            Refusal::InContainer { item, container } => {
                write!(
                    f,
                    "{item} is in {container}. Try \"take {item} from {container}\""
                )
            }
            Refusal::NotInside { item, container } => write!(f, "there's no {item} in {container}"),
            Refusal::AlreadyCarrying(name) => write!(f, "you're already carrying {name}"),
            Refusal::Which { name, options } => {
                let (last, rest) = options.split_last().expect("at least two");
                write!(f, "which {name}: {}, or {last}?", rest.join(", "))
            }
            Refusal::NothingMade => write!(f, "you haven't made anything to name"),
            Refusal::Busy(until) => write!(f, "you're busy until {until} s"),
            Refusal::Later(why) => write!(f, "{why}"),
            Refusal::Interrupted => write!(f, "you were cut short"),
            Refusal::TooStiff(name) => write!(f, "{name} is too stiff to wear"),
            Refusal::OutOfReach(name) => write!(f, "{name} holds things higher than you can get"),
            Refusal::FeetCovered(name) => write!(f, "you're already wearing {name} on your feet"),
            Refusal::NotWearing(name) => write!(f, "you aren't wearing {name}"),
            Refusal::AlreadyWearing(name) => write!(f, "you're already wearing {name}"),
            Refusal::CannotWear => write!(f, "only a living body can wear things"),
            Refusal::TheyreAsleep(name) => write!(f, "{name} is asleep"),
            Refusal::OwnMind(name) => write!(f, "{name} decides for themselves"),
            Refusal::NotTheirWork(name) => write!(f, "{name} won't: it isn't what they do"),
            Refusal::TheySay { who, why } => {
                let stop = if why.ends_with(['.', '?', '!']) {
                    ""
                } else {
                    "."
                };
                write!(f, "{who} says, \"{why}{stop}\"")
            }
            Refusal::NoWordsToLearn(name) => {
                write!(f, "{name} already knows every word in this world")
            }
            Refusal::NotCarrying(name) => write!(f, "you aren't carrying {name}"),
            Refusal::CannotCarry(name) => write!(f, "you can't carry {name}"),
            Refusal::NotAContainer(name) => write!(f, "you can't put things in {name}"),
            Refusal::IntoItself => write!(f, "you can't put something inside itself"),
            Refusal::NoOneHere(name) => write!(f, "there's nobody called {name} here"),
            Refusal::NotYourself => write!(f, "you can't do that to yourself"),
            Refusal::ZeroAmount => write!(f, "you have to pay at least 1 credit"),
            Refusal::NotEnough { have, want } => write!(f, "you only have {have}, not {want}"),
            Refusal::TooMuch => write!(f, "they can't hold that many credits"),
            Refusal::DoesntKnowHow(process) => write!(f, "you don't know how to {process}"),
            Refusal::NotDiggable(name) => write!(f, "you can't dig {name}"),
            Refusal::NotATool(name) => write!(f, "{name} isn't made of anything that could cut"),
            Refusal::TooHard { tool, target } => {
                let verb = if tool == "your hands" {
                    "aren't"
                } else {
                    "isn't"
                };
                write!(f, "{tool} {verb} hard enough to work {target}")
            }
            Refusal::WouldSpoil(name) => write!(f, "pulling {name} apart would spoil it"),
            Refusal::Exhausted(name) => write!(f, "there's too little left of {name} to dig"),
            Refusal::NotAChamber(name) => write!(f, "you can't light {name}"),
            Refusal::AlreadyLit(name) => write!(f, "{name} is already lit"),
            Refusal::NoFuel(name) => write!(f, "there's nothing in {name} that burns"),
            Refusal::NotLiquid(name) => write!(f, "{name} isn't liquid"),
            Refusal::WouldMelt(name) => write!(f, "{name} would melt"),
            Refusal::UnknownShape(name) => write!(f, "you don't know a shape called {name}"),
            Refusal::NotSolid(name) => write!(f, "{name} isn't solid"),
            Refusal::TooHot(name) => write!(f, "{name} is too hot to touch"),
            Refusal::CannotWork(name) => write!(f, "{name} can't be shaped"),
            Refusal::UnknownDesign(name) => write!(f, "you don't know a design called {name}"),
            Refusal::MissingPart { slot, needs } if slot.is_empty() => {
                write!(f, "you need to be carrying a {needs}")
            }
            Refusal::MissingPart { slot, needs } => {
                write!(f, "you need to be carrying a {needs} for the {slot}")
            }
            Refusal::NotAnAssembly(name) => write!(f, "{name} isn't made of parts"),
            Refusal::NotAPart(name) => write!(f, "{name} isn't a shaped part"),
            Refusal::AsFineAsItGets(name) => {
                write!(f, "{name} is as fine as hand work can make it")
            }
            Refusal::CannotEat(name) => write!(f, "you can't eat {name}"),
            Refusal::NotDrinkable(name) => write!(f, "you can't drink {name}"),
            Refusal::NotThirsty => write!(f, "you aren't thirsty"),
            Refusal::NeedsTool { source, tool } => {
                let article = if tool.starts_with(['a', 'e', 'i', 'o', 'u']) {
                    "an"
                } else {
                    "a"
                };
                write!(f, "you need {article} {tool} for {source}")
            }
            Refusal::NoFlame(name) => write!(f, "there's no flame nearby to light {name} from"),
            Refusal::TooHeavy { carrying, limit } => {
                write!(
                    f,
                    "that's too much to carry: you have {carrying} of the {limit} you can manage"
                )
            }
            Refusal::TooWeak { part, holds, load } => write!(
                f,
                "the {part} won't hold it together: it holds up to {holds}, and the rest weighs {load}"
            ),
            Refusal::MustCross { place, liquid } => write!(
                f,
                "the way to {place} crosses {liquid}: you need something that floats to carry you (\"go … on …\")"
            ),
            Refusal::NothingToCross(place) => {
                write!(f, "there's nothing to cross on the way to {place}")
            }
            Refusal::NotAVessel(name) => write!(f, "you can't cross on {name}"),
            Refusal::NoEdge(Some(name)) => write!(f, "{name} has no edge to wound with"),
            Refusal::NotDead(name) => write!(f, "{name} isn't dead"),
            Refusal::NoEdge(None) => write!(f, "you have nothing with an edge to wound with"),
            Refusal::NothingToRead(name) => write!(f, "there's nothing to read on {name}"),
            Refusal::NotAShelter(name) => write!(f, "{name} gives no shelter"),
            Refusal::PutItDown(name) => write!(f, "put the {name} down first"),
            Refusal::TooDarkToSee => write!(f, "it's too dark to see far: wait for daylight"),
            Refusal::CannotFill(name) => write!(f, "{name} can't hold anything poured in"),
            Refusal::Full(name) => write!(f, "{name} is full"),
            Refusal::WouldSoften { container, liquid } => {
                write!(f, "{container} would soften in {liquid}")
            }
            Refusal::DontKnowWay(name) => {
                write!(f, "you don't know a way to {name} from here: try exploring")
            }
            Refusal::Asleep => write!(f, "you're asleep"),
            Refusal::TooDark => write!(
                f,
                "it's too dark to search: wait for daylight, or make a fire here"
            ),
            Refusal::NotTired => write!(f, "you aren't tired enough to sleep"),
            Refusal::NeverSleeps => write!(f, "you don't need sleep"),
            Refusal::Sinks { vessel, liquid } => write!(f, "{vessel} won't float in {liquid}"),
            Refusal::WouldSink {
                vessel,
                carries,
                load,
            } => write!(
                f,
                "{vessel} would sink under you: it carries up to {carries} more, and you weigh {load} with what you carry"
            ),
            Refusal::TheyCantCarry(name) => write!(f, "{name} can't carry that much more"),
            Refusal::NotGatherable(name) => {
                write!(f, "{name} isn't loose pieces you can gather by hand")
            }
        }
    }
}

impl std::error::Error for Refusal {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActError {
    /// The laws said no. Normal play.
    Refused(Refusal),
    /// The laws said yes but the gate said no. Always a bug in a law.
    Fault(Fault),
}

impl fmt::Display for ActError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActError::Refused(refusal) => refusal.fmt(f),
            ActError::Fault(fault) => write!(f, "engine fault: {fault}"),
        }
    }
}

impl std::error::Error for ActError {}

/// What an action will change, and how long it takes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    pub changes: Vec<Change>,
    pub seconds: u64,
}

/// Works out what `intent` would do, without changing anything. An action
/// that takes time is hard work for a living body while it lasts.
pub fn plan(world: &World, actor: EntityId, intent: &Intent) -> Result<Plan, Refusal> {
    let mut changes = changes_for(world, actor, intent)?;
    let seconds = duration(world, actor, intent);
    let resting = matches!(intent, Intent::Sleep { .. });
    if seconds > 0 && world.is_living(actor) && !resting {
        changes.push(Change::Exert {
            agent: actor,
            until: world.tick() + seconds,
        });
    }
    Ok(Plan { changes, seconds })
}

/// Works out what `intent` would change, without changing anything.
pub fn resolve(world: &World, actor: EntityId, intent: &Intent) -> Result<Vec<Change>, Refusal> {
    plan(world, actor, intent).map(|p| p.changes)
}

/// Checks `intent` against the laws and, if allowed, applies it through the
/// gate. Returns the changes that were made. The world's clock doesn't move;
/// see `perform` for actions that take time.
pub fn act(world: &mut World, actor: EntityId, intent: Intent) -> Result<Vec<Change>, ActError> {
    let changes = resolve(world, actor, &intent).map_err(ActError::Refused)?;
    world
        .apply(Cause::Action { actor, intent }, changes.clone())
        .map_err(ActError::Fault)?;
    Ok(changes)
}

/// What starting an action did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Started {
    /// It was carried out at once, or, for sleep and things kept up over
    /// time, has begun changing the world: these changes, and how long it
    /// lasts.
    Now { changes: Vec<Change>, seconds: u64 },
    /// It takes time and will be carried out when it's due, at this tick.
    Due(u64),
}

/// Actions that take time and are carried out when they end: until then,
/// the person is busy, and the world goes on. Sleep and rubbing change the
/// world as they go, so they begin at once.
fn carried_out_at_the_end(intent: &Intent) -> bool {
    matches!(
        intent,
        Intent::Go { .. }
            | Intent::Gather { .. }
            | Intent::Explore
            | Intent::Read { .. }
            | Intent::Attack { .. }
            | Intent::Butcher { .. }
            | Intent::Survey
    )
}

/// Starts an action without waiting for it. It's checked against the laws
/// now; one that takes time is carried out when it's due, and checked again
/// then, since the world may have changed. Someone busy can't start another.
pub fn start(world: &mut World, actor: EntityId, intent: Intent) -> Result<Started, ActError> {
    if let Some(pending) = world.pending(actor) {
        return Err(ActError::Refused(Refusal::Busy(pending.until)));
    }
    let plan = plan(world, actor, &intent).map_err(ActError::Refused)?;
    if plan.seconds > 0 && carried_out_at_the_end(&intent) {
        let until = world.tick() + plan.seconds;
        // Working begins now; the rest waits until it's due.
        let mut changes: Vec<Change> = plan
            .changes
            .into_iter()
            .filter(|c| matches!(c, Change::Exert { .. }))
            .collect();
        changes.push(Change::Begin {
            agent: actor,
            intent: intent.clone(),
            until,
        });
        world
            .apply(Cause::Action { actor, intent }, changes)
            .map_err(ActError::Fault)?;
        return Ok(Started::Due(until));
    }
    world
        .apply(Cause::Action { actor, intent }, plan.changes.clone())
        .map_err(ActError::Fault)?;
    Ok(Started::Now {
        changes: plan.changes,
        seconds: plan.seconds,
    })
}

/// Carries out every action that's due, checking each against the laws as
/// the world is now. One that no longer can be ends having failed.
pub fn complete_due(world: &mut World) -> Result<(), Fault> {
    let due: Vec<(EntityId, Intent)> = world
        .all_pending()
        .filter(|(_, p)| p.until <= world.tick())
        .map(|(id, p)| (id, p.intent.clone()))
        .collect();
    for (actor, intent) in due {
        // Something earlier in this round may have ended it already.
        if world.pending(actor).is_none() {
            continue;
        }
        match changes_for(world, actor, &intent) {
            Ok(changes) => {
                // Over first, so what it does (a cut underfoot, say) doesn't
                // interrupt it.
                let mut all = vec![Change::End {
                    agent: actor,
                    outcome: Outcome::Done(changes.clone()),
                }];
                all.extend(changes);
                world.apply(Cause::Action { actor, intent }, all)?;
            }
            Err(refusal) => world.apply(
                Cause::Nature { tick: world.tick() },
                vec![Change::End {
                    agent: actor,
                    outcome: Outcome::Failed(sentence_of(&refusal)),
                }],
            )?,
        }
    }
    Ok(())
}

/// What a thing is worth to someone with a mind: the highest of their
/// values whose word fits it, as they call things, times its mass. Nothing
/// they have no value for is worth anything to them.
pub(crate) fn worth(world: &World, who: EntityId, thing: EntityId) -> u128 {
    let Some(mind) = world.mind(who) else {
        return 0;
    };
    let per_kg = mind
        .values
        .iter()
        .filter(|(word, _)| is_called(world, who, thing, word) || mentions(world, who, thing, word))
        .map(|&(_, per_kg)| per_kg)
        .max()
        .unwrap_or(0);
    u128::from(per_kg) * u128::from(world.mass(thing).mg())
}

fn sentence_case(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().collect::<String>() + chars.as_str())
        .unwrap_or_default()
}

/// Someone who has seen `actor` take what's theirs wants nothing more to do
/// with them.
fn grudge(world: &World, actor: EntityId, listener: EntityId) -> Result<(), Refusal> {
    if world
        .memory(listener)
        .is_some_and(|m| m.robbed_by.contains_key(&actor))
    {
        return Err(Refusal::TheySay {
            who: named(world, actor, listener),
            why: "You took what's mine".into(),
        });
    }
    Ok(())
}

/// A refusal, as the one refused would say it: "you don't see it" becomes
/// "I don't see it".
fn in_first_person(text: &str) -> String {
    let words: Vec<String> = text
        .split(' ')
        .map(|w| match w {
            "you" => "I".to_string(),
            "you're" => "I'm".to_string(),
            "you've" => "I've".to_string(),
            "your" => "my".to_string(),
            "yourself" => "myself".to_string(),
            _ => w.to_string(),
        })
        .collect();
    let text = words
        .join(" ")
        .replace("I aren't", "I'm not")
        .replace("I are", "I am");
    let mut chars = text.chars();
    chars
        .next()
        .map(|c| c.to_uppercase().collect::<String>() + chars.as_str())
        .unwrap_or_default()
}

fn sentence_of(refusal: &Refusal) -> String {
    refusal.to_string()
}

/// Starts an action and lets the world run until it's over: how a script,
/// or someone acting alone, does one thing after another. Returns what it
/// changed; an action that couldn't be carried out when it was due is
/// refused then, after its time has passed.
pub fn perform(
    world: &mut World,
    actor: EntityId,
    intent: Intent,
) -> Result<Vec<Change>, ActError> {
    match start(world, actor, intent)? {
        Started::Now { changes, seconds } => {
            // Something kept up over time lasts until it's over, which may be
            // early.
            let activity = changes
                .iter()
                .any(|c| matches!(c, Change::StartActivity { .. }));
            if activity {
                nature::run_while(world, seconds, |w| w.activity(actor).is_some())
            } else {
                nature::run(world, seconds)
            }
            .map_err(ActError::Fault)?;
            Ok(changes)
        }
        Started::Due(until) => {
            let seconds = until.saturating_sub(world.tick());
            nature::run_while(world, seconds, |w| w.pending(actor).is_some())
                .map_err(ActError::Fault)?;
            match world.take_outcome(actor) {
                Some(Outcome::Done(changes)) => Ok(changes),
                Some(Outcome::Failed(why)) => Err(ActError::Refused(Refusal::Later(why))),
                Some(Outcome::Interrupted) => Err(ActError::Refused(Refusal::Interrupted)),
                None => Ok(Vec::new()),
            }
        }
    }
}

/// How many seconds an action takes. Most are quick enough to count as none.
pub fn duration(world: &World, actor: EntityId, intent: &Intent) -> u64 {
    match intent {
        Intent::Sleep { seconds, .. } => sleeping_time(world, actor, *seconds),
        Intent::Rub { .. } => usual_duration(world, actor, intent),
        _ => {
            // A tired body works more slowly; so does one out of stamina.
            let usual = usual_duration(world, actor, intent);
            if let Some(vitality) = world.life(actor).and_then(|l| l.vitality.as_ref()) {
                return if vitality.stamina == 0 {
                    u64::try_from(u128::from(usual) * 10_000 / u128::from(vitality.exhausted_pace))
                        .unwrap_or(u64::MAX)
                } else {
                    usual
                };
            }
            match world.life(actor).and_then(|l| l.sleep.as_ref()) {
                Some(sleep) if world.is_tired(actor) => {
                    u64::try_from(u128::from(usual) * 10_000 / u128::from(sleep.tired_pace.max(1)))
                        .unwrap_or(u64::MAX)
                }
                _ => usual,
            }
        }
    }
}

/// How long a sleep lasts: as long as asked, or until fully rested, which
/// takes `need` for every `awake` spent awake.
fn sleeping_time(world: &World, actor: EntityId, seconds: Option<u64>) -> u64 {
    if let Some(seconds) = seconds {
        return seconds;
    }
    let (Some(sleep), Some(awake)) = (
        world.life(actor).and_then(|l| l.sleep.as_ref()),
        world.awake_for(actor),
    ) else {
        return 0;
    };
    u64::try_from(u128::from(awake) * u128::from(sleep.need) / u128::from(sleep.awake.max(1)))
        .unwrap_or(u64::MAX)
}

fn usual_duration(world: &World, actor: EntityId, intent: &Intent) -> u64 {
    match intent {
        Intent::Explore => world.settings().explore_time,
        Intent::Read { .. } => READING_TIME,
        Intent::Attack { .. } => STRIKING_TIME,
        Intent::Butcher { .. } => BUTCHERING_TIME,
        Intent::Survey => world.settings().survey_time,
        Intent::Rub { seconds, .. } => seconds.unwrap_or(world.settings().rubbing_time),
        Intent::Go { place, aboard } => {
            let Ok(reach) = Reach::of(world, actor) else {
                return 0;
            };
            match way(world, actor, &reach, place, aboard.as_deref()) {
                Ok((to, None)) => walking_time(world, actor, reach.here, to),
                Ok((to, Some((vessel, liquid)))) => {
                    paddling_time(world, actor, vessel, liquid, world.distance(reach.here, to))
                }
                // Looking for a way that was told of but isn't there.
                Err(_) if false_way(world, actor, reach.here, place).is_some() => {
                    world.settings().explore_time
                }
                Err(_) => 0,
            }
        }
        Intent::Gather { source } => {
            let Some(reach) = Reach::of(world, actor).ok() else {
                return 0;
            };
            let Some(found) = find(world, actor, reach.around.iter().copied(), source) else {
                return 0;
            };
            let Some(pieces) = world.pieces(found) else {
                return 0;
            };
            // Work with a tool takes longer the blunter the tool.
            match (
                pieces.edge,
                needed_tool(world, &reach.carried, pieces.needs.as_deref()),
            ) {
                (Some(reference), Some(tool)) => {
                    let edge = edge_width(world, tool).max(1);
                    u64::try_from(
                        u128::from(pieces.find_time) * u128::from(edge)
                            / u128::from(reference.max(1)),
                    )
                    .unwrap_or(u64::MAX)
                    .max(1)
                }
                _ => pieces.find_time,
            }
        }
        _ => 0,
    }
}

/// The chance, in parts per ten thousand, that one search of a source finds
/// a piece: the source's own chance when it's full, less as it thins.
pub fn finding_chance(world: &World, source: EntityId) -> u64 {
    let Some(pieces) = world.pieces(source) else {
        return 0;
    };
    let full = u128::from(pieces.full.mg()).max(1);
    let now = u128::from(world.mass(source).mg());
    let fullness = (now * 10_000 / full).min(10_000);
    u64::try_from(fullness * u128::from(pieces.chance) / 10_000).expect("at most ten thousand")
}

/// Finds something `actor` can see or hold, for measuring. "here" is the
/// place itself. A testing tool, so data IDs always work.
pub fn find_reachable(world: &World, actor: EntityId, name: &str) -> Option<EntityId> {
    let reach = Reach::of(world, actor).ok()?;
    if normalize(name) == "here" {
        return Some(reach.here);
    }
    let by_key = std::iter::once(actor)
        .chain(reach.carried.iter().copied())
        .chain(reach.around.iter().copied())
        .chain(reach.inside.iter().copied())
        .find(|&id| normalize(world.key(id)) == normalize(name));
    if by_key.is_some() {
        return by_key;
    }
    if is_called(world, actor, actor, name) || normalize(name) == "me" {
        return Some(actor);
    }
    let nearby = reach.around.iter().chain(&reach.inside).copied();
    find(world, actor, reach.carried.iter().copied(), name)
        .or_else(|| find(world, actor, nearby, name))
}

/// What a person can reach from where they stand.
struct Reach {
    here: EntityId,
    /// Things and people in the same place, not counting the actor.
    around: Vec<EntityId>,
    /// Things inside containers that are around.
    inside: Vec<EntityId>,
    carried: Vec<EntityId>,
}

impl Reach {
    fn of(world: &World, actor: EntityId) -> Result<Reach, Refusal> {
        let here = world
            .location(actor)
            .filter(|&place| world.is_place(place))
            .ok_or(Refusal::Nowhere)?;
        let around: Vec<EntityId> = world
            .contents(here)
            .into_iter()
            .filter(|&e| e != actor)
            .collect();
        let inside = around
            .iter()
            .filter(|&&e| world.is_container(e))
            .flat_map(|&e| world.held(e))
            .collect();
        Ok(Reach {
            here,
            around,
            inside,
            carried: world.contents(actor),
        })
    }

    fn around_or_carried(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.around.iter().chain(&self.carried).copied()
    }
}

fn changes_for(world: &World, actor: EntityId, intent: &Intent) -> Result<Vec<Change>, Refusal> {
    if !world.is_agent(actor) {
        return Err(Refusal::NotAnAgent);
    }
    let reach = Reach::of(world, actor)?;
    if world.is_asleep(actor) {
        return Err(Refusal::Asleep);
    }
    // A name that fits several things that look different: ask which, from
    // what's in hand, then what's around, then what's inside things.
    for name in intent.things_named() {
        for set in [&reach.carried, &reach.around, &reach.inside] {
            let fits = set
                .iter()
                .any(|&id| is_called(world, actor, id, name) || mentions(world, actor, id, name));
            if fits {
                if let Some(refusal) = which(world, actor, set.iter().copied(), name) {
                    return Err(refusal);
                }
                break;
            }
        }
    }
    let carried = || reach.carried.iter().copied();
    let carrying = |name: &String| {
        find(world, actor, carried(), name).ok_or_else(|| Refusal::NotCarrying(name.clone()))
    };

    match intent {
        Intent::Go { place, aboard } => {
            let (to, crossing) = match way(world, actor, &reach, place, aboard.as_deref()) {
                Ok(found) => found,
                // A way someone was told of that isn't there: they look for
                // it, find nothing, and correct their memory.
                Err(refusal) => {
                    let Some((to, source)) = false_way(world, actor, reach.here, place) else {
                        return Err(refusal);
                    };
                    return Ok(vec![Change::Settle {
                        agent: actor,
                        claim: Claim::Way(reach.here, to),
                        held: false,
                        source,
                    }]);
                }
            };
            let mut changes = vec![Change::Move { entity: actor, to }];
            // What you cross on comes with you.
            if let Some((vessel, _)) = crossing.filter(|(v, _)| !reach.carried.contains(v)) {
                changes.push(Change::Move { entity: vessel, to });
            }
            if crossing.is_none() {
                changes.extend(treading(world, actor, reach.here, to));
            }
            changes.extend(arriving(world, actor, reach.here, to));
            Ok(changes)
        }

        Intent::Wear { item, on } => {
            let found = carrying(item)?;
            if !world.is_living(actor) {
                return Err(Refusal::CannotWear);
            }
            if world.worn(found).is_some() {
                return Err(Refusal::AlreadyWearing(named(world, actor, found)));
            }
            // Only what bends like skin can be worn: every piece of it soft
            // enough for bare hands to shape.
            let reference = world.settings().reference_temperature;
            let pieces = world
                .assembly(found)
                .map_or(vec![found], |a| a.parts.clone());
            let stiff = pieces.iter().any(|&piece| {
                let temperature = world.temperature(piece).unwrap_or(reference);
                world.composition(piece).is_none_or(|c| {
                    c.keys().any(|m| {
                        world.materials()[m].hardness_at(temperature, reference)
                            > world.settings().hand_hardness
                    })
                })
            });
            if stiff {
                return Err(Refusal::TooStiff(named(world, actor, found)));
            }
            if *on == Covering::Feet
                && let Some(other) = world.underfoot(actor)
            {
                return Err(Refusal::FeetCovered(named(world, actor, other)));
            }
            Ok(vec![Change::Wear {
                agent: actor,
                item: found,
                worn: Some(Worn {
                    on: *on,
                    sole: world.sole(found),
                    fresh: world.mass(world.sole(found)),
                }),
            }])
        }

        Intent::TakeOff { item } => {
            let found = carrying(item)?;
            if world.worn(found).is_none() {
                return Err(Refusal::NotWearing(named(world, actor, found)));
            }
            Ok(vec![Change::Wear {
                agent: actor,
                item: found,
                worn: None,
            }])
        }

        Intent::Butcher { body, tool } => {
            let carcass = find(world, actor, reach.around.iter().copied(), body)
                .ok_or_else(|| Refusal::NotHere(body.clone()))?;
            let dead = world.life(carcass).is_some_and(|l| l.died_of.is_some());
            if !dead {
                return Err(Refusal::NotDead(named(world, actor, carcass)));
            }
            let tool = carrying(tool)?;
            if !has_edge(world, tool) {
                return Err(Refusal::NoEdge(Some(named(world, actor, tool))));
            }
            harder_than(world, actor, tool, carcass)?;
            let composition = world.composition(carcass).expect("a body is matter");
            let fluid = world.life(carcass).expect("checked").fluid;
            // What stays behind is the hardest part: the frame. Everything
            // else comes away.
            let parts: Vec<(Mass, MaterialId)> = composition
                .iter()
                .filter(|(m, _)| **m != fluid)
                .map(|(&m, &mass)| (mass, m))
                .collect();
            let Some(&(_, kept)) = parts
                .iter()
                .max_by_key(|(mass, m)| (world.materials()[m].hardness, *mass))
            else {
                return Err(Refusal::NotDead(named(world, actor, carcass)));
            };
            // The rest comes away in cuts no heavier than the world's cut; the
            // body's fluid stays with the frame.
            let mut changes = Vec::new();
            for &(mass, material) in &parts {
                if material == kept {
                    continue;
                }
                let take = Composition::from([(material, mass)]);
                let total = matter::total_mass(&take);
                let cut = u128::from(world.settings().cut.mg().max(1));
                let pieces = total.div_ceil(cut).max(1);
                for n in 0..pieces {
                    let share: Composition = take
                        .iter()
                        .map(|(&m, &mass)| {
                            let each = u128::from(mass.mg()) / pieces;
                            let extra = if n == 0 {
                                u128::from(mass.mg()) % pieces
                            } else {
                                0
                            };
                            (
                                m,
                                Mass::from_mg(u64::try_from(each + extra).expect("a part")),
                            )
                        })
                        .filter(|(_, m)| m.mg() > 0)
                        .collect();
                    if !share.is_empty() {
                        changes.push(Change::Split {
                            from: carcass,
                            take: share,
                            at: reach.here,
                        });
                    }
                }
            }
            if changes.is_empty() {
                return Err(Refusal::NotDead(named(world, actor, carcass)));
            }
            Ok(changes)
        }

        Intent::Attack { target, with } => {
            let victim = find(world, actor, reach.around.iter().copied(), target)
                .filter(|&v| world.is_living(v))
                .ok_or_else(|| Refusal::NotHere(target.clone()))?;
            // Someone who set off by the time the strike began has gone:
            // they're on their way, out of reach. A strike already under way
            // lands.
            let struck_at = world.pending(actor).map_or(world.tick(), |p| p.since);
            let left = world
                .pending(victim)
                .is_some_and(|p| matches!(p.intent, Intent::Go { .. }) && p.since <= struck_at);
            if left {
                return Err(Refusal::NotHere(target.clone()));
            }
            // An edge: a tool's, or one the body is born with.
            let edge = match with {
                Some(tool) => {
                    let tool = carrying(tool)?;
                    if !has_edge(world, tool) {
                        return Err(Refusal::NoEdge(Some(named(world, actor, tool))));
                    }
                    edge_width(world, tool)
                }
                None => world.natural_weapon(actor).ok_or(Refusal::NoEdge(None))?,
            };
            // Someone asleep can't dodge; someone awake might.
            let luck = world.roll(u64::from(actor.0) ^ (u64::from(victim.0) << 32) ^ 0xB10);
            let hit = world.is_asleep(victim) || luck < world.settings().hit_chance;
            let rate = u128::from(world.settings().wound_rate) * 1_000 / u128::from(edge.max(1));
            let rate = u64::try_from(rate).unwrap_or(u64::MAX);
            let mut changes = Vec::new();
            if hit {
                changes.push(Change::Wound {
                    agent: victim,
                    rate,
                });
            }
            // Someone with a memory knows whom they went for, and who went
            // for them.
            if world.memory(actor).is_some() {
                changes.push(Change::Struck {
                    agent: actor,
                    at: victim,
                });
            }
            if world.memory(victim).is_some() {
                changes.push(Change::Notice {
                    agent: victim,
                    news: crate::world::News::Attacked {
                        by: actor,
                        wound: hit.then_some(rate),
                    },
                });
            }
            Ok(changes)
        }

        Intent::Read { item } => {
            let found = find(world, actor, reach.around_or_carried(), item)
                .ok_or_else(|| Refusal::NotHere(item.clone()))?;
            let claims = world
                .map(found)
                .ok_or_else(|| Refusal::NothingToRead(named(world, actor, found)))?;
            if world.memory(actor).is_none() {
                return Err(Refusal::NothingToRead(named(world, actor, found)));
            }
            let source = named(world, actor, found);
            Ok(claims
                .iter()
                .map(|claim| Change::Hear {
                    agent: actor,
                    claim: claim.clone(),
                    source: source.clone(),
                })
                .collect())
        }

        Intent::Take { item } => {
            if let Some(found) = find(world, actor, reach.around.iter().copied(), item) {
                return lift(world, actor, found).map(|c| seen_taking(world, actor, found, c));
            }
            if let Some(found) = find(world, actor, reach.inside.iter().copied(), item) {
                let container = world.location(found).expect("inside something");
                return Err(Refusal::InContainer {
                    item: named(world, actor, found),
                    container: named(world, actor, container),
                });
            }
            if find(world, actor, carried(), item).is_some() {
                return Err(Refusal::AlreadyCarrying(item.clone()));
            }
            if is_called(world, actor, actor, item) {
                return Err(Refusal::NotYourself);
            }
            Err(Refusal::NotHere(item.clone()))
        }

        Intent::TakeFrom { item, from } => {
            let container = find(world, actor, reach.around_or_carried(), from)
                .ok_or_else(|| Refusal::NotHere(from.clone()))?;
            if !world.is_container(container) {
                return Err(Refusal::NotAContainer(named(world, actor, container)));
            }
            if out_of_reach(world, actor, container) {
                return Err(Refusal::OutOfReach(named(world, actor, container)));
            }
            let found = find(world, actor, world.held(container), item).ok_or_else(|| {
                Refusal::NotInside {
                    item: item.clone(),
                    container: named(world, actor, container),
                }
            })?;
            lift(world, actor, found).map(|c| seen_taking(world, actor, found, c))
        }

        Intent::Drop { item } => {
            let found = carrying(item)?;
            Ok(vec![Change::Move {
                entity: found,
                to: reach.here,
            }])
        }

        Intent::Put { item, into } => {
            let found = carrying(item)?;
            let container = find(world, actor, reach.around_or_carried(), into)
                .ok_or_else(|| Refusal::NotHere(into.clone()))?;
            if !world.is_container(container) {
                return Err(Refusal::NotAContainer(named(world, actor, container)));
            }
            if out_of_reach(world, actor, container) {
                return Err(Refusal::OutOfReach(named(world, actor, container)));
            }
            if world.is_within(container, found) {
                return Err(Refusal::IntoItself);
            }
            Ok(vec![Change::Move {
                entity: found,
                to: container,
            }])
        }

        Intent::Give { item, to } => {
            let found = carrying(item)?;
            let recipient = person_here(world, actor, &reach.around, to)?;
            can_carry(world, recipient, world.mass(found))
                .map_err(|_| Refusal::TheyCantCarry(named(world, actor, recipient)))?;
            Ok(vec![Change::Move {
                entity: found,
                to: recipient,
            }])
        }

        Intent::Pay { to, amount } => {
            let recipient = person_here(world, actor, &reach.around, to)?;
            if *amount == Credits::ZERO {
                return Err(Refusal::ZeroAmount);
            }
            let have = world.wallet(actor).unwrap_or(Credits::ZERO);
            if have < *amount {
                return Err(Refusal::NotEnough {
                    have,
                    want: *amount,
                });
            }
            let theirs = world.wallet(recipient).unwrap_or(Credits::ZERO);
            theirs.checked_add(*amount).ok_or(Refusal::TooMuch)?;
            Ok(vec![Change::Transfer {
                from: actor,
                to: recipient,
                amount: *amount,
            }])
        }

        Intent::Dig { source, tool } => {
            must_know(world, actor, Process::Dig)?;
            let from = find(world, actor, reach.around.iter().copied(), source)
                .ok_or_else(|| Refusal::NotHere(source.clone()))?;
            let composition = world
                .composition(from)
                .filter(|_| !world.is_portable(from))
                .ok_or_else(|| Refusal::NotDiggable(named(world, actor, from)))?;
            let tool = carrying(tool)?;
            harder_than(world, actor, tool, from)?;
            let amount = world.settings().dig_amount;
            if world.mass(from) <= amount {
                return Err(Refusal::Exhausted(named(world, actor, from)));
            }
            can_carry(world, actor, amount)?;
            let take =
                matter::proportional(composition, amount).expect("there is more than the amount");
            Ok(vec![Change::Split {
                from,
                take,
                at: actor,
            }])
        }

        Intent::Fill { container, source } => {
            // Something in hand first, then something here, and one with room
            // in it before one that's full.
            let within = || reach.carried.iter().chain(&reach.around).copied();
            let has_room = |c: EntityId| {
                let capacity = world
                    .shape(c)
                    .and_then(|s| world.shapes().get(s))
                    .and_then(|def| def.capacity);
                let held: u64 = world.held(c).iter().map(|&e| world.mass(e).mg()).sum();
                capacity.is_some_and(|cap| held < cap.mg())
            };
            let vessel = find(world, actor, within().filter(|&c| has_room(c)), container)
                .or_else(|| find(world, actor, within(), container))
                .ok_or_else(|| Refusal::NotHere(container.clone()))?;
            let capacity = world
                .shape(vessel)
                .and_then(|s| world.shapes().get(s))
                .and_then(|def| def.capacity)
                .filter(|_| world.is_container(vessel))
                .ok_or_else(|| Refusal::CannotFill(named(world, actor, vessel)))?;
            let candidates = reach.around.iter().chain(&reach.inside).copied();
            let from = find(world, actor, candidates.filter(|&e| e != vessel), source)
                .ok_or_else(|| Refusal::NotHere(source.clone()))?;
            if !world.is_all(from, State::Liquid) {
                return Err(Refusal::NotLiquid(named(world, actor, from)));
            }
            softens(world, actor, vessel, from)?;
            let held: u64 = world.held(vessel).iter().map(|&e| world.mass(e).mg()).sum();
            let amount = capacity
                .mg()
                .saturating_sub(held)
                .min(world.mass(from).mg().saturating_sub(1));
            if amount == 0 {
                return Err(Refusal::Full(named(world, actor, vessel)));
            }
            if reach.carried.contains(&vessel) {
                can_carry(world, actor, Mass::from_mg(amount))?;
            }
            let composition = world.composition(from).expect("a liquid is matter");
            let take = matter::proportional(composition, Mass::from_mg(amount))
                .ok_or_else(|| Refusal::NotLiquid(named(world, actor, from)))?;
            Ok(vec![Change::Split {
                from,
                take,
                at: vessel,
            }])
        }

        Intent::Light { chamber } => {
            must_know(world, actor, Process::Light)?;
            let found = find(world, actor, reach.around.iter().copied(), chamber)
                .ok_or_else(|| Refusal::NotHere(chamber.clone()))?;
            let state = world
                .chamber(found)
                .ok_or_else(|| Refusal::NotAChamber(named(world, actor, found)))?;
            if state.lit {
                return Err(Refusal::AlreadyLit(named(world, actor, found)));
            }
            if !holds_fuel(world, found) {
                return Err(Refusal::NoFuel(named(world, actor, found)));
            }
            // Fire comes from fire: something burning must be within reach.
            let flame = reach
                .around
                .iter()
                .chain(&reach.inside)
                .chain(&reach.carried)
                .any(|&e| {
                    e != found && (world.is_burning(e) || world.chamber(e).is_some_and(|c| c.lit))
                });
            if !flame {
                return Err(Refusal::NoFlame(named(world, actor, found)));
            }
            Ok(vec![Change::Light {
                chamber: found,
                lit: true,
            }])
        }

        Intent::Pour { liquid, into } => {
            must_know(world, actor, Process::Pour)?;
            let candidates = reach.around.iter().chain(&reach.inside).copied();
            let found = find(world, actor, candidates, liquid)
                .ok_or_else(|| Refusal::NotHere(liquid.clone()))?;
            if !world.is_all(found, State::Liquid) {
                return Err(Refusal::NotLiquid(named(world, actor, found)));
            }
            // Into something on the ground, or something sitting in another
            // container, like a form sitting inside a chamber.
            let targets = reach
                .around
                .iter()
                .chain(&reach.inside)
                .copied()
                .filter(|&t| t != found);
            let target =
                find(world, actor, targets, into).ok_or_else(|| Refusal::NotHere(into.clone()))?;
            if !world.is_container(target) {
                return Err(Refusal::NotAContainer(named(world, actor, target)));
            }
            softens(world, actor, target, found)?;
            // A container can't hold anything hotter than it can stand.
            let limit = world
                .composition(target)
                .and_then(matter::dominant)
                .map(|m| world.materials()[&m].melting_point);
            if let (Some(limit), Some(heat)) = (limit, world.temperature(found))
                && heat >= limit
            {
                return Err(Refusal::WouldMelt(named(world, actor, target)));
            }
            Ok(vec![Change::Move {
                entity: found,
                to: target,
            }])
        }

        Intent::Work { item, shape, tool } => {
            must_know(world, actor, Process::Work)?;
            let wanted = normalize(shape);
            // A shape you have a word for, or any the world has, for someone
            // from a world with no cultures.
            let shape_key = if world.has_words(actor) {
                world.shape_by_word(actor, &wanted)
            } else {
                world
                    .shapes()
                    .iter()
                    .find(|(key, def)| normalize(key) == wanted || normalize(&def.label) == wanted)
                    .map(|(key, _)| key.clone())
            }
            .ok_or_else(|| Refusal::UnknownShape(shape.clone()))?;
            // What's in hand first, then what's around.
            let nearby = reach.around.iter().chain(&reach.inside).copied();
            let found = find(world, actor, reach.carried.iter().copied(), item)
                .or_else(|| find(world, actor, nearby, item))
                .ok_or_else(|| Refusal::NotHere(item.clone()))?;
            if world.composition(found).is_none() || world.is_container(found) {
                return Err(Refusal::CannotWork(named(world, actor, found)));
            }
            if !world.is_all(found, State::Solid) {
                return Err(Refusal::NotSolid(named(world, actor, found)));
            }
            // A part is at most as precise as the tool that made it. Bare
            // hands shape only what's soft enough, roughly.
            let tolerance = match tool {
                Some(tool) => {
                    let tool = carrying(tool)?;
                    if tool == found {
                        return Err(Refusal::NotYourself);
                    }
                    harder_than(world, actor, tool, found)?;
                    world
                        .tolerance(tool)
                        .unwrap_or(world.settings().rough_tolerance)
                }
                None => {
                    by_hand(world, actor, found)?;
                    world.settings().rough_tolerance
                }
            };
            Ok(vec![Change::Shape {
                entity: found,
                shape: Some((shape_key, tolerance)),
            }])
        }

        Intent::Rub {
            item,
            against,
            into,
            seconds,
        } => {
            must_know(world, actor, Process::Work)?;
            let first = carrying(item)?;
            // Rubbing a thing against another of the same name means two pieces.
            let second = find(world, actor, carried().filter(|&p| p != first), against)
                .or_else(|| find(world, actor, carried(), against))
                .ok_or_else(|| Refusal::NotCarrying(against.clone()))?;
            if first == second {
                return Err(Refusal::NotYourself);
            }
            for piece in [first, second] {
                if world.composition(piece).is_none() {
                    return Err(Refusal::NotAPart(named(world, actor, piece)));
                }
                if !world.is_all(piece, State::Solid) {
                    return Err(Refusal::NotSolid(named(world, actor, piece)));
                }
            }
            let settings = world.settings();
            let mut changes = Vec::new();

            // Rubbing two parts together wears each against the other, and
            // both come out finer than either tool that made them.
            let session = seconds.unwrap_or(settings.rubbing_time);
            // Every shaped part being rubbed comes out finer: two parts against
            // each other, or an edge against something hard. A full session improves
            // by the world's rubbing improvement; shorter or longer, in
            // proportion.
            let parts: Vec<(EntityId, u64)> = [first, second]
                .into_iter()
                .filter_map(|p| world.tolerance(p).map(|t| (p, t)))
                .collect();
            if !parts.is_empty() {
                let improvement = u128::from(settings.rubbing_improvement) * u128::from(session)
                    / u128::from(settings.rubbing_time.max(1));
                let keep = 10_000u64.saturating_sub(u64::try_from(improvement).unwrap_or(10_000));
                for &(part, tolerance) in &parts {
                    let finer = (u128::from(tolerance) * u128::from(keep) / 10_000) as u64;
                    let finer = finer.max(settings.finest_tolerance);
                    if finer < tolerance {
                        changes.push(Change::Refine {
                            entity: part,
                            tolerance: finer,
                        });
                    }
                }
                if changes.is_empty() {
                    return Err(Refusal::AsFineAsItGets(named(world, actor, parts[0].0)));
                }
            }

            // A living worker's effort wears dust off the softer of the two,
            // and friction heats it. The dust falls into `into`, or on the
            // ground. Nature carries the rubbing on for as long as it lasts.
            if world.is_living(actor) {
                let target = match into {
                    Some(name) => {
                        let found = find(world, actor, reach.around.iter().copied(), name)
                            .ok_or_else(|| Refusal::NotHere(name.clone()))?;
                        if !world.is_container(found) {
                            return Err(Refusal::NotAContainer(named(world, actor, found)));
                        }
                        found
                    }
                    None => reach.here,
                };
                let reference = settings.reference_temperature;
                let hardness = |id: EntityId| {
                    world.composition(id).and_then(matter::dominant).map(|m| {
                        world.materials()[&m]
                            .hardness_at(world.temperature(id).unwrap_or(reference), reference)
                    })
                };
                let softer = if hardness(second) < hardness(first) {
                    second
                } else {
                    first
                };
                let composition = world.composition(softer).expect("checked above");
                let wear = settings
                    .wear_rate
                    .max(1)
                    .min(world.mass(softer).mg().saturating_sub(1));
                if let Some(take) =
                    matter::proportional(composition, Mass::from_mg(wear)).filter(|t| !t.is_empty())
                {
                    let dust = world.next_id();
                    if world.activity(actor).is_some() {
                        changes.push(Change::EndActivity { agent: actor });
                    }
                    changes.push(Change::Split {
                        from: softer,
                        take,
                        at: target,
                    });
                    changes.push(Change::StartActivity {
                        agent: actor,
                        activity: crate::world::Activity::Rubbing {
                            first,
                            second,
                            dust,
                            until: world.tick() + session,
                        },
                    });
                }
            }
            if changes.is_empty() {
                return Err(Refusal::NotAPart(named(world, actor, first)));
            }
            Ok(changes)
        }

        Intent::Assemble { design } => {
            must_know(world, actor, Process::Assemble)?;
            let wanted = normalize(design);
            // Someone with words of their own makes what they know a way to
            // make; anyone else, anything the world has a design for.
            let recipes: Vec<Recipe> = if world.has_words(actor) {
                world
                    .recipes_for(actor, &wanted)
                    .into_iter()
                    .cloned()
                    .collect()
            } else {
                world
                    .designs()
                    .iter()
                    .filter(|(key, def)| {
                        normalize(key) == wanted || normalize(&def.label) == wanted
                    })
                    .map(|(key, def)| Recipe {
                        design: Some(key.clone()),
                        slots: def.slots.clone(),
                    })
                    .take(1)
                    .collect()
            };
            let mut first_refusal = None;
            for recipe in recipes {
                match fill_slots(world, actor, &reach, &recipe.slots) {
                    Ok(parts) => {
                        return put_together(world, actor, &reach, recipe.design, &parts);
                    }
                    Err(refusal) => {
                        first_refusal.get_or_insert(refusal);
                    }
                }
            }
            Err(first_refusal.unwrap_or_else(|| Refusal::UnknownDesign(design.clone())))
        }

        Intent::Join { items } => {
            must_know(world, actor, Process::Assemble)?;
            let within: Vec<EntityId> = reach
                .carried
                .iter()
                .chain(&reach.around)
                .copied()
                .filter(|&p| world.is_portable(p) && !world.is_agent(p) && !world.is_container(p))
                .collect();
            let mut parts: Vec<(String, EntityId)> = Vec::new();
            for item in items {
                let unused = within
                    .iter()
                    .copied()
                    .filter(|p| !parts.iter().any(|(_, used)| used == p));
                let found = find(world, actor, unused, item)
                    .ok_or_else(|| Refusal::NotHere(item.clone()))?;
                parts.push((String::new(), found));
            }
            put_together(world, actor, &reach, None, &parts)
        }

        Intent::Call { item, word } => {
            let found = if normalize(item) == "it" {
                world
                    .lexicon(actor)
                    .and_then(|l| l.last_made)
                    .filter(|&made| reach.carried.contains(&made) || reach.around.contains(&made))
                    .ok_or(Refusal::NothingMade)?
            } else {
                find(world, actor, reach.around_or_carried(), item)
                    .ok_or_else(|| Refusal::NotHere(item.clone()))?
            };
            if !world.has_words(actor) {
                return Err(Refusal::NoWordsToLearn("you".into()));
            }
            let word = normalize(word);
            let mut changes = vec![Change::Word {
                agent: actor,
                word: word.clone(),
                meaning: world.meaning_of(found),
            }];
            // Naming what you made yourself keeps the way you made it.
            let made = world.lexicon(actor).and_then(|l| l.last_made) == Some(found);
            if let Some(recipe) = world.recipe_from(found).filter(|_| made) {
                changes.push(Change::Recipe {
                    agent: actor,
                    word,
                    recipe,
                });
            }
            Ok(changes)
        }

        Intent::Tell { person, item, word } => {
            let listener = person_here(world, actor, &reach.around, person)?;
            let found = find(world, actor, reach.around_or_carried(), item)
                .ok_or_else(|| Refusal::NotHere(item.clone()))?;
            if !world.has_words(listener) {
                return Err(Refusal::NoWordsToLearn(named(world, actor, listener)));
            }
            Ok(vec![Change::Word {
                agent: listener,
                word: normalize(word),
                meaning: world.meaning_of(found),
            }])
        }

        Intent::Offer { item, person, want } => {
            let mine = carrying(item)?;
            let listener = person_here(world, actor, &reach.around, person)?;
            let name = || named(world, actor, listener);
            world
                .mind(listener)
                .ok_or_else(|| Refusal::OwnMind(name()))?;
            if world.is_asleep(listener) {
                return Err(Refusal::TheyreAsleep(name()));
            }
            grudge(world, actor, listener)?;
            // What they'd give, in their own words, from what they carry.
            let says = |why: String| Refusal::TheySay { who: name(), why };
            let theirs = find(world, listener, world.contents(listener), want)
                .ok_or_else(|| says(format!("I haven't got {}", want.trim())))?;
            let (get, give) = (worth(world, listener, mine), worth(world, listener, theirs));
            let (mine_named, theirs_named) =
                (named(world, listener, mine), named(world, listener, theirs));
            if get == 0 {
                return Err(says(format!("I've no use for {mine_named}")));
            }
            if get < give {
                return Err(says(sentence_case(&format!(
                    "{theirs_named} is worth more to me than {mine_named}"
                ))));
            }
            can_carry(world, listener, world.mass(mine))
                .map_err(|_| Refusal::TheyCantCarry(name()))?;
            // Both at once, or neither.
            Ok(vec![
                Change::Move {
                    entity: mine,
                    to: listener,
                },
                Change::Move {
                    entity: theirs,
                    to: actor,
                },
            ])
        }

        Intent::Ask { person, request } => {
            let listener = person_here(world, actor, &reach.around, person)?;
            let name = || named(world, actor, listener);
            let mind = world
                .mind(listener)
                .ok_or_else(|| Refusal::OwnMind(name()))?;
            if world.is_asleep(listener) {
                return Err(Refusal::TheyreAsleep(name()));
            }
            grudge(world, actor, listener)?;
            // A confined mind does its own work and nothing else.
            if mind.scope == crate::mind::Scope::Confined
                && !mind.orders.iter().any(|o| o.command == **request)
            {
                return Err(Refusal::NotTheirWork(name()));
            }
            // They weigh it in their own words, against what they see and
            // carry.
            if let Err(refusal) = plan(world, listener, request) {
                return Err(Refusal::TheySay {
                    who: name(),
                    why: in_first_person(&refusal.to_string()),
                });
            }
            Ok(vec![Change::Request {
                agent: listener,
                intent: (**request).clone(),
            }])
        }

        Intent::Disassemble { item } => {
            must_know(world, actor, Process::Assemble)?;
            let found = find(world, actor, reach.around_or_carried(), item)
                .ok_or_else(|| Refusal::NotHere(item.clone()))?;
            if world.assembly(found).is_none() {
                return Err(Refusal::NotAnAssembly(named(world, actor, found)));
            }
            Ok(vec![Change::Disassemble { assembly: found }])
        }

        Intent::Eat { item } => {
            let found = carrying(item)?;
            let life = world
                .life(actor)
                .ok_or_else(|| Refusal::CannotEat(named(world, actor, found)))?;
            let composition = world
                .composition(found)
                .filter(|_| world.assembly(found).is_none() && !world.is_container(found))
                .ok_or_else(|| Refusal::CannotEat(named(world, actor, found)))?;
            // What the body can digest goes in, with the fluid in it; the rest
            // is left in the hand.
            if !composition.keys().any(|m| life.digests.contains(m)) {
                return Err(Refusal::CannotEat(named(world, actor, found)));
            }
            let digestible: Composition = composition
                .iter()
                .filter(|(m, _)| life.digests.contains(m) || **m == life.fluid)
                .map(|(&m, &mass)| (m, mass))
                .collect();
            if world.in_use(found) {
                return Err(Refusal::CannotEat(named(world, actor, found)));
            }
            if digestible.len() == composition.len() {
                Ok(vec![Change::Merge {
                    from: found,
                    into: actor,
                }])
            } else {
                Ok(vec![Change::Shift {
                    from: found,
                    to: actor,
                    take: digestible,
                }])
            }
        }

        Intent::Drink { source } => {
            // What's here, then what's in hand, including inside what you carry.
            let in_carried: Vec<EntityId> = reach
                .carried
                .iter()
                .filter(|&&c| world.is_container(c))
                .flat_map(|&c| world.held(c))
                .collect();
            let candidates = reach
                .around
                .iter()
                .chain(&reach.inside)
                .chain(&reach.carried)
                .chain(&in_carried)
                .copied();
            let found = find(world, actor, candidates, source)
                .ok_or_else(|| Refusal::NotHere(source.clone()))?;
            let life = world
                .life(actor)
                .ok_or_else(|| Refusal::NotDrinkable(named(world, actor, found)))?;
            // Only something that is nothing but the body's fluid, and liquid.
            let pure = world
                .composition(found)
                .is_some_and(|c| c.len() == 1 && c.contains_key(&life.fluid))
                && world.is_all(found, State::Liquid);
            if !pure {
                return Err(Refusal::NotDrinkable(named(world, actor, found)));
            }
            let have = world
                .composition(actor)
                .and_then(|c| c.get(&life.fluid))
                .map_or(0, |m| m.mg());
            let wanted = life
                .fluid_normal
                .mg()
                .saturating_sub(have)
                .min(life.gulp.mg());
            let amount = wanted.min(world.mass(found).mg().saturating_sub(1));
            if amount == 0 {
                return Err(Refusal::NotThirsty);
            }
            Ok(vec![Change::Shift {
                from: found,
                to: actor,
                take: Composition::from([(life.fluid, Mass::from_mg(amount))]),
            }])
        }

        Intent::Divide { item } => {
            let found = carrying(item)?;
            let composition = world
                .composition(found)
                .filter(|_| world.assembly(found).is_none() && !world.is_container(found))
                .ok_or_else(|| Refusal::CannotWork(named(world, actor, found)))?;
            if !world.is_all(found, State::Solid) {
                return Err(Refusal::NotSolid(named(world, actor, found)));
            }
            // Something already made isn't raw material any more.
            if world.shape(found).is_some() {
                return Err(Refusal::WouldSpoil(named(world, actor, found)));
            }
            // Bare hands pull apart only what's soft enough.
            let reference = world.settings().reference_temperature;
            let temperature = world.temperature(found).unwrap_or(reference);
            let hardest = composition
                .keys()
                .map(|m| world.materials()[m].hardness_at(temperature, reference))
                .max()
                .unwrap_or(0);
            if hardest > world.settings().hand_hardness {
                return Err(Refusal::TooHard {
                    tool: "your hands".into(),
                    target: named(world, actor, found),
                });
            }
            let half = Mass::from_mg(world.mass(found).mg() / 2);
            let take = matter::proportional(composition, half)
                .filter(|t| !t.is_empty())
                .ok_or_else(|| Refusal::CannotWork(named(world, actor, found)))?;
            Ok(vec![Change::Split {
                from: found,
                take,
                at: actor,
            }])
        }

        Intent::Survey => {
            // Seeing far needs daylight; a fire lights only what's near it.
            if world.is_night() && world.pending(actor).is_none() {
                return Err(Refusal::TooDarkToSee);
            }
            Ok(in_sight(world, reach.here)
                .into_iter()
                .filter(|sight| !world.has_seen(actor, sight.place))
                .map(|sight| Change::See {
                    agent: actor,
                    place: sight.place,
                })
                .collect())
        }

        Intent::Explore => {
            if world.is_dark(reach.here) && world.pending(actor).is_none() {
                return Err(Refusal::TooDark);
            }
            // The nearest way you don't know is the first you'd find. A
            // search takes its time whether or not it finds anything.
            let unknown = world
                .exits(reach.here)
                .iter()
                .copied()
                .filter(|&to| !world.knows_way(actor, reach.here, to))
                .min_by_key(|&to| (world.distance(reach.here, to), to));
            let luck = world.roll(u64::from(actor.0) ^ (u64::from(reach.here.0) << 32) ^ 0x5EA7);
            match unknown {
                Some(to) if luck < world.settings().explore_chance => Ok(vec![Change::Learn {
                    agent: actor,
                    from: reach.here,
                    to,
                }]),
                _ => Ok(Vec::new()),
            }
        }

        // A player needs no sleep, but may rest a while: time passes, and
        // stamina comes back.
        Intent::Sleep {
            seconds: Some(_), ..
        } if world.life(actor).is_some_and(|l| l.vitality.is_some()) => Ok(Vec::new()),

        Intent::Sleep { seconds, shelter } => {
            // A shelter must be here, on the ground, and built to shelter.
            let shelter = match shelter {
                Some(name) => {
                    let found = find(world, actor, reach.around.iter().copied(), name).ok_or_else(
                        || {
                            if find(world, actor, carried(), name).is_some() {
                                Refusal::PutItDown(name.clone())
                            } else {
                                Refusal::NotHere(name.clone())
                            }
                        },
                    )?;
                    let shelters = world.design_of(found).is_some_and(|d| d.shelter.is_some());
                    if !shelters {
                        return Err(Refusal::NotAShelter(named(world, actor, found)));
                    }
                    Some(found)
                }
                None => None,
            };
            let (Some(sleep), Some(awake)) = (
                world.life(actor).and_then(|l| l.sleep.as_ref()),
                world.awake_for(actor),
            ) else {
                return Err(Refusal::NeverSleeps);
            };
            // Too little awake to sleep, unless asked for a set time.
            if seconds.is_none() && awake < sleep.awake / 16 {
                return Err(Refusal::NotTired);
            }
            let until = world.tick() + sleeping_time(world, actor, *seconds).max(1);
            Ok(vec![Change::Sleep {
                agent: actor,
                until,
                shelter,
            }])
        }

        Intent::Gather { source } => {
            must_know(world, actor, Process::Gather)?;
            let found = find(world, actor, reach.around.iter().copied(), source)
                .ok_or_else(|| Refusal::NotHere(source.clone()))?;
            // Starting a search needs light: daylight, or something burning
            // here. One under way when night falls is finished.
            if world.is_dark(reach.here) && world.pending(actor).is_none() {
                return Err(Refusal::TooDark);
            }
            let (Some(pieces), Some(composition)) = (world.pieces(found), world.composition(found))
            else {
                return Err(Refusal::NotGatherable(named(world, actor, found)));
            };
            if world.mass(found) <= pieces.size {
                return Err(Refusal::Exhausted(named(world, actor, found)));
            }
            can_carry(world, actor, pieces.size)?;
            // Some sources can't be gathered with bare hands.
            if let Some(needs) = &pieces.needs {
                if let Some(tool) = needed_tool(world, &reach.carried, Some(needs)) {
                    harder_than(world, actor, tool, found)?;
                } else {
                    let requirement = if world.shapes().contains_key(needs) {
                        Requirement::Shape(needs.clone())
                    } else {
                        Requirement::Design(needs.clone())
                    };
                    let tool = world.requirement_for(actor, &requirement);
                    return Err(Refusal::NeedsTool {
                        source: named(world, actor, found),
                        tool,
                    });
                }
            }
            // A search takes its time whether or not it finds anything.
            let luck = world.roll(u64::from(actor.0) ^ (u64::from(found.0) << 32));
            if luck >= finding_chance(world, found) {
                return Ok(Vec::new());
            }
            let take = matter::proportional(composition, pieces.size)
                .expect("there is more than one piece");
            Ok(vec![Change::Split {
                from: found,
                take,
                at: actor,
            }])
        }
    }
}

/// What walking from one place to another does underfoot, on ground as
/// rough as the rougher end: what's worn on the feet wears away, a km's
/// roughness at a time, and falls as dust where they arrive; bare feet, or
/// worn-through soles, on ground rough enough to hurt, are cut. Creatures
/// acting on instinct are born with feet for their ground.
fn treading(world: &World, actor: EntityId, from: EntityId, to: EntityId) -> Vec<Change> {
    let rough = world.roughness(from).max(world.roughness(to));
    if rough == 0 || !world.is_living(actor) || world.instinct(actor).is_some() {
        return Vec::new();
    }
    let distance = u128::from(world.distance(from, to));
    let per_km =
        |rate: u64| u64::try_from(u128::from(rate) * distance / 1_000_000_000).unwrap_or(u64::MAX);
    match world.underfoot(actor).filter(|&s| !world.worn_through(s)) {
        Some(shoe) => {
            let sole = world.sole(shoe);
            let wear = per_km(rough).min(world.mass(sole).mg().saturating_sub(1));
            world
                .composition(sole)
                .and_then(|c| matter::proportional(c, Mass::from_mg(wear)))
                .filter(|take| !take.is_empty())
                .map(|take| Change::Split {
                    from: sole,
                    take,
                    at: to,
                })
                .into_iter()
                .collect()
        }
        None if hurts_bare_feet(world, actor, from, to) => vec![Change::Wound {
            agent: actor,
            rate: per_km(world.settings().bare_feet_wound).max(1),
        }],
        None => Vec::new(),
    }
}

/// Whether a barrier keeps `actor` out of a container: it holds what's in
/// it higher than they can climb or jump.
pub fn out_of_reach(world: &World, actor: EntityId, container: EntityId) -> bool {
    let barrier = world.design_of(container).and_then(|d| d.barrier);
    let reach = world.life(actor).and_then(|l| l.reach);
    matches!((barrier, reach), (Some(barrier), Some(reach)) if barrier > reach)
}

/// Whether a walk is on ground rough enough to hurt, for someone with
/// nothing sound on their feet.
fn hurts_bare_feet(world: &World, actor: EntityId, from: EntityId, to: EntityId) -> bool {
    world.roughness(from).max(world.roughness(to)) >= world.settings().bare_feet_limit
        && world.instinct(actor).is_none()
        && world
            .underfoot(actor)
            .is_none_or(|shoe| world.worn_through(shoe))
}

/// Finds a part within reach for each slot: the biggest that fits, taking
/// what's in hand first when two are alike.
fn fill_slots(
    world: &World,
    actor: EntityId,
    reach: &Reach,
    slots: &[(String, Requirement)],
) -> Result<Vec<(String, EntityId)>, Refusal> {
    let mut reachable: Vec<EntityId> = reach
        .carried
        .iter()
        .chain(&reach.around)
        .copied()
        .filter(|&p| world.is_portable(p) && !world.is_agent(p) && world.worn(p).is_none())
        .collect();
    reachable.sort_by_key(|&p| std::cmp::Reverse(world.mass(p)));
    let mut parts: Vec<(String, EntityId)> = Vec::new();
    for (slot, requirement) in slots {
        let fits = |part: EntityId| match requirement {
            Requirement::Shape(shape) => {
                world.shape(part) == Some(shape.as_str()) && world.is_all(part, State::Solid)
            }
            Requirement::Design(inner) => world
                .assembly(part)
                .is_some_and(|a| a.design.as_ref() == Some(inner)),
            // A plain lump of the material, not something already shaped.
            Requirement::Material(material) => {
                world.composition(part).and_then(matter::dominant) == Some(*material)
                    && world.is_all(part, State::Solid)
                    && world.shape(part).is_none()
                    && !world.is_container(part)
            }
        };
        let part = reachable
            .iter()
            .copied()
            .find(|&p| !parts.iter().any(|(_, used)| *used == p) && fits(p))
            .ok_or_else(|| Refusal::MissingPart {
                slot: slot.clone(),
                needs: world.requirement_for(actor, requirement),
            })?;
        parts.push((slot.clone(), part));
    }
    Ok(parts)
}

/// Puts parts together, to a design or to none, as something new. Whatever
/// pulls them together must hold the rest; the whole is measured once, now,
/// from its parts' datasheets, and kept in hand if it can be carried.
fn put_together(
    world: &World,
    actor: EntityId,
    reach: &Reach,
    design: Option<String>,
    chosen: &[(String, EntityId)],
) -> Result<Vec<Change>, Refusal> {
    let parts: Vec<(String, String, Datasheet)> = chosen
        .iter()
        .map(|(slot, part)| {
            (
                slot.clone(),
                world.label(*part),
                datasheet::measure(world, *part),
            )
        })
        .collect();
    let (pulling, rest): (Vec<_>, Vec<_>) = parts
        .iter()
        .map(|(_, _, sheet)| sheet)
        .partition(|sheet| sheet.get(Property::HoldsUpTo).is_some());
    if let Some(first) = pulling.first() {
        let mass_of = |sheet: &&Datasheet| match sheet.get(Property::Mass) {
            Some(Value::Mass(m)) => m.mg(),
            _ => 0,
        };
        let holds: u64 = pulling
            .iter()
            .map(|sheet| match sheet.get(Property::HoldsUpTo) {
                Some(Value::Mass(m)) => m.mg(),
                _ => 0,
            })
            .sum();
        let load: u64 = rest.iter().map(mass_of).sum();
        if holds < load {
            let part = chosen
                .iter()
                .zip(&parts)
                .find(|(_, (_, _, sheet))| std::ptr::eq(sheet, *first))
                .map_or_else(String::new, |((_, id), _)| world.label_for(actor, *id));
            return Err(Refusal::TooWeak {
                part,
                holds: Mass::from_mg(holds),
                load: Mass::from_mg(load),
            });
        }
    }
    let sheet = datasheet::measure_assembly(world.settings(), &parts);
    let used: Vec<EntityId> = chosen.iter().map(|(_, p)| *p).collect();
    let in_hand: u64 = used
        .iter()
        .filter(|p| reach.carried.contains(p))
        .map(|&p| world.mass(p).mg())
        .sum();
    let after = world.carried_mass(actor).mg() - in_hand
        + used.iter().map(|&p| world.mass(p).mg()).sum::<u64>();
    let fits_in_hand = world
        .life(actor)
        .and_then(|l| l.carry_limit)
        .is_none_or(|limit| after <= limit.mg());
    let made = world.next_id();
    let mut changes = vec![Change::Assemble {
        design,
        parts: used,
        at: if fits_in_hand { actor } else { reach.here },
        datasheet: sheet,
    }];
    if world.has_words(actor) {
        changes.push(Change::Made {
            agent: actor,
            thing: made,
        });
    }
    Ok(changes)
}

/// Lifting something: it has to be portable, solid, and cool enough to hold.
/// Taking something someone here believes is theirs, while they're awake to
/// see it: they notice, and remember who. Unseen, nothing happens.
fn seen_taking(
    world: &World,
    actor: EntityId,
    thing: EntityId,
    mut changes: Vec<Change>,
) -> Vec<Change> {
    let Some(here) = world.place_of(actor) else {
        return changes;
    };
    for owner in world.contents(here) {
        let theirs = world.memory(owner).is_some_and(|m| m.owns.contains(&thing));
        if owner != actor && theirs && world.is_living(owner) && !world.is_asleep(owner) {
            changes.push(Change::Notice {
                agent: owner,
                news: crate::world::News::Took { by: actor, thing },
            });
        }
    }
    changes
}

fn lift(world: &World, actor: EntityId, found: EntityId) -> Result<Vec<Change>, Refusal> {
    if !world.is_portable(found) {
        return Err(Refusal::CannotCarry(named(world, actor, found)));
    }
    if world.composition(found).is_some() && !world.is_all(found, State::Solid) {
        return Err(Refusal::NotSolid(named(world, actor, found)));
    }
    can_carry(world, actor, world.mass(found))?;
    if world
        .temperature(found)
        .is_some_and(|t| t > world.settings().max_touch_temperature)
    {
        return Err(Refusal::TooHot(named(world, actor, found)));
    }
    Ok(vec![Change::Move {
        entity: found,
        to: actor,
    }])
}

/// A tool works something only if the tool's main material, at its current
/// temperature, is harder than every part of the target at the target's.
fn harder_than(
    world: &World,
    actor: EntityId,
    tool: EntityId,
    target: EntityId,
) -> Result<(), Refusal> {
    let reference = world.settings().reference_temperature;
    let hardness = |id: EntityId, material| {
        world.materials()[&material]
            .hardness_at(world.temperature(id).unwrap_or(reference), reference)
    };
    // An assembled tool cuts with its edge, as its datasheet measured it.
    let edge_hardness =
        world
            .assembly(tool)
            .and_then(|a| match a.datasheet.get(Property::EdgeHardness) {
                Some(Value::Hardness(h)) => Some(*h),
                _ => None,
            });
    let tool_hardness = world
        .composition(tool)
        .and_then(matter::dominant)
        .map(|m| hardness(tool, m))
        .or(edge_hardness)
        .ok_or_else(|| Refusal::NotATool(named(world, actor, tool)))?;
    let target_hardness = world
        .composition(target)
        .map(|c| c.keys().map(|&m| hardness(target, m)).max().unwrap_or(0))
        .unwrap_or(0);
    if tool_hardness > target_hardness {
        Ok(())
    } else {
        Err(Refusal::TooHard {
            tool: named(world, actor, tool),
            target: named(world, actor, target),
        })
    }
}

/// Seconds to walk between two places: the distance at the walker's speed,
/// which a full load halves. People with no body walk instantly.
/// Where a way out leads and, if its path crosses a liquid, what you cross
/// on and what you cross. Only something that floats, and carries you with
/// all you hold, can take you across.
/// What a person sees on arriving somewhere: the place and the ways they
/// came, now certain; what's there, compared with what they remember; and
/// whether what they were told about it holds.
fn arriving(world: &World, actor: EntityId, from: EntityId, to: EntityId) -> Vec<Change> {
    let Some(memory) = world.memory(actor) else {
        return Vec::new();
    };
    let mut changes = Vec::new();
    if !world.has_seen(actor, to) {
        changes.push(Change::See {
            agent: actor,
            place: to,
        });
    }
    for (a, b) in [(from, to), (to, from)] {
        if memory.finds_ways && !world.knows_way(actor, a, b) && world.exits(a).contains(&b) {
            changes.push(Change::Learn {
                agent: actor,
                from: a,
                to: b,
            });
        }
    }
    // Its fixed things and its creatures: what's worth remembering.
    let things = notable(world, actor, to);
    let gone = memory
        .sightings
        .get(&to)
        .map(|(_, before)| before.difference(&things).copied().collect())
        .unwrap_or_default();
    for (claim, source) in &memory.possible {
        if let Claim::Thing(at, name) = claim
            && *at == to
        {
            let held = world
                .contents(to)
                .into_iter()
                .any(|e| is_called(world, actor, e, name) || mentions(world, actor, e, name));
            changes.push(Change::Settle {
                agent: actor,
                claim: claim.clone(),
                held,
                source: source.clone(),
            });
        }
    }
    changes.push(Change::Sight {
        agent: actor,
        place: to,
        things,
        gone,
    });
    changes
}

/// The things at a place worth remembering: what's fixed there, and who's
/// there.
pub fn notable(world: &World, actor: EntityId, place: EntityId) -> BTreeSet<EntityId> {
    world
        .contents(place)
        .into_iter()
        .filter(|&e| e != actor && !world.is_all(e, State::Gas))
        .filter(|&e| !world.is_portable(e) || world.is_living(e))
        .collect()
}

/// A way someone was told of, from `here` to a place by that name, that
/// isn't there. Returns the place and who told them.
fn false_way(
    world: &World,
    actor: EntityId,
    here: EntityId,
    place: &str,
) -> Option<(EntityId, String)> {
    let memory = world.memory(actor)?;
    memory
        .possible
        .iter()
        .find_map(|(claim, source)| match claim {
            Claim::Way(from, to)
                if *from == here
                    && !world.exits(here).contains(to)
                    && (is_called(world, actor, *to, place)
                        || mentions(world, actor, *to, place)) =>
            {
                Some((*to, source.clone()))
            }
            _ => None,
        })
}

fn way(
    world: &World,
    actor: EntityId,
    reach: &Reach,
    place: &str,
    aboard: Option<&str>,
) -> Result<(EntityId, Option<(EntityId, EntityId)>), Refusal> {
    // Only the ways you know, or were told of: someone finding their way
    // can't be told by the world that one exists.
    let told: Vec<EntityId> = world
        .memory(actor)
        .map(|m| {
            m.possible
                .keys()
                .filter_map(|claim| match claim {
                    Claim::Way(from, to) if *from == reach.here => Some(*to),
                    _ => None,
                })
                .filter(|to| world.exits(reach.here).contains(to))
                .collect()
        })
        .unwrap_or_default();
    let candidates = world.known_exits(actor, reach.here).into_iter().chain(told);
    let to = find(world, actor, candidates, place).ok_or_else(|| {
        if world.finds_ways(actor) {
            Refusal::DontKnowWay(place.to_string())
        } else {
            Refusal::NoSuchExit(place.to_string())
        }
    })?;
    let (liquid, vessel) = match (world.crossing(reach.here, to), aboard) {
        (None, None) => return Ok((to, None)),
        (None, Some(_)) => return Err(Refusal::NothingToCross(world.label(to))),
        (Some(liquid), None) => {
            return Err(Refusal::MustCross {
                place: world.label(to),
                liquid: world.label(liquid),
            });
        }
        (Some(liquid), Some(vessel)) => (liquid, vessel),
    };
    let vessel = find(world, actor, reach.around_or_carried(), vessel)
        .ok_or_else(|| Refusal::NotHere(vessel.to_string()))?;
    if !world.is_portable(vessel) || world.is_agent(vessel) {
        return Err(Refusal::NotAVessel(named(world, actor, vessel)));
    }
    let mut load = world.mass(actor).mg() + world.carried_mass(actor).mg();
    if reach.carried.contains(&vessel) {
        load -= world.mass(vessel).mg();
    }
    match datasheet::carries_afloat(world, vessel, liquid) {
        Some(Some(spare)) if spare.mg() >= load => Ok((to, Some((vessel, liquid)))),
        Some(Some(spare)) => Err(Refusal::WouldSink {
            vessel: named(world, actor, vessel),
            carries: spare,
            load: Mass::from_mg(load),
        }),
        _ => Err(Refusal::Sinks {
            vessel: named(world, actor, vessel),
            liquid: world.label(liquid),
        }),
    }
}

/// A landmark seen from somewhere: which way, and how far.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sight {
    pub place: EntityId,
    /// A compass direction, such as "north-east".
    pub direction: &'static str,
    /// In µm, along the ground.
    pub distance: u64,
}

/// The landmarks in sight from a place, nearest first. Something can be seen
/// when it's no farther than the viewer's horizon and its own added
/// together; a horizon is √(2 × planet radius × height).
pub fn in_sight(world: &World, from: EntityId) -> Vec<Sight> {
    let radius = u128::from(world.settings().planet_radius);
    let Some((x, y)) = world.position(from).filter(|_| radius > 0) else {
        return Vec::new();
    };
    let horizon = |height: u64| (2 * radius * u128::from(height)).isqrt();
    let eye = horizon(world.height(from) + world.settings().eye_height);
    let mut sights: Vec<Sight> = world
        .from_afar
        .keys()
        .copied()
        .filter(|&place| place != from)
        .filter_map(|place| {
            let (px, py) = world.position(place)?;
            let (east, north) = (
                i128::from(px) - i128::from(x),
                i128::from(py) - i128::from(y),
            );
            let distance = u128::try_from(east * east + north * north)
                .expect("a square")
                .isqrt();
            (distance <= eye + horizon(world.height(place))).then(|| Sight {
                place,
                direction: compass(east, north),
                distance: u64::try_from(distance).unwrap_or(u64::MAX),
            })
        })
        .collect();
    sights.sort_by_key(|s| (s.distance, s.place));
    sights
}

/// One of eight compass directions for an offset east and north.
fn compass(east: i128, north: i128) -> &'static str {
    // tan 67.5° ≈ 2.414: beyond it, a direction is straight along an axis.
    let (e, n) = (east.abs() * 1_000, north.abs() * 1_000);
    let along_east = e > north.abs() * 2_414;
    let along_north = n > east.abs() * 2_414;
    match (along_north, along_east, north >= 0, east >= 0) {
        (true, _, true, _) => "north",
        (true, _, false, _) => "south",
        (_, true, _, true) => "east",
        (_, true, _, false) => "west",
        (_, _, true, true) => "north-east",
        (_, _, true, false) => "north-west",
        (_, _, false, true) => "south-east",
        (_, _, false, false) => "south-west",
    }
}

/// How long it takes to push a vessel across a liquid. A share of the
/// worker's effort goes into the liquid: more with something made to push,
/// less with bare hands. Drag takes ½ρCAv³ of power, where A is the face of a
/// cube of the vessel's volume, so speed is the cube root of 2P ÷ (ρCA).
pub fn paddling_time(
    world: &World,
    actor: EntityId,
    vessel: EntityId,
    liquid: EntityId,
    distance: u64,
) -> u64 {
    let Some(life) = world.life(actor).filter(|_| distance > 0) else {
        return 0;
    };
    let settings = world.settings();
    let share = world
        .contents(actor)
        .into_iter()
        .filter_map(|c| world.shape(c).and_then(|s| world.shapes().get(s)))
        .filter_map(|def| def.push)
        .max()
        .unwrap_or(settings.hand_push);
    let power = u128::from(life.working_power) * u128::from(share) / 10_000;
    let (Some(density), Some(volume)) = (
        datasheet::density(world, liquid),
        datasheet::volume(world, vessel),
    ) else {
        return 0;
    };
    let side = matter::cube_root(volume);
    let resistance = (density * side * side).max(1);
    // µW ÷ (g/m³ × µm²) × 2 × 10²⁷ gives µm³/s³.
    let cubed = power * 2_000_000_000_000_000_000_000_000_000 / resistance * 10_000
        / u128::from(settings.drag.max(1));
    let speed = matter::cube_root(cubed).max(1);
    u64::try_from(u128::from(distance) / speed)
        .unwrap_or(u64::MAX)
        .max(1)
}

pub fn walking_time(world: &World, actor: EntityId, from: EntityId, to: EntityId) -> u64 {
    let distance = u128::from(world.distance(from, to));
    let Some(life) = world.life(actor).filter(|_| distance > 0) else {
        return 0;
    };
    let speed = u128::from(life.walking_speed.max(1));
    let (load, limit) = match life.carry_limit {
        Some(limit) => (
            u128::from(world.carried_mass(actor).mg()),
            u128::from(limit.mg()).max(1),
        ),
        None => (0, 1),
    };
    let load = load.min(limit);
    // µm ÷ (mm/s × 1000) is seconds; a load of L of limit M slows it to (2M − L) / 2M.
    let walking = distance * 2 * limit / (speed * 1_000 * (2 * limit - load));
    // Going up, a share of the walker's working power lifts them and their
    // load: m × g × h. Going down costs nothing extra.
    let rise = u128::from(world.height(to).saturating_sub(world.height(from)));
    let lifted = u128::from(world.mass(actor).mg() + world.carried_mass(actor).mg());
    let lifting = u128::from(life.working_power) * u128::from(life.climbing_share) / 10_000;
    // mg × µm × 9.80665 m/s² is 10⁻¹² J × 9.80665; ÷ µW (10⁻⁶ J/s) gives seconds.
    let climbing = if rise == 0 || lifting == 0 {
        0
    } else {
        lifted * rise * 980_665 / 100_000 / 1_000_000 / lifting
    };
    // Bare feet on ground that hurts go slowly and carefully.
    let walking = if hurts_bare_feet(world, actor, from, to) {
        walking * 10_000 / u128::from(world.settings().bare_feet_pace.max(1))
    } else {
        walking
    };
    u64::try_from(walking + climbing).unwrap_or(u64::MAX).max(1)
}

/// Bare hands can work only what's soft enough everywhere.
fn by_hand(world: &World, actor: EntityId, target: EntityId) -> Result<(), Refusal> {
    let reference = world.settings().reference_temperature;
    let temperature = world.temperature(target).unwrap_or(reference);
    let hardest = world
        .composition(target)
        .map(|c| {
            c.keys()
                .map(|m| world.materials()[m].hardness_at(temperature, reference))
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0);
    if hardest > world.settings().hand_hardness {
        return Err(Refusal::TooHard {
            tool: "your hands".into(),
            target: named(world, actor, target),
        });
    }
    Ok(())
}

/// Refuses if a container is made of something the liquid would soften, as
/// some earths turn back to mud when wet.
fn softens(
    world: &World,
    actor: EntityId,
    container: EntityId,
    liquid: EntityId,
) -> Result<(), Refusal> {
    let Some(main) = world.composition(container).and_then(matter::dominant) else {
        return Ok(());
    };
    let softens_in = &world.materials()[&main].softens_in;
    let soaks = world
        .composition(liquid)
        .is_some_and(|c| c.keys().any(|m| softens_in.contains(m)));
    if soaks {
        return Err(Refusal::WouldSoften {
            container: named(world, actor, container),
            liquid: named(world, actor, liquid),
        });
    }
    Ok(())
}

/// Refuses if taking on `extra` would put someone over what they can carry.
fn can_carry(world: &World, who: EntityId, extra: Mass) -> Result<(), Refusal> {
    let Some(limit) = world.life(who).and_then(|l| l.carry_limit) else {
        return Ok(());
    };
    let carrying = world.carried_mass(who);
    if carrying.mg().saturating_add(extra.mg()) > limit.mg() {
        return Err(Refusal::TooHeavy { carrying, limit });
    }
    Ok(())
}

/// The first thing carried that's the shape or design `needs` names.
fn needed_tool(world: &World, carried: &[EntityId], needs: Option<&str>) -> Option<EntityId> {
    let needs = needs?;
    carried.iter().copied().find(|&p| {
        world.shape(p) == Some(needs)
            || world
                .assembly(p)
                .is_some_and(|a| a.design.as_deref() == Some(needs))
    })
}

/// How fine a tool's working edge is, in µm: an assembly's measured edge, a
/// cutting part's tolerance, or the world's rough tolerance for anything else.
/// Whether something has an edge: a cutting shape, or an assembly that
/// measured one.
fn has_edge(world: &World, tool: EntityId) -> bool {
    world
        .shape(tool)
        .and_then(|s| world.shapes().get(s))
        .is_some_and(|s| s.role == Some(crate::world::Role::Cutting))
        || world
            .assembly(tool)
            .is_some_and(|a| a.datasheet.get(Property::EdgeWidth).is_some())
}

fn edge_width(world: &World, tool: EntityId) -> u64 {
    if let Some(Value::Length(edge)) = world
        .assembly(tool)
        .and_then(|a| a.datasheet.get(Property::EdgeWidth))
    {
        return *edge;
    }
    let cutting = world
        .shape(tool)
        .and_then(|s| world.shapes().get(s))
        .is_some_and(|s| s.role == Some(crate::world::Role::Cutting));
    match world.tolerance(tool) {
        Some(tolerance) if cutting => tolerance,
        _ => world.settings().rough_tolerance,
    }
}

/// Whether anything inside `chamber` can burn.
pub fn holds_fuel(world: &World, chamber: EntityId) -> bool {
    world.contents(chamber).iter().any(|&e| {
        world
            .composition(e)
            .is_some_and(|c| c.keys().any(|m| world.materials()[m].burns()))
    })
}

fn must_know(world: &World, actor: EntityId, process: Process) -> Result<(), Refusal> {
    if knows_how(world, actor, process) {
        Ok(())
    } else {
        Err(Refusal::DoesntKnowHow(process))
    }
}

/// Always true until knowledge arrives in slice 4.
fn knows_how(_world: &World, _actor: EntityId, _process: Process) -> bool {
    true
}

/// Finds another person in the same place as `actor`.
fn person_here(
    world: &World,
    actor: EntityId,
    around: &[EntityId],
    name: &str,
) -> Result<EntityId, Refusal> {
    if let Some(found) = find(
        world,
        actor,
        around.iter().copied().filter(|&e| world.is_agent(e)),
        name,
    ) {
        return Ok(found);
    }
    if is_called(world, actor, actor, name) {
        return Err(Refusal::NotYourself);
    }
    Err(Refusal::NoOneHere(name.to_string()))
}

/// The thing among `candidates` that `viewer` would take `name` to mean.
pub(crate) fn finds(
    world: &World,
    viewer: EntityId,
    candidates: impl IntoIterator<Item = EntityId>,
    name: &str,
) -> Option<EntityId> {
    find(world, viewer, candidates, name)
}

/// The finest edge among `things`, if any has one: what someone would
/// strike with.
pub(crate) fn best_edge(world: &World, things: &[EntityId]) -> Option<EntityId> {
    things
        .iter()
        .copied()
        .filter(|&t| has_edge(world, t))
        .min_by_key(|&t| (edge_width(world, t), t))
}

/// How to point at a thing, rather than name it: "#12". Anyone can point at
/// what they perceive.
pub fn pointer(id: EntityId) -> String {
    format!("#{}", id.0)
}

/// Whether `viewer` calls `id` by `name`.
pub(crate) fn calls(world: &World, viewer: EntityId, id: EntityId, name: &str) -> bool {
    is_called(world, viewer, id, name)
}

/// The first candidate called exactly `name`, or failing that, the first
/// whose description contains `name` ("lump" finds "lump of anything"), as
/// `viewer` calls things. Candidates with the same label are interchangeable,
/// so taking the first keeps the choice deterministic.
fn find(
    world: &World,
    viewer: EntityId,
    candidates: impl IntoIterator<Item = EntityId>,
    name: &str,
) -> Option<EntityId> {
    let candidates: Vec<EntityId> = candidates.into_iter().collect();
    // "smallest …" and "largest …" choose by mass among the matches.
    let lower = normalize(name);
    for (word, smallest) in [("smallest ", true), ("largest ", false)] {
        if let Some(rest) = lower.strip_prefix(word) {
            let matching = candidates.iter().copied().filter(|&id| {
                is_called(world, viewer, id, rest) || mentions(world, viewer, id, rest)
            });
            return if smallest {
                matching.min_by_key(|&id| (world.mass(id), id))
            } else {
                matching.max_by_key(|&id| (world.mass(id), std::cmp::Reverse(id)))
            };
        }
    }
    closest(world, viewer, &candidates, name).1.first().copied()
}

/// The candidates `name` fits best, and whether they fit it exactly. Anyone
/// means first what's called exactly that, and last anything whose name
/// mentions it. Someone with words of their own means, between those, what
/// the name says a thing is ("x" before "y of x"), then a plain piece of the
/// material the name is for, before something made partly of it.
fn closest(
    world: &World,
    viewer: EntityId,
    candidates: &[EntityId],
    name: &str,
) -> (bool, Vec<EntityId>) {
    let matching = |test: &dyn Fn(EntityId) -> bool| -> Vec<EntityId> {
        candidates.iter().copied().filter(|&id| test(id)).collect()
    };
    let exact = matching(&|id| is_called(world, viewer, id, name));
    if !exact.is_empty() {
        return (true, exact);
    }
    if world.has_words(viewer) {
        let first = matching(&|id| begins(world, viewer, id, name));
        if !first.is_empty() {
            return (false, first);
        }
        let wanted = normalize(name);
        let plain = matching(&|id| {
            world.assembly(id).is_none()
                && !world.is_agent(id)
                && world.kind_of(id).is_none()
                && world
                    .composition(id)
                    .and_then(matter::dominant)
                    .and_then(|m| world.material_word_for(viewer, m))
                    .is_some_and(|word| {
                        let words: Vec<&str> = word.split_whitespace().collect();
                        let wanted: Vec<&str> = wanted.split_whitespace().collect();
                        !wanted.is_empty() && words.windows(wanted.len()).any(|w| w == wanted)
                    })
        });
        if !plain.is_empty() {
            return (false, plain);
        }
    }
    (false, matching(&|id| mentions(world, viewer, id, name)))
}

/// True if `viewer`'s label for the entity starts with the words of `name`.
fn begins(world: &World, viewer: EntityId, id: EntityId, name: &str) -> bool {
    let wanted = normalize(name);
    let label = normalize(&world.label_for(viewer, id));
    !wanted.is_empty() && (label == wanted || label.starts_with(&format!("{wanted} ")))
}

/// When a name fits several things `viewer` can see that look different,
/// and none exactly, which they might mean, as they'd describe each. Only
/// for someone with words of their own; others take the first that fits.
fn which(
    world: &World,
    viewer: EntityId,
    candidates: impl IntoIterator<Item = EntityId>,
    name: &str,
) -> Option<Refusal> {
    let lower = normalize(name);
    if !world.has_words(viewer) || lower.starts_with("smallest ") || lower.starts_with("largest ") {
        return None;
    }
    let candidates: Vec<EntityId> = candidates.into_iter().collect();
    let (exact, fits) = closest(world, viewer, &candidates, name);
    if exact {
        return None;
    }
    let mut labels: Vec<String> = Vec::new();
    for id in fits {
        let label = named(world, viewer, id);
        if !labels.contains(&label) {
            labels.push(label);
        }
    }
    (labels.len() > 1).then_some(Refusal::Which {
        name: lower,
        options: labels,
    })
}

/// Matches an entity's label, as `viewer` calls it, ignoring case and a
/// leading "the", "a", or "an", or its ID. Someone with words of their own
/// can use only the IDs of things made in play ("#12"), which a client uses
/// to point at something; data IDs are names they may not know.
fn is_called(world: &World, viewer: EntityId, id: EntityId, name: &str) -> bool {
    // "#12" points at the thing itself, whatever anyone calls it.
    if name.trim() == pointer(id) {
        return true;
    }
    let wanted = normalize(name);
    let key = world.key(id);
    let by_key = (!world.has_words(viewer) || key.starts_with('#')) && normalize(key) == wanted;
    by_key || normalize(&world.label_for(viewer, id)) == wanted
}

/// True if the words of `name` appear, in order, in the entity's label, as
/// `viewer` calls it.
fn mentions(world: &World, viewer: EntityId, id: EntityId, name: &str) -> bool {
    let wanted: Vec<String> = normalize(name)
        .split_whitespace()
        .map(String::from)
        .collect();
    if wanted.is_empty() {
        return false;
    }
    let label = normalize(&world.label_for(viewer, id));
    let words: Vec<&str> = label
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|w| !w.is_empty())
        .collect();
    words
        .windows(wanted.len())
        .any(|w| w.iter().zip(&wanted).all(|(a, b)| a == b))
}

/// A name as typed, lower case, without a leading article, and without a
/// trailing "one" as in "the heavy one".
fn normalize(name: &str) -> String {
    let mut lower = name.trim().to_lowercase();
    for article in ["the ", "a ", "an "] {
        if let Some(rest) = lower.strip_prefix(article) {
            lower = rest.trim().to_string();
            break;
        }
    }
    match lower.strip_suffix(" one") {
        Some(rest) if !rest.trim().is_empty() => rest.trim().to_string(),
        _ => lower,
    }
}

/// How to refer to something in a sentence: people and names that already
/// start with "the" as they are, other things with "the" in front.
pub fn named(world: &World, viewer: EntityId, id: EntityId) -> String {
    let label = world.label_for(viewer, id);
    let proper = world.is_agent(id)
        || label.starts_with("the ")
        || label.starts_with("something ")
        || label.chars().next().is_some_and(char::is_uppercase);
    if proper {
        label
    } else {
        format!("the {label}")
    }
}
