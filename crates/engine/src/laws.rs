//! The laws decide whether an intent is allowed and which changes it makes.
//! They never change the world themselves: they hand changes to the gate.
//!
//! Names only resolve among what the actor can perceive. Nothing here asks
//! what something *is* anywhere else in the world (docs/slices.md, "No
//! oracles").

use std::fmt;

use crate::datasheet;
use crate::gate::{Cause, Change, Fault};
use crate::intent::Intent;
use crate::matter::{self, Composition, State};
use crate::nature;
use crate::units::{Credits, Mass};
use crate::world::{EntityId, Requirement, World};

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
    InContainer { item: String, container: String },
    NotInside { item: String, container: String },
    AlreadyCarrying(String),
    NotCarrying(String),
    CannotCarry(String),
    NotAContainer(String),
    IntoItself,
    NoOneHere(String),
    NotYourself,
    ZeroAmount,
    NotEnough { have: Credits, want: Credits },
    TooMuch,
    DoesntKnowHow(Process),
    NotDiggable(String),
    NotATool(String),
    TooHard { tool: String, target: String },
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
    MissingPart { slot: String, needs: String },
    NotAnAssembly(String),
    NotAPart(String),
    AsFineAsItGets(String),
    CannotEat(String),
    NotDrinkable(String),
    NotThirsty,
    NotGatherable(String),
    NeedsTool { source: String, tool: String },
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
                write!(f, "{tool} isn't hard enough to work {target}")
            }
            Refusal::Exhausted(name) => write!(f, "there's too little left of {name} to dig"),
            Refusal::NotAChamber(name) => write!(f, "you can't light {name}"),
            Refusal::AlreadyLit(name) => write!(f, "{name} is already lit"),
            Refusal::NoFuel(name) => write!(f, "there's nothing in {name} that burns"),
            Refusal::NotLiquid(name) => write!(f, "{name} isn't liquid"),
            Refusal::WouldMelt(name) => write!(f, "{name} would melt"),
            Refusal::UnknownShape(name) => write!(f, "there's no shape called {name}"),
            Refusal::NotSolid(name) => write!(f, "{name} isn't solid"),
            Refusal::TooHot(name) => write!(f, "{name} is too hot to touch"),
            Refusal::CannotWork(name) => write!(f, "{name} can't be shaped"),
            Refusal::UnknownDesign(name) => write!(f, "you don't know a design called {name}"),
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
            Refusal::NeedsTool { source, tool } => write!(f, "you need a {tool} for {source}"),
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
    if seconds > 0 && world.is_living(actor) {
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

/// Like `act`, then lets the world run for as long as the action takes.
pub fn perform(
    world: &mut World,
    actor: EntityId,
    intent: Intent,
) -> Result<Vec<Change>, ActError> {
    let plan = plan(world, actor, &intent).map_err(ActError::Refused)?;
    world
        .apply(Cause::Action { actor, intent }, plan.changes.clone())
        .map_err(ActError::Fault)?;
    nature::run(world, plan.seconds).map_err(ActError::Fault)?;
    Ok(plan.changes)
}

/// How many seconds an action takes. Most are quick enough to count as none.
pub fn duration(world: &World, actor: EntityId, intent: &Intent) -> u64 {
    match intent {
        Intent::Rub { seconds, .. } => seconds.unwrap_or(world.settings().rubbing_time),
        Intent::Gather { source } => Reach::of(world, actor)
            .ok()
            .and_then(|reach| find(world, reach.around.iter().copied(), source))
            .and_then(|found| world.pieces(found))
            .map_or(0, |pieces| pieces.find_time),
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
/// place itself.
pub fn find_reachable(world: &World, actor: EntityId, name: &str) -> Option<EntityId> {
    let reach = Reach::of(world, actor).ok()?;
    if normalize(name) == "here" {
        return Some(reach.here);
    }
    if is_called(world, actor, name) || normalize(name) == "me" {
        return Some(actor);
    }
    let nearby = reach.around.iter().chain(&reach.inside).copied();
    find(world, reach.carried.iter().copied(), name).or_else(|| find(world, nearby, name))
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
    let carried = || reach.carried.iter().copied();
    let carrying = |name: &String| {
        find(world, carried(), name).ok_or_else(|| Refusal::NotCarrying(name.clone()))
    };

    match intent {
        Intent::Go { place } => {
            let to = find(world, world.exits(reach.here).iter().copied(), place)
                .ok_or_else(|| Refusal::NoSuchExit(place.clone()))?;
            Ok(vec![Change::Move { entity: actor, to }])
        }

        Intent::Take { item } => {
            if let Some(found) = find(world, reach.around.iter().copied(), item) {
                return lift(world, actor, found);
            }
            if let Some(found) = find(world, reach.inside.iter().copied(), item) {
                let container = world.location(found).expect("inside something");
                return Err(Refusal::InContainer {
                    item: named(world, found),
                    container: named(world, container),
                });
            }
            if find(world, carried(), item).is_some() {
                return Err(Refusal::AlreadyCarrying(item.clone()));
            }
            if is_called(world, actor, item) {
                return Err(Refusal::NotYourself);
            }
            Err(Refusal::NotHere(item.clone()))
        }

        Intent::TakeFrom { item, from } => {
            let container = find(world, reach.around_or_carried(), from)
                .ok_or_else(|| Refusal::NotHere(from.clone()))?;
            if !world.is_container(container) {
                return Err(Refusal::NotAContainer(named(world, container)));
            }
            let found =
                find(world, world.held(container), item).ok_or_else(|| Refusal::NotInside {
                    item: item.clone(),
                    container: named(world, container),
                })?;
            lift(world, actor, found)
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
            let container = find(world, reach.around_or_carried(), into)
                .ok_or_else(|| Refusal::NotHere(into.clone()))?;
            if !world.is_container(container) {
                return Err(Refusal::NotAContainer(named(world, container)));
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
            let from = find(world, reach.around.iter().copied(), source)
                .ok_or_else(|| Refusal::NotHere(source.clone()))?;
            let composition = world
                .composition(from)
                .filter(|_| !world.is_portable(from))
                .ok_or_else(|| Refusal::NotDiggable(named(world, from)))?;
            let tool = carrying(tool)?;
            harder_than(world, tool, from)?;
            let amount = world.settings().dig_amount;
            if world.mass(from) <= amount {
                return Err(Refusal::Exhausted(named(world, from)));
            }
            let take =
                matter::proportional(composition, amount).expect("there is more than the amount");
            Ok(vec![Change::Split {
                from,
                take,
                at: actor,
            }])
        }

        Intent::Light { chamber } => {
            must_know(world, actor, Process::Light)?;
            let found = find(world, reach.around.iter().copied(), chamber)
                .ok_or_else(|| Refusal::NotHere(chamber.clone()))?;
            let state = world
                .chamber(found)
                .ok_or_else(|| Refusal::NotAChamber(named(world, found)))?;
            if state.lit {
                return Err(Refusal::AlreadyLit(named(world, found)));
            }
            if !holds_fuel(world, found) {
                return Err(Refusal::NoFuel(named(world, found)));
            }
            Ok(vec![Change::Light {
                chamber: found,
                lit: true,
            }])
        }

        Intent::Pour { liquid, into } => {
            must_know(world, actor, Process::Pour)?;
            let candidates = reach.around.iter().chain(&reach.inside).copied();
            let found =
                find(world, candidates, liquid).ok_or_else(|| Refusal::NotHere(liquid.clone()))?;
            if !world.is_all(found, State::Liquid) {
                return Err(Refusal::NotLiquid(named(world, found)));
            }
            let target = find(world, reach.around.iter().copied(), into)
                .ok_or_else(|| Refusal::NotHere(into.clone()))?;
            if !world.is_container(target) {
                return Err(Refusal::NotAContainer(named(world, target)));
            }
            // A container can't hold anything hotter than it can stand.
            let limit = world
                .composition(target)
                .and_then(matter::dominant)
                .map(|m| world.materials()[&m].melting_point);
            if let (Some(limit), Some(heat)) = (limit, world.temperature(found))
                && heat >= limit
            {
                return Err(Refusal::WouldMelt(named(world, target)));
            }
            Ok(vec![Change::Move {
                entity: found,
                to: target,
            }])
        }

        Intent::Work { item, shape, tool } => {
            must_know(world, actor, Process::Work)?;
            let wanted = normalize(shape);
            let shape_key = world
                .shapes()
                .iter()
                .find(|(key, def)| normalize(key) == wanted || normalize(&def.label) == wanted)
                .map(|(key, _)| key.clone())
                .ok_or_else(|| Refusal::UnknownShape(shape.clone()))?;
            // What's in hand first, then what's around.
            let nearby = reach.around.iter().chain(&reach.inside).copied();
            let found = find(world, reach.carried.iter().copied(), item)
                .or_else(|| find(world, nearby, item))
                .ok_or_else(|| Refusal::NotHere(item.clone()))?;
            if world.composition(found).is_none() || world.is_container(found) {
                return Err(Refusal::CannotWork(named(world, found)));
            }
            if !world.is_all(found, State::Solid) {
                return Err(Refusal::NotSolid(named(world, found)));
            }
            let tool = carrying(tool)?;
            if tool == found {
                return Err(Refusal::NotYourself);
            }
            harder_than(world, tool, found)?;
            // A part is at most as precise as the tool that made it.
            let tolerance = world
                .tolerance(tool)
                .unwrap_or(world.settings().rough_tolerance);
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
            let second = find(world, carried().filter(|&p| p != first), against)
                .or_else(|| find(world, carried(), against))
                .ok_or_else(|| Refusal::NotCarrying(against.clone()))?;
            if first == second {
                return Err(Refusal::NotYourself);
            }
            for piece in [first, second] {
                if world.composition(piece).is_none() {
                    return Err(Refusal::NotAPart(named(world, piece)));
                }
                if !world.is_all(piece, State::Solid) {
                    return Err(Refusal::NotSolid(named(world, piece)));
                }
            }
            let settings = world.settings();
            let mut changes = Vec::new();

            // Rubbing two parts together wears each against the other, and
            // both come out finer than either tool that made them.
            let session = seconds.unwrap_or(settings.rubbing_time);
            let (a, b) = (world.tolerance(first), world.tolerance(second));
            if let (Some(a), Some(b)) = (a, b) {
                // A full session improves by the world's rubbing improvement;
                // a shorter or longer one in proportion.
                let improvement = u128::from(settings.rubbing_improvement) * u128::from(session)
                    / u128::from(settings.rubbing_time.max(1));
                let keep = 10_000u64.saturating_sub(u64::try_from(improvement).unwrap_or(10_000));
                for (part, tolerance) in [(first, a), (second, b)] {
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
                    return Err(Refusal::AsFineAsItGets(named(world, first)));
                }
            }

            // A living worker's effort wears dust off the softer of the two,
            // and friction heats it. The dust falls into `into`, or on the
            // ground. Nature carries the rubbing on for as long as it lasts.
            if world.is_living(actor) {
                let target = match into {
                    Some(name) => {
                        let found = find(world, reach.around.iter().copied(), name)
                            .ok_or_else(|| Refusal::NotHere(name.clone()))?;
                        if !world.is_container(found) {
                            return Err(Refusal::NotAContainer(named(world, found)));
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
                return Err(Refusal::NotAPart(named(world, first)));
            }
            Ok(changes)
        }

        Intent::Assemble { design } => {
            must_know(world, actor, Process::Assemble)?;
            let wanted = normalize(design);
            let (key, def) = world
                .designs()
                .iter()
                .find(|(key, def)| normalize(key) == wanted || normalize(&def.label) == wanted)
                .ok_or_else(|| Refusal::UnknownDesign(design.clone()))?;
            // Fill each slot with the first carried part that fits it.
            let mut used: Vec<EntityId> = Vec::new();
            let mut parts = Vec::new();
            for (slot, requirement) in &def.slots {
                let fits = |part: EntityId| match requirement {
                    Requirement::Shape(shape) => {
                        world.shape(part) == Some(shape.as_str())
                            && world.is_all(part, State::Solid)
                    }
                    Requirement::Design(inner) => {
                        world.assembly(part).is_some_and(|a| &a.design == inner)
                    }
                    Requirement::Material(material) => {
                        world.composition(part).and_then(matter::dominant) == Some(*material)
                            && world.is_all(part, State::Solid)
                    }
                };
                let part = carried()
                    .find(|&p| !used.contains(&p) && fits(p))
                    .ok_or_else(|| Refusal::MissingPart {
                        slot: slot.clone(),
                        needs: requirement_label(world, requirement),
                    })?;
                used.push(part);
                parts.push((
                    slot.clone(),
                    world.label(part),
                    datasheet::measure(world, part),
                ));
            }
            // Measure it once, now, from the parts' datasheets.
            let sheet = datasheet::measure_assembly(world.settings(), &parts);
            Ok(vec![Change::Assemble {
                design: key.clone(),
                parts: used,
                at: actor,
                datasheet: sheet,
            }])
        }

        Intent::Disassemble { item } => {
            must_know(world, actor, Process::Assemble)?;
            let found = find(world, reach.around_or_carried(), item)
                .ok_or_else(|| Refusal::NotHere(item.clone()))?;
            if world.assembly(found).is_none() {
                return Err(Refusal::NotAnAssembly(named(world, found)));
            }
            Ok(vec![Change::Disassemble { assembly: found }])
        }

        Intent::Eat { item } => {
            let found = carrying(item)?;
            let life = world
                .life(actor)
                .ok_or_else(|| Refusal::CannotEat(named(world, found)))?;
            let composition = world
                .composition(found)
                .filter(|_| world.assembly(found).is_none() && !world.is_container(found))
                .ok_or_else(|| Refusal::CannotEat(named(world, found)))?;
            // Only what the body can digest goes in; the rest is left in the hand.
            let digestible: Composition = composition
                .iter()
                .filter(|(m, _)| life.digests.contains(m))
                .map(|(&m, &mass)| (m, mass))
                .collect();
            if digestible.is_empty() {
                return Err(Refusal::CannotEat(named(world, found)));
            }
            if world.in_use(found) {
                return Err(Refusal::CannotEat(named(world, found)));
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
            let candidates = reach
                .around
                .iter()
                .chain(&reach.inside)
                .chain(&reach.carried)
                .copied();
            let found =
                find(world, candidates, source).ok_or_else(|| Refusal::NotHere(source.clone()))?;
            let life = world
                .life(actor)
                .ok_or_else(|| Refusal::NotDrinkable(named(world, found)))?;
            // Only something that is nothing but the body's fluid, and liquid.
            let pure = world
                .composition(found)
                .is_some_and(|c| c.len() == 1 && c.contains_key(&life.fluid))
                && world.is_all(found, State::Liquid);
            if !pure {
                return Err(Refusal::NotDrinkable(named(world, found)));
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
                .ok_or_else(|| Refusal::CannotWork(named(world, found)))?;
            if !world.is_all(found, State::Solid) {
                return Err(Refusal::NotSolid(named(world, found)));
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
                    target: named(world, found),
                });
            }
            let half = Mass::from_mg(world.mass(found).mg() / 2);
            let take = matter::proportional(composition, half)
                .filter(|t| !t.is_empty())
                .ok_or_else(|| Refusal::CannotWork(named(world, found)))?;
            Ok(vec![Change::Split {
                from: found,
                take,
                at: actor,
            }])
        }

        Intent::Gather { source } => {
            must_know(world, actor, Process::Gather)?;
            let found = find(world, reach.around.iter().copied(), source)
                .ok_or_else(|| Refusal::NotHere(source.clone()))?;
            let (Some(pieces), Some(composition)) = (world.pieces(found), world.composition(found))
            else {
                return Err(Refusal::NotGatherable(named(world, found)));
            };
            if world.mass(found) <= pieces.size {
                return Err(Refusal::Exhausted(named(world, found)));
            }
            // Some sources can't be gathered with bare hands.
            if let Some(needs) = &pieces.needs {
                let fits = |p: EntityId| {
                    world.shape(p) == Some(needs.as_str())
                        || world.assembly(p).is_some_and(|a| &a.design == needs)
                };
                if !carried().any(fits) {
                    let tool = world
                        .shapes()
                        .get(needs)
                        .map(|s| s.label.clone())
                        .or_else(|| world.designs().get(needs).map(|d| d.label.clone()))
                        .unwrap_or_else(|| needs.clone());
                    return Err(Refusal::NeedsTool {
                        source: named(world, found),
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

fn requirement_label(world: &World, requirement: &Requirement) -> String {
    match requirement {
        Requirement::Shape(shape) => world
            .shapes()
            .get(shape)
            .map_or(shape.clone(), |s| s.label.clone()),
        Requirement::Design(design) => world
            .designs()
            .get(design)
            .map_or(design.clone(), |d| d.label.clone()),
        Requirement::Material(material) => {
            format!("piece of {}", world.materials()[material].label)
        }
    }
}

/// Lifting something: it has to be portable, solid, and cool enough to hold.
fn lift(world: &World, actor: EntityId, found: EntityId) -> Result<Vec<Change>, Refusal> {
    if !world.is_portable(found) {
        return Err(Refusal::CannotCarry(named(world, found)));
    }
    if world.composition(found).is_some() && !world.is_all(found, State::Solid) {
        return Err(Refusal::NotSolid(named(world, found)));
    }
    if world
        .temperature(found)
        .is_some_and(|t| t > world.settings().max_touch_temperature)
    {
        return Err(Refusal::TooHot(named(world, found)));
    }
    Ok(vec![Change::Move {
        entity: found,
        to: actor,
    }])
}

/// A tool works something only if the tool's main material, at its current
/// temperature, is harder than every part of the target at the target's.
fn harder_than(world: &World, tool: EntityId, target: EntityId) -> Result<(), Refusal> {
    let reference = world.settings().reference_temperature;
    let hardness = |id: EntityId, material| {
        world.materials()[&material]
            .hardness_at(world.temperature(id).unwrap_or(reference), reference)
    };
    let tool_hardness = world
        .composition(tool)
        .and_then(matter::dominant)
        .map(|m| hardness(tool, m))
        .ok_or_else(|| Refusal::NotATool(named(world, tool)))?;
    let target_hardness = world
        .composition(target)
        .map(|c| c.keys().map(|&m| hardness(target, m)).max().unwrap_or(0))
        .unwrap_or(0);
    if tool_hardness > target_hardness {
        Ok(())
    } else {
        Err(Refusal::TooHard {
            tool: named(world, tool),
            target: named(world, target),
        })
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
        around.iter().copied().filter(|&e| world.is_agent(e)),
        name,
    ) {
        return Ok(found);
    }
    if is_called(world, actor, name) {
        return Err(Refusal::NotYourself);
    }
    Err(Refusal::NoOneHere(name.to_string()))
}

/// The first candidate called exactly `name`, or failing that, the first
/// whose description contains `name` ("lump" finds "lump of anything"). Candidates
/// with the same label are interchangeable, so taking the first keeps the
/// choice deterministic.
fn find(
    world: &World,
    candidates: impl IntoIterator<Item = EntityId>,
    name: &str,
) -> Option<EntityId> {
    let candidates: Vec<EntityId> = candidates.into_iter().collect();
    // "smallest …" and "largest …" choose by mass among the matches.
    let lower = normalize(name);
    for (word, smallest) in [("smallest ", true), ("largest ", false)] {
        if let Some(rest) = lower.strip_prefix(word) {
            let matching = candidates
                .iter()
                .copied()
                .filter(|&id| is_called(world, id, rest) || mentions(world, id, rest));
            return if smallest {
                matching.min_by_key(|&id| (world.mass(id), id))
            } else {
                matching.max_by_key(|&id| (world.mass(id), std::cmp::Reverse(id)))
            };
        }
    }
    candidates
        .iter()
        .copied()
        .find(|&id| is_called(world, id, name))
        .or_else(|| {
            candidates
                .iter()
                .copied()
                .find(|&id| mentions(world, id, name))
        })
}

/// Matches an entity's data ID or its label, ignoring case and a leading
/// "the", "a", or "an".
fn is_called(world: &World, id: EntityId, name: &str) -> bool {
    let wanted = normalize(name);
    normalize(world.key(id)) == wanted || normalize(&world.label(id)) == wanted
}

/// True if the words of `name` appear, in order, in the entity's label.
fn mentions(world: &World, id: EntityId, name: &str) -> bool {
    let wanted: Vec<String> = normalize(name)
        .split_whitespace()
        .map(String::from)
        .collect();
    if wanted.is_empty() {
        return false;
    }
    let label = normalize(&world.label(id));
    let words: Vec<&str> = label.split_whitespace().collect();
    words
        .windows(wanted.len())
        .any(|w| w.iter().zip(&wanted).all(|(a, b)| a == b))
}

fn normalize(name: &str) -> String {
    let lower = name.trim().to_lowercase();
    for article in ["the ", "a ", "an "] {
        if let Some(rest) = lower.strip_prefix(article) {
            return rest.trim().to_string();
        }
    }
    lower
}

/// How to refer to something in a sentence: people and names that already
/// start with "the" as they are, other things with "the" in front.
pub fn named(world: &World, id: EntityId) -> String {
    let label = world.label(id);
    let proper = world.is_agent(id)
        || label.starts_with("the ")
        || label.chars().next().is_some_and(char::is_uppercase);
    if proper {
        label
    } else {
        format!("the {label}")
    }
}
