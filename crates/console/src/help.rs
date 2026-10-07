//! The console's help, from `help.toml`: `help` lists every command by
//! group, a line each; `help <command>` (or another word for it, or a
//! topic) tells all about one: every way to write it, what it does, what
//! it needs, examples, and what's related. Games' consoles work this way
//! (docs/requirements.md, "Help for every command"). `tests/help.rs` checks
//! every command the game accepts has its entry, and every entry is one.

use std::sync::LazyLock;

use serde::Deserialize;

#[derive(Deserialize)]
struct File {
    groups: Vec<String>,
    command: Vec<Entry>,
    topic: Vec<Topic>,
}

/// A command's help.
#[derive(Deserialize)]
pub struct Entry {
    pub name: String,
    #[serde(default)]
    pub also: Vec<String>,
    pub group: String,
    pub summary: String,
    pub usage: Vec<String>,
    pub about: String,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub see: Vec<String>,
}

/// Help on something that isn't one command: naming things, repeating.
#[derive(Deserialize)]
pub struct Topic {
    pub name: String,
    pub summary: String,
    pub about: String,
}

static HELP: LazyLock<File> = LazyLock::new(|| {
    toml::from_str(include_str!("../help.toml")).expect("help.toml is the console's help")
});

/// The groups the commands are listed in, in order.
pub fn groups() -> &'static [String] {
    &HELP.groups
}

/// Every command's help.
pub fn commands() -> &'static [Entry] {
    &HELP.command
}

/// Every topic.
pub fn topics() -> &'static [Topic] {
    &HELP.topic
}

/// The groups of commands for the person at the keyboard, not the
/// islander: left out of what a translator is given.
const NOT_PLAY: [&str; 2] = ["Tools: the person at the keyboard", "Testing"];

/// What `help` says, with or without a word after it.
pub fn reply(rest: &str) -> String {
    let word = rest.split_whitespace().next().unwrap_or("");
    if word.is_empty() {
        overview()
    } else {
        about(word).unwrap_or_else(|| unknown(word))
    }
}

/// Every command by group, a line each, and the topics.
pub fn overview() -> String {
    let mut lines = vec!["Commands (\"help <command>\" tells all about one):".to_string()];
    for group in groups() {
        lines.push(format!(" {group}"));
        for c in commands().iter().filter(|c| &c.group == group) {
            lines.push(format!("  {:<14}{}", c.name, c.summary));
        }
    }
    lines.push(" More".to_string());
    for t in topics() {
        lines.push(format!("  {:<14}{}", format!("help {}", t.name), t.summary));
    }
    lines.join("\n")
}

/// The entry for a command, or another word for one, or a slash tool
/// named without its slash.
pub fn entry(word: &str) -> Option<&'static Entry> {
    let word = word.to_lowercase();
    let is = |c: &&Entry| c.name == word || c.also.contains(&word);
    commands()
        .iter()
        .find(is)
        .or_else(|| commands().iter().find(|c| c.name == format!("/{word}")))
}

/// All about one command or topic.
pub fn about(word: &str) -> Option<String> {
    if let Some(c) = entry(word) {
        let also = if c.also.is_empty() {
            String::new()
        } else {
            let words: Vec<String> = c.also.iter().map(|a| format!("\"{a}\"")).collect();
            format!(" (also {})", words.join(", "))
        };
        let mut lines = vec![format!("{}{also}: {}", c.name, c.summary)];
        lines.extend(c.usage.iter().map(|u| format!("  {u}")));
        lines.extend(c.about.trim().lines().map(str::to_string));
        if !c.examples.is_empty() {
            lines.push("For example:".to_string());
            lines.extend(c.examples.iter().map(|e| format!("  {e}")));
        }
        if !c.see.is_empty() {
            lines.push(format!("See also: {}.", c.see.join(", ")));
        }
        return Some(lines.join("\n"));
    }
    let word = word.to_lowercase();
    topics()
        .iter()
        .find(|t| t.name == word)
        .map(|t| format!("{}: {}\n{}", t.name, t.summary, t.about.trim()))
}

/// Not a command or topic: those it might have been.
fn unknown(word: &str) -> String {
    let start: String = word.to_lowercase().chars().take(2).collect();
    let near: Vec<&str> = names()
        .into_iter()
        .filter(|n| n.trim_start_matches('/').starts_with(&start))
        .collect();
    if near.is_empty() {
        format!("There's no command or topic \"{word}\": \"help\" lists them all.")
    } else {
        format!(
            "There's no command or topic \"{word}\". Did you mean {}? (\"help\" lists them all.)",
            near.join(", ")
        )
    }
}

/// Every command, the other words for them, and the topics: what can follow
/// `help`, for Tab.
pub fn names() -> Vec<&'static str> {
    commands()
        .iter()
        .flat_map(|c| std::iter::once(c.name.as_str()).chain(c.also.iter().map(String::as_str)))
        .chain(topics().iter().map(|t| t.name.as_str()))
        .collect()
}

/// The commands a player uses (not the tools or testing), every way to
/// write each with what it does, and how to name things: what a translator
/// from plain words is given (`Session::scope`).
pub fn for_players() -> String {
    let mut lines = Vec::new();
    for c in commands()
        .iter()
        .filter(|c| !NOT_PLAY.contains(&c.group.as_str()))
    {
        for (n, usage) in c.usage.iter().enumerate() {
            if n == 0 {
                lines.push(format!("  {usage:<36}{}", c.summary));
            } else {
                lines.push(format!("  {usage}"));
            }
        }
    }
    for topic in ["repeat", "names"] {
        if let Some(t) = topics().iter().find(|t| t.name == topic) {
            lines.push(t.about.trim().to_string());
        }
    }
    lines.join("\n")
}
