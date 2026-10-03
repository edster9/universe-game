//! The world as built in Blender (docs/ideas/world-building.md): its scene,
//! exported by blender/convert.sh to assets/worlds/<world>/<world>.gltf,
//! drawn just as it was placed. Scenery is drawn as it is. A source or a
//! thing is drawn by its own models from the scene, and follows where the
//! engine has it, so build mode moves it: a source moves whole, with all the
//! models that show it. People and creatures are drawn by the client, as
//! ever. Anything made in play, which Blender never saw, is drawn by the
//! style.
//!
//! The scene's objects carry their tags (`ug_id`, `ug_entry`) as glTF
//! extras.

use std::collections::BTreeSet;

use bevy::gltf::GltfExtras;
use bevy::prelude::*;
use engine::world::EntityId;

use crate::Sim;
use crate::draw::{Named, spot};
use crate::terrain::Land;

/// Where the world's Blender scene is, if it was built in Blender: a path
/// in the assets folder.
#[derive(Resource, Default)]
pub struct Built {
    pub scene: Option<String>,
    /// Engine things drawn by their models from the scene: the client's own
    /// drawing leaves them alone.
    pub drawn: BTreeSet<EntityId>,
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

/// A source or thing drawn by its scene models, which follow where the
/// engine has it.
#[derive(Component)]
pub struct Follows(EntityId);

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
/// it is: a source or thing is named by it, so pointing and menus work, and
/// follows the engine; a person or creature is left to the client.
pub fn join(
    mut commands: Commands,
    tagged: Query<(Entity, &GltfExtras), Added<GltfExtras>>,
    sim: Res<Sim>,
    mut built: ResMut<Built>,
) {
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
        commands
            .entity(entity)
            .insert((Named(thing), Follows(thing)));
        built.drawn.insert(thing);
    }
}

/// Sources and things drawn by their scene models stand where the engine
/// has them, and
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
