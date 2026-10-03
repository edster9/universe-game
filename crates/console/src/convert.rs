//! The converter: a world built in Blender (docs/ideas/world-building.md),
//! exported as glTF with its tags as extras, made into the world's data file
//! for the engine. It reads a manifest (which files make the world, and the
//! world's settings) and the catalogue (what each entry means), checks every
//! tag against them, and writes places, people and creatures, and items.
//!
//! What the tags mean (see blender/README.md):
//! - `ug_place` marks a place's middle, with `ug_label`, `ug_size`, and its
//!   temperatures. Places in one file all lead to each other, as far apart as
//!   their middles.
//! - `ug_entry` and `ug_id` mark a thing or a source, by its catalogue
//!   entry. A source's children are the models that show it: how many there
//!   are gives its mass (with the entry's `each`), and how far they reach
//!   from it, its spread.
//! - Anything else is scenery, which the engine never hears of.
//!
//! glTF's axes are Blender's turned: x east, y up, and z south.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde_json::Value as Json;
use toml::Value as Toml;

/// A column-major 4×4 transform, as glTF keeps them.
type Matrix = [f64; 16];

const IDENTITY: Matrix = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let mut m = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            m[col * 4 + row] = (0..4).map(|k| a[k * 4 + row] * b[col * 4 + k]).sum();
        }
    }
    m
}

fn apply(m: &Matrix, p: [f64; 3]) -> [f64; 3] {
    [
        m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12],
        m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13],
        m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14],
    ]
}

fn numbers<const N: usize>(v: Option<&Json>, default: [f64; N]) -> [f64; N] {
    let mut out = default;
    if let Some(list) = v.and_then(Json::as_array) {
        for (o, x) in out.iter_mut().zip(list) {
            *o = x.as_f64().unwrap_or(*o);
        }
    }
    out
}

/// A node's own transform: its matrix, or its translation, rotation, and
/// scale.
fn local(node: &Json) -> Matrix {
    if node.get("matrix").is_some() {
        return numbers(node.get("matrix"), IDENTITY);
    }
    let [tx, ty, tz] = numbers(node.get("translation"), [0.0; 3]);
    let [x, y, z, w] = numbers(node.get("rotation"), [0.0, 0.0, 0.0, 1.0]);
    let [sx, sy, sz] = numbers(node.get("scale"), [1.0; 3]);
    [
        (1.0 - 2.0 * (y * y + z * z)) * sx,
        (2.0 * (x * y + z * w)) * sx,
        (2.0 * (x * z - y * w)) * sx,
        0.0,
        (2.0 * (x * y - z * w)) * sy,
        (1.0 - 2.0 * (x * x + z * z)) * sy,
        (2.0 * (y * z + x * w)) * sy,
        0.0,
        (2.0 * (x * z + y * w)) * sz,
        (2.0 * (y * z - x * w)) * sz,
        (1.0 - 2.0 * (x * x + y * y)) * sz,
        0.0,
        tx,
        ty,
        tz,
        1.0,
    ]
}

/// An exported scene: every node's transform in the world, and its parent.
struct Scene {
    nodes: Vec<Json>,
    world: Vec<Matrix>,
    meshes: Vec<Json>,
    accessors: Vec<Json>,
}

impl Scene {
    fn read(gltf: &Json) -> Scene {
        let list = |key: &str| gltf[key].as_array().cloned().unwrap_or_default();
        let nodes = list("nodes");
        let mut world = vec![IDENTITY; nodes.len()];
        let roots: Vec<usize> =
            gltf["scenes"][gltf["scene"].as_u64().unwrap_or(0) as usize]["nodes"]
                .as_array()
                .map(|r| {
                    r.iter()
                        .filter_map(|n| n.as_u64())
                        .map(|n| n as usize)
                        .collect()
                })
                .unwrap_or_default();
        let mut stack: Vec<(usize, Matrix)> = roots.iter().map(|&r| (r, IDENTITY)).collect();
        while let Some((n, parent)) = stack.pop() {
            let Some(node) = nodes.get(n) else { continue };
            let m = multiply(&parent, &local(node));
            world[n] = m;
            for child in node["children"].as_array().into_iter().flatten() {
                if let Some(c) = child.as_u64() {
                    stack.push((c as usize, m));
                }
            }
        }
        Scene {
            nodes,
            world,
            meshes: list("meshes"),
            accessors: list("accessors"),
        }
    }

