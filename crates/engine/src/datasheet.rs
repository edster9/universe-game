//! Datasheets: what the engine measures about something. See
//! docs/ideas/world-engine.md, "Datasheets: test once, then use as a part".
//!
//! Anything can be measured: a lump, a tool, a place. A shaped part is
//! measured by what its shape's role does (an edge, a conductor, a source of
//! charge). An assembly is measured once, when it's put together, from its
//! parts' datasheets alone, and that datasheet is kept. Anything bigger that
//! uses it reads the datasheet and never looks inside. That's how the ladder
//! climbs without the engine simulating every layer at once.

use std::collections::BTreeMap;
use std::fmt;

use crate::matter::{self, Composition, State};
use crate::units::{self, Credits, Energy, Mass, Temperature, show};
use crate::world::{EntityId, Role, Settings, World};

/// Something the engine can measure. Named for the quantity, never for a
/// kind of thing.
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Property {
    Mass,
    Volume,
    State,
    Temperature,
    MeltingPoint,
    Hardness,
    StoredEnergy,
    Tolerance,
    Length,
    EdgeWidth,
    EdgeHardness,
    Voltage,
    Resistance,
    TouchTolerance,
    GlowHeatLoss,
    GlowMeltingPoint,
    Wobble,
    Current,
    Power,
    GlowTemperature,
    Glows,
    BurnsOut,
    Runtime,
    BurnRate,
    HeatLoss,
    Lit,
    Casts,
    CastTolerance,
    MaxTemperature,
    Ambient,
    HeatTakenIn,
    Credits,
    Alive,
    DiedOf,
    BodyFluid,
    Working,
    PieceSize,
    SearchTime,
    FindChance,
    Needs,
    GrowsTo,
    Carrying,
    HoldsUpTo,
    Buoyancy,
    Pushes,
    Awake,
    Capacity,
    Height,
    PlacesSeen,
    Shelters,
    Kind,
    Knows,
    Bleeding,
    Stamina,
}

impl Property {
    pub fn name(self) -> &'static str {
        match self {
            Property::Mass => "mass",
            Property::Volume => "volume",
            Property::State => "state",
            Property::Temperature => "temperature",
            Property::MeltingPoint => "melts at",
            Property::Hardness => "hardness now",
            Property::StoredEnergy => "stored energy",
            Property::Tolerance => "tolerance",
            Property::Length => "length",
            Property::EdgeWidth => "edge width",
            Property::EdgeHardness => "edge hardness",
            Property::Voltage => "voltage",
            Property::Resistance => "resistance",
            Property::TouchTolerance => "surface roughness",
            Property::GlowHeatLoss => "sheds heat",
            Property::GlowMeltingPoint => "fails at",
            Property::Wobble => "play between parts",
            Property::Current => "current",
            Property::Power => "power",
            Property::GlowTemperature => "runs at",
            Property::Glows => "gives light",
            Property::BurnsOut => "burns out",
            Property::Runtime => "runs for",
            Property::BurnRate => "burns up to",
            Property::HeatLoss => "loses heat",
            Property::Lit => "lit",
            Property::Casts => "casts",
            Property::CastTolerance => "casting tolerance",
            Property::MaxTemperature => "withstands up to",
            Property::Ambient => "surroundings at",
            Property::HeatTakenIn => "heat taken in",
            Property::Credits => "credits",
            Property::Alive => "alive",
            Property::DiedOf => "died of",
            Property::BodyFluid => "body fluid",
            Property::Working => "working hard",
            Property::PieceSize => "comes in pieces of",
            Property::SearchTime => "one search takes",
            Property::FindChance => "chance a search finds a piece",
            Property::Needs => "gathering needs",
            Property::GrowsTo => "grows back up to",
            Property::Carrying => "carrying",
            Property::HoldsUpTo => "holds up to",
            Property::Buoyancy => "buoyancy",
            Property::Pushes => "pushes with",
            Property::Awake => "awake for",
            Property::Capacity => "can hold",
            Property::Height => "height",
            Property::PlacesSeen => "places seen",
            Property::Shelters => "keeps in",
            Property::Kind => "kind",
            Property::Knows => "knows",
            Property::Bleeding => "bleeding",
            Property::Stamina => "stamina",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Mass(Mass),
    /// µm³.
    Volume(u128),
    Text(String),
    Temperature(Temperature),
    /// Hundredths.
    Hardness(u64),
    Energy(Energy),
    /// µm.
    Length(u64),
    /// µΩ, or `None` if no current can flow.
    Resistance(Option<u64>),
    /// µV.
    Voltage(u64),
    /// µA.
    Current(u64),
    /// µW.
    Power(u64),
    /// µW per K.
    HeatLoss(u64),
    /// Seconds.
    Duration(u64),
    Flag(bool),
    Credits(Credits),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Mass(m) => write!(f, "{m}"),
            Value::Volume(v) => f.write_str(&units::show(*v, show::VOLUME, 3)),
            Value::Text(t) => f.write_str(t),
            Value::Temperature(t) => write!(f, "{t}"),
            Value::Hardness(h) => write!(
                f,
                "{}",
                units::show(u128::from(*h), &[("", 100)], 2).trim_end()
            ),
            Value::Energy(e) => write!(f, "{e}"),
            Value::Length(l) => f.write_str(&units::show(u128::from(*l), show::LENGTH, 3)),
            Value::Resistance(Some(r)) => {
                f.write_str(&units::show(u128::from(*r), show::RESISTANCE, 3))
            }
            Value::Resistance(None) => f.write_str("no current flows"),
            Value::Voltage(v) => f.write_str(&units::show(u128::from(*v), show::VOLTAGE, 3)),
            Value::Current(c) => f.write_str(&units::show(u128::from(*c), show::CURRENT, 3)),
            Value::Power(p) => f.write_str(&units::show(u128::from(*p), show::POWER, 3)),
            Value::HeatLoss(h) => {
                write!(f, "{} per K", units::show(u128::from(*h), show::POWER, 3))
            }
            Value::Duration(s) => f.write_str(&units::show_duration(*s)),
            Value::Flag(b) => f.write_str(if *b { "yes" } else { "no" }),
            Value::Credits(c) => write!(f, "{c}"),
        }
    }
}

