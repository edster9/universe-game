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
    let mut failures = Vec::new();
    for path in &paths {
        let text = std::fs::read_to_string(path).unwrap();
        match script::run(&text, data) {
            Ok(_) => println!("passed: {}", path.display()),
            Err(e) => failures.push(format!("{}: {e}", path.display())),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
