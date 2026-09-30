//! The "a companion" challenge: the stranger lives on the island by a mind of
//! their own. See docs/challenges/a-companion.md and docs/ideas/npc-minds.md.

use engine::data::load_world_with;
use engine::world::{EntityId, Luck, World};

const COMPANION: &str = include_str!("../../../data/companion.toml");
const THINGS: &str = include_str!("../../../data/island-things.toml");
const FAR_FOLK: &str = include_str!("../../../data/far-folk.toml");

const DAY: u64 = 86_400;

/// The companion's island, with `edit` made to its text, and the islander
/// standing by under players' rules.
fn island(edit: impl Fn(String) -> String) -> (World, EntityId) {
    let world = load_world_with(&edit(COMPANION.to_string()), &[THINGS, FAR_FOLK])
        .unwrap()
        .with_luck(Luck::AVERAGE)
        .with_player_rules("survivor")
        .unwrap();
    let stranger = world.find_by_key("stranger").unwrap();
    (world, stranger)
}

/// Their orders send them for water somewhere that isn't there, as a
/// shopkeeper's orders send them to a shop that has burned down.
fn no_spring(scope: &'static str) -> impl Fn(String) -> String {
    move |text: String| {
        text.replace("mind = \"resident\"", &format!("mind = \"{scope}\""))
            .replace(
                "    \"thirsty, at the stream: drink water\",\n    \"thirsty: go stream\",\n",
                "    \"thirsty: go spring\",\n",
            )
    }
}

#[test]
fn the_stranger_looks_after_themselves() {
    let (mut w, stranger) = island(|t| t);
    let (mass, own) = (w.mass(stranger), w.own_mass());
    engine::nature::run(&mut w, 10 * DAY).unwrap();
    assert!(w.is_living(stranger), "{:?}", w.life(stranger));
    // Fed and watered: within a few kilos of where they started.
    let now = w.mass(stranger).mg();
    assert!(now.abs_diff(mass.mg()) < 3_000_000, "{mass:?} to {now}");
    assert_eq!(w.own_mass(), own);
}

#[test]
#[ignore = "a trial: run with `cargo test -- --ignored --nocapture`"]
fn trial_the_stranger_looks_after_themselves_for_a_month() {
    let started = std::time::Instant::now();
    let mut alive = 0;
    let mut deaths = Vec::new();
    for seed in 1..=10 {
        let (w, stranger) = island(|t| t);
        let mut w = w.with_seed(seed);
        let mass = w.mass(stranger).mg();
        engine::nature::run(&mut w, 30 * DAY).unwrap();
        match w.life(stranger).and_then(|l| l.died_of.clone()) {
            None => {
                alive += 1;
                println!(
                    "seed {seed}: alive, {:.1} kg (from {:.1})",
                    w.mass(stranger).mg() as f64 / 1e6,
                    mass as f64 / 1e6
                );
            }
            Some(cause) => deaths.push(format!("seed {seed}: {cause}")),
        }
    }
    println!(
        "a companion 1: {alive} of 10 strangers alive after 30 days; deaths: {deaths:?}; took {:.1}s",
        started.elapsed().as_secs_f64()
    );
    assert_eq!(alive, 10);
}

#[test]
fn a_confined_mind_lives_by_its_orders() {
    let (mut w, stranger) = island(|t| t.replace("mind = \"resident\"", "mind = \"confined\""));
    let beach = w.find_by_key("beach").unwrap();
    engine::nature::run(&mut w, 6 * DAY).unwrap();
    assert!(w.is_living(stranger), "{:?}", w.life(stranger));
    // By night they've gone back to the beach, as their orders say.
    let evening = 24 * 3_600 - 8 * 3_600 + 22 * 3_600;
    engine::nature::run(&mut w, evening - (6 * DAY) % DAY).unwrap();
    assert!(w.is_night());
    assert_eq!(w.location(stranger), Some(beach));
}

#[test]
fn a_confined_mind_waits_where_its_orders_fail_and_dies_of_thirst() {
    let (mut w, stranger) = island(no_spring("confined"));
    let beach = w.find_by_key("beach").unwrap();
    engine::nature::run(&mut w, 6 * DAY).unwrap();
    let life = w.life(stranger).unwrap();
    assert_eq!(life.died_of.as_deref(), Some("thirst"), "{life:?}");
    assert_eq!(w.location(stranger), Some(beach));
}

#[test]
fn a_resident_mind_finds_water_it_remembers_when_its_orders_fail() {
    let (mut w, stranger) = island(no_spring("resident"));
    engine::nature::run(&mut w, 6 * DAY).unwrap();
    assert!(w.is_living(stranger), "{:?}", w.life(stranger));
}

#[test]
fn orders_must_make_sense() {
    let bad = |order: &str| {
        let text = COMPANION.replace(
            "    \"thirsty: go stream\",\n",
            &format!("    \"thirsty: go stream\",\n    \"{order}\",\n"),
        );
        load_world_with(&text, &[THINGS, FAR_FOLK])
            .unwrap_err()
            .to_string()
    };
    assert!(bad("go stream").contains("colon"));
    assert!(bad("sleepy: sleep").contains("isn't something a person can tell"));
    assert!(bad("thirsty: dance").contains("doesn't end with a command"));
}
