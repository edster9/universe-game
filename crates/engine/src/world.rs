//! The world's state: entities and their components. Engine code is written
//! against components ("is a place", "has mass", "burns fuel inside it"),
//! never against kinds of thing. See docs/ideas/world-engine.md.
//!
//! Only the gate (`World::apply`, in gate.rs) changes a world once it's built.

use std::collections::{BTreeMap, BTreeSet};

use crate::datasheet::Datasheet;
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
    /// Tolerance, in µm, of anything shaped with a tool that isn't itself a
    /// shaped part: a bare lump, say.
    pub rough_tolerance: u64,
    /// The finest tolerance hand work can reach, in µm.
    pub finest_tolerance: u64,
    /// How much one session of rubbing two parts together improves each, in
    /// parts per ten thousand of its tolerance.
    pub rubbing_improvement: u64,
    /// How long one session of rubbing takes, in seconds.
    pub rubbing_time: u64,
    /// Resistance where two surfaces touch, in µΩ per µm of their combined
    /// roughness.
    pub touch_resistance: u64,
    /// The temperature at which something hot gives off visible light.
    pub glow_temperature: Temperature,
    /// Share of the gas in a place's air that passes into its surroundings
    /// each second, in parts per ten thousand.
    pub air_clearing: u64,
    /// The longest step nature takes when nothing fast is happening, in seconds.
    pub calm_step: u64,
    /// Heat passed to open air per m² of surface per K, in mW. Something
    /// whose size is unknown (a gas with no density) loses heat at the flat
    /// `open_air_heat_loss` instead.
    pub convection: Option<u64>,
    /// How well surfaces radiate heat, in parts per ten thousand.
    pub emissivity: u64,
    /// Heat passed between touching things per m² of the smaller one's
    /// surface per K, in mW.
    pub touch_transfer: u64,
    /// Material worn off the softer of two things rubbed together, in mg per
    /// second.
    pub wear_rate: u64,
    /// Share of a worker's extra effort that rubbing turns into heat, in
    /// parts per ten thousand.
    pub friction_share: u64,
    /// Share of the heat a burning piece releases that goes into the things
    /// held with it, in parts per ten thousand.
    pub flame_share: u64,
    /// The hardest material bare hands can pull apart, in hundredths.
    pub hand_hardness: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            reference_temperature: Temperature::from_mk(293_000),
            open_air_heat_loss: 5_000_000,
            dig_amount: Mass::from_mg(5_000_000),
            max_touch_temperature: Temperature::from_mk(330_000),
            rough_tolerance: 5_000,
            finest_tolerance: 1,
            rubbing_improvement: 2_000,
            rubbing_time: 600,
            touch_resistance: 1_000,
            glow_temperature: Temperature::from_mk(1_000_000),
            air_clearing: 10,
            calm_step: 60,
            convection: Some(10_000),
            emissivity: 9_000,
            touch_transfer: 100_000,
            wear_rate: 10,
            friction_share: 5_000,
            flame_share: 5_000,
            hand_hardness: 100,
        }
    }
}

/// Something a person keeps doing over time, advanced by nature each step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Activity {
    /// Rubbing two things together: the softer wears into `dust`, and the
    /// effort becomes heat in the dust.
    Rubbing {
        first: EntityId,
        second: EntityId,
        dust: EntityId,
        until: u64,
    },
}

/// Where chance comes from. Normal play is seeded. Tests can fix luck so the
/// same steps always give the same result however the engine changes: every
/// roll comes up at one value, and a lower roll is luckier.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Luck {
    #[default]
    Seeded,
    Fixed(u64),
}

impl Luck {
    /// Everything that can go right does.
    pub const GOOD: Luck = Luck::Fixed(0);
    /// Every roll lands in the middle: better-than-even chances succeed.
    pub const AVERAGE: Luck = Luck::Fixed(5_000);
    /// Everything that can go wrong does.
    pub const BAD: Luck = Luck::Fixed(9_999);
}

