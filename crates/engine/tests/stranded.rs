//! The stranded challenge (docs/challenges/stranded.md). Two kinds of test:
//!
//! - **Proofs** run every time: exact checks of the laws here, and scripted
//!   walkthroughs with fixed luck in data/scripts.
//! - **Trials** run on demand (`cargo test -- --ignored`): a survivor plays
//!   many seeds with real chance and reports how many live. Dying is an
//!   outcome; running out of logical options is the only failure.

use std::time::Instant;

use engine::data::load_world;
use engine::datasheet::{Property, Value, measure};
use engine::intent::{Command, Intent, parse};
use engine::laws::{ActError, Refusal, act, perform};
use engine::matter::State;
use engine::nature;
use engine::units::{Credits, Mass};
use engine::world::{EntityId, World};
use proptest::prelude::*;

const ISLAND: &str = include_str!("../../../data/stranded.toml");
const DAY: u64 = 86_400;

fn island(seed: u64) -> World {
    load_world(ISLAND).unwrap().with_seed(seed)
}

fn id(world: &World, key: &str) -> EntityId {
    world.find_by_key(key).unwrap_or_else(|| panic!("no {key}"))
}

fn intent(line: &str) -> Intent {
    match parse(line) {
        Ok(Command::Act(intent)) => intent,
        other => panic!("{line:?} should parse to an action, got {other:?}"),
    }
}

fn totals(world: &World) -> (u128, u128, u128) {
    (
        world.total_mass(),
        world.total_energy() - world.sunlight(),
        world.total_credits(),
    )
}

fn fluid(world: &World, body: EntityId) -> Mass {
    let life = world.life(body).unwrap();
    world
        .composition(body)
        .unwrap()
        .get(&life.fluid)
        .copied()
        .unwrap_or(Mass::ZERO)
}

fn material_in(world: &World, body: EntityId, key: &str) -> Mass {
    let material = world.material_by_key(key).unwrap();
    world
        .composition(body)
        .and_then(|c| c.get(&material))
        .copied()
        .unwrap_or(Mass::ZERO)
}

// The survivor ----------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Alive,
    Died {
        cause: String,
        day: u64,
    },
    /// No logical option left: the one real failure.
    Stuck(String),
}

enum Step {
    Do(String),
    Rest(u64),
}

/// A plain survival routine: drink when a kilo short of water, gather and eat
/// once the last meal is used up and fat starts to drop, throw away shells,
/// and otherwise rest. A fisher first makes a spear, as a skill, then fishes
/// instead of gathering shellfish. It only types commands and reads what a
/// person could know about themselves.
struct Survivor {
    body: EntityId,
    starting_fat: Mass,
    /// Steps still to do of a skill being carried out, last first.
    plan: std::cell::RefCell<Vec<String>>,
    fisher: bool,
}

/// Making a spear, as a skill: the steps, in order.
const MAKE_A_SPEAR: [&str; 10] = [
    "go forest",
    "gather sticks",
    "go hillside",
    "gather stones",
    "gather flint",
    "work flint into flake with stone",
    "work wood into shaft with flake",
    "assemble spear",
    "go forest",
    "go beach",
];

impl Survivor {
    fn new(world: &World) -> Survivor {
        let body = id(world, "survivor");
        Survivor {
            body,
            starting_fat: material_in(world, body, "fat"),
            plan: Default::default(),
            fisher: false,
        }
    }

    /// A survivor who first makes a spear, then fishes when hungry.
    fn fisher(world: &World) -> Survivor {
        let survivor = Survivor::new(world);
        *survivor.plan.borrow_mut() = MAKE_A_SPEAR.iter().rev().map(|s| s.to_string()).collect();
        Survivor {
            fisher: true,
            ..survivor
        }
    }