/// A measured description of one thing.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct Datasheet {
    /// Material labels and their share by mass, in parts per ten thousand,
    /// most first.
    pub made_of: Vec<(String, u64)>,
    /// For an assembly: each slot and the label of the part in it.
    pub parts: Vec<(String, String)>,
    pub entries: BTreeMap<Property, Value>,
}

impl Datasheet {
    pub fn get(&self, property: Property) -> Option<&Value> {
        self.entries.get(&property)
    }

    fn set(&mut self, property: Property, value: Value) {
        self.entries.insert(property, value);
    }

    fn number(&self, property: Property) -> Option<u64> {
        match self.get(property)? {
            Value::Length(n)
            | Value::Voltage(n)
            | Value::Hardness(n)
            | Value::HeatLoss(n)
            | Value::Power(n)
            | Value::Current(n)
            | Value::Duration(n) => Some(*n),
            Value::Mass(m) => Some(m.mg()),
            Value::Energy(e) => Some(e.uj()),
            Value::Temperature(t) => Some(t.mk()),
            _ => None,
        }
    }
}

/// Measures anything in the world. An assembly returns the datasheet measured
/// when it was put together.
pub fn measure(world: &World, id: EntityId) -> Datasheet {
    let mut sheet = match world.assembly(id) {
        Some(assembly) => assembly.datasheet.clone(),
        None => measure_thing(world, id),
    };
    if let Some(share) = world
        .assembly(id)
        .and_then(|_| world.design_of(id).and_then(|d| d.shelter))
    {
        sheet.set(
            Property::Shelters,
            Value::Text(format!("{}% of a sleeper's heat", share / 100)),
        );
    }
    buoyancy(world, id, &mut sheet);
    // A kind, and the kinds it belongs to, nearest first.
    if let Some(kind) = world.kind_of(id) {
        let labels: Vec<&str> = world
            .lineage(kind)
            .into_iter()
            .filter_map(|k| world.kinds().get(k).map(|def| def.label.as_str()))
            .collect();
        if let Some((own, broader)) = labels.split_first() {
            let text = if broader.is_empty() {
                (*own).to_string()
            } else {
                format!("{own} ({})", broader.join(", "))
            };
            sheet.set(Property::Kind, Value::Text(text));
        }
    }
    sheet
}

