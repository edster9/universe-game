//! The backpack and the body, as a client's windows show them: measured,
//! never written. See docs/challenges/first-steps.md, stage 4.

use std::path::Path;

use console::session::{Session, backpack, body, format_datasheet};

fn islander() -> Session {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let world = console::load_world_file(&data.join("companion.toml"))
        .unwrap()
        .with_player_rules("survivor")
        .unwrap();
    Session::new(world, "survivor").unwrap()
}

/// Every line of the body, compact or full, is a line of the datasheet,
/// apart from what they're wearing (from what they carry).
fn check_measured(session: &Session) {
    let (world, me) = (session.world(), session.player());
    let sheet: Vec<String> = format_datasheet(&engine::datasheet::measure(world, me))
        .into_iter()
        .map(|l| l.trim().to_string())
        .collect();
    for all in [false, true] {
        let lines = body(world, me, all);
        assert!(lines.len() > 3, "{lines:?}");
        for line in lines.iter().filter(|l| !l.starts_with("wearing: ")) {
            assert!(sheet.contains(line), "{line:?} isn't measured: {sheet:?}");
        }
    }
}

#[test]
fn the_body_shows_what_the_datasheet_measures_and_the_backpack_what_is_carried() {
    let mut s = islander();
    check_measured(&s);
    let vitals = body(s.world(), s.player(), false);
    assert!(
        vitals
            .iter()
            .any(|l| l.starts_with("stamina: 3 MJ of 3 MJ")),
        "{vitals:?}"
    );
    assert_eq!(backpack(s.world(), s.player()).first().unwrap(), "nothing");

    for line in [
        "go forest",
        "go to sticks",
        "gather sticks",
        "gather sticks",
        "go to grass",
        "gather grass",
    ] {
        assert!(!s.handle(line).refused, "{line}");
    }
    let pack = backpack(s.world(), s.player());
    assert_eq!(
        pack,
        [
            "lump of wood x2: 400 g",
            "lump of dry grass: 5 g",
            "carrying 405 g of 40 kg"
        ]
    );
    // Gathering is hard work, and the body shows it.
    check_measured(&s);
    let after = body(s.world(), s.player(), false);
    assert_ne!(vitals, after);
}