    /// Options in order of preference. The first that the laws allow is done.
    fn options(&self, w: &World) -> Vec<Step> {
        // Carry on with a skill in progress first.
        if let Some(step) = self.plan.borrow().last() {
            return vec![Step::Do(step.clone())];
        }
        let me = self.body;
        let here = w.key(w.location(me).unwrap()).to_string();
        let life = w.life(me).unwrap();
        let mut options = Vec::new();

        for piece in w.contents(me) {
            if w.assembly(piece).is_some() {
                continue;
            }
            let food = w
                .composition(piece)
                .is_some_and(|c| c.keys().any(|m| life.digests.contains(m)));
            let key = w.key(piece).to_string();
            options.push(Step::Do(if food {
                format!("eat {key}")
            } else {
                format!("drop {key}")
            }));
        }

        let thirsty = fluid(w, me).mg() + 1_000_000 < life.fluid_normal.mg();
        if thirsty {
            options.push(Step::Do(if here == "stream" {
                "drink water".into()
            } else {
                "go stream".into()
            }));
        }
        // Eat again once the last meal is used up and the body has started on
        // its fat.
        let last_meal_gone = material_in(w, me, "shellfish") == Mass::ZERO
            && material_in(w, me, "fish") == Mass::ZERO;
        let hungry =
            last_meal_gone && material_in(w, me, "fat").mg() + 200_000 < self.starting_fat.mg();
        if hungry {
            let source = if self.fisher {
                "gather fish"
            } else {
                "gather shellfish-bed"
            };
            options.push(Step::Do(if here == "beach" {
                source.into()
            } else {
                "go beach".into()
            }));
        }
        options.push(Step::Rest(1_800));
        options
    }

    fn live(&self, world: &mut World, days: u64) -> Outcome {
        let end = world.tick() + days * DAY;
        let mut idle_actions = 0;
        while world.tick() < end {
            if let Some(cause) = world.life(self.body).and_then(|l| l.died_of.clone()) {
                return Outcome::Died {
                    cause,
                    day: world.tick() / DAY,
                };
            }
            let mut refused = Vec::new();
            let mut done = false;
            for step in self.options(world) {
                match step {
                    Step::Rest(seconds) => {
                        nature::run(world, seconds.min(end - world.tick()))
                            .expect("nature never faults");
                        done = true;
                    }
                    Step::Do(line) => {
                        let before = world.tick();
                        let carried_before = world.contents(self.body).len();
                        match perform(world, self.body, intent(&line)) {
                            Ok(_) => {
                                idle_actions = if world.tick() == before {
                                    idle_actions + 1
                                } else {
                                    0
                                };
                                done = true;
                                // A skill step that worked is done. A search that
                                // found nothing is tried again.
                                let mut plan = self.plan.borrow_mut();
                                let found = !line.starts_with("gather")
                                    || world.contents(self.body).len() > carried_before;
                                if plan.last() == Some(&line) && found {
                                    plan.pop();
                                }
                            }
                            Err(ActError::Refused(r)) => refused.push(format!("{line}: {r}")),
                            Err(ActError::Fault(f)) => panic!("{line} faulted: {f}"),
                        }
                    }
                }
                if done {
                    break;
                }
            }
            if !done {
                return Outcome::Stuck(refused.join("; "));
            }
            if idle_actions > 1_000 {
                return Outcome::Stuck("a thousand actions in a row without time passing".into());
            }
        }
        match world.life(self.body).and_then(|l| l.died_of.clone()) {
            Some(cause) => Outcome::Died {
                cause,
                day: world.tick() / DAY,
            },
            None => Outcome::Alive,
        }
    }
}

// Stage 1 ---------------------------------------------------------------

