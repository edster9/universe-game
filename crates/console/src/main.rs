//! Slice 0 console. Usage:
//!
//!     cargo run -p console [-- --world <file.toml>] [--as <person-id>]
//!
//! Lines can also be piped in, which is how scripted sessions run.

use std::io::{self, BufRead, IsTerminal, Write};
use std::process::ExitCode;

use console::session::Session;
use engine::data::load_world;

const DEFAULT_WORLD: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/slice0.toml");

fn main() -> ExitCode {
    let mut world_path = DEFAULT_WORLD.to_string();
    let mut player = "traveller".to_string();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match (arg.as_str(), args.next()) {
            ("--world", Some(path)) => world_path = path,
            ("--as", Some(id)) => player = id,
            _ => {
                eprintln!("usage: console [--world <file.toml>] [--as <person-id>]");
                return ExitCode::FAILURE;
            }
        }
    }

    let text = match std::fs::read_to_string(&world_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("can't read {world_path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let world = match load_world(&text) {
        Ok(world) => world,
        Err(e) => {
            eprintln!("{world_path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let mut session = match Session::new(world, &player) {
        Ok(session) => session,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    // When input is piped, echo each command so the transcript reads well.
    let interactive = io::stdin().is_terminal();
    println!("Slice 0: the ledger. Type \"help\" for commands.\n");
    println!("{}", session.handle("look").text);
    for line in io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        if !interactive {
            println!("\n> {line}");
        }
        let reply = session.handle(&line);
        if !reply.text.is_empty() {
            println!("{}", reply.text);
        }
        if reply.quit {
            break;
        }
        if interactive {
            print!("> ");
            let _ = io::stdout().flush();
        }
    }
    ExitCode::SUCCESS
}
