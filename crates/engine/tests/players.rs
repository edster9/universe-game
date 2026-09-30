//! Players' rules: a player lives on vitality, a named inflow of energy and
//! body fluid, with stamina for hard work, and needs no food, drink, or sleep.
//! Wounds heal from vitality; cold still kills. See
//! docs/ideas/game-interface.md.

use engine::data::load_world_with;
use engine::gate::{Cause, Change};
use engine::world::{EntityId, Luck, World};

const STRANDED: &str = include_str!("../../../data/stranded.toml");
const THINGS: &str = include_str!("../../../data/island-things.toml");

fn player(extra: &str, who: &str) -> (World, EntityId) {
    let text = format!("{STRANDED}\n{extra}");
    let w = load_world_with(&text, &[THINGS])
        .unwrap()
        .with_luck(Luck::AVERAGE)
        .with_player_rules(who)
        .unwrap();
    let me = w.find_by_key(who).unwrap();
    (w, me)
}

fn fluid(w: &World, me: EntityId) -> u64 {
    let life = w.life(me).unwrap();
    w.composition(me).unwrap()[&life.fluid].mg()
}

fn wound(w: &mut World, me: EntityId, rate: u64) {
    w.apply(
        Cause::Nature { tick: w.tick() },
        vec![Change::Wound { agent: me, rate }],
    )
    .unwrap();
}

#[test]
fn a_players_wound_heals_from_vitality() {
    let (mut w, me) = player("", "survivor");
    let normal = w.life(me).unwrap().fluid_normal.mg();
    let (mass, energy) = (w.own_mass(), w.own_energy());
    // A cut bleeding 0.5 g a second clots within hours, having lost about
    // 400 g.
    wound(&mut w, me, 500);
    engine::nature::run(&mut w, 3 * 3_600).unwrap();
    assert!(w.is_living(me));
    assert_eq!(w.bleeding(me), 0);
    // Vitality brings the lost blood back, at up to 3 kg a day.
    engine::nature::run(&mut w, 86_400).unwrap();
    assert!(
        fluid(&w, me) + 1_000 >= normal,
        "{} of {normal} mg",
        fluid(&w, me)
    );
    // All of it came in through the named inflow: the rest is conserved.
    assert_eq!((w.own_mass(), w.own_energy()), (mass, energy));
}

#[test]
fn a_severe_wound_still_bleeds_a_player_dry() {
    let (mut w, me) = player("", "survivor");
    wound(&mut w, me, 30_000);
    engine::nature::run(&mut w, 3_600).unwrap();
    let died = w.life(me).unwrap().died_of.clone();
    assert_eq!(died.as_deref(), Some("bleeding"));
}

/// A player on an ice field, with 2 kg of leather in hand.
const ICE: &str = r#"
[[place]]
id = "ice"
label = "the ice field"
temperature = "270 K"
night = "265 K"

[[agent]]
id = "walker"
label = "the walker"
at = "ice"
kind = "human"
temperature = "310 K"

[[item]]
id = "cloak"
at = "walker"
mass = "2 kg"
material = "leather"
"#;

#[test]
fn cold_still_kills_a_player_without_clothing() {
    let (mut bare, me) = player(ICE, "walker");
    let mut cloaked = bare.clone();
    let Ok(engine::intent::Command::Act(intent)) = engine::intent::parse("wear leather") else {
        unreachable!()
    };
    engine::laws::perform(&mut cloaked, me, intent).unwrap();
    for w in [&mut bare, &mut cloaked] {
        engine::nature::run(w, 2 * 86_400).unwrap();
    }
    // At 270 K a body loses about 320 W: more than vitality's 200 W. A
    // leather cloak keeps in 40% of it.
    assert_eq!(bare.life(me).unwrap().died_of.as_deref(), Some("cold"));
    assert!(cloaked.is_living(me));
}
