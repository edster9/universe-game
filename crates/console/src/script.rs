//! Scripts: plain text files of console commands with expectations. A script
//! proves that a sequence of steps works under the laws, or that a wrong
//! choice ends the way it should. With luck fixed, a script plays the same way
//! every time.
//!
//! One line each; indentation is ignored and `#` starts a comment.
//!
//! ```text
//! world stranded.toml        the data file to load (first, required)
//! as survivor                who to play (default: traveller)
//! luck average               average, good, bad, or seed <n>
//! include skills/x.txt       the lines of another file, from data/scripts
//! repeat 10                  repeat the lines up to the matching `end`
//!   try drink water          a command that may be refused
//!   go beach                 a command that must not be refused
//! end
//! expect alive               the player is alive
//! expect dead of thirst      the player died of thirst
//! expect said sea            the last reply mentions "sea"
//! expect not said burning    the last reply doesn't mention "burning"
//! rules player               the player lives by players' rules: on vitality, needing no food, drink, or sleep
//! expect conserved           mass, energy (apart from sunlight and vitality), and credits are as they started
//! expect not said burning    the last reply doesn't mention "burning"
//! expect time after 2 day    at least this much time has passed
//! expect time before 3 day   less than this much time has passed
//! ```

use std::path::Path;

use engine::units::{self, property};
use engine::world::Luck;

use crate::session::Session;

enum Node {
    Line { number: usize, text: String },
    Repeat { count: usize, body: Vec<Node> },
}

/// What a script did, as a transcript.
pub struct Report {
    pub transcript: Vec<String>,
}

/// Runs a script. `data_dir` is where world files are found. On failure, the
/// error says which line failed and why, followed by the end of the
/// transcript.
pub fn run(text: &str, data_dir: &Path) -> Result<Report, String> {
    let text = expand_includes(text, &data_dir.join("scripts"), 0)?;
    let mut lines = text
        .lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l.split('#').next().unwrap_or("").trim()))
        .filter(|(_, l)| !l.is_empty())
        .peekable();

    // Header: world, player, luck.
    let (mut world_file, mut player, mut luck) = (None, "traveller".to_string(), Luck::Seeded);
    let mut player_rules = false;
    let mut seed = None;
    while let Some(&(number, line)) = lines.peek() {
        let (word, rest) = line
            .split_once(' ')
            .map_or((line, ""), |(w, r)| (w, r.trim()));
        match word {
            "world" => world_file = Some(rest.to_string()),
            "as" => player = rest.to_string(),
            "rules" if rest == "player" => player_rules = true,
            "luck" => match rest {
                "average" => luck = Luck::AVERAGE,
                "good" => luck = Luck::GOOD,
                "bad" => luck = Luck::BAD,
                other => match other
                    .strip_prefix("seed ")
                    .and_then(|n| n.trim().parse().ok())
                {
                    Some(n) => seed = Some(n),
                    None => {
                        return Err(format!(
                            "line {number}: luck is average, good, bad, or seed <n>"
                        ));
                    }
                },
            },
            _ => break,
        }
        lines.next();
    }
    let world_file = world_file.ok_or("a script must start with \"world <file>\"")?;
    let mut world = crate::load_world_file(&data_dir.join(&world_file))?;
    world = match seed {
        Some(seed) => world.with_seed(seed),
        None => world.with_luck(luck),
    };
    if player_rules {
        world = world.with_player_rules(&player)?;
    }
    let start = (world.own_mass(), world.own_energy(), world.total_credits());
    let mut session = Session::new(world, &player)?;

    let body = parse_block(&mut lines, None)?;
    let mut transcript = Vec::new();
    run_block(&body, &mut session, &mut transcript, start).map_err(|e| {
        let tail = transcript.len().saturating_sub(12);
        format!(
            "{e}\n--- last lines of the transcript ---\n{}",
            transcript[tail..].join("\n")
        )
    })?;
    Ok(Report { transcript })
}

/// Replaces each `include <file>` line with that file's lines, so a recipe
/// (a skill, in effect) can be written once and used by many scripts. Paths
/// are relative to data/scripts.
fn expand_includes(text: &str, scripts: &Path, depth: usize) -> Result<String, String> {
    if depth > 8 {
        return Err("includes nest more than eight deep".into());
    }
    let mut out = Vec::new();
    for line in text.lines() {
        match line.trim().strip_prefix("include ") {
            Some(name) => {
                let name = name.split('#').next().unwrap_or("").trim();
                let included = std::fs::read_to_string(scripts.join(name))
                    .map_err(|e| format!("can't include {name}: {e}"))?;
                out.push(expand_includes(&included, scripts, depth + 1)?);
            }
            None => out.push(line.to_string()),
        }
    }
    Ok(out.join("\n"))
}

