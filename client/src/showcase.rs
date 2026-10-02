//! The showcase, a designer's preview (`/showcase`): models from an asset
//! pack set out on the ground, listed in `style.toml`, to see what they look
//! like in our scene before choosing them for things. They aren't in the
//! world: nothing can be pointed at, picked, or walked into.

use bevy::prelude::*;

use crate::draw::{Kit, Style, on_ground};
use crate::terrain::Land;

/// Whether the showcase is out.
#[derive(Resource)]
pub struct Showcase {
    pub shown: bool,
}

impl Default for Showcase {
    fn default() -> Self {
        Showcase { shown: true }
    }
}

/// A model in the showcase.
#[derive(Component)]
pub struct Shown;

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
            Shown,
        ));
    }
}
