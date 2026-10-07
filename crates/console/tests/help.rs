//! The help and the commands keep in step: every command the game accepts
//! has its help, and every help is for a command it accepts, with examples
//! it understands and links that lead somewhere. A new command fails here
//! until `help.toml` has its entry.

use std::collections::BTreeSet;
use std::path::Path;

use console::help;
use console::session::Session;

/// The words a function's `match` arms accept (`"take" | "get" =>`), read
/// from its source.
fn verbs_in(source: &str, function: &str) -> BTreeSet<String> {
    let start = source.find(function).expect(function);
    let body = &source[start..];
    let end = body
        .find("\n}\n")
        .or_else(|| body.find("\n    }\n"))
        .unwrap_or(body.len());
    let mut words = BTreeSet::new();
    for line in body[..end].lines() {
        let line = line.trim();
        let Some(arm) = line.split("=>").next().filter(|_| line.contains("=>")) else {
            continue;
        };
        let arm = arm.split(" if ").next().unwrap_or(arm).trim();
        let arm = arm.trim_start_matches("tool @ (").trim_end_matches(')');
        let quoted: Vec<&str> = arm.split('|').map(str::trim).collect();
        if quoted
            .iter()
            .all(|q| q.len() > 2 && q.starts_with('"') && q.ends_with('"'))
        {
            words.extend(quoted.iter().map(|q| q.trim_matches('"').to_string()));
        }
    }
    words
}

fn accepted() -> BTreeSet<String> {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
    let parser = std::fs::read_to_string(root.join("crates/engine/src/intent.rs")).unwrap();
    let session = std::fs::read_to_string(root.join("crates/console/src/session.rs")).unwrap();
    let mut words = verbs_in(&parser, "pub fn parse(");
    words.extend(verbs_in(&session, "pub fn handle("));
    // Help is the help itself.
    words.remove("help");
    words.remove("?");
    words.remove("");
    words
}

#[test]
fn every_command_has_its_help_and_every_help_is_a_command() {
    let accepted = accepted();
    assert!(
        accepted.contains("gather") && accepted.contains("/save"),
        "{accepted:?}"
    );
    let helped: BTreeSet<String> = help::commands()
        .iter()
        .flat_map(|c| std::iter::once(c.name.clone()).chain(c.also.clone()))
        .collect();
    let missing: Vec<&String> = accepted.difference(&helped).collect();
    assert!(missing.is_empty(), "commands without help: {missing:?}");
    let extra: Vec<&String> = helped.difference(&accepted).collect();
    assert!(
        extra.is_empty(),
        "help for commands the game doesn't take: {extra:?}"
    );
}

#[test]
fn examples_are_understood_and_links_lead_somewhere() {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let world = console::load_world_file(&data.join("skill-yard.toml")).unwrap();
    let names: BTreeSet<&str> = help::names().into_iter().collect();
    for c in help::commands() {
        assert!(
            help::groups().contains(&c.group),
            "{}: no group {}",
            c.name,
            c.group
        );
        assert!(
            c.summary.chars().count() <= 70,
            "{}: summary too long for a line",
            c.name
        );
        assert!(
            !c.usage.is_empty() && !c.about.trim().is_empty(),
            "{}",
            c.name
        );
        for see in &c.see {
            assert!(
                names.contains(see.as_str()),
                "{}: \"see {see}\" leads nowhere",
                c.name
            );
        }
        for example in &c.examples {
            // Each tried in a fresh world: it may be refused, but it must
            // be understood.
            if example == "quit" {
                continue;
            }
            let mut s = Session::new(world.clone(), "player").unwrap();
            let reply = s.handle(example);
            assert!(
                !reply.text.contains("I don't know how to"),
                "{}: example \"{example}\" isn't understood: {}",
                c.name,
                reply.text
            );
        }
    }
}

#[test]
fn help_lists_everything_and_tells_all_about_one() {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let world = console::load_world_file(&data.join("skill-yard.toml")).unwrap();
    let mut s = Session::new(world, "player").unwrap();
    let all = s.handle("help").text;
    for c in help::commands() {
        assert!(
            all.contains(&format!("  {}", c.name)),
            "help doesn't list {}",
            c.name
        );
    }
    let gather = s.handle("help gather").text;
    for form in ["x3", "500 g", "until full", "For example:", "See also:"] {
        assert!(gather.contains(form), "help gather lacks {form}: {gather}");
    }
    // Another word for it, a tool without its slash, and a topic.
    assert!(s.handle("help collect").text.starts_with("gather"));
    assert!(s.handle("help save").text.starts_with("/save"));
    assert!(s.handle("help names").text.contains("smallest"));
    // Not a command: what it might have been.
    let wrong = s.handle("help gaher").text;
    assert!(
        wrong.contains("Did you mean") && wrong.contains("gather"),
        "{wrong}"
    );
}