/// Whether something floats in a liquid where it is, and how much more it
/// could carry: the mass of liquid its volume displaces, less its own mass.
fn buoyancy(world: &World, id: EntityId, sheet: &mut Datasheet) {
    if !world.is_portable(id) || world.is_agent(id) {
        return;
    }
    let Some(place) = world.place_of(id) else {
        return;
    };
    let liquid = world
        .contents(place)
        .into_iter()
        .find(|&e| !world.is_portable(e) && !world.is_agent(e) && world.is_all(e, State::Liquid));
    let Some(liquid) = liquid else {
        return;
    };
    let Some(spare) = spare_buoyancy(world, sheet, id, liquid) else {
        return;
    };
    let text = match spare {
        Some(spare) => format!(
            "floats in {}, carrying up to {spare} more",
            world.label(liquid)
        ),
        None => format!("sinks in {}", world.label(liquid)),
    };
    sheet.set(Property::Buoyancy, Value::Text(text));
}

/// How much more something could carry afloat in a liquid: the mass of
/// liquid its volume displaces, less its own mass. `Some(None)` if it sinks;
/// `None` if its volume or the liquid's is unknown.
fn spare_buoyancy(
    world: &World,
    sheet: &Datasheet,
    id: EntityId,
    liquid: EntityId,
) -> Option<Option<Mass>> {
    let Some(Value::Volume(volume)) = sheet.get(Property::Volume) else {
        return None;
    };
    let displaced = volume * density(world, liquid)? / 1_000_000_000_000_000;
    let own = u128::from(world.mass(id).mg());
    Some(
        displaced
            .checked_sub(own)
            .map(|spare| Mass::from_mg(u64::try_from(spare).unwrap_or(u64::MAX))),
    )
}

/// What something could carry afloat in a liquid, measured as above.
pub fn carries_afloat(world: &World, id: EntityId, liquid: EntityId) -> Option<Option<Mass>> {
    spare_buoyancy(world, &measure(world, id), id, liquid)
}

/// Something's density, in g per m³, if its volume is known.
pub fn density(world: &World, id: EntityId) -> Option<u128> {
    let volume = world
        .composition(id)
        .and_then(|c| matter::volume(world.materials(), c))
        .filter(|&v| v > 0)?;
    // mg × 10¹⁵ / µm³ gives g per m³.
    Some(u128::from(world.mass(id).mg()) * 1_000_000_000_000_000 / volume)
}

/// Something's volume in µm³, as measured.
pub fn volume(world: &World, id: EntityId) -> Option<u128> {
    match measure(world, id).get(Property::Volume) {
        Some(Value::Volume(v)) => Some(*v),
        _ => None,
    }
}

