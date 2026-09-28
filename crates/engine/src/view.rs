//! What a person perceives. Views are data, not text: the console formats
//! them now, and a browser will later.

use crate::units::{Credits, Mass};
use crate::world::{EntityId, World};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thing {
    pub label: String,
    pub mass: Mass,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Look {
    pub place: String,
    pub exits: Vec<String>,
    pub people: Vec<String>,
    pub things: Vec<Thing>,
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
    let (people, things): (Vec<EntityId>, Vec<EntityId>) = world
        .contents(here)
        .into_iter()
        .filter(|&e| e != actor)
        .partition(|&e| world.is_agent(e));
    Some(Look {
        place: world.label(here).to_string(),
        exits: world
            .exits(here)
            .iter()
            .map(|&e| world.label(e).to_string())
            .collect(),
        people: people.iter().map(|&e| world.label(e).to_string()).collect(),
        things: things.iter().map(|&e| thing(world, e)).collect(),
    })
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
    Thing {
        label: world.label(id).to_string(),
        mass: world.mass(id),
    }
}
