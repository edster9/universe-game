//! The world as built in Blender (docs/ideas/world-building.md): its scene,
//! exported by blender/convert.sh to assets/worlds/<world>/<world>.gltf,
//! drawn just as it was placed. Scenery is drawn as it is. A source is drawn
//! by the models that show it in Blender, where they stand. A thing is drawn
//! by its own model too, but follows the engine, so build mode moves it.
//! People and creatures are drawn by the client, as ever. Anything made in
//! play, which Blender never saw, is drawn by the style.
//!
//! The scene's objects carry their tags (`ug_id`, `ug_entry`) as glTF
//! extras; the catalogue (blender/catalogue.toml) says which entries are
//! sources.

use std::collections::{BTreeMap, BTreeSet};

use bevy::gltf::GltfExtras;
use bevy::prelude::*;
use engine::world::EntityId;

use crate::Sim;
use crate::draw::{Named, spot};
use crate::terrain::Land;

const CATALOGUE: &str = include_str!("../../blender/catalogue.toml");

/// Where the world's Blender scene is, if it was built in Blender: a path
/// in the assets folder.
#[derive(Resource, Default)]
pub struct Built {
    pub scene: Option<String>,
    /// Engine things drawn by their models from the scene: the client's own
    /// drawing leaves them alone.
    pub drawn: BTreeSet<EntityId>,
    /// Sources among them: fixed where Blender put them, so build mode
    /// leaves them be (the owner, 2026-10-02: sources move in Blender).
    pub sources: BTreeSet<EntityId>,
}

impl Built {
    /// The scene for a world's data file (`skill-yard.toml`), if one was
    /// exported for it.
    pub fn for_world(file: &str) -> Built {
        let name = file.trim_end_matches(".toml");
        let path = format!("worlds/{name}/{name}.gltf");
        Built {
            scene: crate::draw::assets_folder()
                .join(&path)
                .is_file()
                .then_some(path),
            ..default()
        }
    }
}

/// A thing drawn by its scene model, which follows where the engine has it.
#[derive(Component)]
pub struct Follows(EntityId);

/// The catalogue's entries that are sources.
fn sources() -> BTreeMap<String, bool> {
    let table: toml::Table = toml::from_str(CATALOGUE).unwrap_or_default();
    table
        .get("entry")
        .and_then(|e| e.as_array())
        .into_iter()
        .flatten()
        .filter_map(|e| {
            Some((
                e.get("id")?.as_str()?.to_string(),
                e.get("is")?.as_str()? == "source",
            ))
        })
        .collect()
}

/// Loads the scene, once.
pub fn spawn(mut commands: Commands, built: Res<Built>, assets: Res<AssetServer>) {
    if let Some(path) = &built.scene {
        commands.spawn((
            bevy::world_serialization::WorldAssetRoot(
                assets.load(bevy::gltf::GltfAssetLabel::Scene(0).from_asset(path.clone())),
            ),
            Transform::default(),
        ));
    }
}

/// As the scene's objects appear, joins each tagged one to the engine thing
/// it is: a source or thing is named by it, so pointing and menus work; a
/// thing follows the engine; a person or creature is left to the client.
pub fn join(
    mut commands: Commands,
    tagged: Query<(Entity, &GltfExtras), Added<GltfExtras>>,
    sim: Res<Sim>,
    mut built: ResMut<Built>,
    mut is_source: Local<Option<BTreeMap<String, bool>>>,
) {
    let is_source = is_source.get_or_insert_with(sources);
    let world = sim.world();
    for (entity, extras) in &tagged {
        let Ok(tags) = serde_json::from_str::<serde_json::Value>(&extras.value) else {
            continue;
        };
        let Some(id) = tags.get("ug_id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(thing) = world.find_by_key(id) else {
            warn!("the Blender scene has {id}, which the world doesn't: convert it again");
            continue;
        };
        if world.is_agent(thing) {
            commands.entity(entity).despawn();
            continue;
        }
        commands.entity(entity).insert(Named(thing));
        built.drawn.insert(thing);
        let entry = tags.get("ug_entry").and_then(|v| v.as_str()).unwrap_or("");
        if is_source.get(entry).copied().unwrap_or(false) {
            built.sources.insert(thing);
        } else {
            commands.entity(entity).insert(Follows(thing));
        }
    }
}

/// Things drawn by their scene models stand where the engine has them, and
/// are hidden when they're not lying anywhere (carried, or gone).
pub fn follow(
    mut models: Query<(&Follows, &mut Transform, &mut Visibility)>,
    sim: Res<Sim>,
    land: Res<Land>,
) {
    let world = sim.world();
    for (Follows(thing), mut transform, mut visible) in &mut models {
        let lying = world.location(*thing).is_some_and(|l| world.is_place(l));
        *visible = if lying {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if lying {
            transform.translation = spot(world, &land, *thing, sim.now());
        }
    }
}
