//! The designer: the person building or testing a world, who can put things
//! in it on demand (`/make` at the console). The designer is a named source,
//! as sunlight is: what they make passes the gate like everything else, and
//! the world keeps count of the matter and heat they gave
//! (`World::designed`), so nothing comes from nowhere. Only the person at the
//! keyboard makes things this way, never anyone in the world. See
//! docs/ideas/tools.md and docs/challenges/skill-grounds.md.

use crate::datasheet;
use crate::gate::{Cause, Change};
use crate::matter::{self, Composition, MaterialId};
use crate::units::{Energy, Mass, Temperature};
use crate::world::{EntityId, Requirement, World};

/// What the designer can make.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Make {
    /// A lump of one material, given a shape if one is named.
    Lump {
        material: MaterialId,
        mass: Mass,
        shape: Option<String>,
    },
    /// Something built to a design, its parts made first.
    Built(String),
    /// A kit from data: a holder with things in it, lit or not.
    Kit(String),
}

/// A set of things from data that `/make` puts down together, such as a fire
/// laid in a ring and burning.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Kit {
    pub holder: Make,
    pub inside: Vec<Make>,
    /// Everything inside that burns is lit.
    pub lit: bool,
}

/// How far past its ignition point the designer's flame heats something.
const FLAME_ABOVE_IGNITION: Temperature = Temperature::from_mk(100_000);

/// Reads what to make, by names in data: an amount of a material ("2 kg
/// <material>"), shaped if it says so ("300 g <material> as <shape>"), a
/// design, or a kit.
pub fn parse(world: &World, text: &str) -> Result<Make, String> {
    let text = text.trim().to_lowercase();
    let same = |name: &str| name.to_lowercase().replace('-', " ") == text.replace('-', " ");
    if let Some(id) = world.kits.keys().find(|id| same(id)) {
        return Ok(Make::Kit(id.clone()));
    }
    if let Some((id, _)) = world
        .designs
        .iter()
        .find(|(id, d)| same(id) || same(&d.label))
    {
        return Ok(Make::Built(id.clone()));
    }
    // An amount of a material, shaped or not.
    let (lump, shape) = match text.split_once(" as ") {
        Some((lump, shape)) => (lump.trim(), Some(shape.trim())),
        None => (text.as_str(), None),
    };
    let words: Vec<&str> = lump.split_whitespace().collect();
    let split = (1..words.len())
        .find(|&n| words[..n].join(" ").parse::<Mass>().is_ok())
        .ok_or_else(|| {
            format!(
                "there's nothing called \"{text}\" to make; say how much of a material, \
                 like \"2 kg <material>\", or name a design or kit"
            )
        })?;
    let mass: Mass = words[..split].join(" ").parse().expect("checked above");
    let name = words[split..].join(" ");
    let material = world
        .materials
        .iter()
        .find(|(_, m)| same_words(&m.key, &name) || same_words(&m.label, &name))
        .map(|(&id, _)| id)
        .ok_or_else(|| format!("there's no material called \"{name}\""))?;
    let shape = match shape {
        Some(shape) => Some(
            world
                .shapes
                .iter()
                .find(|(id, s)| same_words(id, shape) || same_words(&s.label, shape))
                .map(|(id, _)| id.clone())
                .ok_or_else(|| format!("there's no shape called \"{shape}\""))?,
        ),
        None => None,
    };
    if mass == Mass::ZERO {
        return Err("make something that weighs something".into());
    }
    Ok(Make::Lump {
        material,
        mass,
        shape,
    })
}

fn same_words(a: &str, b: &str) -> bool {
    a.to_lowercase().replace('-', " ") == b.to_lowercase().replace('-', " ")
}

/// Makes `what` at `at` (a place, a person, or a container), and returns
/// what was made. Something made in a place lies at `spot`, if given.
pub fn make(
    world: &mut World,
    what: &Make,
    at: EntityId,
    spot: Option<(i64, i64)>,
) -> Result<EntityId, String> {
    let made = match what {
        Make::Lump {
            material,
            mass,
            shape,
        } => lump(world, *material, *mass, shape.as_deref(), at)?,
        Make::Built(design) => built(world, design, at)?,
        Make::Kit(id) => {
            let kit = world.kits.get(id).cloned().ok_or("no such kit")?;
            let holder = make(world, &kit.holder, at, spot)?;
            for thing in &kit.inside {
                let inside = make(world, thing, holder, None)?;
                if kit.lit && burns(world, inside) {
                    light(world, inside)?;
                }
            }
            return Ok(holder);
        }
    };
    if let Some(at) = spot.filter(|_| world.is_place(at)) {
        apply(world, vec![Change::Spot { entity: made, at }])?;
    }
    Ok(made)
}

