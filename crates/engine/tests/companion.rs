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

/// The stranger standing on the hillside with no orders, a confined mind of
/// the given temperament, carrying their barb.
fn standing(temperament: &'static str, text: String) -> String {
    let start = text.find("orders = [\n    \"hungry").unwrap();
    let end = start + text[start..].find("]\n").unwrap() + 2;
    let text = format!("{}orders = []\n{}", &text[..start], &text[end..]);
    text.replace("mind = \"resident\"", "mind = \"confined\"")
        .replace(
            "temperament = \"defend\"",
            &format!("temperament = \"{temperament}\""),
        )
        .replace(
            "label = \"the stranger\"\nat = \"beach\"",
            "label = \"the stranger\"\nat = \"hillside\"",
        )
}

/// A lone boar with nowhere to run, on the hillside where the stranger
/// stands: cornered, it turns on them.
fn cornered(temperament: &'static str) -> impl Fn(String) -> String {
    move |text: String| {
        standing(temperament, text)
            + "\n[[agent]]\nid = \"tusker\"\nlabel = \"a boar\"\nat = \"hillside\"\nkind = \"boar\"\ntemperature = \"311 K\"\nrange = [\"hillside\"]\n"
    }
}

/// How the stranger comes out of an hour with a cornered boar: where they
/// are, whether they're alive, and whether the boar was wounded.
fn against_a_cornered_boar(temperament: &'static str) -> (String, bool, bool) {
    let (mut w, stranger) = island(cornered(temperament));
    let tusker = w.find_by_key("tusker").unwrap();
    engine::nature::run(&mut w, 3_600).unwrap();
    let wounded = !w.life(tusker).unwrap().wounds.is_empty();
    (
        w.key(w.place_of(stranger).unwrap()).to_string(),
        w.is_living(stranger),
        wounded,
    )
}

#[test]
fn against_a_cornered_boar_temperament_decides() {
    // Fleeing, they get away, and the boar, keeping to the hillside, stays.
    assert_eq!(
        against_a_cornered_boar("flee"),
        ("forest".into(), true, false)
    );
    // Giving in, they neither run nor strike, and it gores them to death.
    assert_eq!(
        against_a_cornered_boar("give in"),
        ("hillside".into(), false, false)
    );
    // Fighting or defending, they strike back with the barb and wound it.
    for temperament in ["fight", "defend"] {
        let (place, _, wounded) = against_a_cornered_boar(temperament);
        assert_eq!(place, "hillside");
        assert!(wounded, "{temperament}");
    }
}

/// The islander, carrying a blade, strikes the stranger once and stays:
/// how many times the stranger strikes back in the next ten minutes, and
/// where they are then.
fn struck_once(temperament: &'static str) -> (usize, String) {
    let (mut w, stranger) = island(move |text| {
        standing(temperament, text).replace(
            "label = \"the islander\"\nat = \"beach\"",
            "label = \"the islander\"\nat = \"hillside\"",
        ) + "\n[[item]]\nid = \"blade\"\nat = \"survivor\"\nmass = \"300 g\"\nmaterial = \"iron\"\nshape = \"flake\"\n"
    });
    let islander = w.find_by_key("survivor").unwrap();
    let blade = w.find_by_key("blade").unwrap();
    let Ok(engine::intent::Command::Act(strike)) = engine::intent::parse(&format!(
        "attack {} with {}",
        engine::laws::pointer(stranger),
        engine::laws::pointer(blade)
    )) else {
        panic!("a strike");
    };
    engine::laws::perform(&mut w, islander, strike).unwrap();
    engine::nature::run(&mut w, 600).unwrap();
    let blows = w.life(islander).unwrap().wounds.len();
    (blows, w.key(w.place_of(stranger).unwrap()).to_string())
}

