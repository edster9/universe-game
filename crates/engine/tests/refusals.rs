//! Slice 0's cheat attempts (docs/slices.md): every one is refused, and
//! refusing changes nothing.

use engine::data::load_world;
use engine::gate::{Cause, Change, Fault};
use engine::intent::{Command, Intent, parse};
use engine::laws::{ActError, Refusal, act};
use engine::units::Credits;
use engine::world::{EntityId, World};

const WORLD: &str = include_str!("../../../data/slice0.toml");

fn world() -> World {
    load_world(WORLD).unwrap()
}

fn id(world: &World, key: &str) -> EntityId {
    world.find_by_key(key).unwrap()
}

fn intent(line: &str) -> Intent {
    match parse(line) {
        Ok(Command::Act(intent)) => intent,
        other => panic!("{line:?} should parse to an action, got {other:?}"),
    }
}

/// Runs `line` as `who` and expects `refusal`, with the world untouched.
fn refused(world: &mut World, who: &str, line: &str, refusal: Refusal) {
    let before = world.clone();
    let actor = id(world, who);
    assert_eq!(
        act(world, actor, intent(line)),
        Err(ActError::Refused(refusal)),
        "{line}"
    );
    assert_eq!(*world, before, "refusing {line:?} changed the world");
}

fn ok(world: &mut World, who: &str, line: &str) {
    let actor = id(world, who);
    act(world, actor, intent(line)).unwrap_or_else(|e| panic!("{line:?} should work: {e}"));
}

#[test]
fn you_cant_pay_what_you_dont_have() {
    let mut w = world();
    ok(&mut w, "traveller", "go market");
    refused(
        &mut w,
        "traveller",
        "pay oskar 51",
        Refusal::NotEnough {
            have: Credits::new(50),
            want: Credits::new(51),
        },
    );
    refused(
        &mut w,
        "traveller",
        &format!("pay oskar {}", u64::MAX),
        Refusal::NotEnough {
            have: Credits::new(50),
            want: Credits::new(u64::MAX),
        },
    );
    ok(&mut w, "traveller", "pay oskar 50");
    assert_eq!(w.wallet(id(&w, "traveller")), Some(Credits::ZERO));
    assert_eq!(w.wallet(id(&w, "oskar")), Some(Credits::new(550)));
}

#[test]
fn negative_and_zero_payments_are_refused() {
    assert!(parse("pay oskar -5").is_err());
    let mut w = world();
    ok(&mut w, "traveller", "go market");
    refused(&mut w, "traveller", "pay oskar 0", Refusal::ZeroAmount);
}

#[test]
fn you_cant_pay_or_give_to_yourself() {
    let mut w = world();
    ok(&mut w, "traveller", "take rope");
    refused(&mut w, "traveller", "pay traveller 5", Refusal::NotYourself);
    refused(
        &mut w,
        "traveller",
        "give rope to the traveller",
        Refusal::NotYourself,
    );
    refused(
        &mut w,
        "traveller",
        "take the traveller",
        Refusal::NotYourself,
    );
}

#[test]
fn you_cant_give_what_you_dont_have() {
    let mut w = world();
    ok(&mut w, "traveller", "go smithy");
    refused(
        &mut w,
        "traveller",
        "give hammer to mara",
        Refusal::NotCarrying("hammer".into()),
    );
    refused(
        &mut w,
        "traveller",
        "drop hammer",
        Refusal::NotCarrying("hammer".into()),
    );
}

#[test]
fn you_cant_take_what_you_already_carry() {
    let mut w = world();
    ok(&mut w, "traveller", "take rope");
    refused(
        &mut w,
        "traveller",
        "take rope",
        Refusal::AlreadyCarrying("rope".into()),
    );
}

#[test]
fn you_cant_take_people_fixed_things_or_what_others_hold() {
    let mut w = world();
    ok(&mut w, "traveller", "go smithy");
    refused(
        &mut w,
        "traveller",
        "take mara",
        Refusal::CannotCarry("Mara".into()),
    );
    refused(
        &mut w,
        "traveller",
        "take anvil",
        Refusal::CannotCarry("the anvil".into()),
    );
    // Mara is holding the hammer. Taking from people is stealing: slice 3.
    refused(
        &mut w,
        "traveller",
        "take hammer",
        Refusal::NotHere("hammer".into()),
    );
}

