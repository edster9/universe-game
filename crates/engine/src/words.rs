//! Words: what each person calls things. Names live in minds, not in the
//! world (docs/ideas/vocabulary.md). The world keeps the truth about every
//! thing and no catalogue of inventions; each person keeps their own words,
//! each a name and what it means to them, and recognises things by how they
//! look, never by how they were made.
//!
//! A person without words of their own comes from a world with no cultures,
//! and calls everything by its name from data, as before.

use std::collections::BTreeSet;

use crate::matter::{self, Composition, MaterialId, State};
use crate::units::Mass;
use crate::world::{EntityId, Requirement, Role, World, list_and};

/// Said of something made of a material nobody described.
const UNFAMILIAR: &str = "something unfamiliar";

/// What a word means to the person who knows it.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Meaning {
    /// A material, known when seen.
    Material(MaterialId),
    /// A shape, known when seen, whatever it's made of.
    Shape(String),
    /// A kind of creature or growing thing.
    Kind(String),
    /// Anything that looks like this example.
    Like(Look),
}

/// A way to make something: the parts to put together, and the design they
/// follow, if the way came with one.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Recipe {
    pub design: Option<String>,
    pub slots: Vec<(String, Requirement)>,
}

/// A person's own words, and the ways they know to make things.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct Lexicon {
    /// Each word and what it means. A word can mean several things, and
    /// several words can mean one.
    pub words: Vec<(String, Meaning)>,
    /// Ways to make things, by the word for what they make.
    pub recipes: Vec<(String, Recipe)>,
    /// What they made most recently, which "it" means.
    pub last_made: Option<EntityId>,
}

/// One part of a thing, as the eye sees it.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Part {
    /// A shaped piece, and what it's made of. An example learned from a
    /// design leaves the material open.
    Shaped {
        shape: String,
        material: Option<MaterialId>,
    },
    /// An unshaped piece of a material.
    Lump(MaterialId),
    /// Something itself put together from parts.
    Built,
}

/// How a thing looks: what it does, what it's made of, and how big it is.
/// Recognising compares looks.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Look {
    /// The jobs its parts do, apart from being held.
    pub does: BTreeSet<Role>,
    /// Its parts, in a fixed order.
    pub parts: Vec<Part>,
    /// How heavy it is, if known: an example learned from a design has no
    /// size.
    pub mass: Option<Mass>,
}

/// How closely a thing matches an example, least close first.
#[derive(
    serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum Closeness {
    /// It does the same jobs, at about the same size, from different parts.
    Like,
    /// The same parts at about the same size, of different materials.
    Same,
    /// The same parts, of the same materials, at about the same size.
    Exact,
}

impl Part {
    /// What kind of part it is, leaving out what it's made of.
    fn skeleton(&self) -> (u8, String) {
        match self {
            Part::Shaped { shape, .. } => (0, shape.clone()),
            Part::Lump(material) => (1, format!("{}", material.0)),
            Part::Built => (2, String::new()),
        }
    }
}

impl Look {
    /// How closely `thing` matches this example, if at all. Sizes must be
    /// within half to double of each other when both are known.
    pub fn resembles(&self, thing: &Look) -> Option<Closeness> {
        if let (Some(a), Some(b)) = (self.mass, thing.mass) {
            let (a, b) = (u128::from(a.mg()), u128::from(b.mg()));
            if b * 2 < a || b > a * 2 {
                return None;
            }
        }
        let skeleton = |l: &Look| l.parts.iter().map(Part::skeleton).collect::<Vec<_>>();
        if skeleton(self) == skeleton(thing) {
            let same_materials = self
                .parts
                .iter()
                .zip(&thing.parts)
                .all(|(a, b)| match (a, b) {
                    (Part::Shaped { material: None, .. }, _) => true,
                    (Part::Shaped { material: a, .. }, Part::Shaped { material: b, .. }) => a == b,
                    _ => true,
                });
            return Some(if same_materials {
                Closeness::Exact
            } else {
                Closeness::Same
            });
        }
        // Only an example seen at a size can stand for things merely like it.
        if self.mass.is_some() && !self.does.is_empty() && self.does == thing.does {
            return Some(Closeness::Like);
        }
        None
    }
}

impl World {
    /// Whether this person has words of their own.
    pub fn has_words(&self, person: EntityId) -> bool {
        self.lexicons.contains_key(&person)
    }

    pub fn lexicon(&self, person: EntityId) -> Option<&Lexicon> {
        self.lexicons.get(&person)
    }

