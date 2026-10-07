//! Seeing at a distance: near enough, a thing is made out and called by
//! the viewer's own word; further off, it's only "something", which can be
//! pointed at and walked up to but not named; further still, it isn't seen.
//! Sizes are measured, so a thing made in play is seen by the same law. See
//! docs/ideas/context-menu.md.

use std::path::Path;

use console::session::Session;
use engine::sight::{Sight, sight};

fn islander() -> Session {
    let data = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../data"));
    let world = console::load_world_file(&data.join("companion.toml"))
        .unwrap()
        .with_player_rules("survivor")
        .unwrap();
    Session::new(world, "survivor").unwrap()
}

fn ok(s: &mut Session, line: &str) -> String {
    let reply = s.handle(line);
    assert!(!reply.refused, "{line}: {}", reply.text);
    reply.text
}

#[test]
fn small_things_far_off_are_only_something() {
    let mut s = islander();
    ok(&mut s, "go forest");
    ok(&mut s, "go to sticks; gather sticks");
    ok(&mut s, "drop wood");
    let stick = *s
        .world()
        .held(s.world().place_of(s.player()).unwrap())
        .last()
        .unwrap();
    assert_eq!(sight(s.world(), s.player(), stick), Sight::MadeOut);

    // Across the forest, a 200 g stick, about 7 cm, is too far to make out.
    ok(&mut s, "walk to -25 -20");
    let (world, me) = (s.world(), s.player());
    assert_eq!(sight(world, me, stick), Sight::Seen);
    let look = ok(&mut s, "look");
    assert!(look.contains("something small ("), "{look}");
    assert!(!look.contains("lump of wood"), "{look}");
    // It can't be named, only pointed at, and the menu only walks up to it.
    let menu = console::menu::menu(s.world(), s.player(), stick);
    let labels: Vec<&str> = menu.iter().map(|c| c.label.as_str()).collect();
    assert_eq!(labels, ["walk up to it"]);
    assert!(s.handle("walk to wood stick").refused);
    let walked = ok(&mut s, &menu[0].line);
    // Up close, it's a wood stick again.
    assert!(walked.contains("wood stick"), "{walked}");
    assert_eq!(sight(s.world(), s.player(), stick), Sight::MadeOut);
}

#[test]
fn big_things_are_made_out_from_further_and_darkness_shortens_sight() {
    let mut s = islander();
    ok(&mut s, "go forest");
    ok(&mut s, "walk to -25 -20");
    let (world, me) = (s.world(), s.player());
    // A patch of sticks 20 m across is made out from across the forest.
    let sticks = world.find_by_key("sticks").unwrap();
    assert_eq!(sight(world, me, sticks), Sight::MadeOut);
    // A boar, by its body's size, too.
    let boar = world
        .entities()
        .find(|&e| world.place_of(e) == world.place_of(me) && world.label(e) == "a boar");
    if let Some(boar) = boar {
        assert_eq!(sight(world, me, boar), Sight::MadeOut);
    }
    // A stick 10 m off is made out by day; at night with no fire, it's only
    // something.
    ok(&mut s, "go to sticks; gather sticks");
    ok(&mut s, "drop wood");
    let stick = *s
        .world()
        .held(s.world().place_of(s.player()).unwrap())
        .last()
        .unwrap();
    ok(&mut s, "walk to 10 5");
    let gap = s.world().gap(s.player(), stick);
    assert!((5_000_000..30_000_000).contains(&gap), "{gap}");
    assert_eq!(sight(s.world(), s.player(), stick), Sight::MadeOut);
    ok(&mut s, "wait until 23:00");
    assert_eq!(sight(s.world(), s.player(), stick), Sight::Seen);
}

#[test]
fn something_made_in_play_is_measured_like_anything_else() {
    let mut s = islander();
    ok(&mut s, "go forest");
    ok(&mut s, "go hillside");
    ok(&mut s, "go to stones; gather stones x5");
    let stone = s.world().held(s.player())[0];
    ok(&mut s, "assemble fire ring");
    let (world, me) = (s.world(), s.player());
    let ring = world.held(me)[0];
    // Nobody wrote the ring's size: it's measured from its parts, so it's
    // bigger than one stone, and made out from further away.
    let (one, all) = (
        engine::sight::size(world, stone),
        engine::sight::size(world, ring),
    );
    assert!(all > one && one > 0, "a stone {one} µm, the ring {all} µm");
}

#[test]
fn tab_completes_only_names_for_what_is_made_out() {
    let mut s = islander();
    ok(&mut s, "go forest");
    ok(&mut s, "go to sticks; gather sticks");
    // Carried, it's a name to complete.
    assert!(
        s.names().iter().any(|n| n == "wood stick"),
        "{:?}",
        s.names()
    );
    ok(&mut s, "drop wood");
    assert!(
        s.names().iter().any(|n| n == "wood stick"),
        "{:?}",
        s.names()
    );
    // Across the forest it's only something small: no name to give away.
    ok(&mut s, "walk to -25 -20");
    let names = s.names();
    assert!(!names.iter().any(|n| n == "wood stick"), "{names:?}");
    assert!(
        !names.iter().any(|n| n.starts_with("something")),
        "{names:?}"
    );
}
