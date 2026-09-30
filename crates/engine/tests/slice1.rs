//! Slice 1's tests (docs/slices.md): the engine runs a production chain it
//! has no names for. Swap the data file and the same chain makes a different
//! metal. Mass and energy are conserved, heating costs fuel, a chamber goes
//! out when its fuel runs out, and a tool only works material softer than
//! itself.

use std::collections::BTreeSet;

use engine::data::load_world;
use engine::intent::{Command, Intent, parse};
use engine::laws::{ActError, Refusal, act};
use engine::matter::{self, State};
use engine::nature;
use engine::units::{Credits, Temperature};
use engine::world::{EntityId, World};
use proptest::prelude::*;

const IRON: &str = include_str!("../../../data/slice1-iron.toml");
const COPPER: &str = include_str!("../../../data/slice1-copper.toml");

fn id(world: &World, key: &str) -> EntityId {
    world.find_by_key(key).unwrap_or_else(|| panic!("no {key}"))
}

fn intent(line: &str) -> Intent {
    match parse(line) {
        Ok(Command::Act(intent)) => intent,
        other => panic!("{line:?} should parse to an action, got {other:?}"),
    }
}

fn traveller(world: &World) -> EntityId {
    id(world, "traveller")
}

fn ok(world: &mut World, line: &str) {
    let actor = traveller(world);
    act(world, actor, intent(line)).unwrap_or_else(|e| panic!("{line:?} should work: {e}"));
}

fn refused(world: &mut World, line: &str) -> Refusal {
    let before = world.clone();
    let actor = traveller(world);
    match act(world, actor, intent(line)) {
        Err(ActError::Refused(refusal)) => {
            assert_eq!(*world, before, "refusing {line:?} changed the world");
            refusal
        }
        other => panic!("{line:?} should be refused, got {other:?}"),
    }
}

/// Totals that must never change.
fn totals(world: &World) -> (u128, u128, u128) {
    (
        world.total_mass(),
        world.total_energy(),
        world.total_credits(),
    )
}

/// Runs ticks until `done` or `limit` ticks pass, checking conservation
/// after every tick. Returns how many ticks it took.
fn wait_until(world: &mut World, limit: u64, done: impl Fn(&World) -> bool) -> Option<u64> {
    let start = totals(world);
    for waited in 0..=limit {
        if done(world) {
            return Some(waited);
        }
        nature::tick(world).expect("nature never faults");
        assert_eq!(
            totals(world),
            start,
            "totals changed during tick {}",
            world.tick()
        );
    }
    None
}

/// A piece somewhere inside `holder` made only of `material`, in `state`.
fn pure_piece(world: &World, holder: EntityId, material: &str, state: State) -> Option<EntityId> {
    let material = world.material_by_key(material)?;
    world.contents(holder).into_iter().find(|&e| {
        world
            .composition(e)
            .is_some_and(|c| c.len() == 1 && c.contains_key(&material))
            && world.is_all(e, state)
    })
}

/// Dig, smelt, cast, cool, and pick up a blade of `metal`, using only the
/// commands a player would type.
fn run_chain(text: &str, metal: &str) {
    let mut w = load_world(text).unwrap();
    let start = totals(&w);
    let (hearth, mould) = (id(&w, "hearth"), id(&w, "mould"));

    ok(&mut w, "take pick");
    ok(&mut w, "dig vein with pick");
    ok(&mut w, "go forge");
    ok(&mut w, "put lump in hearth");
    ok(&mut w, "take charcoal-1");
    ok(&mut w, "put charcoal-1 in hearth");
    ok(&mut w, "light hearth");

    wait_until(&mut w, 1_200, |w| {
        pure_piece(w, hearth, metal, State::Liquid).is_some()
    })
    .unwrap_or_else(|| panic!("no molten {metal} within 20 minutes"));
    let molten = pure_piece(&w, hearth, metal, State::Liquid).unwrap();
    let pour = format!("pour {} into mould", w.key(molten));
    ok(&mut w, &pour);

    wait_until(&mut w, 600, |w| {
        pure_piece(w, mould, metal, State::Solid).is_some()
    })
    .unwrap_or_else(|| panic!("the {metal} never set"));
    let blade = pure_piece(&w, mould, metal, State::Solid).unwrap();
    assert_eq!(w.shape(blade), Some("blade"));
    assert_eq!(w.tolerance(blade), Some(1_000), "cast to the mould's 1 mm");
    assert_eq!(w.label(blade), format!("{metal} blade"));
    assert_eq!(
        refused(&mut w, "take blade from mould"),
        Refusal::TooHot(format!("the {metal} blade"))
    );

    let touchable = w.settings().max_touch_temperature;
    wait_until(&mut w, 10_000, |w| {
        w.temperature(blade).unwrap() <= touchable
    })
    .unwrap_or_else(|| panic!("the {metal} blade never cooled"));
    ok(&mut w, "take blade from mould");
    assert_eq!(w.location(blade), Some(traveller(&w)));
    assert_eq!(totals(&w), start);
}

