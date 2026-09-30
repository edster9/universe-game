//! The "a stranger's words" challenge: names live in minds, and things are
//! recognised by how they look. See docs/challenges/strangers-words.md and
//! docs/ideas/vocabulary.md.

use engine::data::load_world_with;
use engine::intent::{Command, parse};
use engine::laws::{perform, resolve};
use engine::units::Mass;
use engine::words::{Closeness, Look, Part};
use engine::world::{EntityId, Luck, World};

const WORDS: &str = include_str!("../../../data/strangers-words.toml");
const WHERE: &str = include_str!("../../../data/where-am-i.toml");
const THINGS: &str = include_str!("../../../data/island-things.toml");

fn island() -> (World, EntityId, EntityId) {
    let w = load_world_with(WORDS, &[THINGS])
        .unwrap()
        .with_luck(Luck::AVERAGE);
    let islander = w.find_by_key("islander").unwrap();
    let stranger = w.find_by_key("stranger").unwrap();
    (w, islander, stranger)
}

fn run(world: &mut World, me: EntityId, line: &str) {
    match parse(line) {
        Ok(Command::Act(intent)) => {
            perform(world, me, intent).unwrap_or_else(|e| panic!("{line}: {e}"));
        }
        other => panic!("{line:?} isn't an action: {other:?}"),
    }
}

fn refusal(world: &World, me: EntityId, line: &str) -> String {
    match parse(line) {
        Ok(Command::Act(intent)) => match resolve(world, me, &intent) {
            Err(refusal) => refusal.to_string(),
            Ok(_) => panic!("{line:?} was allowed"),
        },
        other => panic!("{line:?} isn't an action: {other:?}"),
    }
}

/// The islander makes a spear on the hillside and puts it down there.
fn spear_on_the_hillside() -> (World, EntityId, EntityId, EntityId) {
    let (mut w, islander, stranger) = island();
    for line in [
        "go forest",
        "gather sticks",
        "go hillside",
        "gather stones",
        "gather flint",
        "work flint into flake with stone",
        "work wood into shaft with flake",
        "assemble spear",
        "drop spear",
    ] {
        run(&mut w, islander, line);
    }
    let spear = w
        .contents(w.location(islander).unwrap())
        .into_iter()
        .find(|&e| w.assembly(e).is_some())
        .unwrap();
    (w, islander, stranger, spear)
}

fn look(mass: Option<u64>, parts: Vec<Part>) -> Look {
    Look {
        does: [engine::world::Role::Cutting].into_iter().collect(),
        parts,
        mass: mass.map(Mass::from_mg),
    }
}

fn shaped(shape: &str) -> Part {
    Part::Shaped {
        shape: shape.into(),
        material: None,
    }
}

#[test]
fn one_thing_two_names() {
    let (w, islander, stranger, spear) = spear_on_the_hillside();
    assert_eq!(w.label_for(islander, spear), "spear of flint and wood");
    assert_eq!(
        w.label_for(stranger, spear),
        "barb of glassy grey stone joined to long straight pole of wood"
    );
    // The world's own name for it is unchanged; only minds differ.
    assert_eq!(w.label(spear), "spear");
}

#[test]
fn a_world_without_cultures_names_things_as_before() {
    let w = load_world_with(WHERE, &[THINGS]).unwrap();
    let me = w.find_by_key("survivor").unwrap();
    assert!(!w.has_words(me));
    for thing in w.entities() {
        assert_eq!(w.label_for(me, thing), w.label(thing));
    }
}

#[test]
fn a_word_you_dont_know_finds_nothing_until_you_are_told() {
    let (mut w, islander, stranger, spear) = spear_on_the_hillside();
    assert_eq!(
        refusal(&w, stranger, "take the spear"),
        "you don't see the spear here"
    );
    run(
        &mut w,
        islander,
        "tell the stranger that the spear is a spear",
    );
    assert_eq!(
        w.label_for(stranger, spear),
        "spear of glassy grey stone and wood"
    );
    run(&mut w, stranger, "take the spear");
    assert_eq!(w.location(spear), Some(stranger));
}

