//! The showcase, a designer's preview (`/showcase`): models from an asset
//! pack set out on the ground, listed in `style.toml`, to see what they look
//! like in our scene before choosing them for things. They aren't in the
//! world: pointing names them, and build mode moves them, but nothing else
//! knows they're there, and `/save` doesn't keep where they are.

use bevy::prelude::*;

use crate::draw::{Kit, Style, on_ground};
use crate::terrain::Land;

/// Whether the showcase is out: not unless asked for (`/showcase`), now
/// that the pack has been seen.
#[derive(Resource, Default)]
pub struct Showcase {
    pub shown: bool,
}

/// A model in the showcase, by name.
#[derive(Component)]
pub struct Shown(pub String);

/// A model's name: its file's, without the folders or ending.
fn name(path: &str) -> String {
    let file = path.rsplit('/').next().unwrap_or(path);
    file.strip_suffix(".gltf").unwrap_or(file).replace('_', " ")
}

/// Sets the showcase out, or clears it away, when the setting changes.
#[allow(clippy::too_many_arguments)]
pub fn draw(
    mut commands: Commands,
    showcase: Res<Showcase>,
    style: Res<Style>,
    land: Res<Land>,
    assets: Res<AssetServer>,
    mut kit: ResMut<Kit>,
    out: Query<Entity, With<Shown>>,
    mut was: Local<bool>,
) {
    if showcase.shown == *was {
        return;
    }
    *was = showcase.shown;
    for entity in &out {
        commands.entity(entity).despawn();
    }
    if !showcase.shown {
        return;
    }
    for (i, shown) in style.showcase.iter().enumerate() {
        let at = on_ground(&land, Vec3::new(shown.east, 0.0, -shown.north));
        commands.spawn((
            bevy::world_serialization::WorldAssetRoot(kit.model(&assets, &shown.model)),
            Transform::from_translation(at)
                .with_rotation(Quat::from_rotation_y(i as f32 * 1.3))
                .with_scale(Vec3::splat(shown.scale)),
            Shown(name(&shown.model)),
        ));
    }
}