#[test]
fn you_can_only_reach_what_is_here() {
    let mut w = world();
    refused(
        &mut w,
        "traveller",
        "take grain",
        Refusal::NotHere("grain".into()),
    );
    refused(
        &mut w,
        "traveller",
        "pay oskar 5",
        Refusal::NoOneHere("oskar".into()),
    );
    refused(
        &mut w,
        "traveller",
        "go the moon",
        Refusal::NoSuchExit("the moon".into()),
    );
    ok(&mut w, "traveller", "go market");
    refused(
        &mut w,
        "traveller",
        "go smithy",
        Refusal::NoSuchExit("smithy".into()),
    );
}

#[test]
fn things_that_arent_people_cant_act() {
    let mut w = world();
    refused(&mut w, "rope", "go market", Refusal::NotAnAgent);
    refused(&mut w, "square", "take rope", Refusal::NotAnAgent);
}

#[test]
fn giving_moves_the_thing_and_its_mass() {
    let mut w = world();
    ok(&mut w, "traveller", "go smithy");
    ok(&mut w, "traveller", "take iron ingot");
    ok(&mut w, "traveller", "take iron ingot");
    refused(
        &mut w,
        "traveller",
        "take iron ingot",
        Refusal::AlreadyCarrying("iron ingot".into()),
    );
    ok(&mut w, "traveller", "give iron ingot to Mara");
    let carried: Vec<_> = w.contents(id(&w, "traveller"));
    assert_eq!(carried.len(), 1);
    assert_eq!(
        w.contents(id(&w, "mara")).len(),
        2,
        "Mara has the hammer and an ingot"
    );
}

/// The gate itself refuses bad changes even if a law were to propose them,
/// and leaves the world exactly as it was.
#[test]
fn the_gate_refuses_bad_changes_on_its_own() {
    let mut w = world();
    let before = w.clone();
    let (traveller, mara, rope, square, lantern) = (
        id(&w, "traveller"),
        id(&w, "mara"),
        id(&w, "rope"),
        id(&w, "square"),
        id(&w, "lantern"),
    );
    let cause = Cause::Action {
        actor: traveller,
        intent: intent("take rope"),
    };

    let cases = [
        (
            vec![Change::Transfer {
                from: traveller,
                to: traveller,
                amount: Credits::new(5),
            }],
            "same wallet",
        ),
        (
            vec![Change::Transfer {
                from: traveller,
                to: mara,
                amount: Credits::new(51),
            }],
            "overdraft",
        ),
        (
            vec![Change::Transfer {
                from: traveller,
                to: rope,
                amount: Credits::new(1),
            }],
            "no wallet",
        ),
        (
            vec![Change::Move {
                entity: rope,
                to: rope,
            }],
            "into itself",
        ),
        (
            vec![Change::Move {
                entity: square,
                to: traveller,
            }],
            "a place inside a person",
        ),
        (
            vec![Change::Move {
                entity: traveller,
                to: rope,
            }],
            "a person inside a rope",
        ),
        (
            vec![Change::Move {
                entity: rope,
                to: lantern,
            }],
            "a rope inside a lantern, which can't hold things",
        ),
        (
            // The first change is fine; the second fails. Neither may stick.
            vec![
                Change::Move {
                    entity: rope,
                    to: traveller,
                },
                Change::Transfer {
                    from: traveller,
                    to: mara,
                    amount: Credits::new(1_000),
                },
            ],
            "half good, half bad",
        ),
    ];
    for (changes, what) in cases {
        let result = w.apply(cause.clone(), changes);
        assert!(result.is_err(), "the gate allowed {what}");
        assert_eq!(
            w, before,
            "the gate left changes behind after refusing {what}"
        );
    }
    assert!(matches!(
        w.apply(
            cause,
            vec![Change::Move {
                entity: rope,
                to: rope
            }]
        ),
        Err(Fault::IntoItself(_))
    ));
}