    /// How a thing looks: its parts, what they do, and its mass.
    pub fn look_of(&self, id: EntityId) -> Look {
        let parts: Vec<EntityId> = match self.assemblies.get(&id) {
            Some(assembly) => assembly.parts.clone(),
            None => vec![id],
        };
        let mut look = Look {
            does: BTreeSet::new(),
            parts: parts.iter().map(|&p| self.part_look(p)).collect(),
            mass: Some(self.mass(id)),
        };
        for &part in &parts {
            if let Some(role) = self.part_role(part) {
                look.does.insert(role);
            }
        }
        look.parts.sort();
        look
    }

    /// How something built to a design looks, whatever it's made of and
    /// however big: the example a word from a people's culture stands for.
    pub fn look_of_design(&self, design: &str) -> Option<Look> {
        let def = self.designs.get(design)?;
        let mut look = Look {
            does: BTreeSet::new(),
            parts: Vec::new(),
            mass: None,
        };
        for (_, requirement) in &def.slots {
            look.parts.push(match requirement {
                Requirement::Shape(shape) => {
                    if let Some(role) = self.shapes.get(shape).and_then(|s| s.role)
                        && role != Role::Holding
                    {
                        look.does.insert(role);
                    }
                    Part::Shaped {
                        shape: shape.clone(),
                        material: None,
                    }
                }
                Requirement::Material(material) => Part::Lump(*material),
                Requirement::Design(_) => Part::Built,
            });
        }
        look.parts.sort();
        Some(look)
    }

    fn part_look(&self, part: EntityId) -> Part {
        if self.assemblies.contains_key(&part) {
            return Part::Built;
        }
        let material = self.composition(part).and_then(matter::dominant);
        match (self.shape_of.get(&part), material) {
            (Some(shape), material) => Part::Shaped {
                shape: shape.clone(),
                material,
            },
            (None, Some(material)) => Part::Lump(material),
            (None, None) => Part::Built,
        }
    }

    /// The job a part does, apart from being held.
    fn part_role(&self, part: EntityId) -> Option<Role> {
        self.shape_of
            .get(&part)
            .and_then(|s| self.shapes.get(s))
            .and_then(|s| s.role)
            .filter(|&r| r != Role::Holding)
    }

    /// The word `viewer` would use for a thing put together from parts, or
    /// a single shaped piece, and how close the match is: the closest of
    /// their words, the first learned if two are as close.
    pub fn recognise(&self, viewer: EntityId, id: EntityId) -> Option<(String, Closeness)> {
        let lexicon = self.lexicons.get(&viewer)?;
        let look = self.look_of(id);
        let mut best: Option<(String, Closeness)> = None;
        for (word, meaning) in &lexicon.words {
            let Meaning::Like(example) = meaning else {
                continue;
            };
            if let Some(closeness) = example.resembles(&look)
                && best.as_ref().is_none_or(|(_, b)| closeness > *b)
            {
                best = Some((word.clone(), closeness));
            }
        }
        best
    }

    /// What `viewer` calls a thing: in their own words if they have them,
    /// otherwise its name from data.
    pub fn label_for(&self, viewer: EntityId, id: EntityId) -> String {
        let Some(lexicon) = self.lexicons.get(&viewer) else {
            return self.label(id);
        };
        // People and places go by their names.
        if self.is_place(id) || (self.is_agent(id) && self.instinct(id).is_none()) {
            return self.label(id);
        }
        if let Some(kind) = self.kind_of.get(&id) {
            return self.kind_label_for(lexicon, id, kind);
        }
        if let Some(label) = self.labels.get(&id) {
            // Something named in data is known by that name only to someone
            // who knows what it's made of.
            let known = self
                .composition(id)
                .and_then(matter::dominant)
                .is_none_or(|m| material_word(lexicon, m).is_some());
            if known {
                return label.clone();
            }
            return self.matter_label_for(viewer, lexicon, id);
        }
        if let Some(assembly) = self.assemblies.get(&id) {
            let made_of = self.made_of_for(lexicon, id);
            return match self.recognise(viewer, id) {
                Some((word, Closeness::Like)) => {
                    format!("something like a {word}, made of {made_of}")
                }
                Some((word, _)) => format!("{word} of {made_of}"),
                None => {
                    let mut parts = assembly.parts.clone();
                    parts.sort_by_key(|&p| (std::cmp::Reverse(self.mass(p)), p));
                    self.joined(parts.iter().map(|&p| self.label_for(viewer, p)).collect())
                }
            };
        }
        self.matter_label_for(viewer, lexicon, id)
    }