fn measure_thing(world: &World, id: EntityId) -> Datasheet {
    let mut sheet = Datasheet::default();
    if world.is_place(id) {
        sheet.set(Property::Ambient, Value::Temperature(world.ambient(id)));
        sheet.set(Property::Height, Value::Length(world.height(id)));
        sheet.set(Property::HeatTakenIn, Value::Energy(world.surroundings(id)));
        return sheet;
    }
    sheet.set(Property::Mass, Value::Mass(world.mass(id)));
    if let Some(credits) = world.wallet(id) {
        sheet.set(Property::Credits, Value::Credits(credits));
    }
    if let Some(composition) = world.composition(id) {
        measure_matter(world, id, composition, &mut sheet);
    }
    if let Some(life) = world.life(id) {
        let fluid = world
            .composition(id)
            .and_then(|c| c.get(&life.fluid))
            .copied()
            .unwrap_or(Mass::ZERO);
        sheet.set(Property::Alive, Value::Flag(life.died_of.is_none()));
        if let Some(limit) = life.carry_limit {
            sheet.set(
                Property::Carrying,
                Value::Text(format!("{} of {limit}", world.carried_mass(id))),
            );
        }
        sheet.set(
            Property::BodyFluid,
            Value::Text(format!("{fluid} (dies below {})", life.fluid_minimum)),
        );
        sheet.set(
            Property::Working,
            Value::Flag(life.working_until > world.tick()),
        );
        sheet.set(
            Property::PlacesSeen,
            Value::Text(world.places_seen(id).to_string()),
        );
        if let Some(vitality) = &life.vitality {
            sheet.set(
                Property::Stamina,
                Value::Text(format!(
                    "{} of {}, from {} of vitality",
                    Energy::from_uj(vitality.stamina),
                    Energy::from_uj(vitality.most),
                    crate::units::show(u128::from(vitality.power), crate::units::show::POWER, 3)
                )),
            );
        }
        let bleeding = world.bleeding(id);
        if bleeding > 0 {
            sheet.set(
                Property::Bleeding,
                Value::Text(format!("{} a second", Mass::from_mg(bleeding))),
            );
        }
        if let Some(memory) = world.memory(id) {
            sheet.set(
                Property::Knows,
                Value::Text(format!(
                    "{} for certain, {} possible; {} confirmed, {} corrected",
                    memory.places.len() + memory.ways.len(),
                    memory.possible.len(),
                    memory.confirmed,
                    memory.corrected
                )),
            );
        }
        if let Some(sleep) = &life.sleep {
            let text = if world.is_asleep(id) {
                format!(
                    "asleep, waking in {}",
                    units::show_duration(sleep.until - world.tick())
                )
            } else {
                let awake = world.awake_for(id).unwrap_or(0);
                let tired = if world.is_tired(id) { " (tired)" } else { "" };
                format!("{}{tired}", units::show_duration(awake))
            };
            sheet.set(Property::Awake, Value::Text(text));
        }
        if let Some(cause) = &life.died_of {
            sheet.set(Property::DiedOf, Value::Text(cause.clone()));
        }
    }
    if let Some(pieces) = world.pieces(id) {
        sheet.set(Property::PieceSize, Value::Mass(pieces.size));
        sheet.set(Property::SearchTime, Value::Duration(pieces.find_time));
        let chance = crate::laws::finding_chance(world, id);
        sheet.set(
            Property::FindChance,
            Value::Text(format!("{}%", chance / 100)),
        );
        if let Some(needs) = &pieces.needs {
            let label = world
                .shapes()
                .get(needs)
                .map(|s| s.label.clone())
                .or_else(|| world.designs().get(needs).map(|d| d.label.clone()))
                .unwrap_or_else(|| needs.clone());
            sheet.set(Property::Needs, Value::Text(format!("a {label}")));
        }
    }
    if let Some(growth) = world.growth(id) {
        sheet.set(Property::GrowsTo, Value::Mass(growth.limit));
    }
    if let Some(chamber) = world.chamber(id) {
        sheet.set(
            Property::BurnRate,
            Value::Text(format!("{} a second", chamber.burn_rate)),
        );
        sheet.set(Property::HeatLoss, Value::HeatLoss(chamber.heat_loss));
        sheet.set(Property::Lit, Value::Flag(chamber.lit));
    }
    if let Some(form) = world.form(id) {
        let label = world
            .shapes()
            .get(&form.shape)
            .map_or(form.shape.clone(), |s| s.label.clone());
        sheet.set(Property::Casts, Value::Text(label));
        sheet.set(Property::CastTolerance, Value::Length(form.tolerance));
        if let Some(main) = world.composition(id).and_then(matter::dominant) {
            sheet.set(
                Property::MaxTemperature,
                Value::Temperature(world.materials()[&main].melting_point),
            );
        }
    }
    sheet
}

