//! The "living with the island" challenge. See
//! docs/challenges/living-with-the-island.md.

use engine::data::load_world_with;
use engine::datasheet::{Property, Value, measure};
use engine::intent::{Command, parse};
use engine::laws::{act, perform};
use engine::world::{EntityId, World};

const LIVING: &str = include_str!("../../../data/living.toml");
const THINGS: &str = include_str!("../../../data/island-things.toml");

fn island() -> World {
    load_world_with(LIVING, &[THINGS])
        .unwrap()
        .with_luck(engine::world::Luck::AVERAGE)
}

fn run(world: &mut World, me: EntityId, line: &str) {
    match parse(line) {
        Ok(Command::Act(intent)) => {
            perform(world, me, intent).unwrap_or_else(|e| panic!("{line}: {e:?}"));
        }
        other => panic!("{line:?} isn't an action: {other:?}"),
    }
}

fn stored_energy(world: &World, me: EntityId) -> u64 {
    match measure(world, me).get(Property::StoredEnergy) {
        Some(Value::Energy(e)) => e.uj(),
        _ => 0,
    }
}

/// Builds a lean-to on the beach, then waits until evening.
fn evening_by_a_lean_to() -> (World, EntityId) {
    let mut w = island();
    let me = w.find_by_key("survivor").unwrap();
    run(&mut w, me, "go forest");
    for _ in 0..6 {
        run(&mut w, me, "gather sticks");
    }
    for _ in 0..4 {
        run(&mut w, me, "gather bushes");
    }
    run(&mut w, me, "go beach");
    run(&mut w, me, "assemble lean-to");
    match parse("drop lean-to") {
        Ok(Command::Act(intent)) => {
            act(&mut w, me, intent).unwrap();
        }
        _ => unreachable!(),
    }
    let until_evening = 12 * 3_600 - w.tick();
    engine::nature::run(&mut w, until_evening).unwrap();
    (w, me)
}

#[test]
fn a_night_in_a_shelter_costs_less_than_a_night_in_the_open() {
    let (mut open, me) = evening_by_a_lean_to();
    let mut sheltered = open.clone();
    let before = stored_energy(&open, me);
    run(&mut open, me, "sleep for 10 h");
    run(&mut sheltered, me, "sleep in lean-to for 10 h");
    let (open_mj, sheltered_mj) = (
        (before - stored_energy(&open, me)) / 1_000_000_000_000,
        (before - stored_energy(&sheltered, me)) / 1_000_000_000_000,
    );
    // At rest a body burns 80 W, 2.9 MJ in 10 hours. In the open it shivers
    // against the cool night air; in the lean-to it doesn't need to.
    assert_eq!(
        sheltered_mj, 2,
        "a sheltered night burned {sheltered_mj} MJ"
    );
    assert!(open_mj >= 4, "a night in the open burned only {open_mj} MJ");
}
