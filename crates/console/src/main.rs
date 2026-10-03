//! The text console for the early slices. Usage:
//!
//!     cargo run -p console [-- --world <file.toml>] [--as <person-id>]
//!     cargo run -p console -- --script <file.txt>
//!     cargo run -p console -- --convert <blender/world.world.toml>
//!     cargo run -p console -- --world <file.toml> --as <person-id> --live [--real-time]
//!     cargo run -p console -- --load <save> [--live [--real-time]]
//!
//! `--live` answers each command with a line of JSON, for a program to drive
//! a person; `--real-time` runs the world with the wall clock. `--load`
//! picks up a save from the `saves` folder beside the data folder (`/save`,
//! `/load`); the console, not the live channel, saves as "last" on quitting.
//!
//! Lines can also be piped in, which is how scripted sessions run.

use std::io::{self, BufRead, IsTerminal, Write};
use std::process::ExitCode;

use console::session::Session;

const DEFAULT_WORLD: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/slice0.toml");
const SAVES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../saves");

fn main() -> ExitCode {
    // `--script <file>` plays a script and prints what happened.
    let args: Vec<String> = std::env::args().skip(1).collect();
    // `--convert <manifest>` makes a world built in Blender into its data
    // file (blender/convert.sh runs it after exporting).
    if let [flag, path] = args.as_slice()
        && flag == "--convert"
    {
        return match console::convert::convert(std::path::Path::new(path)) {
            Ok(said) => {
                println!("{said}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        };
    }
    if let [flag, path] = args.as_slice()
        && flag == "--script"
    {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("can't read {path}: {e}");
                return ExitCode::FAILURE;
            }
        };
        let data = std::path::Path::new(DEFAULT_WORLD)
            .parent()
            .expect("the data folder");
        return match console::script::run(&text, data) {
            Ok(report) => {
                println!("{}\n\nThe script passed.", report.transcript.join("\n\n"));
                ExitCode::SUCCESS
            }
            Err(failure) => {
                eprintln!("The script failed: {failure}");
                ExitCode::FAILURE
            }
        };
    }

    let mut world_path = DEFAULT_WORLD.to_string();
    let mut player = "traveller".to_string();
    let (mut live, mut real_time) = (false, false);
    let mut load = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--live" => live = true,
            "--real-time" => real_time = true,
            "--world" | "--as" | "--load" => match (arg.as_str(), args.next()) {
                ("--world", Some(path)) => world_path = path,
                ("--load", Some(name)) => load = Some(name),
                (_, Some(id)) => player = id,
                _ => {
                    eprintln!("{arg} needs a value");
                    return ExitCode::FAILURE;
                }
            },
            _ => {
                eprintln!(
                    "usage: console [--world <file.toml>] [--as <person-id>] [--load <save>] [--live [--real-time]]"
                );
                return ExitCode::FAILURE;
            }
        }
    }

    let saves = std::path::PathBuf::from(SAVES);
    let started = match load {
        Some(name) => Session::resume(saves, &name),
        None => console::load_world_file(std::path::Path::new(&world_path))
            .and_then(|world| Session::new(world, &player))
            .map(|session| session.with_saves(saves)),
    };
    let mut session = match started {
        Ok(session) => session,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    if live {
        return match console::live::run(session, real_time) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        };
    }

    // When input is piped, echo each command so the transcript reads well.
    let interactive = io::stdin().is_terminal();
    println!("Universe console. Type \"help\" for commands.\n");
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
    // A game ends with a save, and the next can start from it.
    if interactive {
        match session.save(console::session::LAST) {
            Ok(text) => println!("{text}"),
            Err(e) => eprintln!("{e}"),
        }
    }
    ExitCode::SUCCESS
}
