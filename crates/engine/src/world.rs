//! The world's state: entities and their components. Engine code is written
//! against components ("is a place", "has mass", "burns fuel inside it"),
//! never against kinds of thing. See docs/ideas/world-engine.md.
//!
//! Only the gate (`World::apply`, in gate.rs) changes a world once it's built.

use std::collections::{BTreeMap, BTreeSet};

use crate::gate::LogEntry;
use crate::matter::{self, Composition, MaterialId, Materials, State};
use crate::units::{Credits, Energy, Mass, Temperature};

/// An entity is just an ID. Everything about it lives in components.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(pub(crate) u32);

/// World-wide constants, from the data file's `[world]` section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settings {
    /// The temperature hardness is measured at, and the default ambient.
    pub reference_temperature: Temperature,
    /// Heat something loses in the open, in µJ per K of difference per tick.
    pub open_air_heat_loss: u64,
    /// How much one dig takes out.
    pub dig_amount: Mass,
    /// The hottest thing a bare hand can hold.
    pub max_touch_temperature: Temperature,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            reference_temperature: Temperature::from_mk(293_000),
            open_air_heat_loss: 5_000_000,
            dig_amount: Mass::from_mg(5_000_000),
            max_touch_temperature: Temperature::from_mk(330_000),
        }
    }
}

/// An insulated enclosure where fuel burns and heats whatever is inside.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chamber {
    /// Most fuel it can burn per tick.
    pub burn_rate: Mass,
    /// Heat it lets out, in µJ per K of difference per tick.
    pub heat_loss: u64,
    pub lit: bool,
}

/// Sorted collections throughout, so iteration order never depends on the
/// machine: the engine must be deterministic.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct World {
    pub(crate) next_id: u32,
    pub(crate) tick: u64,
    pub(crate) settings: Settings,
    pub(crate) materials: Materials,
    /// Shape keys and their labels, from data.
    pub(crate) shapes: BTreeMap<String, String>,

    /// The ID each entity was given in the data file, or "#n" if it was made
    /// during play.
    pub(crate) keys: BTreeMap<EntityId, String>,
    /// What people call it, if data named it. Otherwise it's described from
    /// what it's made of.
    pub(crate) labels: BTreeMap<EntityId, String>,
    /// Mass of things with no composition (people, and early-slice items).
    pub(crate) masses: BTreeMap<EntityId, Mass>,
    /// What things with a composition are made of.
    pub(crate) matter: BTreeMap<EntityId, Composition>,
    /// Heat energy held by each piece of matter.
    pub(crate) heat: BTreeMap<EntityId, Energy>,
    pub(crate) shape_of: BTreeMap<EntityId, String>,
    /// What each entity is in or held by. Places are the only entities that
    /// aren't anywhere.
    pub(crate) locations: BTreeMap<EntityId, EntityId>,
    /// Being a place means having exits, even if there are none.
    pub(crate) exits: BTreeMap<EntityId, Vec<EntityId>>,
    /// Each place's surrounding temperature.
    pub(crate) ambient: BTreeMap<EntityId, Temperature>,
    /// Heat each place's surroundings have taken in.
    pub(crate) surroundings: BTreeMap<EntityId, Energy>,
    pub(crate) agents: BTreeSet<EntityId>,
    pub(crate) portable: BTreeSet<EntityId>,
    pub(crate) containers: BTreeSet<EntityId>,
    pub(crate) chambers: BTreeMap<EntityId, Chamber>,
    /// Containers that give liquid setting inside them a shape.
    pub(crate) forms: BTreeMap<EntityId, String>,
    pub(crate) wallets: BTreeMap<EntityId, Credits>,
    pub(crate) log: Vec<LogEntry>,
}

