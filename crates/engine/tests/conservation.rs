//! Slice 0's test (docs/slices.md): run thousands of random commands. Total
//! mass and total credits never change, a refused command changes nothing,
//! the gate never has to catch a law's mistake, and the same commands always
//! produce the same world.

use engine::data::load_world;
use engine::intent::Intent;
use engine::laws::{ActError, act};
use engine::units::Credits;
use engine::world::{EntityId, World};
use proptest::prelude::*;

const WORLD: &str = include_str!("../../../data/slice0.toml");

/// Raw random numbers for one command. They're turned into an actor and an
/// intent once the world's names are known.
#[derive(Clone, Debug)]
struct Step {
    actor: usize,
    verb: u8,
    first: usize,
    second: usize,
    amount: u64,
}

fn step() -> impl Strategy<Value = Step> {
    let amount = prop_oneof![
        Just(0u64),
        1..100u64,
        100..1_000u64,
        Just(u64::MAX),
        any::<u64>()
    ];
    (
        any::<usize>(),
        0..5u8,
        any::<usize>(),
        any::<usize>(),
        amount,
    )
        .prop_map(|(actor, verb, first, second, amount)| Step {
            actor,
            verb,
            first,
            second,
            amount,
        })
}

/// Every name a command could use: real IDs and labels, spelling variants,
/// and names of things that don't exist.
fn name_pool(world: &World) -> Vec<String> {
    let mut names = Vec::new();
    for id in world.entities() {
        names.push(world.key(id).to_string());
        names.push(world.label(id).to_string());
        names.push(world.label(id).to_uppercase());
    }
    names.extend(
        [
            "nothing",
            "",
            "the",
            "unicorn",
            "a rope",
            "the traveller's rope",
        ]
        .map(String::from),
    );
    names
}

fn intent(step: &Step, names: &[String]) -> Intent {
    let first = names[step.first % names.len()].clone();
    let second = names[step.second % names.len()].clone();
    match step.verb {
        0 => Intent::Go { place: first },
        1 => Intent::Take { item: first },
        2 => Intent::Drop { item: first },
        3 => Intent::Give {
            item: first,
            to: second,
        },
        _ => Intent::Pay {
            to: first,
            amount: Credits::new(step.amount),
        },
    }
}

fn run(steps: &[Step]) -> Result<World, TestCaseError> {
    let mut world = load_world(WORLD).expect("slice 0 world loads");
    let mass = world.total_mass();
    let credits = world.total_credits();
    // Actors include things that aren't people, which must be refused.
    let actors: Vec<EntityId> = world.entities().collect();
    let names = name_pool(&world);

    for step in steps {
        let actor = actors[step.actor % actors.len()];
        let intent = intent(step, &names);
        let before = world.clone();
        match act(&mut world, actor, intent.clone()) {
            Ok(_) => {}
            Err(ActError::Refused(_)) => {
                prop_assert_eq!(&world, &before, "a refused {} changed the world", intent)
            }
            Err(ActError::Fault(fault)) => {
                prop_assert!(
                    false,
                    "the laws allowed {} but the gate caught: {}",
                    intent,
                    fault
                )
            }
        }
        prop_assert_eq!(world.total_mass(), mass, "mass changed after {}", intent);
        prop_assert_eq!(
            world.total_credits(),
            credits,
            "credits changed after {}",
            intent
        );
        prop_assert_eq!(world.check_invariants(), Ok(()));
    }
    Ok(world)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn random_commands_never_create_or_destroy_anything(steps in prop::collection::vec(step(), 1..400)) {
        run(&steps)?;
    }

    #[test]
    fn the_same_commands_always_make_the_same_world(steps in prop::collection::vec(step(), 1..200)) {
        let first = run(&steps)?;
        let second = run(&steps)?;
        prop_assert_eq!(first, second);
    }
}

/// A long, fixed run that is mostly valid moves, so the test exercises
/// successful actions as well as refusals.
#[test]
fn a_busy_afternoon_in_town_conserves_everything() {
    let mut world = load_world(WORLD).unwrap();
    let (mass, credits) = (world.total_mass(), world.total_credits());
    let people: Vec<EntityId> = world.entities().filter(|&e| world.is_agent(e)).collect();
    let names = name_pool(&world);

    // A small deterministic generator, so this test needs no randomness.
    let mut state: u64 = 0x5eed;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut accepted = 0;
    for _ in 0..20_000 {
        let step = Step {
            actor: next() as usize,
            verb: (next() % 5) as u8,
            first: next() as usize,
            second: next() as usize,
            amount: next() % 60,
        };
        let actor = people[step.actor % people.len()];
        if act(&mut world, actor, intent(&step, &names)).is_ok() {
            accepted += 1;
        }
        assert_eq!(world.total_mass(), mass);
        assert_eq!(world.total_credits(), credits);
    }
    assert_eq!(world.check_invariants(), Ok(()));
    assert!(
        accepted > 1_000,
        "only {accepted} actions were accepted; the test isn't exercising much"
    );
    assert_eq!(world.log().len(), accepted);
}
