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
