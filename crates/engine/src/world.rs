//! The world's state: entities and their components. Engine code is written
//! against components ("is a place", "has mass", "burns fuel inside it"),
//! never against kinds of thing. See docs/ideas/world-engine.md.
//!
//! Only the gate (`World::apply`, in gate.rs) changes a world once it's built.

use std::collections::{BTreeMap, BTreeSet};

use crate::datasheet::Datasheet;
use crate::gate::LogEntry;
use crate::journal::{Journal, Set, Table};
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
    /// Share of a worker's effort that bare hands push into a liquid, in
    /// parts per ten thousand.
    pub hand_push: u64,
    /// How bluntly a vessel meets the liquid it's pushed through (its drag
    /// coefficient), in parts per ten thousand.
    pub drag: u64,
    /// The length of a day in seconds, or 0 for a world without days.
    pub day: u64,
    /// The time of day when the world's clock starts, in seconds.
    pub starts_at: u64,
    /// When the sun rises and sets, in seconds after midnight.
    pub sunrise: u64,
    pub sunset: u64,
    /// How long one search for a way out takes, in seconds.
    pub explore_time: u64,
    /// The chance one search finds a way out, in parts per ten thousand.
    pub explore_chance: u64,
    /// How much the air cools for each km of height, in mK.
    pub lapse_rate: u64,
    /// The radius of the world's planet, in µm, which sets how far the
    /// horizon is. 0 means nothing distant can be seen.
    pub planet_radius: u64,
    /// How high a person's eyes are above the ground, in µm.
    pub eye_height: u64,
    /// How long taking in the view takes, in seconds.
    pub survey_time: u64,
    /// How fast a wound from an edge 1 mm wide bleeds, in mg per second. A
    /// finer edge wounds worse, a blunter one less.
    pub wound_rate: u64,
    /// The chance a blow at someone awake lands, in parts per ten thousand.
    pub hit_chance: u64,
    /// The most a cut weighs when a body is butchered.
    pub cut: Mass,
    /// Ground at least this rough, in mg of sole worn away per km, hurts
    /// bare feet. Softer ground doesn't.
    pub bare_feet_limit: u64,
    /// How fast bare feet walk on ground that hurts them, in parts per ten
    /// thousand of their usual pace.
    pub bare_feet_pace: u64,
    /// How fast bare feet bleed, in mg a second, for each km walked on
    /// ground that hurts them.
    pub bare_feet_wound: u64,
    /// How much of a covering it takes to cover a whole body. Less keeps in
    /// less of its warmth, in proportion.
    pub covers: Mass,
    /// What players' bodies are granted: vitality's power in µW, the fluid it
    /// restores in mg per day, the most stamina in µJ, and the pace with none
    /// left, in parts per ten thousand.
    pub vitality: u64,
    pub vitality_restores: u64,
    pub stamina: u64,
    pub exhausted_pace: u64,
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
            hand_push: 500,
            drag: 10_000,
            day: 0,
            starts_at: 0,
            sunrise: 0,
            sunset: 0,
            explore_time: 1_800,
            explore_chance: 6_000,
            lapse_rate: 0,
            planet_radius: 0,
            eye_height: 1_700_000,
            survey_time: 600,
            wound_rate: 0,
            hit_chance: 5_000,
            cut: Mass::from_mg(5_000_000),
            bare_feet_limit: u64::MAX,
            bare_feet_pace: 10_000,
            bare_feet_wound: 0,
            covers: Mass::from_mg(2_000_000),
            vitality: 200_000_000,
            vitality_restores: 3_000_000,
            stamina: 3_000_000_000_000,
            exhausted_pace: 5_000,
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
    /// How high it can get, climbing or jumping, in µm: a barrier higher
    /// than this keeps it out. `None` means nothing does.
    pub reach: Option<u64>,
    /// Walking speed with nothing to carry, in mm per second. A full load
    /// halves it.
    pub walking_speed: u64,
    /// The share of its working power that lifts it when climbing, in parts
    /// per ten thousand.
    pub climbing_share: u64,
    /// Sleep, if it needs it: how long it can stay awake for each night's
    /// sleep, and how long that sleep takes.
    pub sleep: Option<Sleep>,
    /// If it lives on vitality, as players do, rather than on food, drink,
    /// and sleep. See docs/ideas/game-interface.md.
    pub vitality: Option<Vitality>,
    /// How long a wound takes to bleed half as fast as it did, as it clots,
    /// in seconds.
    pub clots: u64,
    /// Its wounds: how fast each bled when it was made, in mg per second, and
    /// when.
    pub wounds: Vec<(u64, u64)>,
    /// Its reserve: the most energy-rich store it digests, and how much of it
    /// the body had to begin with.
    pub reserve: Option<(MaterialId, Mass)>,
    /// How much of a surplus food's energy it keeps when it stores it in its
    /// reserve, in parts per ten thousand. `None` if it can't.
    pub stores: Option<u64>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Pushes against a liquid to move a vessel. Measured: the share of the
    /// worker's effort it delivers.
    Pushing,
    /// Holds things put in it, up to a mass. Measured: what it can hold.
    Containing,
}

/// Something a person can be told or read, and later see for themselves.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Claim {
    /// A place exists.
    Place(EntityId),
    /// There's a way from one place to another.
    Way(EntityId, EntityId),
    /// Something by this name is at a place.
    Thing(EntityId, String),
}

