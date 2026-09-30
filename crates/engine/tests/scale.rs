//! Room to grow: a village's worth of bodies for a month. Reports how fast
//! the world runs and how much it keeps, so we can watch the headroom stage
//! by stage. See docs/ideas/the-road-ahead.md.
//!
//! On demand, built with full optimisation:
//! `cargo test --release -p engine --test scale -- --ignored --nocapture`

use std::time::Instant;

use engine::data::load_world_with;
use engine::world::{LOG_WINDOW, Luck};

const LIVING: &str = include_str!("../../../data/living.toml");
const THINGS: &str = include_str!("../../../data/island-things.toml");

const DAY: u64 = 86_400;

/// The island with `people` more people, living by players' rules so they
/// need no minds to stay alive, and `boars` boars, stepping at most
/// `calm_step` at a time.
fn crowded(people: u32, boars: u32, calm_step: &str) -> engine::world::World {
    let text = LIVING
        .replace("count = 6\n", &format!("count = {boars}\n"))
        .replace(
            "calm_step = \"1 min\"",
            &format!("calm_step = \"{calm_step}\""),
        )
        + &format!(
            r#"
[[agent]]
id = "villager"
label = "a villager"
at = "beach"
kind = "human"
count = {people}
temperature = "310 K"
rules = "player"
"#
        );
    load_world_with(&text, &[THINGS])
        .unwrap()
        .with_luck(Luck::AVERAGE)
}

/// Peak memory of this process, in kB, where the system says.
fn peak_memory_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status
        .lines()
        .find_map(|l| l.strip_prefix("VmHWM:"))?
        .trim()
        .trim_end_matches("kB")
        .trim()
        .parse()
        .ok()
}

#[test]
#[ignore]
fn a_village_of_bodies_for_a_month() {
    month(crowded(60, 60, "1 min"), 30);
}

/// The same, as if something somewhere were always hot or busy (a fire, a
/// cooking pot), which today makes the whole world step each second.
#[test]
#[ignore]
fn a_busy_village_of_bodies_for_a_day() {
    month(crowded(60, 60, "1 s"), 1);
}

fn month(mut world: engine::world::World, days: u64) {
    let bodies = world.entities().filter(|&e| world.is_living(e)).count();
    let started = Instant::now();
    for day in 1..=days {
        engine::nature::run(&mut world, DAY).unwrap();
        // The log keeps a window, not a history.
        assert!(
            world.log().len() < 2 * LOG_WINDOW,
            "the log grew on day {day}"
        );
    }
    let seconds = started.elapsed().as_secs_f64();
    let faster = (days * DAY) as f64 / seconds;
    println!(
        "{bodies} living things, {days} days: {seconds:.1} s, {:.2} s a game day, \
         {faster:.0}x faster than real time; {} sets of changes, {} kept; peak memory {}",
        seconds / days as f64,
        world.logged(),
        world.log().len(),
        peak_memory_kb().map_or("unknown".into(), |kb| format!("{} MB", kb / 1_024)),
    );
    // Everyone is still alive: players' rules and the herd's instinct hold.
    let villagers_alive = world
        .entities()
        .filter(|&e| world.key(e).starts_with("villager") && world.is_living(e))
        .count();
    assert_eq!(villagers_alive, 60);
}