#[test]
fn being_told_about_one_teaches_the_next() {
    let (mut w, islander, stranger, _) = spear_on_the_hillside();
    run(
        &mut w,
        islander,
        "tell the stranger that the spear is a spear",
    );
    // A second spear, made the same way, which the stranger has never been
    // shown.
    for line in [
        "go forest",
        "gather sticks",
        "go hillside",
        "gather flint",
        "work flint into flake with stone",
        "work wood into shaft with flake",
        "assemble spear",
    ] {
        run(&mut w, islander, line);
    }
    let second = w
        .contents(islander)
        .into_iter()
        .find(|&e| w.assembly(e).is_some())
        .unwrap();
    assert_eq!(
        w.label_for(stranger, second),
        "spear of glassy grey stone and wood"
    );
}

#[test]
fn a_creature_you_dont_know_is_seen_by_how_it_looks() {
    let (mut w, islander, stranger) = island();
    let boar = w.find_by_key("boar").unwrap();
    assert_eq!(w.label_for(islander, boar), "a boar");
    assert_eq!(w.label_for(stranger, boar), "a large bristly beast");
    run(&mut w, islander, "go forest");
    run(&mut w, islander, "go hillside");
    if w.location(boar) == w.location(islander) {
        run(
            &mut w,
            islander,
            "tell the stranger that the boar is a boar",
        );
        assert_eq!(w.label_for(stranger, boar), "a boar");
    }
}

#[test]
fn something_new_asks_for_a_name_and_the_name_keeps_the_recipe() {
    let (mut w, _, stranger) = island();
    run(&mut w, stranger, "go forest");
    run(&mut w, stranger, "gather sticks");
    run(&mut w, stranger, "join barb and wood");
    let made = w.lexicon(stranger).unwrap().last_made.unwrap();
    assert_eq!(w.recognise(stranger, made), None);
    run(&mut w, stranger, "call it a stabber");
    assert_eq!(
        w.recognise(stranger, made),
        Some(("stabber".into(), Closeness::Exact))
    );
    assert_eq!(w.recipes_for(stranger, "stabber").len(), 1);
    // Taken apart and made again from the recipe, it's a stabber again.
    run(&mut w, stranger, "disassemble stabber");
    run(&mut w, stranger, "make a stabber");
    let again = w.lexicon(stranger).unwrap().last_made.unwrap();
    assert_ne!(again, made);
    assert_eq!(w.label_for(stranger, again), "stabber of iron and wood");
}

#[test]
fn the_same_word_for_two_things_that_look_different_asks_which() {
    let (mut w, islander, _, _) = spear_on_the_hillside();
    run(&mut w, islander, "take the spear");
    // A second spear, headed with another material's chip.
    let stranger = w.find_by_key("stranger").unwrap();
    let barb = w.find_by_key("barb").unwrap();
    run(&mut w, stranger, "drop the barb");
    for line in [
        "take the flake",
        "go forest",
        "gather sticks",
        "go hillside",
        "work wood into shaft with stone",
        "assemble spear",
    ] {
        run(&mut w, islander, line);
    }
    assert_eq!(w.location(w.location(barb).unwrap()), Some(islander));
    assert_eq!(
        refusal(&w, islander, "drop the spear"),
        "which spear: the spear of flint and wood, or the spear of dark metal and wood?"
    );
    run(&mut w, islander, "drop the metal one");
    assert_eq!(w.location(w.location(barb).unwrap()), w.location(islander));
}

#[test]
fn resemblance_needs_the_same_parts_or_the_same_jobs_at_about_the_same_size() {
    let example = look(Some(700_000), vec![shaped("flake"), shaped("shaft")]);
    // The same parts: exact when the example leaves materials open.
    let same = look(Some(600_000), vec![shaped("flake"), shaped("shaft")]);
    assert_eq!(example.resembles(&same), Some(Closeness::Exact));
    // Too big or too small, whatever its parts.
    let big = look(Some(1_500_000), vec![shaped("flake"), shaped("shaft")]);
    assert_eq!(example.resembles(&big), None);
    let small = look(Some(300_000), vec![shaped("flake"), shaped("shaft")]);
    assert_eq!(example.resembles(&small), None);
    // Different parts doing the same job: only like it.
    let other = look(Some(700_000), vec![shaped("haft")]);
    assert_eq!(example.resembles(&other), Some(Closeness::Like));
    // An example from a design has no size, and stands only for the same
    // parts.
    let from_design = look(None, vec![shaped("flake"), shaped("shaft")]);
    assert_eq!(from_design.resembles(&big), Some(Closeness::Exact));
    assert_eq!(from_design.resembles(&other), None);
}