fn measure_matter(world: &World, id: EntityId, composition: &Composition, sheet: &mut Datasheet) {
    let materials = world.materials();
    let reference = world.settings().reference_temperature;
    let temperature = world.temperature(id).unwrap_or(reference);
    let total = matter::total_mass(composition).max(1);

    let mut parts: Vec<_> = composition.iter().collect();
    parts.sort_by(|(a_id, a), (b_id, b)| b.cmp(a).then(a_id.cmp(b_id)));
    sheet.made_of = parts
        .iter()
        .map(|(m, mass)| {
            let share = u128::from(mass.mg()) * 10_000 / total;
            (
                materials[m].label.clone(),
                u64::try_from(share).expect("a share"),
            )
        })
        .collect();

    if let Some(volume) = matter::volume(materials, composition) {
        sheet.set(Property::Volume, Value::Volume(volume));
    }
    let states: Vec<State> = world.states(id).into_iter().map(|(_, s)| s).collect();
    let state = match states.as_slice() {
        s if s.iter().all(|&x| x == State::Solid) => "solid",
        s if s.iter().all(|&x| x == State::Liquid) => "liquid",
        s if s.iter().all(|&x| x == State::Gas) => "gas",
        _ => "mixed",
    };
    sheet.set(Property::State, Value::Text(state.into()));
    sheet.set(Property::Temperature, Value::Temperature(temperature));
    if let Some(melts) = composition.keys().map(|m| materials[m].melting_point).min() {
        sheet.set(Property::MeltingPoint, Value::Temperature(melts));
    }
    let hardness = composition
        .keys()
        .map(|m| materials[m].hardness_at(temperature, reference))
        .max()
        .unwrap_or(0);
    sheet.set(Property::Hardness, Value::Hardness(hardness));
    let chemical = matter::chemical_energy(materials, composition);
    if chemical > 0 {
        sheet.set(
            Property::StoredEnergy,
            Value::Energy(Energy::from_uj(u64::try_from(chemical).unwrap_or(u64::MAX))),
        );
    }

    let (Some(shape), Some(tolerance)) = (world.shape(id), world.tolerance(id)) else {
        return;
    };
    sheet.set(Property::Tolerance, Value::Length(tolerance));
    let Some(def) = world.shapes().get(shape) else {
        return;
    };
    if let Some(length) = def.length {
        sheet.set(Property::Length, Value::Length(length));
    }
    let main = matter::dominant(composition).map(|m| &materials[&m]);
    match (def.role, main) {
        (Some(Role::Cutting), Some(main)) => {
            // An edge can't be finer than the precision it was made to.
            sheet.set(Property::EdgeWidth, Value::Length(tolerance));
            sheet.set(Property::EdgeHardness, Value::Hardness(main.hardness));
        }
        (Some(Role::Conducting), _) => {
            sheet.set(
                Property::Resistance,
                Value::Resistance(resistance(world, composition, def.length)),
            );
        }
        (Some(Role::Glowing), Some(main)) => {
            sheet.set(
                Property::Resistance,
                Value::Resistance(resistance(world, composition, def.length)),
            );
            sheet.set(
                Property::GlowHeatLoss,
                Value::HeatLoss(def.heat_loss.unwrap_or(0)),
            );
            sheet.set(
                Property::GlowMeltingPoint,
                Value::Temperature(main.melting_point),
            );
        }
        (Some(Role::Source), Some(main)) => {
            sheet.set(Property::Voltage, Value::Voltage(main.voltage.unwrap_or(0)));
        }
        (Some(Role::Touching), _) => {
            sheet.set(Property::TouchTolerance, Value::Length(tolerance));
        }
        (Some(Role::Containing), _) => {
            if let Some(capacity) = def.capacity {
                sheet.set(Property::Capacity, Value::Mass(capacity));
            }
        }
        (Some(Role::Pushing), _) => {
            if let Some(push) = def.push {
                sheet.set(
                    Property::Pushes,
                    Value::Text(format!("{}% of the effort", push / 100)),
                );
            }
        }
        (Some(Role::Pulling), Some(main)) => {
            // Strength × cross-section, where the cross-section is the volume
            // spread along the length. Shown as the mass it would hold up.
            if let (Some(strength), Some(volume), Some(length)) = (
                main.tensile_strength,
                matter::volume(materials, composition),
                def.length,
            ) {
                let area = volume / u128::from(length.max(1));
                // Pa × µm² ÷ 9,806,650 gives the mg a load of that force weighs.
                let holds = u128::from(strength) * area / 9_806_650;
                sheet.set(
                    Property::HoldsUpTo,
                    Value::Mass(Mass::from_mg(u64::try_from(holds).unwrap_or(u64::MAX))),
                );
            }
        }
        _ => {}
    }
}

/// Resistance of a length of material: resistivity × length² / volume. `None`
/// if the main material doesn't conduct.
fn resistance(world: &World, composition: &Composition, length: Option<u64>) -> Option<u64> {
    let main = &world.materials()[&matter::dominant(composition)?];
    let resistivity = u128::from(main.resistivity?);
    let length = u128::from(length?);
    let volume = matter::volume(world.materials(), composition)?.max(1);
    // pΩ·m × µm² / µm³ gives µΩ.
    u64::try_from(resistivity * length * length / volume).ok()
}