fn apply(world: &mut World, changes: Vec<Change>) -> Result<(), String> {
    world
        .apply(Cause::Designer, changes)
        .map_err(|fault| fault.to_string())
}

fn lump(
    world: &mut World,
    material: MaterialId,
    mass: Mass,
    shape: Option<&str>,
    at: EntityId,
) -> Result<EntityId, String> {
    let made = world.next_id();
    let mut changes = vec![Change::Provide {
        at,
        make: Composition::from([(material, mass)]),
    }];
    if let Some(shape) = shape {
        changes.push(Change::Shape {
            entity: made,
            shape: Some((shape.to_string(), world.settings.rough_tolerance)),
        });
    }
    apply(world, changes)?;
    Ok(made)
}

/// Makes each part, then puts them together, measured as any assembly is.
fn built(world: &mut World, design: &str, at: EntityId) -> Result<EntityId, String> {
    let slots = world
        .designs
        .get(design)
        .ok_or("no such design")?
        .slots
        .clone();
    let mut parts = Vec::new();
    for (slot, requirement) in &slots {
        let part = match requirement {
            Requirement::Material(material) => {
                let mass = piece_of(world, *material).ok_or_else(|| {
                    format!(
                        "nothing here says how big a piece of {} is; make the parts \
                         one by one (\"/make 1 kg {0}\") and assemble them",
                        world.materials[material].label
                    )
                })?;
                lump(world, *material, mass, None, at)?
            }
            Requirement::Design(inner) => built(world, inner, at)?,
            Requirement::Shape(shape) => {
                return Err(format!(
                    "the {slot} is shaped ({shape}), and its material is the maker's \
                     choice; make it with \"/make <amount> <material> as {shape}\" and assemble"
                ));
            }
        };
        parts.push((slot.clone(), part));
    }
    let sheets: Vec<(String, String, datasheet::Datasheet)> = parts
        .iter()
        .map(|(slot, part)| {
            (
                slot.clone(),
                world.label(*part),
                datasheet::measure(world, *part),
            )
        })
        .collect();
    let sheet = datasheet::measure_assembly(&world.settings, &sheets);
    let made = world.next_id();
    apply(
        world,
        vec![Change::Assemble {
            design: Some(design.to_string()),
            parts: parts.into_iter().map(|(_, p)| p).collect(),
            at,
            datasheet: sheet,
        }],
    )?;
    Ok(made)
}

/// How big a piece of a material is where it's gathered: the first stock of
/// it that comes in pieces.
fn piece_of(world: &World, material: MaterialId) -> Option<Mass> {
    world.pieces.iter().find_map(|(&stock, pieces)| {
        (world.composition(stock).and_then(matter::dominant) == Some(material))
            .then_some(pieces.size)
    })
}

fn burns(world: &World, thing: EntityId) -> bool {
    world
        .composition(thing)
        .and_then(matter::dominant)
        .is_some_and(|m| world.materials[&m].ignition_point.is_some())
}

/// The designer's flame: heats something past the point where it catches.
pub fn light(world: &mut World, thing: EntityId) -> Result<(), String> {
    let ignition = world
        .composition(thing)
        .and_then(matter::dominant)
        .and_then(|m| world.materials[&m].ignition_point)
        .ok_or("that doesn't burn")?;
    let hot = Temperature::from_mk(ignition.mk() + FLAME_ABOVE_IGNITION.mk());
    let needed = matter::energy_at(hot, world.heat_capacity(thing));
    let has = u128::from(world.heat(thing).unwrap_or_default().uj());
    if needed <= has {
        return Ok(());
    }
    let amount = u64::try_from(needed - has).map_err(|_| "too much heat")?;
    apply(
        world,
        vec![Change::Endow {
            entity: thing,
            amount: Energy::from_uj(amount),
        }],
    )
}