#[test]
fn the_chain_makes_an_iron_blade() {
    run_chain(IRON, "iron");
}

#[test]
fn the_same_chain_makes_a_copper_blade_from_a_different_data_file() {
    run_chain(COPPER, "copper");
}

/// The engine's code must not know any material, shape, item, or design by
/// name. A one-word name is forbidden as a word. A longer name is forbidden
/// as a phrase: the engine may say "fire" (things catch fire) but not "fire
/// ring", which is a thing.
#[test]
fn the_engine_names_no_materials_shapes_or_items() {
    let mut words: BTreeSet<String> = BTreeSet::from(["sword".to_string()]);
    let mut phrases: BTreeSet<String> = BTreeSet::new();
    let ignore = ["the", "and", "of", "with", "for", "a"];
    let normal = |text: &str| {
        text.to_lowercase()
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !w.is_empty() && !ignore.contains(w))
            .collect::<Vec<_>>()
            .join(" ")
    };
    for text in [
        IRON,
        COPPER,
        include_str!("../../../data/slice2.toml"),
        include_str!("../../../data/stranded.toml"),
        include_str!("../../../data/island-things.toml"),
        include_str!("../../../data/strangers-words.toml"),
        include_str!("../../../data/far-folk.toml"),
    ] {
        let data: toml::Table = toml::from_str(text).unwrap();
        // A people's own words are names too.
        for culture in data
            .get("culture")
            .and_then(|v| v.as_array())
            .into_iter()
            .flatten()
        {
            for word in culture
                .get("words")
                .and_then(|w| w.as_table())
                .into_iter()
                .flat_map(|t| t.values())
                .filter_map(|v| v.as_str())
            {
                words.insert(normal(word));
            }
        }
        for section in ["material", "shape", "item", "design", "kind"] {
            let Some(entries) = data.get(section).and_then(|v| v.as_array()) else {
                continue;
            };
            for entry in entries {
                for field in ["id", "label"] {
                    let Some(name) = entry.get(field).and_then(|v| v.as_str()) else {
                        continue;
                    };
                    let name = normal(name);
                    if name.contains(' ') {
                        phrases.insert(name);
                    } else if name.len() > 2 {
                        words.insert(name);
                    }
                }
            }
        }
    }

    let src = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    let mut found = Vec::new();
    for entry in std::fs::read_dir(src).unwrap() {
        let path = entry.unwrap().path();
        let code = std::fs::read_to_string(&path).unwrap();
        let code = format!(" {} ", normal(&code));
        for word in &words {
            if code.contains(&format!(" {word} ")) {
                found.push(format!("{} mentions {word:?}", path.display()));
            }
        }
        for phrase in &phrases {
            if code.contains(&format!(" {phrase} ")) {
                found.push(format!("{} mentions {phrase:?}", path.display()));
            }
        }
    }
    assert!(
        found.is_empty(),
        "the engine names things it shouldn't:\n{}",
        found.join("\n")
    );
    assert!(words.contains("iron") && words.contains("copper") && words.contains("blade"));
    assert!(phrases.contains("fire ring"));
}

