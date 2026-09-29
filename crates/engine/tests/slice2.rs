//! Slice 2's tests (docs/slices.md): parts and datasheets. Two blades from
//! different ores get different datasheets. A composite is used as a part in
//! something larger without the engine looking inside it again. A design that
//! needs a tight fit fails with hand-filed parts, and works after enough slow
//! hand work, with no rule saying it needs a machine. And everything in every
//! world has a datasheet.

use engine::data::load_world;
use engine::datasheet::{Datasheet, Property, Value, measure, measure_assembly};
use engine::intent::{Command, Intent, parse};
use engine::laws::{ActError, Refusal, act, perform};
use engine::nature;
use engine::units::Credits;
use engine::world::{EntityId, World};
use proptest::prelude::*;

const WORKSHOP: &str = include_str!("../../../data/slice2.toml");
const WORLDS: [(&str, &str); 4] = [
    ("slice0", include_str!("../../../data/slice0.toml")),
    (
        "slice1-iron",
        include_str!("../../../data/slice1-iron.toml"),
    ),
    (
        "slice1-copper",
        include_str!("../../../data/slice1-copper.toml"),
    ),
    ("slice2", WORKSHOP),
];

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

/// Does `line` the way the console does, letting time pass for slow work.
fn ok(world: &mut World, line: &str) {
    let actor = traveller(world);
    perform(world, actor, intent(line)).unwrap_or_else(|e| panic!("{line:?} should work: {e}"));
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

/// The newest thing the traveller is carrying: what was just assembled.
fn newest(world: &World) -> EntityId {
    world.contents(traveller(world)).into_iter().max().unwrap()
}

fn glows(sheet: &Datasheet) -> bool {
    sheet.get(Property::Glows) == Some(&Value::Flag(true))
}

fn totals(world: &World) -> (u128, u128, u128) {
    (
        world.total_mass(),
        world.total_energy(),
        world.total_credits(),
    )
}

#[test]
fn two_blades_from_different_ores_get_different_datasheets() {
    let mut w = load_world(WORKSHOP).unwrap();
    ok(&mut w, "take hammer");
    ok(&mut w, "take iron-stock");
    ok(&mut w, "take copper-stock");
    ok(&mut w, "work iron-stock into blade with hammer");
    ok(&mut w, "work copper-stock into blade with hammer");
    let (iron, copper) = (
        measure(&w, id(&w, "iron-stock")),
        measure(&w, id(&w, "copper-stock")),
    );

    assert_ne!(iron, copper);
    assert_eq!(
        iron.get(Property::EdgeHardness),
        Some(&Value::Hardness(400))
    );
    assert_eq!(
        copper.get(Property::EdgeHardness),
        Some(&Value::Hardness(300))
    );
    assert_eq!(iron.made_of, vec![("iron".to_string(), 10_000)]);
    // Both were made with the same hammer, so both have its 2 mm edge.
    assert_eq!(iron.get(Property::EdgeWidth), Some(&Value::Length(2_000)));
    assert_eq!(copper.get(Property::EdgeWidth), Some(&Value::Length(2_000)));
}

#[test]
fn a_part_is_at_most_as_precise_as_the_tool_that_made_it() {
    let mut w = load_world(WORKSHOP).unwrap();
    ok(&mut w, "take hammer");
    ok(&mut w, "take iron-stock");
    ok(&mut w, "take copper-stock");

    // A bare lump isn't a shaped tool, so what it makes is rough.
    ok(&mut w, "work copper-stock into blade with iron-stock");
    assert_eq!(
        w.tolerance(id(&w, "copper-stock")),
        Some(w.settings().rough_tolerance)
    );

    // The hammer is a 2 mm part, and passes that on.
    ok(&mut w, "work iron-stock into blade with hammer");
    assert_eq!(
        w.tolerance(id(&w, "iron-stock")),
        w.tolerance(id(&w, "hammer"))
    );

    // Reworking the copper blade with the 2 mm iron blade makes it 2 mm.
    ok(&mut w, "work copper-stock into blade with iron-stock");
    assert_eq!(w.tolerance(id(&w, "copper-stock")), Some(2_000));
}

#[test]
fn rubbing_makes_both_parts_finer_but_takes_time() {
    let mut w = load_world(WORKSHOP).unwrap();
    let (a, b) = (id(&w, "contact-1"), id(&w, "contact-2"));
    ok(&mut w, "take contact-1");
    ok(&mut w, "take contact-2");
    let start = (w.tick(), totals(&w));

    ok(&mut w, "rub contact-1 against contact-2");
    assert_eq!(w.tolerance(a), Some(1_600));
    assert_eq!(w.tolerance(b), Some(1_600));
    assert_eq!(
        w.tick(),
        start.0 + 600,
        "one session of rubbing takes ten minutes"
    );
    assert_eq!(totals(&w), start.1);

    // Keep going and it bottoms out at the finest hand work can reach.
    for _ in 0..60 {
        let actor = traveller(&w);
        if act(&mut w, actor, intent("rub contact-1 against contact-2")).is_err() {
            break;
        }
    }
    assert_eq!(w.tolerance(a), Some(w.settings().finest_tolerance));
    assert_eq!(
        refused(&mut w, "rub contact-1 against contact-2"),
        Refusal::AsFineAsItGets("the iron contact plate".into())
    );
    assert_eq!(
        refused(&mut w, "rub contact-1 against contact-1"),
        Refusal::NotYourself
    );
}

/// Builds the lamp from carried parts, rubbing the contact plates `rubs`
/// times first. Returns the world and the lamp.
fn lamp_after(rubs: usize) -> (World, EntityId) {
    let mut w = load_world(WORKSHOP).unwrap();
    for part in ["contact-1", "contact-2", "cell", "wire", "filament"] {
        ok(&mut w, &format!("take {part}"));
    }
    for _ in 0..rubs {
        ok(&mut w, "rub contact-1 against contact-2");
    }
    ok(&mut w, "assemble switch");
    ok(&mut w, "assemble lamp");
    let lamp = newest(&w);
    (w, lamp)
}

#[test]
fn a_lamp_with_hand_filed_contacts_doesnt_light_until_theyre_rubbed_fine() {
    // Hand-filed to 2 mm, the plates barely touch: 4 Ω where they meet, and
    // the filament only reaches 421 K.
    let (w, lamp) = lamp_after(0);
    let sheet = measure(&w, lamp);
    assert!(!glows(&sheet), "{sheet:?}");
    assert_eq!(
        sheet
            .get(Property::GlowTemperature)
            .map(ToString::to_string),
        Some("421 K".into())
    );

    // Nine sessions of rubbing isn't quite enough; ten is.
    let (w, lamp) = lamp_after(9);
    assert!(!glows(&measure(&w, lamp)));
    let (w, lamp) = lamp_after(10);
    let sheet = measure(&w, lamp);
    assert!(glows(&sheet), "{sheet:?}");
    assert_eq!(w.tick(), 10 * 600, "an hour and forty minutes of hand work");
    assert_eq!(
        sheet.get(Property::Runtime).map(ToString::to_string),
        Some("3 h 11 min".into())
    );
}

#[test]
fn a_composite_is_used_by_its_datasheet_without_looking_inside() {
    let mut w = load_world(WORKSHOP).unwrap();
    for part in ["contact-1", "contact-2", "cell", "wire", "filament"] {
        ok(&mut w, &format!("take {part}"));
    }
    let sheets = |w: &World, keys: &[&str]| -> Vec<Datasheet> {
        keys.iter().map(|k| measure(w, id(w, k))).collect()
    };
    let parts = sheets(&w, &["cell", "wire", "filament"]);
    ok(&mut w, "assemble switch");
    let switch = newest(&w);
    let switch_sheet = measure(&w, switch);
    ok(&mut w, "assemble lamp");
    let lamp = newest(&w);

    // The lamp's datasheet is exactly what its parts' datasheets make,
    // including the switch's stored one.
    let rebuilt = measure_assembly(
        w.settings(),
        &[
            ("lead".into(), "copper wire".into(), parts[1].clone()),
            ("light".into(), "tungsten filament".into(), parts[2].clone()),
            ("power".into(), "dry cell".into(), parts[0].clone()),
            ("switch".into(), "switch".into(), switch_sheet.clone()),
        ],
    );
    assert_eq!(measure(&w, lamp), rebuilt);
    assert_eq!(
        measure(&w, switch),
        switch_sheet,
        "the switch's datasheet didn't change by being used"
    );

    // Hand the lamp law a made-up switch that's only a datasheet, with no
    // plates behind it at all. The law can't tell: it never looks inside.
    let mut ideal = Datasheet::default();
    ideal
        .entries
        .insert(Property::Resistance, Value::Resistance(Some(10_000)));
    let with_ideal = measure_assembly(
        w.settings(),
        &[
            ("lead".into(), "copper wire".into(), parts[1].clone()),
            ("light".into(), "tungsten filament".into(), parts[2].clone()),
            ("power".into(), "dry cell".into(), parts[0].clone()),
            ("switch".into(), "switch".into(), ideal),
        ],
    );
    assert!(glows(&with_ideal));
}

#[test]
fn assembling_needs_every_part_and_taking_apart_gives_them_back() {
    let mut w = load_world(WORKSHOP).unwrap();
    let start = totals(&w);
    for part in ["cell", "wire", "filament"] {
        ok(&mut w, &format!("take {part}"));
    }
    assert_eq!(
        refused(&mut w, "assemble lamp"),
        Refusal::MissingPart {
            slot: "switch".into(),
            needs: "switch".into()
        }
    );
    assert_eq!(
        refused(&mut w, "assemble telescope"),
        Refusal::UnknownDesign("telescope".into())
    );
    assert_eq!(
        refused(&mut w, "disassemble wire"),
        Refusal::NotAnAssembly("the copper wire".into())
    );

    ok(&mut w, "take contact-1");
    ok(&mut w, "take contact-2");
    ok(&mut w, "assemble switch");
    ok(&mut w, "assemble lamp");
    let lamp = newest(&w);
    assert_eq!(
        w.contents(traveller(&w)),
        vec![lamp],
        "every part went into the lamp"
    );
    assert_eq!(w.mass(lamp).mg(), 140_005, "20 g + 20 g + 5 mg + 2 × 50 g");
    assert_eq!(totals(&w), start);

    ok(&mut w, "disassemble lamp");
    ok(&mut w, "disassemble switch");
    assert_eq!(w.contents(traveller(&w)).len(), 5);
    assert!(w.assembly(lamp).is_none());
    assert_eq!(totals(&w), start);
}

#[test]
fn everything_in_every_world_has_a_datasheet() {
    for (name, text) in WORLDS {
        let w = load_world(text).unwrap();
        for e in w.entities() {
            let sheet = measure(&w, e);
            assert!(
                !sheet.entries.is_empty(),
                "{name}: {} has an empty datasheet",
                w.key(e)
            );
            if w.composition(e).is_some() {
                assert!(
                    !sheet.made_of.is_empty(),
                    "{name}: {} doesn't say what it's made of",
                    w.key(e)
                );
            }
        }
    }
}

#[test]
fn every_item_in_the_iron_chain_has_a_datasheet_that_says_what_its_made_of() {
    let mut w = load_world(WORLDS[1].1).unwrap();
    for line in [
        "take pick",
        "dig vein with pick",
        "go forge",
        "put lump in hearth",
        "take charcoal-1",
        "put charcoal-1 in hearth",
        "light hearth",
    ] {
        ok(&mut w, line);
    }
    nature::run(&mut w, 900).unwrap();
    // Every item, including the slag, ash, and smoke the chain left behind,
    // is made of something and says so.
    for e in w.entities().filter(|&e| !w.is_place(e) && !w.is_agent(e)) {
        let sheet = measure(&w, e);
        assert!(
            !sheet.made_of.is_empty(),
            "{} ({}) doesn't say what it's made of",
            w.label(e),
            w.key(e)
        );
        assert!(sheet.get(Property::Temperature).is_some());
    }
}

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
        0..12u8,
        any::<usize>(),
        any::<usize>(),
        any::<usize>(),
        0..30u64,
    )
        .prop_map(|(verb, a, b, c, amount)| Step {
            verb,
            a,
            b,
            c,
            amount,
        })
}