#[test]
fn struck_once_temperament_decides_how_they_answer() {
    assert_eq!(struck_once("give in"), (0, "hillside".into()));
    assert_eq!(struck_once("flee"), (0, "forest".into()));
    // Defending, they strike back once; fighting, they keep at it while the
    // one who struck them is there.
    assert_eq!(struck_once("defend"), (1, "hillside".into()));
    let (blows, place) = struck_once("fight");
    assert!(blows > 1, "{blows}");
    assert_eq!(place, "hillside");
}

/// What happens when the islander says `line`: the refusal, or nothing if
/// it's allowed.
fn say(w: &mut World, line: &str) -> Option<String> {
    let islander = w.find_by_key("survivor").unwrap();
    let Ok(engine::intent::Command::Act(intent)) = engine::intent::parse(line) else {
        panic!("{line:?} isn't an action");
    };
    engine::laws::perform(w, islander, intent)
        .err()
        .map(|e| e.to_string())
}

fn requests(w: &World, who: EntityId) -> usize {
    w.mind(who).unwrap().requests.len()
}

#[test]
fn a_confined_mind_takes_on_only_its_own_work() {
    let (mut w, stranger) = island(|t| t.replace("mind = \"resident\"", "mind = \"confined\""));
    assert_eq!(
        say(&mut w, "ask the stranger to gather driftwood").as_deref(),
        Some("the stranger won't: it isn't what they do")
    );
    assert_eq!(say(&mut w, "ask the stranger to gather shellfish"), None);
    assert_eq!(requests(&w, stranger), 1);
}

#[test]
fn only_a_mind_of_its_own_takes_requests_and_a_sleeper_doesnt_hear() {
    let (mut w, _) = island(|t| t);
    say(&mut w, "go forest");
    assert_eq!(
        say(&mut w, "ask a boar to go beach").as_deref(),
        Some("a boar decides for themselves")
    );
    say(&mut w, "go beach");
    let stranger = w.find_by_key("stranger").unwrap();
    while !w.is_asleep(stranger) {
        engine::nature::run(&mut w, 600).unwrap();
    }
    assert_eq!(
        say(&mut w, "ask the stranger to gather driftwood").as_deref(),
        Some("the stranger is asleep")
    );
}

#[test]
fn a_request_is_taken_up_once_and_dropped_if_it_can_no_longer_be_done() {
    let (mut w, stranger) = island(|t| t);
    let islander = w.find_by_key("survivor").unwrap();
    // Driftwood, gathered and handed over.
    assert_eq!(say(&mut w, "ask the stranger to gather driftwood"), None);
    engine::nature::run(&mut w, 1_200).unwrap();
    assert_eq!(requests(&w, stranger), 0);
    assert_eq!(
        say(&mut w, "ask the stranger to give wood to the islander"),
        None
    );
    engine::nature::run(&mut w, 1_200).unwrap();
    assert_eq!(w.contents(islander).len(), 1);
    // Asked to take the wood back, after it's been taken away again: they
    // find they can't, and let it go.
    say(&mut w, "drop wood");
    assert_eq!(
        say(&mut w, "ask the stranger to take the lump of wood"),
        None
    );
    say(&mut w, "take the lump of wood");
    engine::nature::run(&mut w, 1_200).unwrap();
    assert_eq!(requests(&w, stranger), 0);
    assert_eq!(w.contents(islander).len(), 1);
}

fn owns(w: &World, who: EntityId, thing: EntityId) -> bool {
    w.memory(who).unwrap().owns.contains(&thing)
}

/// Offering the islander's driftwood for the stranger's barb, with the
/// stranger valuing things as `values` says.
fn wood_for_the_barb(values: &'static str) -> (World, Option<String>) {
    let (mut w, _) = island(move |t| {
        t.replace(
            "values = { barb = 20, fish = 4, \"pale flesh\" = 2, wood = 1 }",
            values,
        )
    });
    say(&mut w, "go to driftwood");
    say(&mut w, "gather driftwood");
    let refusal = say(&mut w, "offer wood to the stranger for barb");
    (w, refusal)
}