#[test]
fn without_water_a_person_dies_of_thirst_in_about_three_days() {
    let mut w = island(1);
    let me = id(&w, "survivor");
    let start = totals(&w);
    let mut died_at = None;
    for hour in 0..24 * 5 {
        nature::run(&mut w, 3_600).unwrap();
        if !w.is_living(me) {
            died_at = Some(hour);
            break;
        }
    }
    let hours = died_at.expect("dies within five days");
    assert_eq!(w.life(me).unwrap().died_of.as_deref(), Some("thirst"));
    assert!((72..=96).contains(&hours), "died after {hours} hours");
    assert_eq!(
        totals(&w),
        start,
        "the body's matter and energy went somewhere, not nowhere"
    );
    // The body stays where it fell, as matter, and can't act.
    assert!(w.composition(me).is_some());
    assert!(matches!(
        act(&mut w, me, intent("go stream")),
        Err(ActError::Refused(Refusal::NotAnAgent))
    ));
}

#[test]
fn the_sea_cant_be_drunk_but_the_stream_can() {
    let mut w = island(1);
    let me = id(&w, "survivor");
    assert_eq!(
        act(&mut w, me, intent("drink sea")),
        Err(ActError::Refused(Refusal::NotDrinkable("the sea".into())))
    );
    act(&mut w, me, intent("go stream")).unwrap();
    assert_eq!(
        act(&mut w, me, intent("drink water")),
        Err(ActError::Refused(Refusal::NotThirsty))
    );
    nature::run(&mut w, 12 * 3_600).unwrap();
    let before = fluid(&w, me);
    act(&mut w, me, intent("drink water")).unwrap();
    assert_eq!(
        fluid(&w, me).mg() - before.mg(),
        500_000,
        "one gulp is half a kilo"
    );
}

#[test]
fn eating_takes_in_what_the_body_digests_and_leaves_the_shells() {
    let mut w = island(1);
    let me = id(&w, "survivor");
    let start = totals(&w);
    perform(&mut w, me, intent("gather shellfish-bed")).unwrap();
    let piece = w.contents(me)[0];
    assert_eq!(w.mass(piece).mg(), 1_000_000);
    let before = material_in(&w, me, "shellfish");
    let eat = format!("eat {}", w.key(piece));
    act(&mut w, me, intent(&eat)).unwrap();
    assert_eq!(
        material_in(&w, me, "shellfish").mg() - before.mg(),
        350_000,
        "35% of a kilo is meat"
    );
    assert_eq!(w.label(piece), "lump of shell");
    assert_eq!(
        act(&mut w, me, intent(&eat)),
        Err(ActError::Refused(Refusal::CannotEat(
            "the lump of shell".into()
        )))
    );
    assert_eq!(totals(&w), start);
}

#[test]
fn a_resting_body_holds_its_temperature_and_burns_about_seven_megajoules_a_day() {
    let mut w = island(1);
    let me = id(&w, "survivor");
    let chemical = |w: &World| match measure(w, me).get(Property::StoredEnergy) {
        Some(Value::Energy(e)) => e.uj(),
        _ => 0,
    };
    let before = chemical(&w);
    nature::run(&mut w, DAY).unwrap();
    let t = w.temperature(me).unwrap().mk();
    assert!((309_000..=311_000).contains(&t), "body at {t} mK");
    let burned_mj = (before - chemical(&w)) / 1_000_000_000_000;
    assert_eq!(burned_mj, 6, "80 W for a day is 6.9 MJ");
}

#[test]
fn hard_work_makes_a_body_thirstier() {
    let (mut resting, mut working) = (island(1), island(1));
    let me = id(&resting, "survivor");
    nature::run(&mut resting, 2 * 3_600).unwrap();
    let start = working.tick();
    while working.tick() < start + 2 * 3_600 {
        perform(&mut working, me, intent("gather shellfish-bed")).unwrap();
        for piece in working.contents(me) {
            let drop = format!("drop {}", working.key(piece));
            act(&mut working, me, intent(&drop)).unwrap();
        }
    }
    let (rest_fluid, work_fluid) = (fluid(&resting, me).mg(), fluid(&working, me).mg());
    assert!(
        work_fluid + 100_000 < rest_fluid,
        "working lost {work_fluid} mg vs resting {rest_fluid} mg"
    );
    // Without sweat, two hours at 300 W would warm the body about 5 K.
    // Sweating holds it within a kelvin of its set point.
    let set_point = working.life(me).unwrap().set_point.mk();
    let t = working.temperature(me).unwrap().mk();
    assert!(
        t <= set_point + 1_000,
        "working body at {t} mK; sweat should hold it near {set_point} mK"
    );
}