/// A living body's needs and limits, from data. The body itself is matter:
/// it burns what it has digested to stay warm and alive, loses its vital
/// fluid, and sweats to cool down.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Life {
    /// Energy it burns at rest, in µW.
    pub resting_power: u64,
    /// Energy it burns while working, in µW.
    pub working_power: u64,
    /// Heat it loses to its surroundings, in µW per K of difference.
    pub heat_loss: u64,
    /// The temperature it sweats to stay under.
    pub set_point: Temperature,
    /// Most fluid it can sweat, in mg per day.
    pub sweat_rate: u64,
    /// Heat carried off by each mg of sweat, in µJ.
    pub sweat_heat: u64,
    /// The fluid it needs.
    pub fluid: MaterialId,
    /// Fluid lost through breath and skin at rest, in mg per day. Double
    /// while working.
    pub fluid_loss: u64,
    /// Below this much fluid, it dies.
    pub fluid_minimum: Mass,
    /// How much fluid it holds when it has had enough to drink.
    pub fluid_normal: Mass,
    /// Materials it can take in from food, and burn for energy.
    pub digests: BTreeSet<MaterialId>,
    /// The most it drinks at once.
    pub gulp: Mass,
    /// It dies outside this range of body temperature.
    pub coldest: Temperature,
    pub hottest: Temperature,
    /// The most it can carry. `None` means no limit.
    pub carry_limit: Option<Mass>,
    /// Walking speed with nothing to carry, in mm per second. A full load
    /// halves it.
    pub walking_speed: u64,
    /// The tick until which it's working hard.
    pub working_until: u64,
    /// Why it died, or `None` while it's alive.
    pub died_of: Option<String>,
}

/// A source made of loose pieces, which can be gathered one piece at a time.
/// A solid source has to be cut or dug instead.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pieces {
    pub size: Mass,
    /// How long one search takes, in seconds.
    pub find_time: u64,
    /// How much the source holds when full.
    pub full: Mass,
    /// The chance a search of a full source finds a piece, in parts per ten
    /// thousand. It falls as the source thins.
    pub chance: u64,
    /// A shape or design the gatherer must be carrying, if bare hands won't do.
    pub needs: Option<String>,
    /// If set, `find_time` is the time with a tool of this edge width, in µm;
    /// a blunter tool takes longer in proportion.
    pub edge: Option<u64>,
}

/// A living source that grows toward a limit, drawing matter from another
/// piece (a population growing from what surrounds it) and energy from
/// sunlight.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Growth {
    /// Growth when small, in parts per ten thousand of its mass per day. It
    /// slows as it nears its limit.
    pub rate: u64,
    pub limit: Mass,
    /// Where the matter for growth comes from.
    pub from: EntityId,
}

/// What a shaped part does, which decides what gets measured about it.
/// These are laws, so they're named for what they do, not what they're called.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Cuts. Measured: edge width and hardness.
    Cutting,
    /// Is held.
    Holding,
    /// Carries current. Measured: resistance.
    Conducting,
    /// Carries current and glows when hot enough. Measured: resistance, and
    /// how fast it sheds heat.
    Glowing,
    /// Supplies charge. Measured: voltage and stored energy.
    Source,
    /// Presses against another surface to carry current. Its roughness sets
    /// the resistance where they touch.
    Touching,
    /// Is a form: liquid setting inside it takes the shape it casts.
    Casting,
    /// Is pulled along its length. Measured: the load it holds before
    /// breaking.
    Pulling,
}

/// A shape from data, and what it takes to measure it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShapeDef {
    pub label: String,
    pub role: Option<Role>,
    /// In µm.
    pub length: Option<u64>,
    /// Heat it sheds per kelvin above its surroundings, in µW per K.
    pub heat_loss: Option<u64>,
    /// For a casting shape: the shape liquid takes when it sets inside.
    pub casts: Option<String>,
}

/// What fills one slot of a design.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Requirement {
    Shape(String),
    Design(String),
    /// Any solid piece made mostly of this material.
    Material(MaterialId),
}

/// A design from data: which parts go together.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Design {
    pub label: String,
    /// Slot name and what fills it.
    pub slots: Vec<(String, Requirement)>,
    /// Things can be put in what's built to this design.
    pub holds: bool,
    /// What's built to this design encloses heat and burns fuel inside it.
    pub chamber: Option<Chamber>,
}

/// A container that shapes liquid setting inside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Form {
    pub shape: String,
    /// Tolerance of what sets in it, in µm.
    pub tolerance: u64,
}

