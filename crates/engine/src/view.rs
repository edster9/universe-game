//! What a person perceives. Views are data, not text: the console formats
//! them now, and a browser will later.

use crate::matter::State;
use crate::sight::{Eyes, Sight};
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
    /// Out of reach where it lies: how far, in µm, and which way.
    pub away: Option<(u64, &'static str)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Look {
    pub place: String,
    pub exits: Vec<String>,
    pub people: Vec<String>,
    pub things: Vec<Thing>,
    /// Gases hanging in the place.
    pub air: Vec<Thing>,
    /// The day, counting from 1, and seconds since midnight, in a world with
    /// days.
    pub time: Option<(u64, u64)>,
    pub night: bool,
    /// Too dark to see by: night, with no fire.
    pub dark: bool,
    /// Whether the ways out are only those the person knows of.
    pub finding_ways: bool,
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
        exits: world
            .known_exits(actor, here)
            .into_iter()
            .map(|e| world.label(e))
            .collect(),
        people: Vec::new(),
        things: Vec::new(),
        air: Vec::new(),
        time: world.time_of_day().map(|t| {
            let day = world.settings().day;
            ((world.tick() + world.settings().starts_at) / day + 1, t)
        }),
        night: world.is_night(),
        dark: world.is_dark(here),
        finding_ways: world.finds_ways(actor),
    };
    // Only what they can see from where they stand, and what they can't
    // make out only by how big it looks.
    let eyes = Eyes::of(world, actor);
    for id in world.contents(here).into_iter().filter(|&e| e != actor) {
        let sight = eyes.sight(world, id);
        if sight == Sight::Unseen {
            continue;
        }
        if world.is_agent(id) {
            look.people.push(eyes.label(world, id));
        } else if world.is_all(id, State::Gas) {
            look.air.push(thing(world, actor, id));
        } else {
            let mut seen = if sight == Sight::Seen {
                // Too far to tell what it is, or how heavy.
                Thing {
                    label: crate::sight::something(world, id),
                    mass: Mass::ZERO,
                    temperature: None,
                    notes: Vec::new(),
                    contents: Vec::new(),
                    away: None,
                }
            } else {
                thing(world, actor, id)
            };
            if !world.within_reach(actor, id) {
                seen.away = Some((world.gap(actor, id), crate::laws::way_to(world, actor, id)));
            }
            look.things.push(seen);
        }
    }
    Some(look)
}

/// What `actor` is carrying, and their credits.
pub fn inventory(world: &World, actor: EntityId) -> Inventory {
    let things: Vec<Thing> = world
        .contents(actor)
        .into_iter()
        .map(|e| thing(world, actor, e))
        .collect();
    Inventory {
        things,
        carried: world.carried_mass(actor),
        credits: world.wallet(actor).unwrap_or(Credits::ZERO),
    }
}

fn thing(world: &World, viewer: EntityId, id: EntityId) -> Thing {
    let ambient = world
        .place_of(id)
        .map_or(world.settings().reference_temperature, |p| world.ambient(p));
    let temperature = world
        .temperature(id)
        .filter(|t| t.mk().abs_diff(ambient.mk()) >= NOTICEABLE_MK);
    let mut notes = match world.chamber(id) {
        Some(chamber) if chamber.lit => vec!["lit".to_string()],
        _ => Vec::new(),
    };
    if world.life(id).is_some_and(|l| l.died_of.is_some()) {
        notes.push("dead".to_string());
    }
    if world.is_burning(id) {
        notes.push("burning".to_string());
    }
    match world.worn(id).map(|w| w.on) {
        Some(crate::world::Covering::Feet) => notes.push("on your feet".to_string()),
        Some(crate::world::Covering::Body) => notes.push("worn".to_string()),
        None => {}
    }
    if world.worn_through(id) {
        notes.push("worn through".to_string());
    }
    let contents = if world.is_container(id) {
        world
            .held(id)
            .into_iter()
            .map(|e| thing(world, viewer, e))
            .collect()
    } else {
        Vec::new()
    };
    Thing {
        label: world.label_for(viewer, id),
        mass: world.mass(id),
        temperature,
        notes,
        contents,
        away: None,
    }
}

/// What a person can picture of the world around them: everything they can
/// see where they stand, and the fixed things they remember at other places, from when
/// they last saw them. A client draws only these (no oracles): creatures and
/// loose things elsewhere stay unseen until the person goes and looks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scene {
    /// The place they stand in, if any.
    pub here: Option<EntityId>,
    /// Everything they can see at that place, the person too.
    pub in_sight: Vec<EntityId>,
    /// Fixed things remembered at other places: where, and what.
    pub remembered: Vec<(EntityId, EntityId)>,
}

/// What `viewer` can picture now.
pub fn scene(world: &World, viewer: EntityId) -> Scene {
    let here = world.place_of(viewer);
    let eyes = Eyes::of(world, viewer);
    let in_sight = here
        .map(|p| world.contents(p))
        .unwrap_or_default()
        .into_iter()
        .filter(|&id| eyes.sight(world, id) != Sight::Unseen)
        .collect();
    let remembered = world
        .memory(viewer)
        .map(|memory| {
            memory
                .sightings
                .iter()
                .filter(|&(&place, _)| Some(place) != here)
                .flat_map(|(&place, (_, things))| things.iter().map(move |&t| (place, t)))
                // What's gone is no longer there to draw; what moves is
                // wherever it went since.
                .filter(|&(_, t)| world.exists(t) && !world.is_agent(t) && !world.is_portable(t))
                .collect()
        })
        .unwrap_or_default();
    Scene {
        here,
        in_sight,
        remembered,
    }
}
