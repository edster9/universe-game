//! The world's state: entities and their components. Engine code is written
//! against components ("is a place", "has mass"), never against kinds of
//! thing. See docs/ideas/world-engine.md.
//!
//! Only the gate (`World::apply`, in gate.rs) changes a world once it's built.

use std::collections::{BTreeMap, BTreeSet};

use crate::gate::LogEntry;
use crate::units::{Credits, Mass};

/// An entity is just an ID. Everything about it lives in components.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(u32);

/// Sorted collections throughout, so iteration order never depends on the
/// machine: the engine must be deterministic.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct World {
    pub(crate) next_id: u32,
    /// The ID each entity was given in the data file.
    pub(crate) keys: BTreeMap<EntityId, String>,
    /// What people call it. The engine doesn't care.
    pub(crate) labels: BTreeMap<EntityId, String>,
    pub(crate) masses: BTreeMap<EntityId, Mass>,
    /// What each entity is in or held by. Places are the only entities that
    /// aren't anywhere.
    pub(crate) locations: BTreeMap<EntityId, EntityId>,
    /// Being a place means having exits, even if there are none.
    pub(crate) exits: BTreeMap<EntityId, Vec<EntityId>>,
    pub(crate) agents: BTreeSet<EntityId>,
    pub(crate) portable: BTreeSet<EntityId>,
    pub(crate) wallets: BTreeMap<EntityId, Credits>,
    pub(crate) log: Vec<LogEntry>,
}

impl World {
    pub(crate) fn spawn(&mut self, key: &str, label: &str) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        self.keys.insert(id, key.to_string());
        self.labels.insert(id, label.to_string());
        id
    }

    /// Every entity, in a fixed order.
    pub fn entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.keys.keys().copied()
    }

    pub fn exists(&self, id: EntityId) -> bool {
        self.keys.contains_key(&id)
    }

    pub fn key(&self, id: EntityId) -> &str {
        self.keys.get(&id).map_or("", String::as_str)
    }

    pub fn label(&self, id: EntityId) -> &str {
        self.labels.get(&id).map_or("", String::as_str)
    }

    pub fn find_by_key(&self, key: &str) -> Option<EntityId> {
        self.keys.iter().find(|(_, k)| *k == key).map(|(&id, _)| id)
    }

    pub fn mass(&self, id: EntityId) -> Mass {
        self.masses.get(&id).copied().unwrap_or(Mass::ZERO)
    }

    pub fn location(&self, id: EntityId) -> Option<EntityId> {
        self.locations.get(&id).copied()
    }

    pub fn is_place(&self, id: EntityId) -> bool {
        self.exits.contains_key(&id)
    }

    pub fn is_agent(&self, id: EntityId) -> bool {
        self.agents.contains(&id)
    }

    pub fn is_portable(&self, id: EntityId) -> bool {
        self.portable.contains(&id)
    }

    pub fn exits(&self, place: EntityId) -> &[EntityId] {
        self.exits.get(&place).map_or(&[], Vec::as_slice)
    }

    pub fn wallet(&self, id: EntityId) -> Option<Credits> {
        self.wallets.get(&id).copied()
    }

    /// Everything directly in or held by `holder`, in a fixed order.
    pub fn contents(&self, holder: EntityId) -> Vec<EntityId> {
        self.locations
            .iter()
            .filter(|&(_, &location)| location == holder)
            .map(|(&id, _)| id)
            .collect()
    }

    /// Every change that has passed the gate, oldest first.
    pub fn log(&self) -> &[LogEntry] {
        &self.log
    }

    /// Total mass of everything in the world. Only the gate could change it,
    /// and the gate refuses to.
    pub fn total_mass(&self) -> u128 {
        self.masses.values().map(|m| u128::from(m.mg())).sum()
    }

    /// Total credits in every wallet. Conserved, like mass.
    pub fn total_credits(&self) -> u128 {
        self.wallets.values().map(|c| u128::from(c.amount())).sum()
    }

    /// Structural rules that must always hold. The gate checks these after
    /// every change.
    pub fn check_invariants(&self) -> Result<(), String> {
        for (&id, &location) in &self.locations {
            if !self.exists(location) {
                return Err(format!(
                    "{} is inside something that doesn't exist",
                    self.key(id)
                ));
            }
            if self.is_place(id) {
                return Err(format!("the place {} is inside something", self.key(id)));
            }
            if self.is_agent(id) && !self.is_place(location) {
                return Err(format!("{} isn't standing in a place", self.key(id)));
            }
        }
        for id in self.entities() {
            if !self.is_place(id) && self.location(id).is_none() {
                return Err(format!("{} is nowhere", self.key(id)));
            }
            // Following locations upwards must reach a place, not loop forever.
            let mut current = id;
            let mut steps = 0;
            while let Some(next) = self.location(current) {
                current = next;
                steps += 1;
                if steps > self.keys.len() {
                    return Err(format!("{} is inside itself", self.key(id)));
                }
            }
        }
        Ok(())
    }
}