#[test]
fn heating_costs_fuel_and_a_chamber_goes_out_when_it_runs_out() {
    let mut w = load_world(IRON).unwrap();
    let hearth = id(&w, "hearth");
    ok(&mut w, "take pick");
    ok(&mut w, "dig vein with pick");
    ok(&mut w, "go forge");
    ok(&mut w, "put lump in hearth");
    assert_eq!(
        refused(&mut w, "light hearth"),
        Refusal::NoFuel("the stone hearth".into())
    );
    ok(&mut w, "take charcoal-small");
    ok(&mut w, "put charcoal-small in hearth");

    // Only what's in the hearth, and the smoke it makes: the forge's campfire
    // is burning too, but it isn't this test's fuel.
    let chemical = |w: &World| -> u128 {
        w.contents(hearth)
            .into_iter()
            .filter_map(|e| w.composition(e))
            .map(|c| matter::chemical_energy(w.materials(), c))
            .sum()
    };
    let (chemical_before, total_before) = (chemical(&w), w.total_energy());

    ok(&mut w, "light hearth");
    let peak = std::cell::Cell::new(Temperature::from_mk(0));
    let ore = w
        .contents(hearth)
        .into_iter()
        .find(|&e| w.label(e).contains("rock"))
        .unwrap();
    let burned_out = wait_until(&mut w, 600, |w| {
        peak.set(peak.get().max(w.temperature(ore).unwrap()));
        !w.chamber(hearth).unwrap().lit
    })
    .expect("the hearth goes out when its fuel is gone");
    assert!(
        burned_out >= 50,
        "100 g at 2 g a second burns for 50 seconds, not {burned_out}"
    );

    // All 100 g burned: 3 MJ of chemical energy became exactly 3 MJ of heat.
    let released = 100_000u128 * 30_000_000;
    assert_eq!(chemical_before - chemical(&w), released);
    assert_eq!(
        w.total_energy(),
        total_before,
        "and every joule of it became heat somewhere"
    );

    // That wasn't enough to melt anything, and it's cooling now.
    let iron = w.material_by_key("iron").unwrap();
    assert!(
        peak.get() < w.materials()[&iron].melting_point,
        "peaked at {}",
        peak.get()
    );
    assert!(pure_piece(&w, hearth, "iron", State::Liquid).is_none());
    nature::run(&mut w, 300).unwrap();
    assert!(w.temperature(ore).unwrap() < peak.get());
}

#[test]
fn stone_cant_shape_cold_hardened_steel_but_can_shape_it_hot() {
    let mut w = load_world(IRON).unwrap();
    let bar = id(&w, "bar");
    ok(&mut w, "go forge");
    ok(&mut w, "take hammer");
    assert_eq!(
        refused(&mut w, "work bar into blade with hammer"),
        Refusal::TooHard {
            tool: "the stone hammer".into(),
            target: "the lump of hardened steel".into()
        }
    );

    // Heat the bar in the hearth. Hardness falls with temperature, so hot
    // steel becomes softer than stone: forging, from one law.
    ok(&mut w, "take bar");
    ok(&mut w, "put bar in hearth");
    ok(&mut w, "take charcoal-1");
    ok(&mut w, "put charcoal-1 in hearth");
    ok(&mut w, "light hearth");
    wait_until(&mut w, 600, |w| {
        w.temperature(bar).unwrap() > Temperature::from_mk(900_000)
    })
    .expect("the bar heats up");
    ok(&mut w, "work bar into blade with hammer");
    assert_eq!(w.shape(bar), Some("blade"));
    assert_eq!(w.label(bar), "hardened steel blade");
}

#[test]
fn digging_needs_a_tool_harder_than_the_ground() {
    let mut w = load_world(IRON).unwrap();
    ok(&mut w, "go forge");
    ok(&mut w, "take charcoal-1");
    ok(&mut w, "go hillside");
    assert_eq!(
        refused(&mut w, "dig vein with charcoal-1"),
        Refusal::TooHard {
            tool: "the lump of charcoal".into(),
            target: "the vein of ore".into()
        }
    );
    assert_eq!(
        refused(&mut w, "dig vein with pick"),
        Refusal::NotCarrying("pick".into())
    );
    ok(&mut w, "take pick");
    let before = w.mass(id(&w, "vein"));
    ok(&mut w, "dig vein with pick");
    assert_eq!(
        before.mg() - w.mass(id(&w, "vein")).mg(),
        5_000_000,
        "one dig takes 5 kg"
    );
}

#[test]
fn molten_things_cant_be_held_and_forms_cant_hold_what_would_melt_them() {
    let mut w = load_world(IRON).unwrap();
    let hearth = id(&w, "hearth");
    ok(&mut w, "take pick");
    ok(&mut w, "dig vein with pick");
    ok(&mut w, "go forge");
    ok(&mut w, "put lump in hearth");
    ok(&mut w, "take charcoal-1");
    ok(&mut w, "put charcoal-1 in hearth");
    ok(&mut w, "light hearth");
    wait_until(&mut w, 1_200, |w| {
        pure_piece(w, hearth, "iron", State::Liquid).is_some()
    })
    .unwrap();
    let molten = pure_piece(&w, hearth, "iron", State::Liquid).unwrap();
    let key = w.key(molten).to_string();
    assert_eq!(
        refused(&mut w, &format!("take {key} from hearth")),
        Refusal::NotSolid("the molten iron".into())
    );
    assert_eq!(
        refused(&mut w, "pour rock into mould"),
        Refusal::NotLiquid("the lump of rock".into())
    );
    // The stone hammer isn't a container.
    assert_eq!(
        refused(&mut w, &format!("pour {key} into hammer")),
        Refusal::NotAContainer("the stone hammer".into())
    );
}