/// What a person remembers. What they've seen for themselves is certain;
/// what they've been told or read is only possible, until they see it. See
/// docs/ideas/memory.md.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    /// Whether they have to find ways out, rather than knowing them all.
    pub finds_ways: bool,
    /// Places they've been to or seen.
    pub places: BTreeSet<EntityId>,
    /// Ways they've found or walked.
    pub ways: BTreeSet<(EntityId, EntityId)>,
    /// What they last saw at each place (its fixed things and creatures),
    /// and when.
    pub sightings: BTreeMap<EntityId, (u64, BTreeSet<EntityId>)>,
    /// What they've been told or read and not yet seen, and where it came
    /// from.
    pub possible: BTreeMap<Claim, String>,
    /// How many possibles they've seen to be true.
    pub confirmed: u64,
    /// How many things they've had to correct: possibles that were wrong,
    /// and things they saw that have gone.
    pub corrected: u64,
}

/// A kind of creature or growing thing, from data. Kinds form a hierarchy:
/// every kind but the broadest has a parent. See docs/ideas/kinds.md.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kind {
    pub label: String,
    pub parent: Option<String>,
    /// If members act on instinct, the rules they follow. Otherwise they're
    /// persons, acting on commands.
    pub instinct: Option<Instinct>,
    /// The width of the edge members are born with, such as tusks or claws,
    /// in µm.
    pub weapon: Option<u64>,
    /// How members look to someone who doesn't know the kind.
    pub looks: Option<String>,
}

/// The rules an instinct follows, beyond looking after its own body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instinct {
    /// Kinds it runs from when one is at the same place.
    pub flees: Vec<String>,
    /// How long it rests when it has nothing to do, in seconds.
    pub rest: u64,
    /// The chance it wanders when it has nothing to do, in parts per ten
    /// thousand.
    pub wander: u64,
    /// How long it keeps away from a place where it met what it flees, in
    /// seconds.
    pub wary: u64,
    /// Whether, cornered or hurt, it attacks what it fears instead of
    /// fleeing.
    pub charges: bool,
}

/// Vitality: a steady flow of energy, and of the body's fluid, that a
/// universe grants to players' bodies, as the sun grants the world light. It
/// enters as a named inflow the gate accounts for. What a body doesn't use
/// tops up its stamina; hard work draws stamina down.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vitality {
    /// The energy it brings, in µW.
    pub power: u64,
    /// The most lost fluid it restores, in mg per day.
    pub restores: u64,
    /// Energy held in reserve for hard work, in µJ.
    pub stamina: u64,
    /// The most stamina holds, in µJ.
    pub most: u64,
    /// How fast it works with no stamina left, in parts per ten thousand of
    /// its usual pace.
    pub exhausted_pace: u64,
}

/// A body's need for sleep.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sleep {
    /// How long it can stay awake before it's tired, in seconds.
    pub awake: u64,
    /// How long a full sleep takes after that long awake, in seconds.
    pub need: u64,
    /// How fast it works when tired, in parts per ten thousand of its usual
    /// pace.
    pub tired_pace: u64,
    /// After this long awake it falls asleep wherever it is, in seconds.
    pub collapse: u64,
    /// Seconds of wakefulness it had at `since`.
    pub debt: u64,
    /// The tick `debt` was counted to. Wakefulness grows from here.
    pub since: u64,
    /// The tick it's asleep until.
    pub until: u64,
    /// What it's sleeping in, if anything.
    pub shelter: Option<EntityId>,
}

/// An action someone has started, to be carried out when its time is up.
/// Until then they're busy with it, and the world goes on around them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pending {
    pub intent: crate::intent::Intent,
    pub until: u64,
}

/// Something that happened to someone, kept for them to hear of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum News {
    /// `by` went for them, and wounded them (bleeding `wound` mg a second) or
    /// missed.
    Attacked { by: EntityId, wound: Option<u64> },
}

/// How an action someone started came out, kept for them to hear of.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Carried out: what it changed.
    Done(Vec<crate::gate::Change>),
    /// The world had changed by the time it was due, and it couldn't be.
    Failed(String),
    /// Something cut it short: a wound.
    Interrupted,
}

/// Where on a body something is worn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Covering {
    /// Underfoot, between the body and the ground, where walking wears it.
    Feet,
    /// About the body, where it keeps in warmth.
    Body,
}

/// Something worn: where, which part of it takes the wear, and how heavy
/// that part was when it was put on. Half of that worn away, it's worn
/// through.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worn {
    pub on: Covering,
    pub sole: EntityId,
    pub fresh: Mass,
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
    /// For a pushing shape: the share of effort it delivers, in parts per
    /// ten thousand.
    pub push: Option<u64>,
    /// For a containing shape: the most it holds.
    pub capacity: Option<Mass>,
    /// For a casting shape: the shape liquid takes when it sets inside.
    pub casts: Option<String>,
    /// How the shape looks to someone with no word for it.
    pub form: Option<String>,
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
    /// The share of a sleeper's body heat that what's built to this design
    /// keeps in, in parts per ten thousand.
    pub shelter: Option<u64>,
    /// How high what's built to this design holds what's in it, in µm: out
    /// of reach of anything that can't get that high.
    pub barrier: Option<u64>,
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
    /// The design it was built to, or none if it was put together without
    /// one, as something new.
    pub design: Option<String>,
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

