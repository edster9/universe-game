//! What a person perceives. Views are data, not text: the console formats
//! them now, and a browser will later.

use crate::matter::State;
use crate::units::{Credits, Mass, Temperature};
use crate::world::{EntityId, World};

/// Temperatures closer than this to the surroundings aren't worth mentioning.
const NOTICEABLE_MK: u64 = 5_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thing {
    pub label: String,
    pub mass: Mass,
    /// Only when noticeably warmer or colder than the surroundings.
    pub temperature: Option<Temperature>,
    /// Short notes such as "lit".
    pub notes: Vec<String>,
    /// What's inside, if it's a container.
    pub contents: Vec<Thing>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Look {
    pub place: String,
    pub exits: Vec<String>,
    pub people: Vec<String>,
    pub things: Vec<Thing>,
    /// Gases hanging in the place.
    pub air: Vec<Thing>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inventory {
    pub things: Vec<Thing>,
    pub carried: Mass,
    pub credits: Credits,
}

/// What `actor` sees where they stand. `None` if they aren't in a place.
pub fn look(world: &World, actor: EntityId) -> Option<Look> {
    let here = world.location(actor).filter(|&p| world.is_place(p))?;
    let mut look = Look {
        place: world.label(here),
        exits: world.exits(here).iter().map(|&e| world.label(e)).collect(),
        people: Vec::new(),
        things: Vec::new(),
        air: Vec::new(),
    };
    for id in world.contents(here).into_iter().filter(|&e| e != actor) {
        if world.is_agent(id) {
            look.people.push(world.label(id));
        } else if world.is_all(id, State::Gas) {
            look.air.push(thing(world, id));
        } else {
            look.things.push(thing(world, id));
        }
    }
    Some(look)
}

/// What `actor` is carrying, and their credits.
pub fn inventory(world: &World, actor: EntityId) -> Inventory {
    let things: Vec<Thing> = world
        .contents(actor)
        .into_iter()
        .map(|e| thing(world, e))
        .collect();
    // The loader guarantees the whole world's mass fits in a Mass, so any
    // part of it does too.
    let carried = things.iter().map(|t| u128::from(t.mass.mg())).sum::<u128>();
    Inventory {
        things,
        carried: Mass::from_mg(u64::try_from(carried).expect("world mass fits in a Mass")),
        credits: world.wallet(actor).unwrap_or(Credits::ZERO),
    }
}

fn thing(world: &World, id: EntityId) -> Thing {
    let ambient = world
        .place_of(id)
        .map_or(world.settings().reference_temperature, |p| world.ambient(p));
    let temperature = world
        .temperature(id)
        .filter(|t| t.mk().abs_diff(ambient.mk()) >= NOTICEABLE_MK);
    let notes = match world.chamber(id) {
        Some(chamber) if chamber.lit => vec!["lit".to_string()],
        _ => Vec::new(),
    };
    let contents = if world.is_container(id) {
        world
            .contents(id)
            .into_iter()
            .map(|e| thing(world, e))
            .collect()
    } else {
        Vec::new()
    };
    Thing {
        label: world.label(id),
        mass: world.mass(id),
        temperature,
        notes,
        contents,
    }
}