fn parse_block<'a>(
    lines: &mut std::iter::Peekable<impl Iterator<Item = (usize, &'a str)>>,
    opened_at: Option<usize>,
) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();
    while let Some((number, line)) = lines.next() {
        if line == "end" {
            return match opened_at {
                Some(_) => Ok(nodes),
                None => Err(format!("line {number}: \"end\" without \"repeat\"")),
            };
        }
        if let Some(count) = line.strip_prefix("repeat ") {
            let count = count
                .trim()
                .parse()
                .map_err(|_| format!("line {number}: repeat needs a number"))?;
            let body = parse_block(lines, Some(number))?;
            nodes.push(Node::Repeat { count, body });
        } else {
            nodes.push(Node::Line {
                number,
                text: line.to_string(),
            });
        }
    }
    match opened_at {
        Some(number) => Err(format!("line {number}: \"repeat\" without \"end\"")),
        None => Ok(nodes),
    }
}

type Totals = (u128, u128, u128);

fn run_block(
    nodes: &[Node],
    session: &mut Session,
    transcript: &mut Vec<String>,
    start: Totals,
) -> Result<(), String> {
    for node in nodes {
        match node {
            Node::Repeat { count, body, .. } => {
                for _ in 0..*count {
                    run_block(body, session, transcript, start)?;
                }
            }
            Node::Line { number, text } => run_line(*number, text, session, transcript, start)?,
        }
    }
    Ok(())
}

fn run_line(
    number: usize,
    line: &str,
    session: &mut Session,
    transcript: &mut Vec<String>,
    start: Totals,
) -> Result<(), String> {
    if let Some(expectation) = line.strip_prefix("expect ") {
        return check(number, expectation.trim(), session, transcript, start);
    }
    let (optional, command) = match line.strip_prefix("try ") {
        Some(command) => (true, command.trim()),
        None => (false, line),
    };
    let reply = session.handle(command);
    transcript.push(format!("> {command}\n{}", reply.text));
    if reply.refused && !optional {
        return Err(format!(
            "line {number}: \"{command}\" was refused: {}",
            reply.text
        ));
    }
    Ok(())
}

fn check(
    number: usize,
    expectation: &str,
    session: &Session,
    transcript: &[String],
    start: Totals,
) -> Result<(), String> {
    let world = session.world();
    let player = session.player();
    let fail = |why: String| Err(format!("line {number}: expected {expectation}, but {why}"));
    let died_of = world.life(player).and_then(|l| l.died_of.clone());
    let seconds = |text: &str| {
        units::parse_quantity(text, property::DURATION, "a time like \"2 day\"")
            .map_err(|e| format!("line {number}: {e}"))
    };

    if expectation == "conserved" {
        let now = (world.own_mass(), world.own_energy(), world.total_credits());
        return if now == start {
            Ok(())
        } else {
            fail(format!("totals went from {start:?} to {now:?}"))
        };
    }
    if expectation == "alive" {
        return match died_of {
            None => Ok(()),
            Some(cause) => fail(format!(
                "the player died of {cause} at {}",
                units::show_duration(world.tick())
            )),
        };
    }
    if let Some(cause) = expectation.strip_prefix("dead of ") {
        return match died_of {
            Some(actual) if actual == cause.trim() => Ok(()),
            Some(actual) => fail(format!("the player died of {actual}")),
            None => fail("the player is alive".into()),
        };
    }
    if let Some(text) = expectation.strip_prefix("not said ") {
        let last = transcript.last().map_or("", String::as_str).to_lowercase();
        return if last.contains(&text.trim().to_lowercase()) {
            fail(format!("the last reply was: {last}"))
        } else {
            Ok(())
        };
    }
    if let Some(text) = expectation.strip_prefix("not said ") {
        let last = transcript.last().map_or("", String::as_str).to_lowercase();
        return if last.contains(&text.trim().to_lowercase()) {
            fail(format!("the last reply was: {last}"))
        } else {
            Ok(())
        };
    }
    if let Some(text) = expectation.strip_prefix("said ") {
        let last = transcript.last().map_or("", String::as_str).to_lowercase();
        return if last.contains(&text.trim().to_lowercase()) {
            Ok(())
        } else {
            fail(format!("the last reply was: {last}"))
        };
    }
    if let Some(time) = expectation.strip_prefix("time after ") {
        let limit = seconds(time)?;
        return if world.tick() >= limit {
            Ok(())
        } else {
            fail(format!(
                "only {} has passed",
                units::show_duration(world.tick())
            ))
        };
    }
    if let Some(time) = expectation.strip_prefix("time before ") {
        let limit = seconds(time)?;
        return if world.tick() < limit {
            Ok(())
        } else {
            fail(format!("{} has passed", units::show_duration(world.tick())))
        };
    }
    Err(format!(
        "line {number}: I don't know how to expect \"{expectation}\""
    ))
}