/// How many recent sets of changes a world's log keeps unless told
/// otherwise: enough to look back over, not a whole history. A host that
/// wants the history keeps it itself.
pub const LOG_WINDOW: usize = 1_000;

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
    pub(crate) keys: Table<EntityId, String>,
    /// What people call it, if data named it. Otherwise it's described from
    /// what it's made of.
    pub(crate) labels: Table<EntityId, String>,
    /// Mass of things with no composition (people, and early-slice items).
    pub(crate) masses: Table<EntityId, Mass>,
    /// What things with a composition are made of.
    pub(crate) matter: Table<EntityId, Composition>,
    /// Heat energy held by each piece of matter.
    pub(crate) heat: Table<EntityId, Energy>,
    pub(crate) shape_of: Table<EntityId, String>,
    /// How closely a shaped part matches its shape, in µm. Smaller is finer.
    pub(crate) tolerance: Table<EntityId, u64>,
    /// What each entity is in or held by. Places are the only entities that
    /// aren't anywhere.
    pub(crate) locations: Table<EntityId, EntityId>,
    /// What's directly in or held by each thing: `locations` the other way
    /// round, kept in step with it by [`World::put`] and [`World::unput`].
    pub(crate) inside: Table<EntityId, BTreeSet<EntityId>>,
    /// Being a place means having exits, even if there are none.
    pub(crate) exits: Table<EntityId, Vec<EntityId>>,
    /// How far it is between two places, in µm. Missing means no distance.
    pub(crate) distances: Table<(EntityId, EntityId), u64>,
    /// How rough each place's ground is: the mg of a sole that a km of
    /// walking wears away. Missing means smooth.
    pub(crate) roughness: Table<EntityId, u64>,
    /// What people are wearing. What's worn stays among what they carry.
    pub(crate) worn: Table<EntityId, Worn>,
    /// Each place's height, in µm. Missing means zero.
    pub(crate) heights: Table<EntityId, u64>,
    /// Where each place is, in µm east and north of the world's origin.
    pub(crate) positions: Table<EntityId, (i64, i64)>,
    /// How landmarks look from far away. Only these can be seen from afar.
    pub(crate) from_afar: Table<EntityId, String>,
    /// What each person remembers. Creatures acting on instinct have none.
    pub(crate) memories: Table<EntityId, Memory>,
    /// What each map claims.
    pub(crate) maps: Table<EntityId, Vec<Claim>>,
    /// Kinds of creatures and growing things, by id.
    pub(crate) kinds: BTreeMap<String, Kind>,
    /// What kind each thing is, for things that have one.
    pub(crate) kind_of: Table<EntityId, String>,
    /// The places a creature keeps to.
    pub(crate) ranges: Table<EntityId, BTreeSet<EntityId>>,
    /// The tick until which a creature acting on instinct is busy.
    pub(crate) busy_until: Table<EntityId, u64>,
    /// Actions people have started and not yet carried out.
    pub(crate) pending: Table<EntityId, Pending>,
    /// How each person's last started action came out, until they hear of
    /// it.
    pub(crate) outcomes: Table<EntityId, Outcome>,
    /// The minds of people nobody plays.
    pub(crate) minds: Table<EntityId, crate::mind::Mind>,
    /// What has happened to each person that they haven't heard of yet.
    pub(crate) news: Table<EntityId, Vec<News>>,
    /// Places a creature keeps away from, and until when.
    pub(crate) avoiding: Table<EntityId, BTreeMap<EntityId, u64>>,
    /// Paths that cross a liquid, and the liquid they cross.
    pub(crate) crossings: Table<(EntityId, EntityId), EntityId>,
    /// Each place's surrounding temperature.
    pub(crate) ambient: Table<EntityId, Temperature>,
    /// Each place's surrounding temperature in the coldest hour of the night,
    /// where days are warmer than nights.
    pub(crate) night_ambient: Table<EntityId, Temperature>,
    /// Heat each place's surroundings have taken in.
    pub(crate) surroundings: Table<EntityId, Energy>,
    /// Matter each place's surroundings have taken in: breath, sweat, and gas
    /// from the air that has cleared.
    pub(crate) reservoir: Table<EntityId, Composition>,
    pub(crate) life: Table<EntityId, Life>,
    pub(crate) activities: Table<EntityId, Activity>,
    pub(crate) pieces: Table<EntityId, Pieces>,
    pub(crate) growth: Table<EntityId, Growth>,
    /// Energy that has entered the world as sunlight. The gate conserves
    /// total energy minus this.
    pub(crate) sunlight: u128,
    /// Energy, and matter in mg, that have entered players' bodies as
    /// vitality. The gate conserves the totals less these too.
    pub(crate) vital_energy: u128,
    pub(crate) vital_matter: u128,
    pub(crate) agents: Set<EntityId>,
    pub(crate) portable: Set<EntityId>,
    pub(crate) containers: Set<EntityId>,
    pub(crate) chambers: Table<EntityId, Chamber>,
    pub(crate) forms: Table<EntityId, Form>,
    pub(crate) assemblies: Table<EntityId, Assembly>,
    /// Each person's own words, if they have them. Someone without is from a
    /// world with no cultures, and calls everything by its name from data.
    pub(crate) lexicons: Table<EntityId, crate::words::Lexicon>,
    pub(crate) wallets: Table<EntityId, Credits>,
    /// The most recent sets of changes that passed the gate, oldest first.
    pub(crate) log: Vec<LogEntry>,
    /// How many sets of changes have ever passed the gate.
    pub(crate) logged: u64,
    /// How many recent sets the log keeps; older ones are forgotten.
    pub(crate) log_window: usize,
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
            ^ self.logged.wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
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

    /// The world's mass, less what has entered as vitality: what the gate
    /// conserves.
    pub fn own_mass(&self) -> u128 {
        self.total_mass() - self.vital_matter
    }

    /// The world's energy, less what has entered as sunlight and vitality:
    /// what the gate conserves.
    pub fn own_energy(&self) -> u128 {
        self.total_energy() - self.sunlight - self.vital_energy
    }

    /// The same world with this person living by players' rules: on
    /// vitality, with no need of food, drink, or sleep, and stamina full.
    /// Only for setting up a run; data can also say so.
    pub fn with_player_rules(mut self, person: &str) -> Result<World, String> {
        let id = self
            .find_by_key(person)
            .filter(|&id| self.is_agent(id))
            .ok_or_else(|| format!("there's no person with the id {person:?}"))?;
        self.make_player(id)?;
        Ok(self)
    }

    pub(crate) fn make_player(&mut self, id: EntityId) -> Result<(), String> {
        let settings = self.settings.clone();
        let life = self
            .life
            .get_mut(&id)
            .ok_or_else(|| format!("{} has no living body", self.keys[&id]))?;
        life.sleep = None;
        life.vitality = Some(Vitality {
            power: settings.vitality,
            restores: settings.vitality_restores,
            stamina: settings.stamina,
            most: settings.stamina,
            exhausted_pace: settings.exhausted_pace.max(1),
        });
        Ok(())
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
            return match assembly.design.as_ref().and_then(|d| self.designs.get(d)) {
                Some(design) => design.label.clone(),
                None => self.joined(assembly.parts.iter().map(|&p| self.label(p)).collect()),
            };
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

    /// How something put together without a design is described: its parts,
    /// biggest first, "a joined to b and c".
    pub(crate) fn joined(&self, labels: Vec<String>) -> String {
        match labels.split_first() {
            None => String::new(),
            Some((first, [])) => first.clone(),
            Some((first, rest)) => format!("{first} joined to {}", list_and(rest)),
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

    /// How rough a place's ground is, in mg of sole worn away per km.
    pub fn roughness(&self, place: EntityId) -> u64 {
        self.roughness.get(&place).copied().unwrap_or(0)
    }

    /// Whether, and where, something is worn.
    pub fn worn(&self, item: EntityId) -> Option<&Worn> {
        self.worn.get(&item)
    }

    /// What someone wears on their feet, if anything.
    pub fn underfoot(&self, person: EntityId) -> Option<EntityId> {
        self.contents(person)
            .into_iter()
            .find(|i| self.worn.get(i).is_some_and(|w| w.on == Covering::Feet))
    }

    /// The part of something that would take the wear if it were worn: the
    /// heaviest part of something put together, or the thing itself. Once
    /// worn, it's the part that was heaviest when it was put on.
    pub fn sole(&self, item: EntityId) -> EntityId {
        if let Some(worn) = self.worn.get(&item) {
            return worn.sole;
        }
        self.assemblies.get(&item).map_or(item, |a| {
            a.parts
                .iter()
                .copied()
                .max_by_key(|&p| (self.mass(p), std::cmp::Reverse(p)))
                .unwrap_or(item)
        })
    }

    /// Whether something worn has lost half of what takes the wear.
    pub fn worn_through(&self, item: EntityId) -> bool {
        self.worn
            .get(&item)
            .is_some_and(|w| self.mass(self.sole(item)).mg() * 2 < w.fresh.mg())
    }

    /// The share of a body's heat what it wears keeps in, in parts per ten
    /// thousand: each covering keeps in its main material's share, in
    /// proportion to how much of a body it covers, and what one lets out the
    /// next can keep in.
    pub fn clothed(&self, body: EntityId) -> u64 {
        let mut escapes: u128 = 10_000;
        for item in self.contents(body) {
            if self.worn.get(&item).is_none_or(|w| w.on != Covering::Body) {
                continue;
            }
            let main = self.composition(self.sole(item)).and_then(matter::dominant);
            let share = main.map_or(0, |m| self.materials[&m].insulates);
            let covers = u128::from(self.settings.covers.mg().max(1));
            let cover = u128::from(self.mass(item).mg()).min(covers);
            let kept = u128::from(share) * cover / covers;
            escapes = escapes * (10_000 - kept.min(10_000)) / 10_000;
        }
        u64::try_from(10_000 - escapes).unwrap_or(0)
    }

    /// The design something was built to, if it was built to one.
    pub fn design_of(&self, id: EntityId) -> Option<&Design> {
        self.assemblies
            .get(&id)
            .and_then(|a| a.design.as_ref())
            .and_then(|d| self.designs.get(d))
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

    /// What someone remembers, if they're a person.
    /// The mind of someone nobody plays, if they have one.
    pub fn mind(&self, who: EntityId) -> Option<&crate::mind::Mind> {
        self.minds.get(&who)
    }

    pub fn memory(&self, who: EntityId) -> Option<&Memory> {
        self.memories.get(&who)
    }

    /// What a map claims, if something is a map.
    pub fn map(&self, id: EntityId) -> Option<&[Claim]> {
        self.maps.get(&id).map(Vec::as_slice)
    }

    /// Whether someone knows for certain the way from one place to another.
    pub fn knows_way(&self, who: EntityId, from: EntityId, to: EntityId) -> bool {
        self.memories
            .get(&who)
            .is_none_or(|m| !m.finds_ways || m.ways.contains(&(from, to)))
    }

    /// Whether someone has to find their way, rather than knowing every way.
    pub fn finds_ways(&self, who: EntityId) -> bool {
        self.memories.get(&who).is_some_and(|m| m.finds_ways)
    }

    /// The ways out of a place that someone knows.
    pub fn known_exits(&self, who: EntityId, place: EntityId) -> Vec<EntityId> {
        self.exits(place)
            .iter()
            .copied()
            .filter(|&to| self.knows_way(who, place, to))
            .collect()
    }

    /// The liquid a path between two places crosses, if any.
    pub fn crossing(&self, from: EntityId, to: EntityId) -> Option<EntityId> {
        self.crossings.get(&(from, to)).copied()
    }

    /// The total mass someone is carrying.
    pub fn carried_mass(&self, holder: EntityId) -> Mass {
        let total: u64 = self
            .contents(holder)
            .iter()
            .map(|&c| self.weight(c).mg())
            .sum();
        Mass::from_mg(total)
    }

    /// What something weighs in the hand: its own mass, and whatever it holds.
    pub fn weight(&self, id: EntityId) -> Mass {
        let inside: u64 = if self.is_container(id) {
            self.held(id).iter().map(|&e| self.weight(e).mg()).sum()
        } else {
            0
        };
        Mass::from_mg(self.mass(id).mg() + inside)
    }

    /// Where a place is, in µm east and north, if it has a position.
    pub fn position(&self, place: EntityId) -> Option<(i64, i64)> {
        self.positions.get(&place).copied()
    }

    /// How a landmark looks from far away.
    pub fn from_afar(&self, place: EntityId) -> Option<&str> {
        self.from_afar.get(&place).map(String::as_str)
    }

    pub fn kinds(&self) -> &BTreeMap<String, Kind> {
        &self.kinds
    }

    /// What kind something is, if it has one.
    pub fn kind_of(&self, id: EntityId) -> Option<&str> {
        self.kind_of.get(&id).map(String::as_str)
    }

    /// A kind and the kinds above it, nearest first.
    pub fn lineage<'a>(&'a self, kind: &'a str) -> Vec<&'a str> {
        let mut line = Vec::new();
        let mut next = Some(kind);
        while let Some(k) = next.filter(|k| !line.contains(k)) {
            line.push(k);
            next = self.kinds.get(k).and_then(|def| def.parent.as_deref());
        }
        line
    }

    /// Whether something is of a kind, or of a kind beneath it.
    pub fn is_kind(&self, id: EntityId, kind: &str) -> bool {
        self.kind_of(id)
            .is_some_and(|own| self.lineage(own).contains(&kind))
    }

    /// The natural weapon something is born with, from its nearest kind that
    /// has one: an edge width in µm.
    pub fn natural_weapon(&self, id: EntityId) -> Option<u64> {
        self.lineage(self.kind_of(id)?)
            .into_iter()
            .find_map(|k| self.kinds.get(k)?.weapon)
    }

    /// How fast someone is bleeding now, in mg per second. Each wound bleeds
    /// half as fast for every clotting time since it was made.
    pub fn bleeding(&self, id: EntityId) -> u64 {
        let Some(life) = self.life.get(&id) else {
            return 0;
        };
        life.wounds
            .iter()
            .map(|&(rate, since)| {
                let halvings = self.tick.saturating_sub(since) / life.clots.max(1);
                rate.checked_shr(u32::try_from(halvings).unwrap_or(u32::MAX))
                    .unwrap_or(0)
            })
            .sum()
    }

    /// The instinct something acts on, from its nearest kind that has one.
    pub fn instinct(&self, id: EntityId) -> Option<&Instinct> {
        self.lineage(self.kind_of(id)?)
            .into_iter()
            .find_map(|k| self.kinds.get(k)?.instinct.as_ref())
    }

    /// The places a creature keeps to, if it keeps to some.
    pub fn range(&self, id: EntityId) -> Option<&BTreeSet<EntityId>> {
        self.ranges.get(&id)
    }

    /// Whether a creature is keeping away from a place, having been scared
    /// there.
    pub fn is_avoiding(&self, id: EntityId, place: EntityId) -> bool {
        self.avoiding
            .get(&id)
            .and_then(|places| places.get(&place))
            .is_some_and(|&until| until > self.tick)
    }

    /// Whether a creature acting on instinct is still busy.
    pub fn is_busy(&self, id: EntityId) -> bool {
        self.busy_until.get(&id).is_some_and(|&t| t > self.tick) || self.pending.contains_key(&id)
    }

    /// The action someone has started and not yet carried out, if any.
    pub fn pending(&self, id: EntityId) -> Option<&Pending> {
        self.pending.get(&id)
    }

    /// Everyone's actions in progress, and when each is due.
    pub fn all_pending(&self) -> impl Iterator<Item = (EntityId, &Pending)> {
        self.pending.iter().map(|(&id, p)| (id, p))
    }

    /// How someone's last started action came out, if they haven't heard
    /// yet. Hearing of it takes it away: it's news for a person, not part of
    /// the world, so it doesn't pass through the gate.
    /// What has happened to someone since they last heard, oldest first.
    pub fn take_news(&mut self, id: EntityId) -> Vec<News> {
        self.news.remove(&id).unwrap_or_default()
    }

    pub fn take_outcome(&mut self, id: EntityId) -> Option<Outcome> {
        self.outcomes.remove(&id)
    }

    pub fn outcome(&self, id: EntityId) -> Option<&Outcome> {
        self.outcomes.get(&id)
    }

    /// How many places someone has seen, from afar or by being there.
    pub fn places_seen(&self, who: EntityId) -> usize {
        self.memories.get(&who).map_or(0, |m| m.places.len())
    }

    /// Whether someone has seen a place, from afar or by being there.
    pub fn has_seen(&self, who: EntityId, place: EntityId) -> bool {
        self.memories
            .get(&who)
            .is_some_and(|m| m.places.contains(&place))
    }

    /// A place's height, in µm.
    pub fn height(&self, place: EntityId) -> u64 {
        self.heights.get(&place).copied().unwrap_or(0)
    }

    /// A place's surrounding temperature now. Where nights are colder, it
    /// falls steadily from the warmest at midday to the coldest at midnight.
    /// A place's temperatures are given as at zero height: the air cools with
    /// height.
    pub fn ambient(&self, place: EntityId) -> Temperature {
        let cooling = u64::try_from(
            u128::from(self.settings.lapse_rate) * u128::from(self.height(place)) / 1_000_000_000,
        )
        .unwrap_or(u64::MAX);
        let at_zero_height = self.ambient_at_zero_height(place);
        Temperature::from_mk(at_zero_height.mk().saturating_sub(cooling))
    }

    fn ambient_at_zero_height(&self, place: EntityId) -> Temperature {
        let warmest = self
            .ambient
            .get(&place)
            .copied()
            .unwrap_or(self.settings.reference_temperature);
        let (Some(time), Some(coldest)) = (self.time_of_day(), self.night_ambient.get(&place))
        else {
            return warmest;
        };
        let day = self.settings.day;
        let noon = (self.settings.sunrise + self.settings.sunset) / 2;
        let from_noon = time.abs_diff(noon).min(day - time.abs_diff(noon));
        let span = warmest.mk().saturating_sub(coldest.mk());
        let drop = u128::from(span) * u128::from(from_noon) / u128::from((day / 2).max(1));
        Temperature::from_mk(warmest.mk() - u64::try_from(drop).expect("within the span"))
    }

    /// Seconds since midnight, in a world with days.
    pub fn time_of_day(&self) -> Option<u64> {
        let day = self.settings.day;
        (day > 0).then(|| (self.tick + self.settings.starts_at) % day)
    }

    /// Whether the sun is down.
    pub fn is_night(&self) -> bool {
        self.time_of_day()
            .is_some_and(|t| t < self.settings.sunrise || t >= self.settings.sunset)
    }

    /// Whether it's too dark to see at a place: night, with nothing burning
    /// there to see by.
    pub fn is_dark(&self, place: EntityId) -> bool {
        self.is_night()
            && !self
                .entities()
                .any(|e| self.is_burning(e) && self.place_of(e) == Some(place))
    }

    /// How long someone has been awake since they were last fully rested, in
    /// seconds. While asleep, what's left once they wake.
    pub fn awake_for(&self, id: EntityId) -> Option<u64> {
        let sleep = self.life.get(&id)?.sleep.as_ref()?;
        Some(sleep.debt + self.tick.saturating_sub(sleep.since))
    }

    pub fn is_asleep(&self, id: EntityId) -> bool {
        self.life
            .get(&id)
            .and_then(|l| l.sleep.as_ref())
            .is_some_and(|s| s.until > self.tick)
    }

    /// Whether someone has been awake longer than they comfortably can.
    pub fn is_tired(&self, id: EntityId) -> bool {
        if let Some(vitality) = self.life.get(&id).and_then(|l| l.vitality.as_ref()) {
            return vitality.stamina == 0;
        }
        let Some(sleep) = self.life.get(&id).and_then(|l| l.sleep.as_ref()) else {
            return false;
        };
        self.awake_for(id).is_some_and(|a| a > sleep.awake)
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
        self.inside
            .get(&holder)
            .map(|held| held.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Puts `id` in or on `holder`, wherever it was before.
    pub(crate) fn put(&mut self, id: EntityId, holder: EntityId) {
        self.unput(id);
        self.locations.insert(id, holder);
        self.inside.or_insert_with(holder, BTreeSet::new).insert(id);
    }

    /// Takes `id` out of wherever it is, leaving it nowhere.
    pub(crate) fn unput(&mut self, id: EntityId) {
        if let Some(old) = self.locations.remove(&id) {
            let now_empty = self.inside.get_mut(&old).is_some_and(|held| {
                held.remove(&id);
                held.is_empty()
            });
            if now_empty {
                self.inside.remove(&old);
            }
        }
    }

    /// The most recent sets of changes that passed the gate, oldest first.
    /// Each has its number in the order they passed.
    pub fn log(&self) -> &[LogEntry] {
        &self.log
    }

    /// How many sets of changes have ever passed the gate.
    pub fn logged(&self) -> u64 {
        self.logged
    }

    /// The same world, keeping the last `sets` sets of changes in its log.
    /// Only for setting up a run.
    pub fn with_log_window(mut self, sets: usize) -> World {
        self.log_window = sets;
        self
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
        let stamina: u128 = self
            .life
            .values()
            .filter_map(|l| l.vitality.as_ref())
            .map(|v| u128::from(v.stamina))
            .sum();
        heat + given + chemical + stamina
    }

    /// Calls `f` on every table that records its changes. Every field of
    /// the world is named here, so a new one can't be left out by accident:
    /// a table must record, and anything else the gate changes must be
    /// restored by [`crate::gate`]'s totals.
    pub(crate) fn journals(&mut self, mut f: impl FnMut(&mut dyn Journal)) {
        let World {
            keys,
            labels,
            masses,
            matter,
            heat,
            shape_of,
            tolerance,
            locations,
            inside,
            exits,
            distances,
            roughness,
            worn,
            heights,
            positions,
            from_afar,
            memories,
            maps,
            kind_of,
            ranges,
            busy_until,
            pending,
            outcomes,
            minds,
            news,
            avoiding,
            crossings,
            ambient,
            night_ambient,
            surroundings,
            reservoir,
            life,
            activities,
            pieces,
            growth,
            agents,
            portable,
            containers,
            chambers,
            forms,
            assemblies,
            lexicons,
            wallets,
            next_id: _,
            tick: _,
            seed: _,
            luck: _,
            settings: _,
            materials: _,
            shapes: _,
            designs: _,
            kinds: _,
            sunlight: _,
            vital_energy: _,
            vital_matter: _,
            log: _,
            logged: _,
            log_window: _,
        } = self;
        f(keys);
        f(labels);
        f(masses);
        f(matter);
        f(heat);
        f(shape_of);
        f(tolerance);
        f(locations);
        f(inside);
        f(exits);
        f(distances);
        f(roughness);
        f(worn);
        f(heights);
        f(positions);
        f(from_afar);
        f(memories);
        f(maps);
        f(kind_of);
        f(ranges);
        f(busy_until);
        f(pending);
        f(outcomes);
        f(minds);
        f(news);
        f(avoiding);
        f(crossings);
        f(ambient);
        f(night_ambient);
        f(surroundings);
        f(reservoir);
        f(life);
        f(activities);
        f(pieces);
        f(growth);
        f(agents);
        f(portable);
        f(containers);
        f(chambers);
        f(forms);
        f(assemblies);
        f(lexicons);
        f(wallets);
    }

    /// Every entity with an entry that changed since the tables began
    /// recording.
    fn touched(&self) -> BTreeSet<EntityId> {
        let mut ids = BTreeSet::new();
        ids.extend(self.keys.originals().into_keys().copied());
        ids.extend(self.labels.originals().into_keys().copied());
        ids.extend(self.masses.originals().into_keys().copied());
        ids.extend(self.matter.originals().into_keys().copied());
        ids.extend(self.heat.originals().into_keys().copied());
        ids.extend(self.shape_of.originals().into_keys().copied());
        ids.extend(self.tolerance.originals().into_keys().copied());
        ids.extend(self.locations.originals().into_keys().copied());
        ids.extend(self.inside.originals().into_keys().copied());
        ids.extend(self.exits.originals().into_keys().copied());
        ids.extend(self.roughness.originals().into_keys().copied());
        ids.extend(self.worn.originals().into_keys().copied());
        ids.extend(self.heights.originals().into_keys().copied());
        ids.extend(self.positions.originals().into_keys().copied());
        ids.extend(self.from_afar.originals().into_keys().copied());
        ids.extend(self.memories.originals().into_keys().copied());
        ids.extend(self.maps.originals().into_keys().copied());
        ids.extend(self.kind_of.originals().into_keys().copied());
        ids.extend(self.ranges.originals().into_keys().copied());
        ids.extend(self.busy_until.originals().into_keys().copied());
        ids.extend(self.pending.originals().into_keys().copied());
        ids.extend(self.outcomes.originals().into_keys().copied());
        ids.extend(self.news.originals().into_keys().copied());
        ids.extend(self.minds.originals().into_keys().copied());
        ids.extend(self.avoiding.originals().into_keys().copied());
        ids.extend(self.ambient.originals().into_keys().copied());
        ids.extend(self.night_ambient.originals().into_keys().copied());
        ids.extend(self.surroundings.originals().into_keys().copied());
        ids.extend(self.reservoir.originals().into_keys().copied());
        ids.extend(self.life.originals().into_keys().copied());
        ids.extend(self.activities.originals().into_keys().copied());
        ids.extend(self.pieces.originals().into_keys().copied());
        ids.extend(self.growth.originals().into_keys().copied());
        ids.extend(self.agents.originals().into_keys().copied());
        ids.extend(self.portable.originals().into_keys().copied());
        ids.extend(self.containers.originals().into_keys().copied());
        ids.extend(self.chambers.originals().into_keys().copied());
        ids.extend(self.forms.originals().into_keys().copied());
        ids.extend(self.assemblies.originals().into_keys().copied());
        ids.extend(self.lexicons.originals().into_keys().copied());
        ids.extend(self.wallets.originals().into_keys().copied());
        ids.extend(
            self.distances
                .originals()
                .into_keys()
                .flat_map(|&(a, b)| [a, b]),
        );
        ids.extend(
            self.crossings
                .originals()
                .into_keys()
                .flat_map(|&(a, b)| [a, b]),
        );
        ids
    }

    /// The structural rules of [`World::check_invariants`], for just what
    /// changed since the tables began recording: everything touched, and
    /// whatever is in something that's gone or can no longer hold things.
    pub(crate) fn check_touched(&self) -> Result<(), String> {
        let touched = self.touched();
        // Things that were there before and are gone, or could hold things
        // before and can't now: whatever is in them must be checked too.
        let mut was: Vec<(EntityId, bool)> = Vec::new();
        was.extend(
            self.keys
                .originals()
                .into_iter()
                .map(|(&k, v)| (k, v.is_some())),
        );
        was.extend(self.agents.originals().into_iter().map(|(&k, v)| (k, v)));
        was.extend(
            self.containers
                .originals()
                .into_iter()
                .map(|(&k, v)| (k, v)),
        );
        was.extend(
            self.assemblies
                .originals()
                .into_iter()
                .map(|(&k, v)| (k, v.is_some())),
        );
        was.extend(
            self.life
                .originals()
                .into_iter()
                .map(|(&k, v)| (k, v.is_some())),
        );
        was.extend(
            self.exits
                .originals()
                .into_iter()
                .map(|(&k, v)| (k, v.is_some())),
        );
        let vacated: BTreeSet<EntityId> = was
            .into_iter()
            .filter(|&(id, there)| there && (!self.exists(id) || !self.holds(id)))
            .map(|(id, _)| id)
            .collect();
        for &id in &touched {
            self.check_entity(id)?;
        }
        if !vacated.is_empty() {
            for (&id, location) in &self.locations {
                if vacated.contains(location) {
                    self.check_entity(id)?;
                }
            }
            for (&agent, activity) in &self.activities {
                let Activity::Rubbing {
                    first,
                    second,
                    dust,
                    ..
                } = *activity;
                if [first, second, dust].iter().any(|id| vacated.contains(id)) {
                    self.check_entity(agent)?;
                }
            }
        }
        Ok(())
    }

    /// Whether something can have things in or on it.
    fn holds(&self, id: EntityId) -> bool {
        self.is_place(id)
            || self.is_agent(id)
            || self.is_container(id)
            || self.assemblies.contains_key(&id)
            // A body still holds what it carried, alive or dead.
            || self.life.contains_key(&id)
    }

    /// The structural rules for one entity, whether or not it still exists.
    fn check_entity(&self, id: EntityId) -> Result<(), String> {
        if let Some(&location) = self.locations.get(&id) {
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
            if !self.holds(location) {
                return Err(format!(
                    "{} is inside {}, which can't hold things",
                    self.key(id),
                    self.key(location)
                ));
            }
        }
        let listed = |holder: &EntityId| self.inside.get(holder).is_some_and(|h| h.contains(&id));
        if self
            .locations
            .get(&id)
            .is_some_and(|holder| !listed(holder))
        {
            return Err(format!("{} isn't listed in what holds it", self.key(id)));
        }
        if let Some(held) = self.inside.get(&id)
            && (held.is_empty() || held.iter().any(|x| self.locations.get(x) != Some(&id)))
        {
            return Err(format!("{} lists something that isn't in it", self.key(id)));
        }
        if self.exists(id) {
            if !self.is_place(id) && self.location(id).is_none() {
                return Err(format!("{} is nowhere", self.key(id)));
            }
            if self.place_of(id).is_none() {
                return Err(format!("{} is inside itself", self.key(id)));
            }
        }
        if let Some(composition) = self.matter.get(&id) {
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
        if self.heat.contains_key(&id) && !self.matter.contains_key(&id) {
            return Err("heat is held by something that isn't matter".into());
        }
        if self.tolerance.contains_key(&id) && !self.shape_of.contains_key(&id) {
            return Err("a tolerance belongs to something with no shape".into());
        }
        if self.assemblies.contains_key(&id)
            && (self.matter.contains_key(&id) || self.masses.contains_key(&id))
        {
            return Err("an assembly has a mass of its own besides its parts".into());
        }
        if let Some(life) = self.life.get(&id) {
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
        if let Some(&Activity::Rubbing {
            first,
            second,
            dust,
            ..
        }) = self.activities.get(&id)
            && (!self.is_agent(id)
                || [first, second, dust]
                    .iter()
                    .any(|thing| !self.matter.contains_key(thing)))
        {
            return Err(format!(
                "{} is busy with something that doesn't exist",
                self.key(id)
            ));
        }
        if self.reservoir.contains_key(&id) && !self.is_place(id) {
            return Err("a reservoir belongs to something that isn't a place".into());
        }
        if self.surroundings.contains_key(&id) && !self.is_place(id) {
            return Err("surroundings belong to something that isn't a place".into());
        }
        Ok(())
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
        let listed: usize = self.inside.values().map(BTreeSet::len).sum();
        if listed != self.locations.len()
            || self.inside.iter().any(|(holder, held)| {
                held.is_empty() || held.iter().any(|x| self.locations.get(x) != Some(holder))
            })
        {
            return Err("what things hold doesn't match where things are".into());
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

/// "a", "a and b", or "a, b and c".
pub(crate) fn list_and(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}