impl World {
    pub(crate) fn spawn(&mut self, key: Option<&str>, label: Option<&str>) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        let key = key.map_or_else(|| format!("#{}", id.0), str::to_string);
        self.keys.insert(id, key);
        if let Some(label) = label {
            self.labels.insert(id, label.to_string());
        }
        id
    }

    /// Every entity, in a fixed order.
    pub fn entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.keys.keys().copied()
    }

    pub fn exists(&self, id: EntityId) -> bool {
        self.keys.contains_key(&id)
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn materials(&self) -> &Materials {
        &self.materials
    }

    pub fn material_by_key(&self, key: &str) -> Option<MaterialId> {
        self.materials
            .iter()
            .find(|(_, m)| m.key == key)
            .map(|(&id, _)| id)
    }

    /// Shape keys and labels.
    pub fn shapes(&self) -> &BTreeMap<String, String> {
        &self.shapes
    }

    pub fn key(&self, id: EntityId) -> &str {
        self.keys.get(&id).map_or("", String::as_str)
    }

    /// What people call it: its name from data, or a description of what it's
    /// made of, its state, and its shape.
    pub fn label(&self, id: EntityId) -> String {
        if let Some(label) = self.labels.get(&id) {
            return label.clone();
        }
        let Some(composition) = self.matter.get(&id) else {
            return String::new();
        };
        let names = self.describe_composition(composition);
        let temperature = self.temperature(id).unwrap_or_default();
        let states: Vec<State> = composition
            .keys()
            .map(|m| self.materials[m].state_at(temperature))
            .collect();
        if states.iter().all(|&s| s == State::Gas) {
            names
        } else if states.iter().all(|&s| s == State::Liquid) {
            format!("molten {names}")
        } else if let Some(shape) = self.shape_of.get(&id) {
            format!(
                "{names} {}",
                self.shapes
                    .get(shape)
                    .map_or(shape.as_str(), String::as_str)
            )
        } else {
            format!("lump of {names}")
        }
    }

    /// "a", "a and b", or "a, b and c": materials, most first.
    pub fn describe_composition(&self, composition: &Composition) -> String {
        let mut parts: Vec<(&MaterialId, &Mass)> = composition.iter().collect();
        parts.sort_by(|(a_id, a), (b_id, b)| b.cmp(a).then(a_id.cmp(b_id)));
        let names: Vec<&str> = parts
            .iter()
            .map(|(id, _)| self.materials[id].label.as_str())
            .collect();
        match names.as_slice() {
            [] => String::new(),
            [one] => (*one).to_string(),
            [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
        }
    }

    pub fn find_by_key(&self, key: &str) -> Option<EntityId> {
        self.keys.iter().find(|(_, k)| *k == key).map(|(&id, _)| id)
    }

    pub fn mass(&self, id: EntityId) -> Mass {
        match self.matter.get(&id) {
            Some(composition) => Mass::from_mg(
                u64::try_from(matter::total_mass(composition))
                    .expect("the loader keeps masses in range"),
            ),
            None => self.masses.get(&id).copied().unwrap_or(Mass::ZERO),
        }
    }

    pub fn composition(&self, id: EntityId) -> Option<&Composition> {
        self.matter.get(&id)
    }

    pub fn heat(&self, id: EntityId) -> Option<Energy> {
        self.heat.get(&id).copied()
    }

    /// Heat capacity in µJ per K. Zero for things with no composition.
    pub fn heat_capacity(&self, id: EntityId) -> u128 {
        self.matter
            .get(&id)
            .map_or(0, |c| matter::heat_capacity(&self.materials, c))
    }

    /// Temperature of a piece of matter. `None` for things with no composition.
    pub fn temperature(&self, id: EntityId) -> Option<Temperature> {
        let energy = self.heat.get(&id)?;
        Some(matter::temperature(*energy, self.heat_capacity(id)))
    }

    /// The state of each material in something, at its current temperature.
    pub fn states(&self, id: EntityId) -> Vec<(MaterialId, State)> {
        let Some(composition) = self.matter.get(&id) else {
            return Vec::new();
        };
        let temperature = self.temperature(id).unwrap_or_default();
        composition
            .keys()
            .map(|&m| (m, self.materials[&m].state_at(temperature)))
            .collect()
    }

    /// True if something is matter and every part of it is in `state`.
    pub fn is_all(&self, id: EntityId, state: State) -> bool {
        let states = self.states(id);
        !states.is_empty() && states.iter().all(|&(_, s)| s == state)
    }

    pub fn shape(&self, id: EntityId) -> Option<&str> {
        self.shape_of.get(&id).map(String::as_str)
    }

    pub fn location(&self, id: EntityId) -> Option<EntityId> {
        self.locations.get(&id).copied()
    }

    /// The place something is in, however deeply it's held.
    pub fn place_of(&self, id: EntityId) -> Option<EntityId> {
        let mut current = id;
        for _ in 0..=self.keys.len() {
            if self.is_place(current) {
                return Some(current);
            }
            current = self.location(current)?;
        }
        None
    }

    /// True if `inner` is `outer`, or is in or held by it at any depth.
    pub fn is_within(&self, inner: EntityId, outer: EntityId) -> bool {
        let mut current = inner;
        for _ in 0..=self.keys.len() {
            if current == outer {
                return true;
            }
            match self.location(current) {
                Some(next) => current = next,
                None => return false,
            }
        }
        false
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

    pub fn is_container(&self, id: EntityId) -> bool {
        self.containers.contains(&id)
    }

    pub fn chamber(&self, id: EntityId) -> Option<&Chamber> {
        self.chambers.get(&id)
    }

    /// The shape key a form gives liquid that sets inside it.
    pub fn form(&self, id: EntityId) -> Option<&str> {
        self.forms.get(&id).map(String::as_str)
    }

    pub fn exits(&self, place: EntityId) -> &[EntityId] {
        self.exits.get(&place).map_or(&[], Vec::as_slice)
    }

    pub fn ambient(&self, place: EntityId) -> Temperature {
        self.ambient
            .get(&place)
            .copied()
            .unwrap_or(self.settings.reference_temperature)
    }

    pub fn surroundings(&self, place: EntityId) -> Energy {
        self.surroundings
            .get(&place)
            .copied()
            .unwrap_or(Energy::ZERO)
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
        let plain: u128 = self.masses.values().map(|m| u128::from(m.mg())).sum();
        let matter: u128 = self.matter.values().map(matter::total_mass).sum();
        plain + matter
    }

    /// Total credits in every wallet. Conserved, like mass.
    pub fn total_credits(&self) -> u128 {
        self.wallets.values().map(|c| u128::from(c.amount())).sum()
    }

    /// Total energy: heat in matter, heat given to surroundings, and chemical
    /// energy in anything that burns. Conserved.
    pub fn total_energy(&self) -> u128 {
        let heat: u128 = self.heat.values().map(|e| u128::from(e.uj())).sum();
        let given: u128 = self.surroundings.values().map(|e| u128::from(e.uj())).sum();
        let chemical: u128 = self
            .matter
            .values()
            .map(|c| matter::chemical_energy(&self.materials, c))
            .sum();
        heat + given + chemical
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
            if !(self.is_place(location) || self.is_agent(location) || self.is_container(location))
            {
                return Err(format!(
                    "{} is inside {}, which can't hold things",
                    self.key(id),
                    self.key(location)
                ));
            }
        }
        for id in self.entities() {
            if !self.is_place(id) && self.location(id).is_none() {
                return Err(format!("{} is nowhere", self.key(id)));
            }
            if self.place_of(id).is_none() {
                return Err(format!("{} is inside itself", self.key(id)));
            }
        }
        for (&id, composition) in &self.matter {
            if !self.exists(id)
                || composition.is_empty()
                || composition.values().any(|m| *m == Mass::ZERO)
            {
                return Err(format!(
                    "{} has an empty or broken composition",
                    self.key(id)
                ));
            }
            if self.masses.contains_key(&id) || !self.heat.contains_key(&id) {
                return Err(format!(
                    "{} has matter but mixed-up mass or heat",
                    self.key(id)
                ));
            }
        }
        if self.heat.keys().any(|id| !self.matter.contains_key(id)) {
            return Err("heat is held by something that isn't matter".into());
        }
        if self.surroundings.keys().any(|&id| !self.is_place(id)) {
            return Err("surroundings belong to something that isn't a place".into());
        }
        Ok(())
    }
}