/// Raw random numbers for one step, turned into a command once the world's
/// current names are known.
#[derive(Clone, Debug)]
struct Step {
    verb: u8,
    a: usize,
    b: usize,
    c: usize,
    amount: u64,
}

fn step() -> impl Strategy<Value = Step> {
    (
        0..14u8,
        any::<usize>(),
        any::<usize>(),
        any::<usize>(),
        0..90u64,
    )
        .prop_map(|(verb, a, b, c, amount)| Step {
            verb,
            a,
            b,
            c,
            amount,
        })
}

fn names(world: &World) -> Vec<String> {
    let mut names: Vec<String> = world
        .entities()
        .flat_map(|e| [world.key(e).to_string(), world.label(e)])
        .collect();
    names.extend(world.shapes().keys().cloned());
    names.extend(["lump", "molten", "smoke", "nothing", "#3", "#40"].map(String::from));
    names
}

/// Returns the step as an intent, or as a number of seconds to wait.
fn command(world: &World, step: &Step) -> Result<Intent, u64> {
    let names = names(world);
    let pick = |n: usize| names[n % names.len()].clone();
    let (a, b, c) = (pick(step.a), pick(step.b), pick(step.c));
    Ok(match step.verb {
        0 => Intent::Go {
            place: a,
            aboard: None,
        },
        1 => Intent::Take { item: a },
        2 => Intent::TakeFrom { item: a, from: b },
        3 => Intent::Drop { item: a },
        4 => Intent::Put { item: a, into: b },
        5 => Intent::Give { item: a, to: b },
        6 => Intent::Pay {
            to: a,
            amount: Credits::new(step.amount),
        },
        7 => Intent::Dig { source: a, tool: b },
        8 => Intent::Light { chamber: a },
        9 => Intent::Pour { liquid: a, into: b },
        10 => Intent::Work {
            item: a,
            shape: b,
            tool: Some(c),
        },
        _ => return Err(step.amount + 1),
    })
}

/// A forge already at work: ore and charcoal burning in a lit hearth, so
/// random steps meet fire, melting, and pouring, not just an idle world.
fn busy_forge() -> World {
    let mut world = load_world(IRON).unwrap();
    for line in [
        "take pick",
        "dig vein with pick",
        "go forge",
        "put lump in hearth",
        "take charcoal-1",
        "put charcoal-1 in hearth",
        "take charcoal-2",
        "light hearth",
    ] {
        ok(&mut world, line);
    }
    world
}

fn run_random(steps: &[Step]) -> Result<World, TestCaseError> {
    let mut world = busy_forge();
    let start = totals(&world);
    let actor = traveller(&world);
    for step in steps {
        match command(&world, step) {
            Ok(intent) => {
                let before = world.clone();
                match act(&mut world, actor, intent.clone()) {
                    Ok(_) => {}
                    Err(ActError::Refused(_)) => {
                        prop_assert_eq!(&world, &before, "a refused {} changed the world", intent)
                    }
                    Err(ActError::Fault(fault)) => prop_assert!(
                        false,
                        "the laws allowed {} but the gate caught: {}",
                        intent,
                        fault
                    ),
                }
            }
            Err(seconds) => {
                if let Err(fault) = nature::run(&mut world, seconds) {
                    prop_assert!(false, "nature faulted: {}", fault);
                }
            }
        }
        prop_assert_eq!(totals(&world), start);
        prop_assert_eq!(world.check_invariants(), Ok(()));
        // Nothing here started colder than its surroundings, and heat only
        // leaks outwards, so nothing may cool below them (allowing 1 K for
        // rounding).
        for piece in world.entities().filter(|&e| world.composition(e).is_some()) {
            let ambient = world.ambient(world.place_of(piece).unwrap());
            let temperature = world.temperature(piece).unwrap();
            prop_assert!(
                temperature.mk() + 1_000 >= ambient.mk(),
                "{} cooled to {}",
                world.label(piece),
                temperature
            );
        }
    }
    Ok(world)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn random_actions_and_time_never_create_or_destroy_anything(steps in prop::collection::vec(step(), 1..150)) {
        run_random(&steps)?;
    }

    #[test]
    fn the_same_actions_and_time_always_make_the_same_world(steps in prop::collection::vec(step(), 1..80)) {
        let first = run_random(&steps)?;
        let second = run_random(&steps)?;
        prop_assert_eq!(first, second);
    }
}
