//! The live channel: a way for a program (Claude, a test harness, later a
//! browser's server) to drive a person. It takes the same commands a person
//! types, one a line, and answers each with one line of JSON: the reply, the
//! time, whether the person is busy, and what they see and carry, the same
//! views the console prints as text.
//!
//! With `--real-time`, the world runs with the wall clock: before each
//! command, it catches up on the real seconds since the last. Otherwise the
//! clock moves only as commands and `wait` move it.

use std::io::{self, BufRead, Write};
use std::time::Instant;

use engine::view::{self, Thing};

use crate::session::{Reply, Session};

/// Plays commands from standard input, answering each on standard output.
pub fn run(mut session: Session, real_time: bool) -> io::Result<()> {
    let mut out = io::stdout().lock();
    writeln!(out, "{}", state(&session, None))?;
    let mut last = Instant::now();
    for line in io::stdin().lock().lines() {
        let line = line?;
        if real_time {
            let elapsed = last.elapsed().as_secs();
            if elapsed > 0 {
                session.advance(elapsed);
                last += std::time::Duration::from_secs(elapsed);
            }
        }
        let reply = session.handle(&line);
        writeln!(out, "{}", state(&session, Some(&reply)))?;
        out.flush()?;
        if reply.quit {
            break;
        }
    }
    Ok(())
}

/// The session's state, and the last reply, as one line of JSON.
pub fn state(session: &Session, reply: Option<&Reply>) -> String {
    let world = session.world();
    let me = session.player();
    let mut fields = vec![
        ("tick", world.tick().to_string()),
        ("clock", text(&session.clock(world.tick()))),
        ("as", text(world.key(me))),
        ("name", text(&world.label(me))),
        ("alive", world.is_living(me).to_string()),
        (
            "busy_until",
            world
                .pending(me)
                .map_or("null".into(), |p| text(&session.clock(p.until))),
        ),
    ];
    if let Some(reply) = reply {
        fields.push(("reply", text(&reply.text)));
        fields.push(("refused", reply.refused.to_string()));
    }
    if let Some(look) = view::look(world, me) {
        fields.push((
            "view",
            object(&[
                ("place", text(&look.place)),
                ("exits", list(look.exits.iter().map(|e| text(e)))),
                ("people", list(look.people.iter().map(|p| text(p)))),
                ("things", list(look.things.iter().map(thing))),
                ("air", list(look.air.iter().map(thing))),
                ("night", look.night.to_string()),
                ("dark", look.dark.to_string()),
            ]),
        ));
    }
    let carried = view::inventory(world, me);
    fields.push((
        "carrying",
        object(&[
            ("things", list(carried.things.iter().map(thing))),
            ("mass_mg", carried.carried.mg().to_string()),
        ]),
    ));
    object(&fields)
}

fn thing(t: &Thing) -> String {
    object(&[
        ("label", text(&t.label)),
        ("mass_mg", t.mass.mg().to_string()),
        (
            "temperature_mk",
            t.temperature.map_or("null".into(), |k| k.mk().to_string()),
        ),
        ("notes", list(t.notes.iter().map(|n| text(n)))),
        ("contents", list(t.contents.iter().map(thing))),
    ])
}

fn object(fields: &[(&str, String)]) -> String {
    let inner: Vec<String> = fields
        .iter()
        .map(|(k, v)| format!("{}:{v}", text(k)))
        .collect();
    format!("{{{}}}", inner.join(","))
}

fn list(items: impl Iterator<Item = String>) -> String {
    format!("[{}]", items.collect::<Vec<_>>().join(","))
}

/// A JSON string.
fn text(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