    /// Where a node is: metres east and north.
    fn at(&self, n: usize) -> (f64, f64) {
        let m = &self.world[n];
        // Adding nought turns a negative zero into a plain one.
        (m[12] + 0.0, -m[14] + 0.0)
    }

    fn children(&self, n: usize) -> Vec<usize> {
        self.nodes[n]["children"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|c| c.as_u64().map(|c| c as usize))
            .collect()
    }

    /// How far a node's meshes, and its children's, reach across the
    /// ground from `middle`.
    fn reach(&self, n: usize, middle: (f64, f64)) -> f64 {
        let mut most: f64 = 0.0;
        let mut stack = vec![n];
        while let Some(n) = stack.pop() {
            if let Some(mesh) = self.nodes[n]["mesh"].as_u64() {
                for primitive in self.meshes[mesh as usize]["primitives"]
                    .as_array()
                    .into_iter()
                    .flatten()
                {
                    let Some(a) = primitive["attributes"]["POSITION"].as_u64() else {
                        continue;
                    };
                    let accessor = &self.accessors[a as usize];
                    let lo = numbers(accessor.get("min"), [0.0; 3]);
                    let hi = numbers(accessor.get("max"), [0.0; 3]);
                    for corner in 0..8 {
                        let p = [
                            if corner & 1 == 0 { lo[0] } else { hi[0] },
                            if corner & 2 == 0 { lo[1] } else { hi[1] },
                            if corner & 4 == 0 { lo[2] } else { hi[2] },
                        ];
                        let q = apply(&self.world[n], p);
                        most = most.max((q[0] - middle.0).hypot(-q[2] - middle.1));
                    }
                }
            }
            stack.extend(self.children(n));
        }
        most
    }
}

/// A place, from its marker.
struct Place {
    id: String,
    file: String,
    label: String,
    at: (f64, f64),
    size: f64,
    temperature: Option<String>,
    night: Option<String>,
}

/// A thing or a source, from its marker.
struct Placed {
    id: String,
    entry: String,
    at: (f64, f64),
    label: Option<String>,
    mass: Option<String>,
    /// For a source: how many models show it, and how far it spreads.
    shown: usize,
    spread: f64,
}

/// A mass, in grams, from "2 t", "50 kg", "5 g".
fn grams(text: &str) -> Result<f64, String> {
    let (number, unit) = text
        .trim()
        .split_once(' ')
        .ok_or_else(|| format!("\"{text}\" isn't a mass"))?;
    let n: f64 = number
        .parse()
        .map_err(|_| format!("\"{text}\" isn't a mass"))?;
    Ok(n * match unit {
        "t" => 1e6,
        "kg" => 1e3,
        "g" => 1.0,
        _ => return Err(format!("\"{text}\": the converter knows t, kg, and g")),
    })
}

fn mass_text(g: f64) -> String {
    let (n, unit) = if g >= 1e6 {
        (g / 1e6, "t")
    } else if g >= 1e3 {
        (g / 1e3, "kg")
    } else {
        (g, "g")
    };
    let n = format!("{n:.2}");
    let n = n.trim_end_matches('0').trim_end_matches('.');
    format!("{n} {unit}")
}

fn metres(text: &str) -> Result<f64, String> {
    text.trim()
        .strip_suffix(" m")
        .and_then(|n| n.trim().parse().ok())
        .ok_or_else(|| format!("\"{text}\" isn't a length in metres"))
}