/// Parts put together to a design, and the datasheet measured when it was
/// assembled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assembly {
    pub design: String,
    /// The parts it's made of. Anything else inside is being held.
    pub parts: Vec<EntityId>,
    pub datasheet: Datasheet,
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
    /// Where chance comes from. The same seed replays the same luck.
    pub(crate) seed: u64,
    pub(crate) luck: Luck,
    pub(crate) settings: Settings,
    pub(crate) materials: Materials,
    pub(crate) shapes: BTreeMap<String, ShapeDef>,
    pub(crate) designs: BTreeMap<String, Design>,

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
    /// How closely a shaped part matches its shape, in µm. Smaller is finer.
    pub(crate) tolerance: BTreeMap<EntityId, u64>,
    /// What each entity is in or held by. Places are the only entities that
    /// aren't anywhere.
    pub(crate) locations: BTreeMap<EntityId, EntityId>,
    /// Being a place means having exits, even if there are none.
    pub(crate) exits: BTreeMap<EntityId, Vec<EntityId>>,
    /// How far it is between two places, in µm. Missing means no distance.
    pub(crate) distances: BTreeMap<(EntityId, EntityId), u64>,
    /// Each place's surrounding temperature.
    pub(crate) ambient: BTreeMap<EntityId, Temperature>,
    /// Heat each place's surroundings have taken in.
    pub(crate) surroundings: BTreeMap<EntityId, Energy>,
    /// Matter each place's surroundings have taken in: breath, sweat, and gas
    /// from the air that has cleared.
    pub(crate) reservoir: BTreeMap<EntityId, Composition>,
    pub(crate) life: BTreeMap<EntityId, Life>,
    pub(crate) activities: BTreeMap<EntityId, Activity>,
    pub(crate) pieces: BTreeMap<EntityId, Pieces>,
    pub(crate) growth: BTreeMap<EntityId, Growth>,
    /// Energy that has entered the world as sunlight. The gate conserves
    /// total energy minus this.
    pub(crate) sunlight: u128,
    pub(crate) agents: BTreeSet<EntityId>,
    pub(crate) portable: BTreeSet<EntityId>,
    pub(crate) containers: BTreeSet<EntityId>,
    pub(crate) chambers: BTreeMap<EntityId, Chamber>,
    pub(crate) forms: BTreeMap<EntityId, Form>,
    pub(crate) assemblies: BTreeMap<EntityId, Assembly>,
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

    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// The same world with different luck. Only for setting up a run.
    pub fn with_seed(mut self, seed: u64) -> World {
        self.seed = seed;
        self.luck = Luck::Seeded;
        self
    }

    /// The same world with luck fixed. Only for setting up a run.
    pub fn with_luck(mut self, luck: Luck) -> World {
        self.luck = luck;
        self
    }

    /// A number from 0 to 9,999, drawn from the world's seed, unless luck is
    /// fixed. Lower is luckier: something with a chance of `c` in ten thousand
    /// happens when the roll is below `c`. The same seed, clock, history, and
    /// `salt` always give the same number.
    pub fn roll(&self, salt: u64) -> u64 {
        if let Luck::Fixed(value) = self.luck {
            return value.min(9_999);
        }
        let mut x = self.seed
            ^ self.tick.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ (self.log.len() as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
            ^ salt.wrapping_mul(0x1656_67B1_9E37_79F9);
        // SplitMix64 finaliser.
        x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        (x ^ (x >> 31)) % 10_000
    }

    pub fn life(&self, id: EntityId) -> Option<&Life> {
        self.life.get(&id)
    }

    /// What a person is busy doing, if anything.
    pub fn activity(&self, id: EntityId) -> Option<&Activity> {
        self.activities.get(&id)
    }

    /// True if someone's activity is using this piece.
    pub fn in_use(&self, id: EntityId) -> bool {
        self.activities.values().any(|a| {
            let Activity::Rubbing {
                first,
                second,
                dust,
                ..
            } = *a;
            [first, second, dust].contains(&id)
        })
    }

    /// The ID the next new entity will get. Laws use it to refer to
    /// something made earlier in the same set of changes.
    pub fn next_id(&self) -> EntityId {
        EntityId(self.next_id)
    }

    /// True if something is burning: it holds a material that catches fire on
    /// its own, and is hotter than that material's ignition point.
    pub fn is_burning(&self, id: EntityId) -> bool {
        let (Some(composition), Some(temperature)) = (self.matter.get(&id), self.temperature(id))
        else {
            return false;
        };
        composition.keys().any(|m| {
            let material = &self.materials[m];
            material.burns()
                && material
                    .ignition_point
                    .is_some_and(|point| temperature >= point)
        })
    }

    /// True for a body that is alive.
    pub fn is_living(&self, id: EntityId) -> bool {
        self.life.get(&id).is_some_and(|l| l.died_of.is_none())
    }

    pub fn pieces(&self, id: EntityId) -> Option<&Pieces> {
        self.pieces.get(&id)
    }

    pub fn growth(&self, id: EntityId) -> Option<&Growth> {
        self.growth.get(&id)
    }

    /// Energy that has entered the world as sunlight so far, in µJ.
    pub fn sunlight(&self) -> u128 {
        self.sunlight
    }

    /// Matter a place's surroundings have taken in.
    pub fn reservoir(&self, place: EntityId) -> Option<&Composition> {
        self.reservoir.get(&place)
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

    pub fn shapes(&self) -> &BTreeMap<String, ShapeDef> {
        &self.shapes
    }

    pub fn designs(&self) -> &BTreeMap<String, Design> {
        &self.designs
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
        if let Some(assembly) = self.assemblies.get(&id) {
            return self
                .designs
                .get(&assembly.design)
                .map_or_else(|| assembly.design.clone(), |d| d.label.clone());
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
            // "Molten" only for what is liquid because it's hot.
            let ambient = self
                .place_of(id)
                .map_or(self.settings.reference_temperature, |p| self.ambient(p));
            let hot = composition
                .keys()
                .any(|m| self.materials[m].melting_point > ambient);
            if hot {
                format!("molten {names}")
            } else {
                names
            }
        } else if !self.shape_of.contains_key(&id) && self.mass(id) < Mass::from_mg(1_000) {
            // A few grams or less of unshaped solid is dust.
            format!("{names} dust")
        } else if let Some(shape) = self.shape_of.get(&id) {
            format!(
                "{names} {}",
                self.shapes
                    .get(shape)
                    .map_or(shape.as_str(), |s| s.label.as_str())
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
            None if self.assemblies.contains_key(&id) => {
                let total: u64 = self
                    .contents(id)
                    .iter()
                    .map(|&part| self.mass(part).mg())
                    .sum();
                Mass::from_mg(total)
            }
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

    /// A shaped part's tolerance in µm.
    pub fn tolerance(&self, id: EntityId) -> Option<u64> {
        self.tolerance.get(&id).copied()
    }

    pub fn assembly(&self, id: EntityId) -> Option<&Assembly> {
        self.assemblies.get(&id)
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

    pub fn form(&self, id: EntityId) -> Option<&Form> {
        self.forms.get(&id)
    }

    pub fn exits(&self, place: EntityId) -> &[EntityId] {
        self.exits.get(&place).map_or(&[], Vec::as_slice)
    }

    /// How far it is from one place to another, in µm.
    pub fn distance(&self, from: EntityId, to: EntityId) -> u64 {
        self.distances.get(&(from, to)).copied().unwrap_or(0)
    }

    /// The total mass someone is carrying.
    pub fn carried_mass(&self, holder: EntityId) -> Mass {
        let total: u64 = self
            .contents(holder)
            .iter()
            .map(|&c| self.mass(c).mg())
            .sum();
        Mass::from_mg(total)
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

    /// What `holder` is holding: its contents, not counting the parts it's
    /// made of.
    pub fn held(&self, holder: EntityId) -> Vec<EntityId> {
        let parts = self
            .assemblies
            .get(&holder)
            .map(|a| a.parts.as_slice())
            .unwrap_or_default();
        self.contents(holder)
            .into_iter()
            .filter(|e| !parts.contains(e))
            .collect()
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
        let taken_in: u128 = self.reservoir.values().map(matter::total_mass).sum();
        plain + matter + taken_in
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
            .chain(self.reservoir.values())
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
            let holds = self.is_place(location)
                || self.is_agent(location)
                || self.is_container(location)
                || self.assemblies.contains_key(&location)
                // A body still holds what it carried, alive or dead.
                || self.life.contains_key(&location);
            if !holds {
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
        if self
            .tolerance
            .keys()
            .any(|id| !self.shape_of.contains_key(id))
        {
            return Err("a tolerance belongs to something with no shape".into());
        }
        if self
            .assemblies
            .keys()
            .any(|id| self.matter.contains_key(id) || self.masses.contains_key(id))
        {
            return Err("an assembly has a mass of its own besides its parts".into());
        }
        for (&id, life) in &self.life {
            if !self.matter.contains_key(&id) {
                return Err(format!(
                    "{} is alive but isn't made of anything",
                    self.key(id)
                ));
            }
            if life.died_of.is_none() && !self.is_agent(id) {
                return Err(format!("{} is alive but isn't a person", self.key(id)));
            }
        }
        for (&agent, activity) in &self.activities {
            let Activity::Rubbing {
                first,
                second,
                dust,
                ..
            } = *activity;
            if !self.is_agent(agent)
                || [first, second, dust]
                    .iter()
                    .any(|id| !self.matter.contains_key(id))
            {
                return Err(format!(
                    "{} is busy with something that doesn't exist",
                    self.key(agent)
                ));
            }
        }
        if self.reservoir.keys().any(|&id| !self.is_place(id)) {
            return Err("a reservoir belongs to something that isn't a place".into());
        }
        if self.surroundings.keys().any(|&id| !self.is_place(id)) {
            return Err("surroundings belong to something that isn't a place".into());
        }
        Ok(())
    }
}