    /// A creature, or a growing thing of a kind: by the viewer's word for
    /// its kind; by how it looks if they don't know it; or by the nearest
    /// broader kind they know.
    fn kind_label_for(&self, lexicon: &Lexicon, id: EntityId, kind: &str) -> String {
        let data_label = self.labels.get(&id);
        if let Some(word) = kind_word(lexicon, kind) {
            // Data may name it more fully, as "a" something, or a place's
            // stand of them; keep that if the viewer's word is the kind's own.
            return match data_label {
                Some(label) if self.kinds.get(kind).is_some_and(|k| k.label == word) => {
                    label.clone()
                }
                _ => word.to_string(),
            };
        }
        if let Some(looks) = self.kinds.get(kind).and_then(|k| k.looks.clone()) {
            return looks;
        }
        let mut broader = self.kinds.get(kind).and_then(|k| k.parent.clone());
        while let Some(parent) = broader {
            if let Some(word) = kind_word(lexicon, &parent) {
                return format!("some kind of {word}");
            }
            broader = self.kinds.get(&parent).and_then(|k| k.parent.clone());
        }
        "something alive".to_string()
    }

    /// A piece of matter, described as the viewer would: its materials in
    /// their words, and its shape by their word for it or by its form.
    fn matter_label_for(&self, viewer: EntityId, lexicon: &Lexicon, id: EntityId) -> String {
        let Some(composition) = self.matter.get(&id) else {
            return self.label(id);
        };
        let names = self.describe_with(lexicon, composition);
        let temperature = self.temperature(id).unwrap_or_default();
        let states: Vec<State> = composition
            .keys()
            .map(|m| self.materials[m].state_at(temperature))
            .collect();
        if states.iter().all(|&s| s == State::Gas) {
            return names;
        }
        if states.iter().all(|&s| s == State::Liquid) {
            let ambient = self
                .place_of(id)
                .map_or(self.settings.reference_temperature, |p| self.ambient(p));
            let hot = composition
                .keys()
                .any(|m| self.materials[m].melting_point > ambient);
            return if hot {
                format!("molten {names}")
            } else {
                names
            };
        }
        let Some(shape) = self.shape_of.get(&id) else {
            return if self.mass(id) < Mass::from_mg(1_000) {
                format!("{names} dust")
            } else {
                format!("lump of {names}")
            };
        };
        // A shaped piece someone has named by example; failing that, their
        // word for its shape; failing that, something it's like.
        let recognised = self.recognise(viewer, id);
        if let Some((word, Closeness::Same | Closeness::Exact)) = &recognised {
            return format!("{word} of {names}");
        }
        let materials_known = composition
            .keys()
            .all(|&m| material_word(lexicon, m).is_some());
        match (shape_word(lexicon, shape), recognised) {
            (Some(word), _) if materials_known => format!("{names} {word}"),
            (Some(word), _) => format!("{word} of {names}"),
            (None, Some((word, _))) => format!("something like a {word}, made of {names}"),
            (None, None) => {
                let form = self
                    .shapes
                    .get(shape)
                    .and_then(|s| s.form.clone())
                    .unwrap_or_else(|| "shaped piece".to_string());
                format!("{form} of {names}")
            }
        }
    }

    /// What a thing put together is made of, in the viewer's words: its
    /// parts' main materials, most first.
    fn made_of_for(&self, lexicon: &Lexicon, id: EntityId) -> String {
        let mut composition = Composition::new();
        let parts = self
            .assemblies
            .get(&id)
            .map_or_else(Vec::new, |a| a.parts.clone());
        for part in parts {
            if let Some(c) = self.composition(part) {
                for (&m, &mass) in c {
                    let entry = composition.entry(m).or_insert(Mass::ZERO);
                    *entry = Mass::from_mg(entry.mg() + mass.mg());
                }
            }
        }
        self.describe_with(lexicon, &composition)
    }

    /// Materials, most first, in `viewer`'s words: by name if they know it,
    /// by how it looks if they don't.
    pub fn describe_composition_for(&self, viewer: EntityId, composition: &Composition) -> String {
        match self.lexicons.get(&viewer) {
            Some(lexicon) => self.describe_with(lexicon, composition),
            None => self.describe_composition(composition),
        }
    }

    fn describe_with(&self, lexicon: &Lexicon, composition: &Composition) -> String {
        let mut parts: Vec<(&MaterialId, &Mass)> = composition.iter().collect();
        parts.sort_by(|(a_id, a), (b_id, b)| b.cmp(a).then(a_id.cmp(b_id)));
        let mut names: Vec<String> = Vec::new();
        for (id, _) in parts {
            let name = material_word(lexicon, *id).map_or_else(
                || {
                    self.materials[id]
                        .looks
                        .clone()
                        .unwrap_or_else(|| UNFAMILIAR.to_string())
                },
                str::to_string,
            );
            if !names.contains(&name) {
                names.push(name);
            }
        }
        list_and(&names)
    }

