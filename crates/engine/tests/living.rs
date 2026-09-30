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

fn boars(world: &World) -> Vec<EntityId> {
    (1..=6)
        .map(|n| world.find_by_key(&format!("boar-{n}")).unwrap())
        .collect()
}

fn material(world: &World, id: EntityId, key: &str) -> u64 {
    let m = world.material_by_key(key).unwrap();
    world
        .composition(id)
        .and_then(|c| c.get(&m))
        .map_or(0, |mass| mass.mg())
}

#[test]
#[ignore = "a trial: run with `cargo test -- --ignored --nocapture`"]
fn trial_a_herd_of_boars_looks_after_itself_for_a_month() {
    let started = std::time::Instant::now();
    let mut alive_total = 0;
    for seed in 1..=10 {
        let mut w = island().with_seed(seed);
        let start = (w.total_mass(), w.total_energy() - w.sunlight());
        for day in 1..=30 {
            engine::nature::run(&mut w, 86_400).unwrap();
            if seed == 1 && (day == 1 || day % 10 == 0) {
                let report: Vec<String> = boars(&w)
                    .iter()
                    .map(|&b| {
                        format!(
                            "{} fat {:.1} water {:.1}{}",
                            w.key(w.place_of(b).unwrap()),
                            material(&w, b, "fat") as f64 / 1e6,
                            material(&w, b, "water") as f64 / 1e6,
                            w.life(b)
                                .unwrap()
                                .died_of
                                .as_ref()
                                .map_or(String::new(), |c| format!(" DEAD of {c}"))
                        )
                    })
                    .collect();
                println!("seed 1 day {day}: {}", report.join("; "));
            }
        }
        assert_eq!(
            (w.total_mass(), w.total_energy() - w.sunlight()),
            start,
            "seed {seed}: totals changed"
        );
        let alive = boars(&w).iter().filter(|&&b| w.is_living(b)).count();
        alive_total += alive;
        if seed == 1 {
            println!(
                "seed 1 forage: forest {}, slopes {}",
                w.mass(w.find_by_key("forage").unwrap()),
                w.mass(w.find_by_key("slope-forage").unwrap())
            );
        }
    }
    println!(
        "living 2: {alive_total} of 60 boars alive after 30 days, over 10 islands; took {:.1?}",
        started.elapsed()
    );
    assert!(alive_total > 0);
}

#[test]
fn a_well_fed_body_stores_its_surplus_as_fat_again() {
    // Two days without food, then ten helpings of shellfish (about 12 MJ),
    // then half a day's rest: the body lives on the shellfish and turns what
    // it won't need soon into fat, so it has more than when it began eating.
    // (Two days later it would be thinner again: 12 MJ is less than two days
    // at rest.)
    let mut w = island();
    let me = w.find_by_key("survivor").unwrap();
    run(&mut w, me, "go stream");
    for _ in 0..2 {
        for _ in 0..6 {
            let _ = parse("drink water").map(|c| match c {
                Command::Act(intent) => perform(&mut w, me, intent).ok(),
                _ => None,
            });
        }
        engine::nature::run(&mut w, 14 * 3_600).unwrap();
        run(&mut w, me, "sleep for 10 h");
    }
    let hungry = material(&w, me, "fat");
    run(&mut w, me, "go beach");
    for _ in 0..10 {
        run(&mut w, me, "gather shellfish");
        let piece = *w.contents(me).last().unwrap();
        let eat = format!("eat {}", w.key(piece));
        run(&mut w, me, &eat);
        for shell in w.contents(me) {
            let line = format!("drop {}", w.key(shell));
            let Ok(Command::Act(intent)) = parse(&line) else {
                unreachable!()
            };
            act(&mut w, me, intent).unwrap();
        }
    }
    engine::nature::run(&mut w, 12 * 3_600).unwrap();
    let fed = material(&w, me, "fat");
    assert!(
        fed > hungry,
        "fat went from {hungry} mg when hungry to {fed} mg when fed"
    );
}