fn command(world: &World, step: &Step) -> Result<Intent, u64> {
    let mut names: Vec<String> = world
        .entities()
        .flat_map(|e| [world.key(e).to_string(), world.label(e)])
        .collect();
    names.extend(world.shapes().keys().cloned());
    names.extend(world.designs().keys().cloned());
    names.extend(["lump", "plate", "nothing"].map(String::from));
    let pick = |n: usize| names[n % names.len()].clone();
    let (a, b, c) = (pick(step.a), pick(step.b), pick(step.c));
    Ok(match step.verb {
        0 => Intent::Take { item: a },
        1 => Intent::Drop { item: a },
        2 => Intent::Give { item: a, to: b },
        3 => Intent::Pay {
            to: a,
            amount: Credits::new(step.amount),
        },
        4 => Intent::Work {
            item: a,
            shape: b,
            tool: c,
        },
        5 => Intent::Rub {
            item: a,
            against: b,
            into: None,
            seconds: None,
        },
        6 | 7 => Intent::Assemble { design: a },
        8 => Intent::Disassemble { item: a },
        9 => Intent::Put { item: a, into: b },
        _ => return Err(step.amount),
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn random_building_never_creates_or_destroys_anything(steps in prop::collection::vec(step(), 1..120)) {
        let mut world = load_world(WORKSHOP).unwrap();
        let start = totals(&world);
        let actor = traveller(&world);
        for step in &steps {
            match command(&world, step) {
                // `act`, not `perform`, keeps rubbing instant so the test runs fast.
                Ok(intent) => {
                    let before = world.clone();
                    match act(&mut world, actor, intent.clone()) {
                        Ok(_) => {}
                        Err(ActError::Refused(_)) => prop_assert_eq!(&world, &before, "a refused {} changed the world", intent),
                        Err(ActError::Fault(fault)) => prop_assert!(false, "the laws allowed {} but the gate caught: {}", intent, fault),
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
            // Every assembly's stored datasheet matches the mass of what's in it.
            for e in world.entities().filter(|&e| world.assembly(e).is_some()) {
                let sheet = measure(&world, e);
                prop_assert_eq!(sheet.get(Property::Mass), Some(&Value::Mass(world.mass(e))));
            }
        }
    }
}