    /// `viewer`'s word for a material, if they have one.
    pub fn material_word_for(&self, viewer: EntityId, material: MaterialId) -> Option<String> {
        material_word(self.lexicons.get(&viewer)?, material).map(str::to_string)
    }

    /// The shape a word means to `viewer`, if it means one.
    pub fn shape_by_word(&self, viewer: EntityId, word: &str) -> Option<String> {
        self.lexicons
            .get(&viewer)?
            .words
            .iter()
            .find_map(|(w, meaning)| match meaning {
                Meaning::Shape(shape) if w == word => Some(shape.clone()),
                _ => None,
            })
    }

    /// The ways `viewer` knows to make what a word means, first learned
    /// first.
    pub fn recipes_for(&self, viewer: EntityId, word: &str) -> Vec<&Recipe> {
        self.lexicons.get(&viewer).map_or_else(Vec::new, |l| {
            l.recipes
                .iter()
                .filter(|(w, _)| w == word)
                .map(|(_, r)| r)
                .collect()
        })
    }

    /// What a word would mean if someone named this thing with it: a kind
    /// for a creature or growing thing, a material for an unshaped piece of one, and
    /// otherwise anything that looks like it.
    pub fn meaning_of(&self, id: EntityId) -> Meaning {
        if let Some(kind) = self.kind_of.get(&id) {
            return Meaning::Kind(kind.clone());
        }
        let unshaped = !self.assemblies.contains_key(&id) && !self.shape_of.contains_key(&id);
        match self.composition(id).and_then(matter::dominant) {
            Some(material) if unshaped => Meaning::Material(material),
            _ => Meaning::Like(self.look_of(id)),
        }
    }

    /// How to make another thing like one that was put together: a part of
    /// the same shape, or the same material, for each of its parts.
    pub fn recipe_from(&self, id: EntityId) -> Option<Recipe> {
        let assembly = self.assemblies.get(&id)?;
        let slots = assembly
            .parts
            .iter()
            .map(|&part| {
                let requirement = match self.part_look(part) {
                    Part::Shaped { shape, .. } => Requirement::Shape(shape),
                    Part::Lump(material) => Requirement::Material(material),
                    Part::Built => Requirement::Design(
                        self.assemblies
                            .get(&part)
                            .and_then(|a| a.design.clone())
                            .unwrap_or_default(),
                    ),
                };
                // Parts of something new have no names of their own.
                (String::new(), requirement)
            })
            .collect();
        Some(Recipe {
            design: assembly.design.clone(),
            slots,
        })
    }

    /// What a part a recipe needs is called, in `viewer`'s words.
    pub fn requirement_for(&self, viewer: EntityId, requirement: &Requirement) -> String {
        let Some(lexicon) = self.lexicons.get(&viewer) else {
            return match requirement {
                Requirement::Shape(shape) => self
                    .shapes
                    .get(shape)
                    .map_or(shape.clone(), |s| s.label.clone()),
                Requirement::Design(design) => self
                    .designs
                    .get(design)
                    .map_or(design.clone(), |d| d.label.clone()),
                Requirement::Material(material) => {
                    format!("piece of {}", self.materials[material].label)
                }
            };
        };
        match requirement {
            Requirement::Shape(shape) => shape_word(lexicon, shape).map_or_else(
                || {
                    self.shapes
                        .get(shape)
                        .and_then(|s| s.form.clone())
                        .unwrap_or_else(|| "shaped piece".to_string())
                },
                str::to_string,
            ),
            Requirement::Design(design) => lexicon
                .recipes
                .iter()
                .find(|(_, r)| r.design.as_deref() == Some(design))
                .map_or_else(|| "part put together".to_string(), |(w, _)| w.clone()),
            Requirement::Material(material) => format!(
                "piece of {}",
                self.describe_with(lexicon, &Composition::from([(*material, Mass::ZERO)]))
            ),
        }
    }
}

fn material_word(lexicon: &Lexicon, material: MaterialId) -> Option<&str> {
    lexicon.words.iter().find_map(|(w, m)| match m {
        Meaning::Material(id) if *id == material => Some(w.as_str()),
        _ => None,
    })
}

fn shape_word<'a>(lexicon: &'a Lexicon, shape: &str) -> Option<&'a str> {
    lexicon.words.iter().find_map(|(w, m)| match m {
        Meaning::Shape(s) if s == shape => Some(w.as_str()),
        _ => None,
    })
}

fn kind_word<'a>(lexicon: &'a Lexicon, kind: &str) -> Option<&'a str> {
    lexicon.words.iter().find_map(|(w, m)| match m {
        Meaning::Kind(k) if k == kind => Some(w.as_str()),
        _ => None,
    })
}
