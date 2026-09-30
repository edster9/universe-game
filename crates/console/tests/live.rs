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