/// Measures an assembly from its parts' datasheets alone. It never sees the
/// parts themselves, so a part that is itself an assembly is used as it was
/// measured, not simulated again.
///
/// Laws applied: masses add; play between parts is the sum of their
/// tolerances; an edge is its part's edge; and electrical parts form one loop
/// in series, with each pair of touching surfaces adding resistance for their
/// roughness.
pub fn measure_assembly(settings: &Settings, parts: &[(String, String, Datasheet)]) -> Datasheet {
    let mut sheet = Datasheet {
        parts: parts
            .iter()
            .map(|(slot, label, _)| (slot.clone(), label.clone()))
            .collect(),
        ..Datasheet::default()
    };
    let sheets: Vec<&Datasheet> = parts.iter().map(|(_, _, s)| s).collect();
    let sum =
        |property: Property| -> u64 { sheets.iter().filter_map(|s| s.number(property)).sum() };

    sheet.set(
        Property::Mass,
        Value::Mass(Mass::from_mg(sum(Property::Mass))),
    );
    sheet.set(
        Property::Wobble,
        Value::Length(sum(Property::Tolerance) + sum(Property::Wobble)),
    );
    // Volumes add, so an assembly displaces what its parts do.
    let volumes: Vec<u128> = sheets
        .iter()
        .filter_map(|s| match s.get(Property::Volume) {
            Some(Value::Volume(v)) => Some(*v),
            _ => None,
        })
        .collect();
    if !volumes.is_empty() {
        sheet.set(Property::Volume, Value::Volume(volumes.iter().sum()));
    }
    if let Some(edge) = sheets.iter().find(|s| s.get(Property::EdgeWidth).is_some()) {
        for property in [Property::EdgeWidth, Property::EdgeHardness] {
            if let Some(value) = edge.get(property) {
                sheet.set(property, value.clone());
            }
        }
    }

    // Electricity: everything that conducts, supplies, or touches is in one
    // series loop.
    let electrical = sheets.iter().any(|s| {
        s.get(Property::Resistance).is_some()
            || s.get(Property::Voltage).is_some()
            || s.get(Property::TouchTolerance).is_some()
    });
    if !electrical {
        return sheet;
    }
    let voltage = sum(Property::Voltage);
    let stored: u64 = sheets
        .iter()
        .filter(|s| s.get(Property::Voltage).is_some())
        .filter_map(|s| s.number(Property::StoredEnergy))
        .sum();
    let touching =
        u128::from(sum(Property::TouchTolerance)) * u128::from(settings.touch_resistance);
    let resistance =
        sheets
            .iter()
            .try_fold(touching, |total, s| match s.get(Property::Resistance) {
                Some(Value::Resistance(Some(r))) => Some(total + u128::from(*r)),
                Some(Value::Resistance(None)) => None,
                _ => Some(total),
            });
    let resistance = resistance.and_then(|r| u64::try_from(r).ok());
    sheet.set(Property::Resistance, Value::Resistance(resistance));
    if voltage == 0 {
        return sheet;
    }
    sheet.set(Property::Voltage, Value::Voltage(voltage));
    sheet.set(
        Property::StoredEnergy,
        Value::Energy(Energy::from_uj(stored)),
    );

    let glowing: Vec<&&Datasheet> = sheets
        .iter()
        .filter(|s| s.get(Property::GlowHeatLoss).is_some())
        .collect();
    let Some(total) = resistance.filter(|&r| r > 0) else {
        sheet.set(Property::Current, Value::Current(0));
        if !glowing.is_empty() {
            sheet.set(Property::Glows, Value::Flag(false));
        }
        return sheet;
    };
    let (v, r) = (u128::from(voltage), u128::from(total));
    // µV / µΩ = A, so × 10⁶ for µA. µV² / µΩ = µW.
    let current = v * 1_000_000 / r;
    let power = v * v / r;
    sheet.set(
        Property::Current,
        Value::Current(u64::try_from(current).unwrap_or(u64::MAX)),
    );
    sheet.set(
        Property::Power,
        Value::Power(u64::try_from(power).unwrap_or(u64::MAX)),
    );
    if let Some(runtime) = u128::from(stored).checked_div(power) {
        sheet.set(
            Property::Runtime,
            Value::Duration(u64::try_from(runtime).unwrap_or(u64::MAX)),
        );
    }

    if let Some(glow) = glowing.first() {
        let own = match glow.get(Property::Resistance) {
            Some(Value::Resistance(Some(own))) => u128::from(*own),
            _ => 0,
        };
        let shed = u128::from(glow.number(Property::GlowHeatLoss).unwrap_or(0)).max(1);
        let fails_at = glow.number(Property::GlowMeltingPoint).unwrap_or(u64::MAX);
        // The glowing part's share of the power heats it until it sheds as
        // much as it takes in.
        let its_power = v * v * own / (r * r);
        let rise = its_power * 1_000 / shed;
        let runs_at = u64::try_from(u128::from(settings.reference_temperature.mk()) + rise)
            .unwrap_or(u64::MAX);
        sheet.set(
            Property::GlowTemperature,
            Value::Temperature(Temperature::from_mk(runs_at)),
        );
        let burns_out = runs_at >= fails_at;
        sheet.set(
            Property::Glows,
            Value::Flag(!burns_out && runs_at >= settings.glow_temperature.mk()),
        );
        if burns_out {
            sheet.set(Property::BurnsOut, Value::Flag(true));
        }
    }
    sheet
}
