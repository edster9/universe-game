//! Saving and resuming: a save picks up exactly where it was. A fire saved
//! while burning, loaded, and left to burn comes out just as the fire that
//! was never saved. See docs/challenges/skill-grounds.md.

use std::path::{Path, PathBuf};

use console::session::Session;

fn islander(saves: &Path) -> Session {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let world = console::load_world_file(&data.join("companion.toml"))
        .unwrap()
        .with_player_rules("survivor")
        .unwrap();
    Session::new(world, "survivor")
        .unwrap()
        .with_saves(saves.to_path_buf())
}

fn ok(s: &mut Session, line: &str) -> String {
    let reply = s.handle(line);
    assert!(!reply.refused, "{line}: {}", reply.text);
    reply.text
}

fn folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("universe-saves-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    folder
}

#[test]
fn a_fire_saved_while_burning_burns_on_the_same_after_loading() {
    let saves = folder("fire");
    let mut s = islander(&saves);
    for line in [
        "go forest",
        "go to sticks; gather sticks x2",
        "go to grass; gather grass x3",
        "go to twigs; gather twigs x3",
        "go hillside",
        "go to stones; gather stones x5",
        "assemble fire ring",
        "go forest",
        "drop fire ring",
        "put grass in ring",
        "rub wood against wood into ring",
        "wait 10",
        "put grass in ring",
        "put smallest wood in ring",
    ] {
        ok(&mut s, line);
    }
    assert!(ok(&mut s, "look").contains("burning"));
    ok(&mut s, "/save fire");

    // The same world, read back.
    let mut loaded = Session::resume(saves.clone(), "fire").unwrap();
    assert!(loaded.world() == s.world(), "the loaded world differs");

    // Both burn on, and are fed, alike.
    for line in [
        "wait 30",
        "put smallest wood in ring",
        "put wood in ring",
        "wait 5 min",
    ] {
        assert_eq!(ok(&mut s, line), ok(&mut loaded, line), "after {line}");
    }
    assert!(loaded.world() == s.world(), "the worlds went apart");
    assert_eq!(ok(&mut s, "look"), ok(&mut loaded, "look"));

    // Loading in a session already going goes back to the save.
    let then = ok(&mut s, "/load fire");
    assert!(then.starts_with("Loaded \"fire\""), "{then}");
    assert!(s.world() == Session::resume(saves.clone(), "fire").unwrap().world());
    let _ = std::fs::remove_dir_all(&saves);
}

#[test]
fn saves_are_named_listed_and_missing_ones_refused() {
    let saves = folder("names");
    let mut s = islander(&saves);
    assert!(ok(&mut s, "/saves").contains("no saves"));
    assert!(s.handle("/save ../escape").refused);
    assert!(s.handle("/save").refused);
    assert!(s.handle("/load nothing-here").refused);
    ok(&mut s, "/save first");
    ok(&mut s, "go forest");
    ok(&mut s, "/save second");
    assert!(ok(&mut s, "/saves").contains("second, first"));
    let back = ok(&mut s, "/load first");
    assert!(back.contains("The beach"), "{back}");
    let _ = std::fs::remove_dir_all(&saves);
}
