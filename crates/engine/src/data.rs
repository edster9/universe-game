//! Builds a world from a TOML data file. Things live in data, laws in code:
//! nothing here knows what any particular item is.

use std::collections::BTreeSet;
use std::fmt;

use serde::Deserialize;

use crate::units::{Credits, Mass};
use crate::world::World;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WorldFile {
    #[serde(default, rename = "place")]
    places: Vec<PlaceDef>,
    #[serde(default, rename = "agent")]
    agents: Vec<AgentDef>,
    #[serde(default, rename = "item")]
    items: Vec<ItemDef>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PlaceDef {
    id: String,
    label: String,
    #[serde(default)]
    exits: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentDef {
    id: String,
    label: String,
    at: String,
    mass: String,
    #[serde(default)]
    credits: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ItemDef {
    id: String,
    label: String,
    at: String,
    mass: String,
    /// Too big or fixed in place to pick up.
    #[serde(default)]
    fixed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadError(String);

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for LoadError {}

fn fail<T>(message: impl Into<String>) -> Result<T, LoadError> {
    Err(LoadError(message.into()))
}

/// Loads a world from the text of a data file. Entities get IDs in file
/// order (places, then agents, then items), so the same file always builds
/// the same world.
pub fn load_world(text: &str) -> Result<World, LoadError> {
    let file: WorldFile = toml::from_str(text).map_err(|e| LoadError(e.to_string()))?;
    let mut world = World::default();

    let mut seen = BTreeSet::new();
    let all_ids = file
        .places
        .iter()
        .map(|p| &p.id)
        .chain(file.agents.iter().map(|a| &a.id))
        .chain(file.items.iter().map(|i| &i.id));
    for id in all_ids {
        if !seen.insert(id.as_str()) {
            return fail(format!("the id {id:?} is used twice"));
        }
    }

    // Mark every place first, so exits can point at places listed later.
    let places: Vec<_> = file
        .places
        .iter()
        .map(|p| world.spawn(&p.id, &p.label))
        .collect();
    for &place in &places {
        world.exits.insert(place, Vec::new());
    }
    for (&place, def) in places.iter().zip(&file.places) {
        let mut exits = Vec::new();
        for exit in &def.exits {
            match world.find_by_key(exit) {
                Some(to) if world.is_place(to) => exits.push(to),
                _ => {
                    return fail(format!(
                        "{} has an exit to {exit:?}, which isn't a place",
                        def.id
                    ));
                }
            }
        }
        world.exits.insert(place, exits);
    }

    for def in &file.agents {
        let agent = world.spawn(&def.id, &def.label);
        let at = match world.find_by_key(&def.at) {
            Some(at) if world.is_place(at) => at,
            _ => {
                return fail(format!(
                    "{} is at {:?}, which isn't a place",
                    def.id, def.at
                ));
            }
        };
        world.locations.insert(agent, at);
        world.masses.insert(agent, parse_mass(&def.id, &def.mass)?);
        world.agents.insert(agent);
        world.wallets.insert(agent, Credits::new(def.credits));
    }

    for def in &file.items {
        let item = world.spawn(&def.id, &def.label);
        let at = match world.find_by_key(&def.at) {
            Some(at) if world.is_place(at) || world.is_agent(at) => at,
            _ => {
                return fail(format!(
                    "{} is at {:?}, which isn't a place or a person",
                    def.id, def.at
                ));
            }
        };
        world.locations.insert(item, at);
        world.masses.insert(item, parse_mass(&def.id, &def.mass)?);
        if !def.fixed {
            world.portable.insert(item);
        }
    }

    // Totals must fit in a single value, so no sum of any part of the world
    // can overflow later.
    if world.total_mass() > u128::from(u64::MAX) {
        return fail("the world's total mass is too large");
    }
    if world.total_credits() > u128::from(u64::MAX) {
        return fail("the world's total credits are too large");
    }
    world.check_invariants().map_err(LoadError)?;
    Ok(world)
}

fn parse_mass(id: &str, text: &str) -> Result<Mass, LoadError> {
    text.parse().map_err(|e| LoadError(format!("{id}: {e}")))
}