#[test]
#[ignore = "a trial: run with `cargo test -- --ignored --nocapture`"]
fn trial_a_survivor_who_drinks_and_eats_lives_through_ten_days_on_most_islands() {
    let started = Instant::now();
    let mut outcomes = Vec::new();
    for seed in 1..=30 {
        let mut w = island(seed);
        let start = totals(&w);
        let survivor = Survivor::new(&w);
        let outcome = survivor.live(&mut w, 10);
        assert_eq!(totals(&w), start, "seed {seed}: totals changed");
        if seed == 1 {
            let me = id(&w, "survivor");
            println!(
                "seed 1 after 10 days: shellfish bed {}, body fat {}, body water {}, body at {}",
                w.mass(id(&w, "shellfish-bed")),
                material_in(&w, me, "fat"),
                fluid(&w, me),
                w.temperature(me).unwrap()
            );
        }
        assert!(
            !matches!(outcome, Outcome::Stuck(_)),
            "seed {seed}: {outcome:?}"
        );
        outcomes.push(outcome);
    }
    let alive = outcomes.iter().filter(|o| **o == Outcome::Alive).count();
    let deaths: Vec<_> = outcomes.iter().filter(|o| **o != Outcome::Alive).collect();
    println!(
        "stage 1: {alive} of 30 alive after 10 days; deaths: {deaths:?}; 300 island-days took {:.1?}",
        started.elapsed()
    );
    assert!(alive > 0);
}

// Stage 3 ---------------------------------------------------------------

#[test]
fn a_thinned_shellfish_bed_grows_back_toward_its_limit() {
    let mut w = island(1).with_luck(engine::world::Luck::AVERAGE);
    let me = id(&w, "survivor");
    let bed = id(&w, "shellfish-bed");
    for _ in 0..20 {
        perform(&mut w, me, intent("gather shellfish-bed")).unwrap();
        for piece in w.contents(me) {
            let drop = format!("drop {}", w.key(piece));
            act(&mut w, me, intent(&drop)).unwrap();
        }
    }
    let thinned = w.mass(bed).mg();
    assert!(
        thinned < 45_000_000,
        "20 searches took the bed down to {thinned} mg"
    );

    // Left alone, a person lives on fat for a month while the bed regrows.
    nature::run(&mut w, 30 * DAY).unwrap();
    let regrown = w.mass(bed).mg();
    let limit = w.growth(bed).unwrap().limit.mg();
    assert!(
        regrown > thinned + 15_000_000,
        "grew only from {thinned} to {regrown} mg"
    );
    assert!(regrown <= limit, "grew past its limit");
}

#[test]
fn sunlight_brings_exactly_the_energy_that_growing_things_store() {
    let mut w = island(1);
    let (bed, fish) = (id(&w, "shellfish-bed"), id(&w, "fish"));
    let stored = |w: &World| -> u128 {
        [bed, fish]
            .iter()
            .map(|&e| engine::matter::chemical_energy(w.materials(), w.composition(e).unwrap()))
            .sum()
    };
    let (before, start) = (stored(&w), totals(&w));
    nature::run(&mut w, 5 * DAY).unwrap();
    assert!(w.sunlight() > 0, "nothing grew");
    assert_eq!(stored(&w) - before, w.sunlight());
    assert_eq!(totals(&w), start, "everything else is conserved");
}