/// "4.0 m north, 2.8 m west", from the middle of a place.
fn spot(from: (f64, f64), to: (f64, f64)) -> String {
    let (east, north) = (to.0 - from.0, to.1 - from.1);
    format!(
        "{:.1} m {}, {:.1} m {}",
        north.abs(),
        if north < 0.0 { "south" } else { "north" },
        east.abs(),
        if east < 0.0 { "west" } else { "east" }
    )
}

fn string(extras: &Json, key: &str) -> Option<String> {
    extras.get(key).and_then(|v| match v {
        Json::String(s) => Some(s.clone()),
        Json::Number(n) => Some(n.to_string()),
        _ => None,
    })
}

fn quoted(s: &str) -> String {
    Toml::String(s.to_string()).to_string()
}

/// Converts the world a manifest describes; returns what it wrote, and where.
pub fn convert(manifest: &Path) -> Result<String, String> {
    let folder = manifest.parent().unwrap_or(Path::new("."));
    let read = |path: &Path| {
        std::fs::read_to_string(path).map_err(|e| format!("can't read {}: {e}", path.display()))
    };
    let man: toml::Table =
        toml::from_str(&read(manifest)?).map_err(|e| format!("{}: {e}", manifest.display()))?;
    let field = |key: &str| {
        man.get(key)
            .and_then(Toml::as_str)
            .ok_or_else(|| format!("the manifest needs `{key}`"))
    };
    let catalogue_path = folder.join(field("catalogue")?);
    let catalogue: toml::Table = toml::from_str(&read(&catalogue_path)?)
        .map_err(|e| format!("{}: {e}", catalogue_path.display()))?;
    let entries: BTreeMap<String, toml::Table> = catalogue
        .get("entry")
        .and_then(Toml::as_array)
        .into_iter()
        .flatten()
        .filter_map(|e| e.as_table())
        .filter_map(|e| Some((e.get("id")?.as_str()?.to_string(), e.clone())))
        .collect();
    let files: Vec<String> = man
        .get("files")
        .and_then(Toml::as_array)
        .into_iter()
        .flatten()
        .filter_map(|f| f.as_str().map(String::from))
        .collect();

    let mut places: Vec<Place> = Vec::new();
    let mut placed: Vec<(String, Placed)> = Vec::new();
    let mut problems: Vec<String> = Vec::new();
    for file in &files {
        let path = folder
            .join(field("exported")?)
            .join(file)
            .join(format!("{file}.gltf"));
        let gltf: Json =
            serde_json::from_str(&read(&path)?).map_err(|e| format!("{}: {e}", path.display()))?;
        let scene = Scene::read(&gltf);
        for (n, node) in scene.nodes.iter().enumerate() {
            let Some(extras) = node.get("extras") else {
                continue;
            };
            let name = node["name"].as_str().unwrap_or("?");
            if let Some(id) = string(extras, "ug_place") {
                let size = string(extras, "ug_size").unwrap_or_else(|| "30 m".into());
                places.push(Place {
                    id,
                    file: file.clone(),
                    label: string(extras, "ug_label").unwrap_or_else(|| name.to_string()),
                    at: scene.at(n),
                    size: metres(&size)?,
                    temperature: string(extras, "ug_temperature"),
                    night: string(extras, "ug_night"),
                });
                continue;
            }
            let Some(entry) = string(extras, "ug_entry") else {
                continue;
            };
            let Some(def) = entries.get(&entry) else {
                problems.push(format!(
                    "{file}: \"{name}\" is tagged {entry}, which isn't in the catalogue"
                ));
                continue;
            };
            let is = def.get("is").and_then(Toml::as_str).unwrap_or("");
            if is == "scenery" {
                continue;
            }
            let Some(id) = string(extras, "ug_id") else {
                problems.push(format!("{file}: \"{name}\" ({entry}) needs a `ug_id`"));
                continue;
            };
            let at = scene.at(n);
            let shown = scene.children(n);
            // A source spreads as far as its models stand from its middle;
            // a single model in the middle, as far as it reaches.
            let spread = shown
                .iter()
                .map(|&c| {
                    let (x, y) = scene.at(c);
                    (x - at.0).hypot(y - at.1)
                })
                .fold(0.0, f64::max);
            let spread = if spread < 0.1 {
                shown
                    .iter()
                    .map(|&c| scene.reach(c, at))
                    .fold(0.0, f64::max)
            } else {
                spread
            };
            placed.push((
                file.clone(),
                Placed {
                    id,
                    entry,
                    at,
                    label: string(extras, "ug_label"),
                    mass: string(extras, "ug_mass"),
                    shown: shown.len(),
                    spread: spread.max(0.5),
                },
            ));
        }
    }

    // Every id once.
    let mut seen = BTreeSet::new();
    for id in places
        .iter()
        .map(|p| &p.id)
        .chain(placed.iter().map(|(_, p)| &p.id))
    {
        if !seen.insert(id.clone()) {
            problems.push(format!(
                "two things are called {id}: give one a new `ug_id`"
            ));
        }
    }
    // Each thing is in the smallest place whose circle it's in.
    let place_of = |file: &str, at: (f64, f64)| {
        places
            .iter()
            .filter(|p| p.file == file && (at.0 - p.at.0).hypot(at.1 - p.at.1) <= p.size)
            .min_by(|a, b| a.size.total_cmp(&b.size))
    };

    let mut out = String::new();
    out.push_str(&format!(
        "# Made by the converter from {}; don't edit it here. Change the world in\n\
         # Blender and run blender/convert.sh again (docs/ideas/world-building.md).\n",
        manifest.display()
    ));
    if let Some(uses) = man.get("uses") {
        out.push_str(&format!("uses = {uses}\n"));
    }
    if let Some(world) = man.get("world").and_then(Toml::as_table) {
        out.push_str("\n[world]\n");
        out.push_str(&toml::to_string(world).map_err(|e| e.to_string())?);
    }

    for p in &places {
        out.push_str("\n[[place]]\n");
        out.push_str(&format!(
            "id = {}\nlabel = {}\n",
            quoted(&p.id),
            quoted(&p.label)
        ));
        // Places in one file lead to each other, as far apart as their
        // middles.
        let others: Vec<&Place> = places
            .iter()
            .filter(|o| o.file == p.file && o.id != p.id)
            .collect();
        if !others.is_empty() {
            let exits: Vec<String> = others.iter().map(|o| quoted(&o.id)).collect();
            out.push_str(&format!("exits = [{}]\n", exits.join(", ")));
            let distances: Vec<String> = others
                .iter()
                .map(|o| {
                    let d = (o.at.0 - p.at.0).hypot(o.at.1 - p.at.1);
                    format!("{} = \"{d:.1} m\"", quoted(&o.id))
                })
                .collect();
            out.push_str(&format!("distances = {{ {} }}\n", distances.join(", ")));
        }
        out.push_str(&format!(
            "east = \"{:.1} m\"\nnorth = \"{:.1} m\"\nsize = \"{:.1} m\"\n",
            p.at.0, p.at.1, p.size
        ));
        if let Some(t) = &p.temperature {
            out.push_str(&format!("temperature = {}\n", quoted(t)));
        }
        if let Some(t) = &p.night {
            out.push_str(&format!("night = {}\n", quoted(t)));
        }
    }

    let mut agents = String::new();
    let mut items = String::new();
    for (file, p) in &placed {
        let def = &entries[&p.entry];
        let get = |key: &str| def.get(key);
        let Some(place) = place_of(file, p.at) else {
            problems.push(format!("{file}: {} isn't inside any place's circle", p.id));
            continue;
        };
        let label = p
            .label
            .clone()
            .or_else(|| get("label").and_then(Toml::as_str).map(String::from))
            .unwrap_or_else(|| p.id.clone());
        let mut lines = format!(
            "id = {}\nlabel = {}\nat = {}\nspot = \"{}\"\n",
            quoted(&p.id),
            quoted(&label),
            quoted(&place.id),
            spot(place.at, p.at)
        );
        let copy = |lines: &mut String, keys: &[&str]| {
            for key in keys {
                if let Some(v) = get(key) {
                    lines.push_str(&format!("{key} = {v}\n"));
                }
            }
        };
        if get("agent").and_then(Toml::as_bool) == Some(true) {
            copy(&mut lines, &["kind", "culture", "temperature", "rules"]);
            // A creature keeps to the places of its own file.
            if get("rules").is_none() {
                let range: Vec<String> = places
                    .iter()
                    .filter(|o| o.file == *file)
                    .map(|o| quoted(&o.id))
                    .collect();
                lines.push_str(&format!("range = [{}]\n", range.join(", ")));
            }
            agents.push_str(&format!("\n[[agent]]\n{lines}"));
            continue;
        }
        let is = get("is").and_then(Toml::as_str).unwrap_or("");
        let mass = match (&p.mass, get("each").and_then(Toml::as_str), get("mass")) {
            (Some(m), _, _) => m.clone(),
            (None, Some(each), _) if is == "source" => {
                if p.shown == 0 {
                    problems.push(format!(
                        "{file}: the source {} has no models showing it",
                        p.id
                    ));
                    continue;
                }
                mass_text(grams(each)? * p.shown as f64)
            }
            (None, _, Some(m)) => m.as_str().unwrap_or("").to_string(),
            _ => {
                problems.push(format!("{file}: {} has no mass", p.id));
                continue;
            }
        };
        lines.push_str(&format!("mass = {}\n", quoted(&mass)));
        lines.push_str("fixed = true\n");
        copy(&mut lines, &["material", "composition", "kind", "pieces"]);
        let spread = if is == "source" { p.spread } else { 1.0 };
        lines.push_str(&format!("spread = \"{spread:.1} m\"\n"));
        if let Some(grows) = get("grows").and_then(Toml::as_table) {
            // It grows from the nearest source of the named entry in its
            // place, up to a share of its own mass.
            let from = grows.get("from").and_then(Toml::as_str).unwrap_or("");
            let nearest = placed
                .iter()
                .filter(|(f, o)| {
                    f == file
                        && o.entry == from
                        && place_of(f, o.at).is_some_and(|op| op.id == place.id)
                })
                .min_by(|(_, a), (_, b)| {
                    let d = |o: &Placed| (o.at.0 - p.at.0).hypot(o.at.1 - p.at.1);
                    d(a).total_cmp(&d(b))
                });
            let Some((_, source)) = nearest else {
                problems.push(format!(
                    "{file}: {} grows from {from}, but there's none in {}",
                    p.id, place.id
                ));
                continue;
            };
            let share = grows
                .get("limit")
                .and_then(Toml::as_str)
                .and_then(|l| l.trim_end_matches('%').trim().parse::<f64>().ok())
                .unwrap_or(100.0);
            let rate = grows.get("rate").and_then(Toml::as_str).unwrap_or("10%");
            lines.push_str(&format!(
                "grows = {{ rate = {}, limit = {}, from = {} }}\n",
                quoted(rate),
                quoted(&mass_text(grams(&mass)? * share / 100.0)),
                quoted(&source.id)
            ));
        }
        items.push_str(&format!("\n[[item]]\n{lines}"));
    }
    if !problems.is_empty() {
        return Err(problems.join("\n"));
    }
    out.push_str(&agents);
    out.push_str(&items);
    let output = folder.join(field("output")?);
    std::fs::write(&output, &out).map_err(|e| format!("can't write {}: {e}", output.display()))?;
    Ok(format!(
        "Wrote {}: {} places, {} people and creatures, {} things and sources.",
        output.display(),
        places.len(),
        agents.matches("[[agent]]").count(),
        items.matches("[[item]]").count()
    ))
}