#[test]
fn a_trade_is_made_only_if_what_they_get_is_worth_as_much_to_them() {
    // Something they've no use for.
    let (_, refusal) = wood_for_the_barb("values = { barb = 20 }");
    assert_eq!(
        refusal.as_deref(),
        Some("the stranger says, \"I've no use for the lump of wood.\"")
    );
    // Worth less to them than what they'd give: refused, and nothing
    // changes hands.
    let (w, refusal) = wood_for_the_barb("values = { barb = 20, wood = 1 }");
    assert_eq!(
        refusal.as_deref(),
        Some("the stranger says, \"The iron barb is worth more to me than the lump of wood.\"")
    );
    let (stranger, barb) = (
        w.find_by_key("stranger").unwrap(),
        w.find_by_key("barb").unwrap(),
    );
    assert_eq!(w.location(barb), Some(stranger));
    assert!(owns(&w, stranger, barb));
    // Worth as much: both change hands at once, and in each mind the thing
    // that came to them is theirs, and the thing they handed over isn't.
    let (w, refusal) = wood_for_the_barb("values = { barb = 1, wood = 1 }");
    assert_eq!(refusal, None);
    let islander = w.find_by_key("survivor").unwrap();
    let wood = w
        .contents(stranger)
        .into_iter()
        .find(|&t| w.label(t).contains("wood"))
        .unwrap();
    assert_eq!(w.location(barb), Some(islander));
    assert!(owns(&w, islander, barb) && !owns(&w, stranger, barb));
    assert!(owns(&w, stranger, wood) && !owns(&w, islander, wood));
}

#[test]
fn whose_a_thing_is_lives_in_minds_and_only_a_theft_seen_is_remembered() {
    let (mut w, stranger) = island(|t| t);
    let islander = w.find_by_key("survivor").unwrap();
    // The stranger's shells pile up on the beach.
    engine::nature::run(&mut w, 2 * 3_600).unwrap();
    let shells = |w: &World| -> Vec<EntityId> {
        w.contents(w.find_by_key("beach").unwrap())
            .into_iter()
            .filter(|&t| {
                w.label(t) == "lump of shell" || w.label_for(islander, t) == "lump of shell"
            })
            .collect()
    };
    let shell = shells(&w)[0];
    assert!(owns(&w, stranger, shell));
    // Asleep, they don't see it taken: each now believes it's theirs.
    while !w.is_asleep(stranger) || w.place_of(stranger) != w.place_of(islander) {
        engine::nature::run(&mut w, 600).unwrap();
    }
    say(&mut w, &format!("go to {}", engine::laws::pointer(shell)));
    say(&mut w, &format!("take {}", engine::laws::pointer(shell)));
    assert!(owns(&w, islander, shell) && owns(&w, stranger, shell));
    assert!(w.memory(stranger).unwrap().robbed_by.is_empty());
    // Awake and watching, they see it, and won't be asked anything more.
    while w.is_asleep(stranger) || w.place_of(stranger) != w.place_of(islander) {
        engine::nature::run(&mut w, 600).unwrap();
    }
    let another = shells(&w)[0];
    let taking = say(&mut w, &format!("take {}", engine::laws::pointer(another)));
    assert_eq!(taking, None);
    assert!(
        w.memory(stranger)
            .unwrap()
            .robbed_by
            .contains_key(&islander)
    );
    assert_eq!(
        say(&mut w, "ask the stranger to gather driftwood").as_deref(),
        Some("the stranger says, \"You took what's mine.\"")
    );
}

#[test]
fn a_walker_s_journey_is_known_while_they_re_on_their_way() {
    let (mut w, _) = island(|t| t);
    let islander = w.find_by_key("survivor").unwrap();
    let (beach, forest) = (
        w.find_by_key("beach").unwrap(),
        w.find_by_key("forest").unwrap(),
    );
    assert_eq!(engine::laws::journey(&w, islander), None);
    let Ok(engine::intent::Command::Act(go)) = engine::intent::parse("go forest") else {
        panic!("a walk");
    };
    let now = w.tick();
    let started = engine::laws::start(&mut w, islander, go).unwrap();
    let engine::laws::Started::Due(until) = started else {
        panic!("a walk takes time");
    };
    assert_eq!(
        engine::laws::journey(&w, islander),
        Some((beach, forest, now, until))
    );
}