#[test]
fn boars_run_from_a_person_who_comes_among_them_and_keep_away() {
    // Mid-morning, set the islander down wherever most of the herd is. Within
    // ten minutes every boar that was awake there has gone, and none comes
    // back within the hour.
    let mut w = island();
    let me = w.find_by_key("survivor").unwrap();
    engine::nature::run(&mut w, 2 * 3_600).unwrap();
    let herd = boars(&w);
    let mut places: Vec<EntityId> = herd.iter().map(|&b| w.place_of(b).unwrap()).collect();
    places.sort();
    let busiest = *places
        .iter()
        .max_by_key(|&&p| places.iter().filter(|&&q| q == p).count())
        .unwrap();
    let awake: Vec<EntityId> = herd
        .iter()
        .copied()
        .filter(|&b| w.place_of(b) == Some(busiest) && !w.is_asleep(b))
        .collect();
    assert!(!awake.is_empty(), "no boars awake together");
    w.apply(
        engine::gate::Cause::Nature { tick: w.tick() },
        vec![engine::gate::Change::Move {
            entity: me,
            to: busiest,
        }],
    )
    .unwrap();
    engine::nature::run(&mut w, 10 * 60).unwrap();
    for &b in &awake {
        assert_ne!(w.place_of(b), Some(busiest), "a boar stayed");
    }
    engine::nature::run(&mut w, 3_600).unwrap();
    for &b in &awake {
        assert_ne!(w.place_of(b), Some(busiest), "a boar came back");
    }
}

/// The living island with a piece of leather in the islander's hands, of
/// `mass`.
fn island_with_leather(mass: &str) -> (World, EntityId) {
    let text = format!(
        "{LIVING}\n[[item]]\nid = \"skin\"\nat = \"survivor\"\nmass = \"{mass}\"\nmaterial = \"leather\"\n"
    );
    let w = load_world_with(&text, &[THINGS])
        .unwrap()
        .with_luck(engine::world::Luck::AVERAGE);
    let me = w.find_by_key("survivor").unwrap();
    (w, me)
}

fn wounds(world: &World, me: EntityId) -> usize {
    world.life(me).unwrap().wounds.len()
}

#[test]
fn stony_ground_cuts_and_slows_bare_feet_but_wears_shoes_instead() {
    let (mut bare, me) = island_with_leather("400 g");
    let skin = bare.find_by_key("skin").unwrap();
    let mut shod = bare.clone();
    run(&mut shod, me, "wear dried skin on your feet");
    for w in [&mut bare, &mut shod] {
        run(w, me, "go forest");
        run(w, me, "go hillside");
    }
    // Sand, the forest floor, and the hillside don't hurt bare feet.
    assert_eq!(wounds(&bare, me), 0);
    // Walking wears the sole by the ground's roughness: the rougher end, a
    // km at a time. Beach to forest, 300 m at 1 g/km; forest to hillside,
    // 500 m at 4 g/km.
    assert_eq!(400_000 - shod.mass(skin).mg(), 300 + 2_000);

    let (bare_start, shod_start) = (bare.tick(), shod.tick());
    run(&mut bare, me, "go slopes");
    run(&mut shod, me, "go slopes");
    // 5 km at 10 g/km: 50 g of sole, and no wound.
    assert_eq!(400_000 - shod.mass(skin).mg(), 2_300 + 50_000);
    assert_eq!(wounds(&shod, me), 0);
    // Bare feet are cut, bleeding 20 mg/s for each of the 5 km, and walk at
    // half pace.
    assert_eq!(bare.life(me).unwrap().wounds.last().unwrap().0, 100);
    let (bare_time, shod_time) = (bare.tick() - bare_start, shod.tick() - shod_start);
    assert!(
        bare_time > shod_time * 3 / 2,
        "bare {bare_time} s, shod {shod_time} s"
    );
}

#[test]
fn shoes_that_have_worn_through_are_as_good_as_bare_feet() {
    let (mut w, me) = island_with_leather("200 g");
    run(&mut w, me, "wear dried skin on your feet");
    let skin = w.find_by_key("skin").unwrap();
    for place in ["forest", "hillside", "slopes", "ridge"] {
        run(&mut w, me, &format!("go {place}"));
    }
    // 2.3 g, then 50 g, then 100 g: more than half of 200 g is gone.
    assert!(w.worn_through(skin));
    assert_eq!(wounds(&w, me), 0);
    run(&mut w, me, "go slopes");
    assert_eq!(wounds(&w, me), 1);
}

