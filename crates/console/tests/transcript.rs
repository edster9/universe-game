//! A scripted console session: what a person typing would see.

use console::session::Session;
use engine::data::load_world;

const WORLD: &str = include_str!("../../../data/slice0.toml");

fn session() -> Session {
    Session::new(load_world(WORLD).unwrap(), "traveller").unwrap()
}

#[test]
fn a_short_walk_through_town() {
    let mut s = session();
    let totals = s.handle("totals").text;
    assert_eq!(totals, "The world holds 374.4 kg and 750 credits in total.");

    let script = [
        (
            "look",
            "The town square\nWays out: the market, Mara's smithy\nThings here: rope (1.2 kg), lantern (900 g)",
        ),
        ("take the rope", "You take the rope."),
        ("take rope", "You're already carrying rope."),
        (
            "i",
            "You're carrying rope (1.2 kg), 1.2 kg in all.\nYou have 50 credits.",
        ),
        ("go market", "You go to the market."),
        (
            "look",
            "The market\nWays out: the town square\nPeople here: Oskar\nThings here: sack of grain (25 kg)",
        ),
        ("pay oskar -5", "\"-5\" isn't a whole number of credits."),
        (
            "pay oskar 500",
            "You only have 50 credits, not 500 credits.",
        ),
        ("pay Oskar 10", "You pay Oskar 10 credits."),
        ("give rope to oskar", "You give the rope to Oskar."),
        ("take apples", "You don't see apples here."),
        (
            "dance",
            "I don't know how to \"dance\". Type \"help\" for commands.",
        ),
    ];
    for (line, expected) in script {
        assert_eq!(s.handle(line).text, expected, "after {line:?}");
    }

    assert_eq!(s.handle("totals").text, totals, "totals never change");
    assert_eq!(
        s.handle("log 1").text,
        "#3 the traveller: give rope to oskar (rope -> Oskar)"
    );
    assert!(s.handle("quit").quit);
}

#[test]
fn becoming_someone_else() {
    let mut s = session();
    assert!(
        s.handle("become mara")
            .text
            .starts_with("You are now Mara.\nMara's smithy")
    );
    assert_eq!(
        s.handle("give hammer to traveller").text,
        "There's nobody called traveller here."
    );
}
