//! The live channel: each reply comes with the person's state as one line
//! of JSON, for a program to drive them.

use std::path::Path;

use console::live::state;
use console::session::Session;

#[test]
fn each_reply_comes_with_the_persons_state_as_json() {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let world = console::load_world_file(&data.join("strangers-words.toml")).unwrap();
    let mut session = Session::new(world, "islander").unwrap();
    let reply = session.handle("start go forest");
    let json = state(&session, Some(&reply));
    assert!(json.starts_with('{') && json.ends_with('}'), "{json}");
    for field in [
        r#""as":"islander""#,
        r#""clock":"08:00""#,
        r#""busy_until":"08:06""#,
        r#""reply":"You start to go forest. You'll be done at 08:06.""#,
        r#""place":"the beach""#,
    ] {
        assert!(json.contains(field), "{field} missing from {json}");
    }
    let reply = session.handle("wait until free");
    let json = state(&session, Some(&reply));
    assert!(json.contains(r#""place":"the forest""#), "{json}");
    assert!(json.contains(r#""busy_until":null"#), "{json}");
    assert!(json.contains(r#"You go to the forest.""#), "{json}");
}

/// As a game client plays: the clock runs on its own, commands that take
/// time start rather than moving it, and how they came out is told when
/// they're over.
#[test]
fn in_real_time_actions_start_and_their_outcomes_are_told_when_they_come() {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let world = console::load_world_file(&data.join("companion.toml"))
        .unwrap()
        .with_player_rules("survivor")
        .unwrap();
    let mut session = Session::new(world, "survivor").unwrap().with_real_time();
    let before = session.world().tick();
    let reply = session.handle("go forest");
    assert!(
        reply.text.starts_with("You start to go forest"),
        "{}",
        reply.text
    );
    assert_eq!(session.world().tick(), before, "the clock didn't wait");
    // Instant things still happen at once, even while walking.
    assert!(session.handle("look").text.contains("beach"));
    assert_eq!(session.catch_up(), "");
    // The walk is over once its time has passed, and told once.
    let until = session.world().pending(session.player()).unwrap().until;
    session.advance(until - before);
    assert_eq!(session.catch_up(), "You go to the forest.");
    assert_eq!(session.catch_up(), "");
    assert!(session.handle("look").text.contains("forest"));
}