#[test]
fn a_leather_cloak_keeps_in_body_heat_on_a_cold_night() {
    let (mut bare, me) = island_with_leather("2 kg");
    let mut cloaked = bare.clone();
    run(&mut cloaked, me, "wear dried skin");
    // 2 kg covers a whole body, and leather keeps in 40% of its heat.
    assert_eq!(cloaked.clothed(me), 4_000);
    let until_evening = 20 * 3_600 - bare.tick();
    let before = stored_energy(&bare, me);
    for w in [&mut bare, &mut cloaked] {
        engine::nature::run(w, until_evening).unwrap();
        run(w, me, "sleep for 10 h");
    }
    let (bare_burned, cloaked_burned) = (
        before - stored_energy(&bare, me),
        before - stored_energy(&cloaked, me),
    );
    // About 8.7 MJ against 11.3 MJ: the cloak saves over a fifth.
    assert!(
        cloaked_burned < bare_burned * 85 / 100,
        "cloaked {cloaked_burned} µJ, bare {bare_burned} µJ"
    );
}

#[test]
fn only_something_soft_can_be_worn() {
    let (mut w, me) = island_with_leather("400 g");
    run(&mut w, me, "go forest");
    run(&mut w, me, "go hillside");
    run(&mut w, me, "gather stones");
    let Ok(Command::Act(intent)) = parse("wear stone on your feet") else {
        unreachable!()
    };
    let refused = engine::laws::resolve(&w, me, &intent).unwrap_err();
    assert_eq!(
        refused.to_string(),
        "the lump of stone is too stiff to wear"
    );
}

/// The living island with 2 kg of fat at `at`: left lying in the forest, or
/// in the islander's hands.
fn island_with_fat(at: &str) -> (World, EntityId, EntityId) {
    let text = format!(
        "{LIVING}\n[[item]]\nid = \"stores\"\nat = \"{at}\"\nmass = \"2 kg\"\nmaterial = \"fat\"\n"
    );
    let w = load_world_with(&text, &[THINGS])
        .unwrap()
        .with_luck(engine::world::Luck::AVERAGE);
    let me = w.find_by_key("survivor").unwrap();
    let fat = w.find_by_key("stores").unwrap();
    (w, me, fat)
}

#[test]
fn hungry_boars_eat_food_left_lying_about() {
    let (mut w, _, fat) = island_with_fat("forest");
    engine::nature::run(&mut w, 86_400).unwrap();
    let eaten = !w.exists(fat) || w.location(fat).is_some_and(|l| w.is_agent(l));
    assert!(eaten, "the fat is still at {:?}", w.location(fat));
}

#[test]
fn a_raised_cache_keeps_food_out_of_a_boars_reach() {
    // Carried in: fat left in the forest would be eaten before the islander
    // got there.
    let (mut w, me, fat) = island_with_fat("survivor");
    run(&mut w, me, "go forest");
    for _ in 0..6 {
        run(&mut w, me, "gather sticks");
    }
    for _ in 0..2 {
        run(&mut w, me, "gather bushes");
    }
    run(&mut w, me, "assemble raised cache");
    run(&mut w, me, "drop cache");
    run(&mut w, me, "put fat in cache");
    let cache = w.location(fat).unwrap();
    assert_eq!(w.design_of(cache).and_then(|d| d.barrier), Some(2_000_000));
    // A boar reaches 1 m; the cache holds things 2 m up.
    let boar = boars(&w)[0];
    assert!(engine::laws::out_of_reach(&w, boar, cache));
    assert!(!engine::laws::out_of_reach(&w, me, cache));
    run(&mut w, me, "go beach");
    engine::nature::run(&mut w, 86_400).unwrap();
    assert_eq!(w.location(fat), Some(cache));
    // The islander can reach it.
    run(&mut w, me, "go forest");
    run(&mut w, me, "take fat from cache");
    assert_eq!(w.location(fat), Some(me));
}
