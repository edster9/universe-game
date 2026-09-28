//! The laws decide whether an intent is allowed and which changes it makes.
//! They never change the world themselves: they hand changes to the gate.
//!
//! Names only resolve among what the actor can perceive. Nothing here asks
//! what something *is* anywhere else in the world (docs/slices.md, "No
//! oracles").

use std::fmt;

use crate::gate::{Change, Fault};
use crate::intent::Intent;
use crate::units::Credits;
use crate::world::{EntityId, World};

/// Why the laws refused an intent. A refused intent changes nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    NotAnAgent,
    Nowhere,
    NoSuchExit(String),
    NotHere(String),
    AlreadyCarrying(String),
    NotCarrying(String),
    CannotCarry(String),
    NoOneHere(String),
    NotYourself,
    ZeroAmount,
    NotEnough { have: Credits, want: Credits },
    TooMuch,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::NotAnAgent => write!(f, "only a person can do that"),
            Refusal::Nowhere => write!(f, "you aren't standing anywhere"),
            Refusal::NoSuchExit(name) => write!(f, "there's no way to {name} from here"),
            Refusal::NotHere(name) => write!(f, "you don't see {name} here"),
            Refusal::AlreadyCarrying(name) => write!(f, "you're already carrying {name}"),
            Refusal::NotCarrying(name) => write!(f, "you aren't carrying {name}"),
            Refusal::CannotCarry(label) => write!(f, "you can't pick up {label}"),
            Refusal::NoOneHere(name) => write!(f, "there's nobody called {name} here"),
            Refusal::NotYourself => write!(f, "you can't do that to yourself"),
            Refusal::ZeroAmount => write!(f, "you have to pay at least 1 credit"),
            Refusal::NotEnough { have, want } => write!(f, "you only have {have}, not {want}"),
            Refusal::TooMuch => write!(f, "they can't hold that many credits"),
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

/// Checks `intent` against the laws and, if allowed, applies it through the
/// gate. Returns the changes that were made.
pub fn act(world: &mut World, actor: EntityId, intent: Intent) -> Result<Vec<Change>, ActError> {
    let changes = resolve(world, actor, &intent).map_err(ActError::Refused)?;
    world
        .apply(actor, intent, changes.clone())
        .map_err(ActError::Fault)?;
    Ok(changes)
}

/// Works out what `intent` would change, without changing anything.
pub fn resolve(world: &World, actor: EntityId, intent: &Intent) -> Result<Vec<Change>, Refusal> {
    if !world.is_agent(actor) {
        return Err(Refusal::NotAnAgent);
    }
    let here = world
        .location(actor)
        .filter(|&place| world.is_place(place))
        .ok_or(Refusal::Nowhere)?;
    let around: Vec<EntityId> = world
        .contents(here)
        .into_iter()
        .filter(|&e| e != actor)
        .collect();
    let carried = world.contents(actor);

    match intent {
        Intent::Go { place } => {
            let to = find(world, world.exits(here).iter().copied(), place)
                .ok_or_else(|| Refusal::NoSuchExit(place.clone()))?;
            Ok(vec![Change::Move { entity: actor, to }])
        }

        Intent::Take { item } => {
            if let Some(found) = find(world, around.iter().copied(), item) {
                if !world.is_portable(found) {
                    return Err(Refusal::CannotCarry(world.label(found).to_string()));
                }
                return Ok(vec![Change::Move {
                    entity: found,
                    to: actor,
                }]);
            }
            if find(world, carried.iter().copied(), item).is_some() {
                return Err(Refusal::AlreadyCarrying(item.clone()));
            }
            if is_called(world, actor, item) {
                return Err(Refusal::NotYourself);
            }
            Err(Refusal::NotHere(item.clone()))
        }

        Intent::Drop { item } => {
            let found = find(world, carried.iter().copied(), item)
                .ok_or_else(|| Refusal::NotCarrying(item.clone()))?;
            Ok(vec![Change::Move {
                entity: found,
                to: here,
            }])
        }

        Intent::Give { item, to } => {
            let found = find(world, carried.iter().copied(), item)
                .ok_or_else(|| Refusal::NotCarrying(item.clone()))?;
            let recipient = person_here(world, actor, &around, to)?;
            Ok(vec![Change::Move {
                entity: found,
                to: recipient,
            }])
        }

        Intent::Pay { to, amount } => {
            let recipient = person_here(world, actor, &around, to)?;
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
    }
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

/// The first candidate called `name`. Candidates with the same label are
/// interchangeable, so taking the first keeps the choice deterministic.
fn find(
    world: &World,
    candidates: impl IntoIterator<Item = EntityId>,
    name: &str,
) -> Option<EntityId> {
    candidates
        .into_iter()
        .find(|&id| is_called(world, id, name))
}

/// Matches an entity's data ID or its label, ignoring case and a leading
/// "the", "a", or "an".
fn is_called(world: &World, id: EntityId, name: &str) -> bool {
    let wanted = normalize(name);
    normalize(world.key(id)) == wanted || normalize(world.label(id)) == wanted
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