#[test]
#[ignore = "a trial: run with `cargo test -- --ignored --nocapture`"]
fn trial_a_survivor_who_makes_a_spear_and_fishes_lives_through_a_month() {
    let started = Instant::now();
    let mut outcomes = Vec::new();
    for seed in 1..=30 {
        let mut w = island(seed);
        let start = totals(&w);
        let outcome = Survivor::fisher(&w).live(&mut w, 30);
        assert_eq!(
            totals(&w),
            start,
            "seed {seed}: totals changed apart from sunlight"
        );
        assert!(
            !matches!(outcome, Outcome::Stuck(_)),
            "seed {seed}: {outcome:?}"
        );
        if seed == 1 {
            let me = id(&w, "survivor");
            println!(
                "seed 1 after 30 days: fish in the shallows {}, body fat {}, body water {}",
                w.mass(id(&w, "fish")),
                material_in(&w, me, "fat"),
                fluid(&w, me)
            );
        }
        outcomes.push(outcome);
    }
    let alive = outcomes.iter().filter(|o| **o == Outcome::Alive).count();
    let deaths: Vec<_> = outcomes.iter().filter(|o| **o != Outcome::Alive).collect();
    println!(
        "stage 3: {alive} of 30 fishers alive after 30 days; deaths: {deaths:?}; 900 island-days took {:.1?}",
        started.elapsed()
    );
    assert!(alive > 0);
}

// Random play ------------------------------------------------------------

#[derive(Clone, Debug)]
struct Random {
    verb: u8,
    a: usize,
    b: usize,
    amount: u64,
}

fn random() -> impl Strategy<Value = Random> {
    (0..12u8, any::<usize>(), any::<usize>(), 0..20_000u64)
        .prop_map(|(verb, a, b, amount)| Random { verb, a, b, amount })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]

    #[test]
    fn random_island_life_never_creates_or_destroys_anything(steps in prop::collection::vec(random(), 1..80)) {
        let mut world = island(7);
        let me = id(&world, "survivor");
        // Carry fuel and tinder, so random steps can rub, divide, and burn.
        for line in ["go forest", "gather grass", "gather twigs", "gather sticks", "gather sticks", "divide grass", "divide grass"] {
            perform(&mut world, me, intent(line)).unwrap();
        }
        let start = totals(&world);
        for step in &steps {
            let names: Vec<String> = world.entities().flat_map(|e| [world.key(e).to_string(), world.label(e)]).collect();
            let (a, b) = (names[step.a % names.len()].clone(), names[step.b % names.len()].clone());
            let intent = match step.verb {
                0 => Intent::Go {
                    place: a,
                    aboard: None,
                },
                1 => Intent::Drink { source: a },
                2 => Intent::Eat { item: a },
                3 => Intent::Gather { source: a },
                4 => Intent::Take { item: a },
                5 => Intent::Drop { item: a },
                6 => Intent::Pay { to: a, amount: Credits::new(1) },
                7 => Intent::Put { item: a, into: b },
                8 => Intent::Rub { item: a, against: b, into: None, seconds: Some(step.amount % 120 + 1) },
                9 => Intent::Divide { item: a },
                10 => Intent::Assemble { design: a },
                _ => {
                    nature::run(&mut world, step.amount).map_err(|f| TestCaseError::fail(f.to_string()))?;
                    prop_assert_eq!(totals(&world), start);
                    continue;
                }
            };
            let before = world.clone();
            match perform(&mut world, me, intent.clone()) {
                Ok(_) => {}
                Err(ActError::Refused(_)) => prop_assert_eq!(&world, &before, "a refused {} changed the world", intent),
                Err(ActError::Fault(fault)) => prop_assert!(false, "{} faulted: {}", intent, fault),
            }
            prop_assert_eq!(totals(&world), start);
            prop_assert_eq!(world.check_invariants(), Ok(()));
            // A living body never comes apart: its fluid stays in it.
            if world.is_living(me) {
                prop_assert!(!world.is_all(me, State::Gas));
            }
        }
    }
}