#[test]
fn only_a_mind_of_its_own_takes_offers() {
    let (mut w, _) = island(|t| t);
    say(&mut w, "go to driftwood");
    say(&mut w, "gather driftwood");
    say(&mut w, "go forest");
    assert_eq!(
        say(&mut w, "offer wood to a boar for roots").as_deref(),
        Some("a boar decides for themselves")
    );
}

#[test]
fn a_person_pictures_only_what_s_where_they_stand_and_what_they_remember() {
    let (mut w, _) = island(|t| t);
    let islander = w.find_by_key("survivor").unwrap();
    let (beach, forest) = (
        w.find_by_key("beach").unwrap(),
        w.find_by_key("forest").unwrap(),
    );
    let pictured = |w: &World| {
        let scene = engine::view::scene(w, islander);
        let mut all = scene.in_sight.clone();
        all.extend(scene.remembered.iter().map(|&(_, t)| t));
        (scene, all)
    };
    // On the beach, never having been anywhere: only the beach.
    let (scene, all) = pictured(&w);
    assert_eq!(scene.here, Some(beach));
    assert!(scene.in_sight.contains(&islander));
    assert!(scene.remembered.is_empty(), "{:?}", scene.remembered);
    assert!(all.iter().all(|&t| w.place_of(t) == Some(beach)));
    let forest_things: Vec<EntityId> = w
        .contents(forest)
        .into_iter()
        .filter(|&t| !w.is_agent(t) && !w.is_portable(t))
        .collect();
    assert!(!forest_things.is_empty());
    assert!(forest_things.iter().all(|t| !all.contains(t)));

    // To the forest and back: its fixed things are remembered where they
    // stand; the boars, which move, are not.
    say(&mut w, "go forest");
    let (scene, _) = pictured(&w);
    assert_eq!(scene.here, Some(forest));
    assert!(scene.in_sight.iter().any(|&t| w.kind_of(t) == Some("boar")));
    say(&mut w, "go beach");
    let (scene, all) = pictured(&w);
    assert_eq!(scene.here, Some(beach));
    for t in &forest_things {
        assert!(scene.remembered.contains(&(forest, *t)), "{}", w.label(*t));
    }
    // What's here is seen, not remembered.
    assert!(scene.remembered.iter().all(|&(place, _)| place != beach));
    for &t in &all {
        let at_hand = w.place_of(t) == Some(beach);
        assert!(
            at_hand || (!w.is_agent(t) && !w.is_portable(t)),
            "{}",
            w.label(t)
        );
    }
}

#[test]
fn minds_walk_up_to_what_they_act_on() {
    let (mut w, stranger) = island(|t| t);
    let bed = w.find_by_key("shellfish-bed").unwrap();
    let start = w.spot(stranger).unwrap();
    // The shellfish lie in a patch by the rocks, 30 m off: hungry, the
    // stranger walks there and gathers, never from where they stood.
    assert!(!w.within_reach(stranger, bed));
    let mut gathered_from = Vec::new();
    for _ in 0..6 * 60 {
        engine::nature::run(&mut w, 60).unwrap();
        let carrying_flesh = w
            .contents(stranger)
            .into_iter()
            .any(|t| w.label_for(stranger, t).contains("pale flesh"));
        if carrying_flesh {
            gathered_from.push(w.spot(stranger).unwrap());
            break;
        }
    }
    let at = *gathered_from.first().expect("they gathered some");
    assert_ne!(at, start);
    assert!(w.within_reach(stranger, bed), "{at:?}");
}
