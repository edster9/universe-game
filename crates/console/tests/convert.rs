//! The converter (docs/ideas/world-building.md): a world exported from
//! Blender becomes the world's data file, and the engine loads it. These
//! tests use a small export written by hand, so they run without Blender: a
//! place, a thing, and a source shown by three stones.

use std::path::{Path, PathBuf};

const CATALOGUE: &str = r#"
[[entry]]
id = "big-rock"
is = "thing"
label = "a big rock"
material = "rock"
mass = "2 t"

[[entry]]
id = "stones"
is = "source"
label = "loose stones"
material = "rock"
each = "50 kg"
pieces = { size = "1 kg", find_time = "5 s" }

[[entry]]
id = "flowers"
is = "scenery"
"#;

/// A node's extras, as Blender exports custom properties.
fn node(name: &str, at: [f64; 3], extras: &str, children: &[usize]) -> String {
    let children = if children.is_empty() {
        String::new()
    } else {
        format!(
            r#", "children": [{}]"#,
            children
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let extras = if extras.is_empty() {
        String::new()
    } else {
        format!(r#", "extras": {{{extras}}}"#)
    };
    format!(
        r#"{{"name": "{name}", "translation": [{}, {}, {}]{extras}{children}}}"#,
        at[0], at[1], at[2]
    )
}

/// A world in a fresh folder: a manifest, the catalogue, and one exported
/// file, `tagged` being the rock's extras. glTF's z is south.
fn world(name: &str, rock: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("universe-convert-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(folder.join("exported/field")).unwrap();
    std::fs::write(folder.join("catalogue.toml"), CATALOGUE).unwrap();
    std::fs::write(
        folder.join("field.world.toml"),
        r#"
files = ["field"]
uses = ["island-things.toml"]
catalogue = "catalogue.toml"
exported = "exported"
output = "field.toml"
[world]
seed = 1
"#,
    )
    .unwrap();
    let nodes = [
        node(
            "place: field",
            [0.0, 0.0, 0.0],
            r#""ug_place": "field", "ug_label": "the field", "ug_size": "20 m""#,
            &[],
        ),
        node("thing: rock", [3.0, 0.0, -4.0], rock, &[]),
        // A source 5 m east, 2 m south, shown by three stones up to 1.5 m out.
        node(
            "source: stones",
            [5.0, 0.0, 2.0],
            r#""ug_entry": "stones", "ug_id": "stones""#,
            &[3, 4, 5],
        ),
        node("stone 1", [1.5, 0.0, 0.0], "", &[]),
        node("stone 2", [0.0, 0.0, 1.0], "", &[]),
        node("stone 3", [-0.5, 0.0, 0.0], "", &[]),
        // Decoration: scenery, never heard of.
        node("flowers", [1.0, 0.0, 1.0], r#""ug_entry": "flowers""#, &[]),
        node("ground", [0.0, 0.0, 0.0], "", &[]),
    ];
    std::fs::write(
        folder.join("exported/field/field.gltf"),
        format!(
            r#"{{"scene": 0, "scenes": [{{"nodes": [0, 1, 2, 6, 7]}}], "nodes": [{}]}}"#,
            nodes.join(", ")
        ),
    )
    .unwrap();
    folder
}

#[test]
fn a_world_from_blender_loads_with_its_things_and_sources() {
    let folder = world("loads", r#""ug_entry": "big-rock", "ug_id": "rock""#);
    let said = console::convert::convert(&folder.join("field.world.toml")).unwrap();
    assert!(said.contains("1 places"), "{said}");
    let text = std::fs::read_to_string(folder.join("field.toml")).unwrap();
    // The source's mass is its three stones', and it spreads as far as they
    // stand; the rock is where it was put, 4 m north and 3 m east.
    assert!(text.contains(r#"mass = "150 kg""#), "{text}");
    assert!(text.contains(r#"spread = "1.5 m""#), "{text}");
    assert!(
        text.contains(r#"spot = "4.0 m north, 3.0 m east""#),
        "{text}"
    );
    assert!(
        text.contains(r#"spot = "2.0 m south, 5.0 m east""#),
        "{text}"
    );
    // Scenery isn't in the world at all.
    assert!(!text.contains("flowers"), "{text}");
    // With the islanders' library, where rock is a material.
    let library = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/island-things.toml"
    ))
    .unwrap();
    let world = engine::data::load_world_with(&text, &[&library]).unwrap();
    let names: Vec<String> = world.entities().map(|id| world.label(id)).collect();
    assert!(names.iter().any(|n| n == "a big rock"), "{names:?}");
    assert!(names.iter().any(|n| n == "loose stones"), "{names:?}");
}

#[test]
fn a_tag_the_catalogue_doesnt_know_is_refused() {
    let folder = world("unknown", r#""ug_entry": "boulder", "ug_id": "rock""#);
    let e = console::convert::convert(&folder.join("field.world.toml")).unwrap_err();
    assert!(e.contains("boulder, which isn't in the catalogue"), "{e}");
    assert!(!Path::new(&folder.join("field.toml")).exists());
}

#[test]
fn a_thing_without_an_id_or_with_anothers_is_refused() {
    let folder = world("no-id", r#""ug_entry": "big-rock""#);
    let e = console::convert::convert(&folder.join("field.world.toml")).unwrap_err();
    assert!(e.contains("needs a `ug_id`"), "{e}");
    let folder = world("same-id", r#""ug_entry": "big-rock", "ug_id": "stones""#);
    let e = console::convert::convert(&folder.join("field.world.toml")).unwrap_err();
    assert!(e.contains("two things are called stones"), "{e}");
}
