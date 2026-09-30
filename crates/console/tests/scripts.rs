//! Plays every script in data/scripts. Each one states its world, its luck,
//! and how it should end; see src/script.rs for the format.

use std::path::Path;

use console::script;

#[test]
fn every_script_ends_as_it_says() {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let mut paths: Vec<_> = std::fs::read_dir(data.join("scripts"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no scripts found");
    // Each script is its own world, so they can all play at once.
    let results: Vec<_> = std::thread::scope(|scope| {
        let runs: Vec<_> = paths
            .iter()
            .map(|path| {
                scope.spawn(move || {
                    let text = std::fs::read_to_string(path).unwrap();
                    script::run(&text, data)
                })
            })
            .collect();
        runs.into_iter().map(|run| run.join().unwrap()).collect()
    });
    let mut failures = Vec::new();
    for (path, result) in paths.iter().zip(results) {
        match result {
            Ok(_) => println!("passed: {}", path.display()),
            Err(e) => failures.push(format!("{}: {e}", path.display())),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// The stories about making and finding things, played again by players'
/// rules: on vitality, needing no food, drink, or sleep (a player rests
/// instead). They must end the same way. Stories about needs themselves,
/// thirst, sleep, and shivering, and ones that time each step exactly, aren't
/// here: a player's day runs differently. See
/// docs/ideas/game-interface.md.
#[test]
fn the_making_stories_end_the_same_for_a_player() {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let stories = [
        "stranded-2-everything-at-once.txt",
        "stranded-2-fire.txt",
        "stranded-2-no-tinder.txt",
        "stranded-2-skip-the-twigs.txt",
        "stranded-2-stones-make-no-fire.txt",
        "stranded-3-bad-luck.txt",
        "stranded-3-spear-and-fish.txt",
        "stranded-4-axe.txt",
        "stranded-4-no-flame.txt",
        "stranded-4-unfired-mould.txt",
        "stranded-5-blunt-axe.txt",
        "stranded-5-no-axe.txt",
        "stranded-6-rope.txt",
        "stranded-7-driftwood-raft.txt",
        "stranded-7-raft.txt",
        "stranded-8-cant-cross.txt",
        "where-1-night-and-fire.txt",
        "where-2-explore.txt",
        "where-3-unfired-pot.txt",
        "where-5-the-view.txt",
        "memory-1-map.txt",
        "memory-2-changes.txt",
        "living-2-boars.txt",
        "living-3-injury.txt",
        "living-3-a-killing-blow.txt",
        "living-4-the-hunt.txt",
        "living-5-bare-feet.txt",
        "words-1-two-peoples.txt",
        "words-2-inventing.txt",
    ];
    let results: Vec<_> = std::thread::scope(|scope| {
        let runs: Vec<_> = stories
            .iter()
            .map(|name| {
                scope.spawn(move || {
                    let text = std::fs::read_to_string(data.join("scripts").join(name)).unwrap();
                    script::run(&format!("rules player\n{text}"), data)
                })
            })
            .collect();
        runs.into_iter().map(|run| run.join().unwrap()).collect()
    });
    let failures: Vec<String> = stories
        .iter()
        .zip(results)
        .filter_map(|(name, result)| result.err().map(|e| format!("{name}, as a player: {e}")))
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
