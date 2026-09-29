//! Builds a world from a TOML data file. Things live in data, laws in code:
//! nothing here knows what any particular material or item is.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Deserialize;

use crate::matter::{self, Composition, Material, MaterialId};
use crate::units::{
    self, Credits, Energy, Mass, Temperature, parse_number, parse_percent, parse_quantity, property,
};
use crate::world::{self, Chamber, Design, Form, Requirement, Role, Settings, World};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WorldFile {
    /// Libraries: other data files whose materials, shapes, and designs this
    /// world shares. A library holds nothing else.
    #[serde(default)]
    uses: Vec<String>,
    #[serde(default)]
    world: SettingsDef,
    #[serde(default, rename = "material")]
    materials: Vec<MaterialDef>,
    #[serde(default, rename = "shape")]
    shapes: Vec<ShapeDef>,
    #[serde(default, rename = "design")]
    designs: Vec<DesignDef>,
    #[serde(default, rename = "place")]
    places: Vec<PlaceDef>,
    #[serde(default, rename = "agent")]
    agents: Vec<AgentDef>,
    #[serde(default, rename = "item")]
    items: Vec<ItemDef>,
    #[serde(default, rename = "kind")]
    kinds: Vec<KindDef>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct KindDef {
    id: String,
    label: String,
    /// The broader kind it belongs to.
    parent: Option<String>,
    /// A typical member's mass, which members inherit.
    mass: Option<String>,
    /// What members are made of, which they inherit.
    composition: Option<BTreeMap<String, String>>,
    /// The figures of members' lives, which they inherit.
    life: Option<LifeDef>,
    /// Members act on instinct, following these rules.
    instinct: Option<InstinctDef>,
    /// The weapon members are born with, by how it does harm: tusks, claws,
    /// teeth. See docs/ideas/harm.md.
    weapon: Option<WeaponDef>,
}

/// How a natural weapon does harm. Only an edge so far; blunt force comes
/// later.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WeaponDef {
    /// The width of its edge or point.
    edge: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InstinctDef {
    /// Kinds it runs from.
    #[serde(default)]
    flees: Vec<String>,
    /// How long it rests with nothing to do.
    rest: String,
    /// The chance it wanders with nothing to do.
    wander: String,
    /// How long it keeps away from where it met what it flees.
    wary: Option<String>,
    /// Cornered or hurt, it attacks what it fears.
    #[serde(default)]
    charges: bool,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct SettingsDef {
    reference_temperature: Option<String>,
    open_air_heat_loss: Option<String>,
    dig_amount: Option<String>,
    max_touch_temperature: Option<String>,
    rough_tolerance: Option<String>,
    finest_tolerance: Option<String>,
    rubbing_improvement: Option<String>,
    rubbing_time: Option<String>,
    touch_resistance: Option<String>,
    glow_temperature: Option<String>,
    /// Where chance comes from. Tests replace it to try different luck.
    seed: Option<u64>,
    /// Share of a place's airborne gas that clears each second.
    air_clearing: Option<String>,
    /// The longest step nature takes when all is calm.
    calm_step: Option<String>,
    /// Heat passed to open air per m² of surface per K. Without it, every
    /// object loses heat at the flat `open_air_heat_loss`.
    convection: Option<String>,
    emissivity: Option<String>,
    touch_transfer: Option<String>,
    wear_rate: Option<String>,
    friction_share: Option<String>,
    flame_share: Option<String>,
    hand_hardness: Option<String>,
    hand_push: Option<String>,
    drag: Option<String>,
    /// The length of a day. Without it, the world has no day and night.
    day: Option<String>,
    /// The time of day the world starts at, and when the sun rises and sets.
    starts_at: Option<String>,
    sunrise: Option<String>,
    sunset: Option<String>,
    /// How long one search for a way out takes, and its chance.
    explore_time: Option<String>,
    explore_chance: Option<String>,
    /// How fast the air cools with height.
    lapse_rate: Option<String>,
    /// The planet's radius, which sets how far the horizon is.
    planet_radius: Option<String>,
    /// How high a person's eyes are.
    eye_height: Option<String>,
    /// How long taking in the view takes.
    survey_time: Option<String>,
    /// How fast a wound from an edge 1 mm wide bleeds.
    wound_rate: Option<String>,
    /// The chance a blow at someone awake lands.
    hit_chance: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MaterialDef {
    id: String,
    label: String,
    melting_point: String,
    boiling_point: String,
    specific_heat: String,
    hardness: String,
    energy_density: Option<String>,
    /// Material id to percentage by mass.
    #[serde(default)]
    burns_to: BTreeMap<String, String>,
    density: Option<String>,
    speed_of_sound: Option<String>,
    resistivity: Option<String>,
    voltage: Option<String>,
    /// Above this temperature it catches fire in the open.
    ignition_point: Option<String>,
    /// How fast its burning surface burns away.
    burn_speed: Option<String>,
    /// What it turns into, and at what temperature.
    becomes: Option<BecomesDef>,
    /// Strength when pulled.
    tensile_strength: Option<String>,
    /// Materials it softens in, as some earths do when wet.
    #[serde(default)]
    softens_in: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BecomesDef {
    material: String,
    at: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ShapeDef {
    id: String,
    label: String,
    /// What the shape does: cutting, holding, conducting, glowing, source,
    /// or touching.
    role: Option<String>,
    length: Option<String>,
    heat_loss: Option<String>,
    /// For the pushing role: the share of effort it delivers.
    push: Option<String>,
    /// For the containing role: the most it holds.
    holds: Option<String>,
    /// For the casting role: the shape it gives liquid that sets inside.
    casts: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DesignDef {
    id: String,
    label: String,
    /// Slot name to the shape, design, or material that fills it.
    parts: BTreeMap<String, String>,
    /// Things can be put in what's built to it.
    #[serde(default)]
    holds: bool,
    /// What's built to it encloses heat and burns fuel inside.
    chamber: Option<ChamberDef>,
    /// The share of a sleeper's body heat it keeps in.
    shelter: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PlaceDef {
    id: String,
    label: String,
    #[serde(default)]
    exits: Vec<String>,
    /// How far it is to each exit. A distance given from either end counts
    /// both ways.
    #[serde(default)]
    distances: BTreeMap<String, String>,
    /// Exits whose path crosses a liquid, and the item that liquid is. Only
    /// something that floats can take you.
    #[serde(default)]
    crossings: BTreeMap<String, String>,
    /// Height. Its temperatures are given as at zero height.
    height: Option<String>,
    /// Where it is: how far east and north of the world's origin. Either
    /// may be negative.
    east: Option<String>,
    north: Option<String>,
    /// How it looks from far away, which makes it a landmark that can be
    /// seen from afar.
    from_afar: Option<String>,
    /// The warmest it gets, at midday.
    temperature: Option<String>,
    /// The coldest it gets, at midnight, in a world with days.
    night: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentDef {
    id: String,
    label: String,
    at: String,
    /// Optional if its kind gives one.
    mass: Option<String>,
    /// What kind of creature it is. Its body and mind come from its kind
    /// unless given here.
    kind: Option<String>,
    /// How many alike to make, with ids numbered from 1.
    count: Option<u32>,
    /// The places it keeps to.
    #[serde(default)]
    range: Vec<String>,
    #[serde(default)]
    credits: u64,
    /// A body made of materials, as material id to percentage by mass.
    composition: Option<BTreeMap<String, String>>,
    /// Starting temperature, if not the surroundings'.
    temperature: Option<String>,
    /// What the body needs to live. Requires a composition.
    life: Option<LifeDef>,
    /// Starts knowing no way out of anywhere, and must find them.
    #[serde(default)]
    lost: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LifeDef {
    resting_power: String,
    working_power: String,
    heat_loss: String,
    set_point: String,
    sweat_rate: String,
    sweat_heat: String,
    fluid: String,
    fluid_loss: String,
    fluid_minimum: String,
    digests: Vec<String>,
    gulp: String,
    coldest: String,
    hottest: String,
    /// The most it can carry.
    carry: Option<String>,
    /// Walking speed unloaded.
    walk: Option<String>,
    /// The share of working power that lifts the body when climbing.
    climb: Option<String>,
    /// How much of a surplus food's energy it keeps when storing it.
    stores: Option<String>,
    /// How long a wound takes to bleed half as fast, as it clots.
    clots: Option<String>,
    /// How long it can stay awake before it's tired.
    awake: Option<String>,
    /// How long a full sleep takes.
    sleep: Option<String>,
    /// How fast it works when tired, compared with rested.
    tired_pace: Option<String>,
    /// After how long awake it falls asleep wherever it is.
    collapse: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PiecesDef {
    size: String,
    find_time: String,
    /// The chance a search of a full source finds a piece. Certain if not given.
    chance: Option<String>,
    /// A shape or design the gatherer must carry.
    needs: Option<String>,
    /// The tool edge that `find_time` assumes; blunter tools take longer.
    edge: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GrowsDef {
    /// Growth per day while small, as a percentage of its mass.
    rate: String,
    limit: String,
    /// The item whose matter it grows from.
    from: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ItemDef {
    id: String,
    /// Optional for things made of materials: they can be described instead.
    label: Option<String>,
    at: String,
    mass: String,
    /// Too big or fixed in place to carry.
    #[serde(default)]
    fixed: bool,
    /// Made of one material.
    material: Option<String>,
    /// Made of several, as material id to percentage by mass.
    composition: Option<BTreeMap<String, String>>,
    /// Starting temperature, if not the surroundings'.
    temperature: Option<String>,
    /// Things can be put in it.
    #[serde(default)]
    container: bool,
    /// Burns fuel inside it and holds the heat.
    chamber: Option<ChamberDef>,
    /// Liquid setting inside it takes a shape.
    form: Option<FormDef>,
    /// Its own shape, if it starts as a shaped part.
    shape: Option<String>,
    /// How closely it matches that shape.
    tolerance: Option<String>,
    /// It's made of loose pieces that can be gathered.
    pieces: Option<PiecesDef>,
    /// It's alive and grows.
    grows: Option<GrowsDef>,
    /// What kind of creature it is, for a population of creatures.
    kind: Option<String>,
    /// It's a map, and these are what it claims. A map can be wrong.
    #[serde(default)]
    map: Vec<ClaimDef>,
}

/// One thing a map claims: that a place exists, that there's a way between
/// two places, or that something by a name is at a place.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaimDef {
    place: Option<String>,
    way: Option<[String; 2]>,
    at: Option<String>,
    thing: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FormDef {
    shape: String,
    /// Tolerance of what sets in it.
    tolerance: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChamberDef {
    /// Fuel burned per second, as a mass.
    burn_rate: String,
    heat_loss: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadError(String);

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for LoadError {}

impl From<units::UnitError> for LoadError {
    fn from(e: units::UnitError) -> Self {
        LoadError(e.to_string())
    }
}

fn fail<T>(message: impl Into<String>) -> Result<T, LoadError> {
    Err(LoadError(message.into()))
}

/// Loads a world from the text of a data file. Entities get IDs in file
/// order (places, then agents, then items), so the same file always builds
/// the same world.
pub fn load_world(text: &str) -> Result<World, LoadError> {
    load_world_with(text, &[])
}

/// The libraries a world file uses, by name, in order. Whoever reads files
/// reads them and passes their text to `load_world_with`.
pub fn libraries(text: &str) -> Result<Vec<String>, LoadError> {
    let file: WorldFile = toml::from_str(text).map_err(|e| LoadError(e.to_string()))?;
    Ok(file.uses)
}

/// Builds a world from its data file and the text of each library it uses,
/// in the order it names them.
pub fn load_world_with(text: &str, libraries: &[&str]) -> Result<World, LoadError> {
    let mut file: WorldFile = toml::from_str(text).map_err(|e| LoadError(e.to_string()))?;
    if file.uses.len() != libraries.len() {
        return fail(format!(
            "this world uses {} libraries ({}), but {} were given",
            file.uses.len(),
            file.uses.join(", "),
            libraries.len()
        ));
    }
    for (name, library) in file.uses.iter().zip(libraries).rev() {
        let library: WorldFile =
            toml::from_str(library).map_err(|e| LoadError(format!("{name}: {e}")))?;
        if !library.uses.is_empty()
            || !library.places.is_empty()
            || !library.agents.is_empty()
            || !library.items.is_empty()
        {
            return fail(format!(
                "{name} is a library, so it holds only materials, shapes, and designs"
            ));
        }
        file.materials.splice(0..0, library.materials);
        file.shapes.splice(0..0, library.shapes);
        file.designs.splice(0..0, library.designs);
        file.kinds.splice(0..0, library.kinds);
    }
    let mut world = World {
        settings: load_settings(&file.world)?,
        seed: file.world.seed.unwrap_or(1),
        ..World::default()
    };
    load_materials(&mut world, &file.materials)?;
    for shape in &file.shapes {
        let def = load_shape(shape)?;
        if world.shapes.insert(shape.id.clone(), def).is_some() {
            return fail(format!("the shape {:?} is defined twice", shape.id));
        }
    }
    for (id, shape) in &world.shapes {
        if let Some(casts) = &shape.casts
            && !world.shapes.contains_key(casts)
        {
            return fail(format!(
                "the shape {id} casts {casts:?}, which isn't a shape"
            ));
        }
    }
    load_designs(&mut world, &file.designs)?;

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
        .map(|p| world.spawn(Some(&p.id), Some(&p.label)))
        .collect();
    for (&place, def) in places.iter().zip(&file.places) {
        world.exits.insert(place, Vec::new());
        if let Some(t) = &def.temperature {
            world.ambient.insert(place, t.parse()?);
        }
        if let Some(t) = &def.night {
            world.night_ambient.insert(place, t.parse()?);
        }
        if let Some(h) = &def.height {
            world.heights.insert(place, parse_length(&def.id, h)?);
        }
        match (&def.east, &def.north) {
            (Some(east), Some(north)) => {
                let offset = |text: &str| -> Result<i64, LoadError> {
                    let (sign, rest) = match text.trim().strip_prefix('-') {
                        Some(rest) => (-1, rest),
                        None => (1, text),
                    };
                    let length = i64::try_from(parse_length(&def.id, rest)?)
                        .map_err(|_| LoadError(format!("{}: too far", def.id)))?;
                    Ok(sign * length)
                };
                world
                    .positions
                    .insert(place, (offset(east)?, offset(north)?));
            }
            (None, None) => {}
            _ => return fail(format!("{} needs both east and north, or neither", def.id)),
        }
        if let Some(looks) = &def.from_afar {
            if !world.positions.contains_key(&place) {
                return fail(format!(
                    "{} is seen from afar, so it needs a position",
                    def.id
                ));
            }
            world.from_afar.insert(place, looks.clone());
        }
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
        for (to, distance) in &def.distances {
            let other = world
                .find_by_key(to)
                .filter(|o| exits.contains(o))
                .ok_or_else(|| {
                    LoadError(format!(
                        "{} has a distance to {to:?}, which isn't one of its exits",
                        def.id
                    ))
                })?;
            let length = parse_length(&def.id, distance)?;
            world.distances.insert((place, other), length);
            world.distances.entry((other, place)).or_insert(length);
        }
        world.exits.insert(place, exits);
    }

    load_kinds(&mut world, &file.kinds)?;
    for def in &file.agents {
        let count = def.count.unwrap_or(1);
        if count == 0 {
            return fail(format!("{} has a count of 0", def.id));
        }
        for n in 1..=count {
            let id = if def.count.is_some() {
                format!("{}-{n}", def.id)
            } else {
                def.id.clone()
            };
            load_agent(&mut world, def, &id, &file.kinds)?;
        }
    }

    for def in &file.items {
        load_item(&mut world, def)?;
        if !def.map.is_empty() {
            let item = world.find_by_key(&def.id).expect("just loaded");
            let place = |key: &str| {
                world
                    .find_by_key(key)
                    .filter(|&p| world.is_place(p))
                    .ok_or_else(|| {
                        LoadError(format!(
                            "{}'s map shows {key:?}, which isn't a place",
                            def.id
                        ))
                    })
            };
            let mut claims = Vec::new();
            for claim in &def.map {
                claims.push(match (&claim.place, &claim.way, &claim.at, &claim.thing) {
                    (Some(p), None, None, None) => world::Claim::Place(place(p)?),
                    (None, Some([a, b]), None, None) => world::Claim::Way(place(a)?, place(b)?),
                    (None, None, Some(at), Some(thing)) => {
                        world::Claim::Thing(place(at)?, thing.clone())
                    }
                    _ => {
                        return fail(format!(
                            "{}'s map has a claim that isn't a place, a way, or a thing at a place",
                            def.id
                        ));
                    }
                });
            }
            world.maps.insert(item, claims);
        }
        if let Some(kind) = &def.kind {
            if !world.kinds.contains_key(kind) {
                return fail(format!(
                    "{} is of kind {kind:?}, which isn't a kind",
                    def.id
                ));
            }
            let item = world.find_by_key(&def.id).expect("just loaded");
            world.kind_of.insert(item, kind.clone());
        }
    }
    // Crossings refer to items, which are loaded after places.
    for def in &file.places {
        let place = world.find_by_key(&def.id).expect("just loaded");
        for (to, liquid) in &def.crossings {
            let other = world
                .find_by_key(to)
                .filter(|o| world.exits(place).contains(o))
                .ok_or_else(|| {
                    LoadError(format!(
                        "{} has a crossing to {to:?}, which isn't one of its exits",
                        def.id
                    ))
                })?;
            let liquid = world
                .find_by_key(liquid)
                .filter(|&l| world.matter.contains_key(&l))
                .ok_or_else(|| {
                    LoadError(format!(
                        "{} crosses {liquid:?}, which isn't a material item",
                        def.id
                    ))
                })?;
            world.crossings.insert((place, other), liquid);
            world.crossings.insert((other, place), liquid);
        }
    }
    // Growth refers to other items, which may be listed after it.
    for def in &file.items {
        let Some(grows) = &def.grows else { continue };
        let item = world.find_by_key(&def.id).expect("just loaded");
        let from = world
            .find_by_key(&grows.from)
            .filter(|&f| world.matter.contains_key(&f) && f != item)
            .ok_or_else(|| {
                LoadError(format!(
                    "{} grows from {:?}, which isn't another material item",
                    def.id, grows.from
                ))
            })?;
        if !world.matter.contains_key(&item) {
            return fail(format!("{} grows but isn't made of a material", def.id));
        }
        world.growth.insert(
            item,
            world::Growth {
                rate: parse_percent(&grows.rate)?,
                limit: parse_mass(&def.id, &grows.limit)?,
                from,
            },
        );
    }

    // Totals must fit in a single value, so no sum of any part of the world
    // can overflow later.
    if world.total_mass() > u128::from(u64::MAX) {
        return fail("the world's total mass is too large");
    }
    if world.total_credits() > u128::from(u64::MAX) {
        return fail("the world's total credits are too large");
    }
    if world.total_energy() > u128::from(u64::MAX) {
        return fail("the world's total energy is too large");
    }
    world.check_invariants().map_err(LoadError)?;
    Ok(world)
}

fn load_settings(def: &SettingsDef) -> Result<Settings, LoadError> {
    let mut settings = Settings::default();
    if let Some(t) = &def.reference_temperature {
        settings.reference_temperature = t.parse()?;
    }
    if let Some(rate) = &def.open_air_heat_loss {
        settings.open_air_heat_loss =
            parse_quantity(rate, property::HEAT_LOSS, "a heat loss like \"5 W/K\"")?;
    }
    if let Some(mass) = &def.dig_amount {
        settings.dig_amount = parse_mass("dig_amount", mass)?;
    }
    if let Some(t) = &def.max_touch_temperature {
        settings.max_touch_temperature = t.parse()?;
    }
    if let Some(t) = &def.rough_tolerance {
        settings.rough_tolerance = parse_quantity(t, property::LENGTH, "a length like \"5 mm\"")?;
    }
    if let Some(t) = &def.finest_tolerance {
        settings.finest_tolerance = parse_quantity(t, property::LENGTH, "a length like \"1 um\"")?;
    }
    if let Some(p) = &def.rubbing_improvement {
        settings.rubbing_improvement = parse_percent(p)?;
        if settings.rubbing_improvement >= 10_000 {
            return fail("rubbing_improvement must be under 100%");
        }
    }
    if let Some(t) = &def.rubbing_time {
        settings.rubbing_time = parse_quantity(t, property::DURATION, "a time like \"10 min\"")?;
    }
    if let Some(r) = &def.touch_resistance {
        settings.touch_resistance = parse_quantity(
            r,
            property::TOUCH_RESISTANCE,
            "a resistance like \"1 mOhm/um\"",
        )?;
    }
    if let Some(t) = &def.glow_temperature {
        settings.glow_temperature = t.parse()?;
    }
    if let Some(p) = &def.air_clearing {
        settings.air_clearing = parse_percent(p)?;
    }
    if let Some(c) = &def.convection {
        settings.convection = Some(parse_quantity(
            c,
            property::HEAT_TRANSFER,
            "a heat transfer like \"10 W/(m2*K)\"",
        )?);
    }
    if let Some(e) = &def.emissivity {
        settings.emissivity = parse_percent(e)?;
    }
    if let Some(c) = &def.touch_transfer {
        settings.touch_transfer = parse_quantity(
            c,
            property::HEAT_TRANSFER,
            "a heat transfer like \"100 W/(m2*K)\"",
        )?;
    }
    if let Some(w) = &def.wear_rate {
        settings.wear_rate =
            parse_quantity(w, property::MASS_PER_SECOND, "a rate like \"10 mg/s\"")?;
    }
    if let Some(f) = &def.friction_share {
        settings.friction_share = parse_percent(f)?;
    }
    if let Some(f) = &def.flame_share {
        settings.flame_share = parse_percent(f)?;
    }
    if let Some(h) = &def.hand_hardness {
        settings.hand_hardness = parse_number(h, 100, "a hardness like \"1\"")?;
    }
    if let Some(p) = &def.hand_push {
        settings.hand_push = parse_percent(p)?;
    }
    if let Some(d) = &def.drag {
        settings.drag = parse_percent(d)?;
    }
    let time = |t: &String| parse_quantity(t, property::DURATION, "a time like \"6 h\"");
    if let Some(t) = &def.explore_time {
        settings.explore_time = time(t)?.max(1);
    }
    if let Some(c) = &def.explore_chance {
        settings.explore_chance = parse_percent(c)?;
    }
    if let Some(r) = &def.planet_radius {
        settings.planet_radius = parse_quantity(r, property::LENGTH, "a length like \"6371 km\"")?;
    }
    if let Some(e) = &def.eye_height {
        settings.eye_height = parse_quantity(e, property::LENGTH, "a length like \"1.7 m\"")?;
    }
    if let Some(w) = &def.wound_rate {
        settings.wound_rate =
            parse_quantity(w, property::MASS_PER_SECOND_FLOW, "a rate like \"20 g/s\"")?;
    }
    if let Some(h) = &def.hit_chance {
        settings.hit_chance = parse_percent(h)?;
    }
    if let Some(t) = &def.survey_time {
        settings.survey_time = time(t)?.max(1);
    }
    if let Some(l) = &def.lapse_rate {
        settings.lapse_rate = parse_quantity(l, property::LAPSE_RATE, "a rate like \"6.5 K/km\"")?;
    }
    if let Some(d) = &def.day {
        settings.day = time(d)?;
        settings.starts_at = def.starts_at.as_ref().map(time).transpose()?.unwrap_or(0);
        settings.sunrise = def.sunrise.as_ref().map(time).transpose()?.unwrap_or(0);
        settings.sunset = def
            .sunset
            .as_ref()
            .map(time)
            .transpose()?
            .unwrap_or(settings.day);
        let day = settings.day;
        if day == 0
            || settings.sunrise > settings.sunset
            || settings.sunset > day
            || settings.starts_at >= day
        {
            return fail(
                "a day needs sunrise before sunset, and every time within the day".to_string(),
            );
        }
    }
    if let Some(t) = &def.calm_step {
        settings.calm_step = parse_quantity(t, property::DURATION, "a time like \"1 min\"")?.max(1);
    }
    Ok(settings)
}

fn load_materials(world: &mut World, defs: &[MaterialDef]) -> Result<(), LoadError> {
    let mut ids = BTreeMap::new();
    for (index, def) in defs.iter().enumerate() {
        let id =
            MaterialId(u16::try_from(index).map_err(|_| LoadError("too many materials".into()))?);
        if ids.insert(def.id.clone(), id).is_some() {
            return fail(format!("the material {:?} is defined twice", def.id));
        }
    }
    for (index, def) in defs.iter().enumerate() {
        let what = |property: &str| format!("{}: {property}", def.id);
        let melting_point: Temperature = def.melting_point.parse()?;
        let boiling_point: Temperature = def.boiling_point.parse()?;
        if boiling_point < melting_point {
            return fail(format!("{} boils before it melts", def.id));
        }
        let mut burns_to = Vec::new();
        for (product, share) in &def.burns_to {
            let product_id = *ids.get(product).ok_or_else(|| {
                LoadError(format!(
                    "{} burns to {product:?}, which isn't a material",
                    def.id
                ))
            })?;
            burns_to.push((product_id, parse_percent(share)?));
        }
        if !burns_to.is_empty() && burns_to.iter().map(|(_, s)| s).sum::<u64>() != 10_000 {
            return fail(what("burns_to must add up to 100%"));
        }
        let material = Material {
            key: def.id.clone(),
            label: def.label.clone(),
            melting_point,
            boiling_point,
            specific_heat: parse_quantity(
                &def.specific_heat,
                property::SPECIFIC_HEAT,
                "a specific heat like \"449 J/(kg*K)\"",
            )?,
            hardness: parse_number(&def.hardness, 100, "a hardness like \"4.5\"")?,
            energy_density: def
                .energy_density
                .as_deref()
                .map(|e| {
                    parse_quantity(
                        e,
                        property::ENERGY_DENSITY,
                        "an energy density like \"30 MJ/kg\"",
                    )
                })
                .transpose()?
                .unwrap_or(0),
            burns_to,
            density: def
                .density
                .as_deref()
                .map(|d| parse_quantity(d, property::DENSITY, "a density like \"7874 kg/m3\""))
                .transpose()?,
            speed_of_sound: def
                .speed_of_sound
                .as_deref()
                .map(|s| parse_quantity(s, property::SPEED, "a speed like \"5120 m/s\""))
                .transpose()?,
            resistivity: def
                .resistivity
                .as_deref()
                .map(|r| {
                    parse_quantity(
                        r,
                        property::RESISTIVITY,
                        "a resistivity like \"16.8 nOhm*m\"",
                    )
                })
                .transpose()?,
            voltage: def
                .voltage
                .as_deref()
                .map(|v| parse_quantity(v, property::VOLTAGE, "a voltage like \"1.5 V\""))
                .transpose()?,
            ignition_point: def.ignition_point.as_deref().map(str::parse).transpose()?,
            tensile_strength: def
                .tensile_strength
                .as_deref()
                .map(|t| parse_quantity(t, property::STRESS, "a strength like \"50 MPa\""))
                .transpose()?,
            softens_in: def
                .softens_in
                .iter()
                .map(|m| {
                    ids.get(m).copied().ok_or_else(|| {
                        LoadError(format!(
                            "{} softens in {m:?}, which isn't a material",
                            def.id
                        ))
                    })
                })
                .collect::<Result<_, _>>()?,
            becomes: match &def.becomes {
                Some(becomes) => {
                    let target = *ids.get(&becomes.material).ok_or_else(|| {
                        LoadError(format!(
                            "{} becomes {:?}, which isn't a material",
                            def.id, becomes.material
                        ))
                    })?;
                    Some((target, becomes.at.parse()?))
                }
                None => None,
            },
            burn_speed: def
                .burn_speed
                .as_deref()
                .map(|b| {
                    parse_quantity(b, property::BURN_SPEED, "a burn speed like \"15 g/(m2*s)\"")
                })
                .transpose()?
                .unwrap_or(0),
        };
        if material.ignition_point.is_some() && (material.burn_speed == 0 || !material.burns()) {
            return fail(format!(
                "{} has an ignition point, so it needs a burn speed and what it burns to",
                def.id
            ));
        }
        let id = MaterialId(u16::try_from(index).expect("counted above"));
        world.materials.insert(id, material);
    }
    Ok(())
}

fn load_item(world: &mut World, def: &ItemDef) -> Result<(), LoadError> {
    let mass = parse_mass(&def.id, &def.mass)?;
    let composition = parse_composition(
        world,
        &def.id,
        mass,
        def.material.as_deref(),
        def.composition.as_ref(),
    )?;
    if composition.is_none() && def.label.is_none() {
        return fail(format!(
            "{} needs a label, or a material to be described by",
            def.id
        ));
    }

    let item = world.spawn(Some(&def.id), def.label.as_deref());
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

    match composition {
        Some(composition) => {
            let temperature = match &def.temperature {
                Some(t) => t.parse()?,
                None => world.ambient(
                    world
                        .place_of(at)
                        .expect("items start in a place or with a person"),
                ),
            };
            insert_matter(world, item, composition, temperature, &def.id)?;
        }
        None => {
            if def.temperature.is_some() {
                return fail(format!(
                    "{} has a temperature but isn't made of a material",
                    def.id
                ));
            }
            world.masses.insert(item, mass);
        }
    }

    if !def.fixed {
        world.portable.insert(item);
    }
    if def.container || def.chamber.is_some() || def.form.is_some() {
        world.containers.insert(item);
    }
    if let Some(chamber) = &def.chamber {
        world.chambers.insert(
            item,
            Chamber {
                burn_rate: parse_mass(&def.id, &chamber.burn_rate)?,
                heat_loss: parse_quantity(
                    &chamber.heat_loss,
                    property::HEAT_LOSS,
                    "a heat loss like \"30 W/K\"",
                )?,
                lit: false,
            },
        );
    }
    if let Some(form) = &def.form {
        if !world.shapes.contains_key(&form.shape) {
            return fail(format!(
                "{} forms {:?}, which isn't a shape",
                def.id, form.shape
            ));
        }
        let tolerance = match &form.tolerance {
            Some(t) => parse_length(&def.id, t)?,
            None => world.settings.rough_tolerance,
        };
        world.forms.insert(
            item,
            Form {
                shape: form.shape.clone(),
                tolerance,
            },
        );
    }
    if let Some(pieces) = &def.pieces {
        if !world.matter.contains_key(&item) {
            return fail(format!(
                "{} has pieces but isn't made of a material",
                def.id
            ));
        }
        world.pieces.insert(
            item,
            world::Pieces {
                size: parse_mass(&def.id, &pieces.size)?,
                find_time: parse_quantity(
                    &pieces.find_time,
                    property::DURATION,
                    "a time like \"5 min\"",
                )?,
                full: mass,
                chance: pieces
                    .chance
                    .as_deref()
                    .map(parse_percent)
                    .transpose()?
                    .unwrap_or(10_000),
                edge: pieces
                    .edge
                    .as_deref()
                    .map(|e| parse_length(&def.id, e))
                    .transpose()?,
                needs: match &pieces.needs {
                    Some(needs)
                        if world.shapes.contains_key(needs)
                            || world.designs.contains_key(needs) =>
                    {
                        Some(needs.clone())
                    }
                    Some(needs) => {
                        return fail(format!(
                            "{} needs {needs:?}, which isn't a shape or design",
                            def.id
                        ));
                    }
                    None => None,
                },
            },
        );
    }
    match (&def.shape, &def.tolerance) {
        (Some(shape), tolerance) => {
            if !world.matter.contains_key(&item) {
                return fail(format!(
                    "{} has a shape but isn't made of a material",
                    def.id
                ));
            }
            if !world.shapes.contains_key(shape) {
                return fail(format!(
                    "{} has the shape {shape:?}, which isn't a shape",
                    def.id
                ));
            }
            let tolerance = match tolerance {
                Some(t) => parse_length(&def.id, t)?,
                None => world.settings.rough_tolerance,
            };
            world.shape_of.insert(item, shape.clone());
            world.tolerance.insert(item, tolerance);
        }
        (None, Some(_)) => return fail(format!("{} has a tolerance but no shape", def.id)),
        (None, None) => {}
    }
    Ok(())
}

fn parse_composition(
    world: &World,
    id: &str,
    mass: Mass,
    material: Option<&str>,
    composition: Option<&BTreeMap<String, String>>,
) -> Result<Option<Composition>, LoadError> {
    match (material, composition) {
        (Some(_), Some(_)) => fail(format!("{id} has both a material and a composition")),
        (Some(material), None) => Ok(Some(Composition::from([(
            material_id(world, id, material)?,
            mass,
        )]))),
        (None, Some(parts)) => {
            let mut fractions = Vec::new();
            for (material, share) in parts {
                fractions.push((material_id(world, id, material)?, parse_percent(share)?));
            }
            if fractions.iter().map(|(_, s)| s).sum::<u64>() != 10_000 {
                return fail(format!("{id}: the composition must add up to 100%"));
            }
            let mut composition = Composition::new();
            for (material, part) in matter::split_by_fractions(mass, &fractions) {
                if part != Mass::ZERO {
                    composition.insert(material, part);
                }
            }
            Ok(Some(composition))
        }
        (None, None) => Ok(None),
    }
}

fn insert_matter(
    world: &mut World,
    id: world::EntityId,
    composition: Composition,
    temperature: Temperature,
    name: &str,
) -> Result<(), LoadError> {
    let capacity = matter::heat_capacity(&world.materials, &composition);
    let heat = u64::try_from(matter::energy_at(temperature, capacity))
        .map_err(|_| LoadError(format!("{name} holds too much heat")))?;
    world.matter.insert(id, composition);
    world.heat.insert(id, Energy::from_uj(heat));
    Ok(())
}

/// Loads the hierarchy of kinds, checking every parent is a kind and no kind is
/// its own ancestor.
fn load_kinds(world: &mut World, defs: &[KindDef]) -> Result<(), LoadError> {
    for def in defs {
        let instinct = match &def.instinct {
            Some(i) => Some(world::Instinct {
                flees: i.flees.clone(),
                rest: parse_quantity(&i.rest, property::DURATION, "a time like \"30 min\"")?.max(1),
                wander: parse_percent(&i.wander)?,
                wary: i
                    .wary
                    .as_deref()
                    .map(|w| parse_quantity(w, property::DURATION, "a time like \"3 h\""))
                    .transpose()?
                    .unwrap_or(0),
                charges: i.charges,
            }),
            None => None,
        };
        let kind = world::Kind {
            label: def.label.clone(),
            parent: def.parent.clone(),
            instinct,
            weapon: def
                .weapon
                .as_ref()
                .map(|w| parse_length(&def.id, &w.edge))
                .transpose()?,
        };
        if world.kinds.insert(def.id.clone(), kind).is_some() {
            return fail(format!("the kind {:?} is defined twice", def.id));
        }
    }
    for (id, kind) in &world.kinds {
        if let Some(parent) = &kind.parent
            && !world.kinds.contains_key(parent)
        {
            return fail(format!(
                "the kind {id} belongs to {parent:?}, which isn't a kind"
            ));
        }
        // A lineage stops if it comes back on itself, so its last kind then
        // still has a parent.
        let lineage = world.lineage(id);
        let last = lineage.last().and_then(|k| world.kinds.get(*k));
        if last.is_some_and(|k| {
            k.parent
                .as_ref()
                .is_some_and(|p| world.kinds.contains_key(p))
        }) {
            return fail(format!("the kind {id} is its own ancestor"));
        }
        for fled in kind.instinct.iter().flat_map(|i| &i.flees) {
            if !world.kinds.contains_key(fled) {
                return fail(format!("the kind {id} flees {fled:?}, which isn't a kind"));
            }
        }
    }
    Ok(())
}

/// Loads one person or creature. What isn't given comes from its kind, and
/// the kinds above it, nearest first.
fn load_agent(
    world: &mut World,
    def: &AgentDef,
    id: &str,
    kinds: &[KindDef],
) -> Result<(), LoadError> {
    let lineage: Vec<&KindDef> = match &def.kind {
        Some(kind) => {
            if !world.kinds.contains_key(kind) {
                return fail(format!("{id} is of kind {kind:?}, which isn't a kind"));
            }
            world
                .lineage(kind)
                .iter()
                .filter_map(|k| kinds.iter().find(|d| d.id == *k))
                .collect()
        }
        None => Vec::new(),
    };
    let mass_text = def
        .mass
        .as_ref()
        .or_else(|| lineage.iter().find_map(|k| k.mass.as_ref()))
        .ok_or_else(|| LoadError(format!("{id} needs a mass, or a kind that gives one")))?;
    let composition = def
        .composition
        .as_ref()
        .or_else(|| lineage.iter().find_map(|k| k.composition.as_ref()));
    let life_def = def
        .life
        .as_ref()
        .or_else(|| lineage.iter().find_map(|k| k.life.as_ref()));

    let agent = world.spawn(Some(id), Some(&def.label));
    let at = match world.find_by_key(&def.at) {
        Some(at) if world.is_place(at) => at,
        _ => {
            return fail(format!("{id} is at {:?}, which isn't a place", def.at));
        }
    };
    world.locations.insert(agent, at);
    let mass = parse_mass(id, mass_text)?;
    match parse_composition(world, id, mass, None, composition)? {
        Some(composition) => {
            let temperature = match &def.temperature {
                Some(t) => t.parse()?,
                None => world.ambient(at),
            };
            insert_matter(world, agent, composition, temperature, id)?;
        }
        None => {
            world.masses.insert(agent, mass);
        }
    }
    world.agents.insert(agent);
    if let Some(kind) = &def.kind {
        world.kind_of.insert(agent, kind.clone());
    }
    // Persons remember; creatures on instinct know their range.
    if world.instinct(agent).is_none() {
        world.memories.insert(
            agent,
            world::Memory {
                finds_ways: def.lost,
                places: BTreeSet::from([at]),
                ..world::Memory::default()
            },
        );
    }
    if !def.range.is_empty() {
        let mut range = BTreeSet::new();
        for place in &def.range {
            match world.find_by_key(place) {
                Some(p) if world.is_place(p) => {
                    range.insert(p);
                }
                _ => return fail(format!("{id} keeps to {place:?}, which isn't a place")),
            }
        }
        world.ranges.insert(agent, range);
    }
    world.wallets.insert(agent, Credits::new(def.credits));
    if let Some(life) = life_def {
        let life = load_life(world, agent, id, life)?;
        world.life.insert(agent, life);
    }
    Ok(())
}

fn load_life(
    world: &World,
    agent: world::EntityId,
    id: &str,
    def: &LifeDef,
) -> Result<world::Life, LoadError> {
    let body = world
        .matter
        .get(&agent)
        .ok_or_else(|| LoadError(format!("{id} is alive, so it needs a composition")))?;
    let power = |text: &str| parse_quantity(text, property::POWER, "a power like \"80 W\"");
    let rate = |text: &str| parse_quantity(text, property::MASS_RATE, "a rate like \"2 kg/day\"");
    let fluid = material_id(world, id, &def.fluid)?;
    let fluid_normal = body.get(&fluid).copied().unwrap_or(Mass::ZERO);
    let mut digests = BTreeSet::new();
    for material in &def.digests {
        digests.insert(material_id(world, id, material)?);
    }
    Ok(world::Life {
        resting_power: power(&def.resting_power)?,
        working_power: power(&def.working_power)?,
        heat_loss: parse_quantity(
            &def.heat_loss,
            property::HEAT_LOSS,
            "a heat loss like \"8 W/K\"",
        )?,
        set_point: def.set_point.parse()?,
        sweat_rate: rate(&def.sweat_rate)?,
        sweat_heat: parse_quantity(
            &def.sweat_heat,
            property::ENERGY_DENSITY,
            "an energy like \"2.4 MJ/kg\"",
        )?,
        fluid,
        fluid_loss: rate(&def.fluid_loss)?,
        fluid_minimum: parse_mass(id, &def.fluid_minimum)?,
        fluid_normal,
        digests,
        gulp: parse_mass(id, &def.gulp)?,
        coldest: def.coldest.parse()?,
        hottest: def.hottest.parse()?,
        carry_limit: def
            .carry
            .as_deref()
            .map(|c| parse_mass(id, c))
            .transpose()?,
        walking_speed: def
            .walk
            .as_deref()
            .map(|w| parse_quantity(w, property::SPEED, "a speed like \"1.2 m/s\""))
            .transpose()?
            .unwrap_or(1_200),
        climbing_share: def
            .climb
            .as_deref()
            .map(parse_percent)
            .transpose()?
            .unwrap_or(4_500),
        sleep: match (&def.awake, &def.sleep) {
            (Some(awake), Some(need)) => {
                let time = |t: &str| parse_quantity(t, property::DURATION, "a time like \"8 h\"");
                let awake = time(awake)?.max(1);
                Some(world::Sleep {
                    awake,
                    need: time(need)?.max(1),
                    tired_pace: def
                        .tired_pace
                        .as_deref()
                        .map(parse_percent)
                        .transpose()?
                        .unwrap_or(10_000)
                        .max(1),
                    collapse: def
                        .collapse
                        .as_deref()
                        .map(time)
                        .transpose()?
                        .unwrap_or(awake * 5 / 2)
                        .max(awake),
                    debt: 0,
                    since: 0,
                    until: 0,
                    shelter: None,
                })
            }
            (None, None) => None,
            _ => {
                return fail(format!(
                    "{id} needs both how long it stays awake and how long it sleeps"
                ));
            }
        },
        stores: def.stores.as_deref().map(parse_percent).transpose()?,
        clots: def
            .clots
            .as_deref()
            .map(|c| parse_quantity(c, property::DURATION, "a time like \"10 min\""))
            .transpose()?
            .unwrap_or(600)
            .max(1),
        wounds: Vec::new(),
        reserve: {
            // The most energy-rich store it digests that the body holds.
            let digests: Vec<MaterialId> = def
                .digests
                .iter()
                .map(|m| material_id(world, id, m))
                .collect::<Result<_, _>>()?;
            digests
                .iter()
                .filter_map(|m| {
                    body.get(m)
                        .map(|mass| (world.materials[m].energy_density, *m, *mass))
                })
                .max()
                .map(|(_, m, mass)| (m, mass))
        },
        working_until: 0,
        died_of: None,
    })
}

fn parse_length(id: &str, text: &str) -> Result<u64, LoadError> {
    parse_quantity(text, property::LENGTH, "a length like \"2 mm\"")
        .map_err(|e| LoadError(format!("{id}: {e}")))
}

fn load_shape(def: &ShapeDef) -> Result<world::ShapeDef, LoadError> {
    let role = match def.role.as_deref() {
        None => None,
        Some("cutting") => Some(Role::Cutting),
        Some("holding") => Some(Role::Holding),
        Some("conducting") => Some(Role::Conducting),
        Some("glowing") => Some(Role::Glowing),
        Some("source") => Some(Role::Source),
        Some("touching") => Some(Role::Touching),
        Some("casting") => Some(Role::Casting),
        Some("pulling") => Some(Role::Pulling),
        Some("pushing") => Some(Role::Pushing),
        Some("containing") => Some(Role::Containing),
        Some(other) => {
            return fail(format!(
                "the shape {} has the role {other:?}; roles are cutting, holding, conducting, glowing, source, touching, casting, pulling, pushing, and containing",
                def.id
            ));
        }
    };
    let length = def
        .length
        .as_deref()
        .map(|l| parse_length(&def.id, l))
        .transpose()?;
    if matches!(role, Some(Role::Conducting | Role::Glowing | Role::Pulling)) && length.is_none() {
        return fail(format!(
            "the shape {} conducts, so it needs a length",
            def.id
        ));
    }
    let heat_loss = def
        .heat_loss
        .as_deref()
        .map(|h| parse_quantity(h, property::HEAT_LOSS, "a heat loss like \"1 mW/K\""))
        .transpose()?;
    let push = def.push.as_deref().map(parse_percent).transpose()?;
    if matches!(role, Some(Role::Pushing)) != push.is_some() {
        return fail(format!(
            "the shape {} needs both the pushing role and how much it pushes, or neither",
            def.id
        ));
    }
    let capacity = def
        .holds
        .as_deref()
        .map(|h| parse_mass(&def.id, h))
        .transpose()?;
    if matches!(role, Some(Role::Containing)) != capacity.is_some() {
        return fail(format!(
            "the shape {} needs both the containing role and how much it holds, or neither",
            def.id
        ));
    }
    if matches!(role, Some(Role::Casting)) != def.casts.is_some() {
        return fail(format!(
            "the shape {} needs both the casting role and what it casts, or neither",
            def.id
        ));
    }
    Ok(world::ShapeDef {
        label: def.label.clone(),
        role,
        length,
        heat_loss,
        push,
        capacity,
        casts: def.casts.clone(),
    })
}

fn load_designs(world: &mut World, defs: &[DesignDef]) -> Result<(), LoadError> {
    for def in defs {
        if world.shapes.contains_key(&def.id) {
            return fail(format!(
                "the design {:?} has the same id as a shape",
                def.id
            ));
        }
        if world.designs.contains_key(&def.id) {
            return fail(format!("the design {:?} is defined twice", def.id));
        }
    }
    let design_ids: BTreeSet<&str> = defs.iter().map(|d| d.id.as_str()).collect();
    for def in defs {
        let mut slots = Vec::new();
        for (slot, needs) in &def.parts {
            let requirement = if world.shapes.contains_key(needs) {
                Requirement::Shape(needs.clone())
            } else if design_ids.contains(needs.as_str()) && needs != &def.id {
                Requirement::Design(needs.clone())
            } else if let Some(material) = world.material_by_key(needs) {
                Requirement::Material(material)
            } else {
                return fail(format!(
                    "the design {} needs {needs:?}, which isn't a shape, a material, or another design",
                    def.id
                ));
            };
            slots.push((slot.clone(), requirement));
        }
        if slots.is_empty() {
            return fail(format!("the design {} has no parts", def.id));
        }
        world.designs.insert(
            def.id.clone(),
            Design {
                label: def.label.clone(),
                slots,
                holds: def.holds || def.chamber.is_some(),
                chamber: match &def.chamber {
                    Some(chamber) => Some(Chamber {
                        burn_rate: parse_mass(&def.id, &chamber.burn_rate)?,
                        heat_loss: parse_quantity(
                            &chamber.heat_loss,
                            property::HEAT_LOSS,
                            "a heat loss like \"30 W/K\"",
                        )?,
                        lit: false,
                    }),
                    None => None,
                },
                shelter: def.shelter.as_deref().map(parse_percent).transpose()?,
            },
        );
    }
    Ok(())
}

fn material_id(world: &World, item: &str, key: &str) -> Result<MaterialId, LoadError> {
    world
        .material_by_key(key)
        .ok_or_else(|| LoadError(format!("{item} is made of {key:?}, which isn't a material")))
}

fn parse_mass(id: &str, text: &str) -> Result<Mass, LoadError> {
    text.parse().map_err(|e| LoadError(format!("{id}: {e}")))
}
