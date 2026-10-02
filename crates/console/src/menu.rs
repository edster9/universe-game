//! What a client offers when a thing is clicked: the laws' own choices
//! (`laws::choices`), in the player's words, each with the command it sends.
//! Choosing one is the same as typing that command: the laws decide again.

use engine::laws;
use engine::world::{EntityId, World};

/// One choice: what the menu shows, the command it sends, and how that
/// command reads in the player's words, for the console's log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    pub label: String,
    pub line: String,
    pub said: String,
}

/// What `who` could do with `thing` now, for a menu.
pub fn menu(world: &World, who: EntityId, thing: EntityId) -> Vec<Choice> {
    let it = laws::pointer(thing);
    let mut menu = Vec::new();
    for choice in laws::choices(world, who, thing) {
        // Once each: how to choose how many times is still open (the
        // owner, 2026-10-01: canned "x3" and "x10" aren't good enough).
        let command = choice.intent.to_string();
        let label = if matches!(choice.intent, engine::intent::Intent::Walk { .. }) {
            "walk up to it".into()
        } else {
            words(world, who, &command, Some(&it))
        };
        let line = if choice.walk_first {
            format!("walk to {it}; {command}")
        } else {
            command
        };
        let said = words(world, who, &line, None);
        let keep = matches!(choice.intent, engine::intent::Intent::Gather { .. })
            .then(|| (format!("{line} until full"), format!("{said} until full")));
        menu.push(Choice { label, line, said });
        // Gathering can go on until the pack is full or the source is bare,
        // or the player stops it.
        if let Some((line, said)) = keep {
            menu.push(Choice {
                label: "keep gathering".into(),
                line,
                said,
            });
        }
    }
    menu
}

/// Text in the player's words: each "#12" by what they call it.
pub fn in_words(world: &World, who: EntityId, text: &str) -> String {
    words(world, who, text, None)
}

/// A command in the player's words: each "#12" by what they call it, or,
/// for the thing clicked (`it`), left out, as a menu under its name shows.
fn words(world: &World, who: EntityId, command: &str, it: Option<&str>) -> String {
    let tokens: Vec<&str> = command.split_whitespace().collect();
    let mut out: Vec<String> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        let next = tokens.get(i + 1).copied();
        if Some(*token) == it || (*token == "from" && next.is_some() && next == it) {
            continue;
        }
        // A pointer may end a command in a row, or a sentence: "#12;".
        let bare = token.trim_end_matches([';', ',', '.']);
        let end = &token[bare.len()..];
        out.push(match laws::pointed(world, bare) {
            Some(id) if it.is_some() => {
                format!(
                    "{} ({}){end}",
                    engine::sight::label(world, who, id),
                    world.weight(id)
                )
            }
            Some(id) => format!("{}{end}", engine::sight::label(world, who, id)),
            None => token.to_string(),
        });
    }
    out.join(" ")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::session::Session;

    #[test]
    fn a_click_offers_what_the_laws_allow_and_sends_a_command() {
        let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
        let world = crate::load_world_file(&data.join("companion.toml"))
            .unwrap()
            .with_player_rules("survivor")
            .unwrap();
        let mut session = Session::new(world, "survivor").unwrap();
        assert!(!session.handle("go forest").refused);
        let (world, me) = (session.world(), session.player());
        let sticks = world.find_by_key("sticks").unwrap();
        let labels = |menu: &[Choice]| menu.iter().map(|c| c.label.clone()).collect::<Vec<_>>();

        // Out of reach: walk up first.
        let far = menu(world, me, sticks);
        assert_eq!(labels(&far), ["walk up to it", "gather", "keep gathering"]);
        let line = far[1].line.clone();
        assert_eq!(
            line,
            format!("walk to {0}; gather from {0}", laws::pointer(sticks))
        );
        assert_eq!(
            far[1].said,
            "walk to fallen sticks; gather from fallen sticks"
        );
        let reply = session.handle(&line);
        assert!(!reply.refused, "{}", reply.text);
        assert!(
            reply.text.contains("You find 200 g of wood, a wood stick."),
            "{}",
            reply.text
        );
        // Told in the player's words, not by pointers.
        assert!(!reply.text.contains('#'), "{}", reply.text);

        // Within reach now, and carrying wood: no walking, and what's carried
        // can be dropped.
        let (world, me) = (session.world(), session.player());
        assert_eq!(
            labels(&menu(world, me, sticks)),
            ["gather", "keep gathering"]
        );
        let wood = world.held(me)[0];
        assert_eq!(labels(&menu(world, me, wood)), ["drop"]);
        // Something nobody here can see offers nothing.
        let flint = world.find_by_key("flint").unwrap();
        assert!(menu(world, me, flint).is_empty());
    }
}
