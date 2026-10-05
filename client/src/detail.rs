//! Level of detail: what's far away is drawn more simply, or not at all,
//! as games do (docs/research/rendering-benchmarks.md). A model with
//! simpler copies (made by blender/make_catalogue.py, an entry's `lods`,
//! each tagged `ug_lod` 1, 2, ... in the export, the full model 0) shows one
//! of them by its distance from the eye, fading from one to the next; its
//! last (the cards, pictures of the model, if it has them) is drawn as far
//! as the view goes. A
//! small model without them fades out at a distance in proportion to its
//! size, as sight does (`engine::sight`): grass long before a boulder. The
//! `detail` setting says how soon. Nothing's drawn beyond the `view`
//! setting, except what's vast, like the ground: Bevy culls by distance only
//! this way, not by the camera's far plane.

use bevy::camera::primitives::Aabb;
use bevy::camera::visibility::VisibilityRange;
use bevy::gltf::GltfExtras;
use bevy::prelude::*;
use bevy::world_serialization::WorldAssetRoot;

use crate::graphics::{Graphics, Level};

/// A mesh that's one level of a model's detail: 0 the full model; and
/// whether it's the model's last, drawn as far as the view goes.
#[derive(Component)]
pub struct Lod {
    level: u8,
    last: bool,
}

/// A mesh from a model, drawn only so far away for its size (its widest,
/// in metres).
#[derive(Component)]
pub struct Sized(f32);

/// Smaller than this, a thing without simpler copies fades out at a
/// distance for its size; bigger, it's drawn as far as the view goes.
const SMALL: f32 = 3.0;
/// Bigger than this (the ground), it's drawn however far away it is.
const VAST: f32 = 50.0;

/// Tags each mesh of a model with simpler copies with its level. A model's
/// levels are sibling nodes, which arrive together.
pub fn tag_levels(
    mut commands: Commands,
    nodes: Query<(&GltfExtras, &Children, &ChildOf), Added<GltfExtras>>,
    meshes: Query<(), With<Mesh3d>>,
) {
    let level_of = |extras: &GltfExtras| {
        serde_json::from_str::<serde_json::Value>(&extras.value)
            .ok()
            .and_then(|v| v.get("ug_lod")?.as_u64())
            .map(|l| l as u8)
    };
    let mut last: std::collections::HashMap<Entity, u8> = Default::default();
    for (extras, _, model) in &nodes {
        if let Some(level) = level_of(extras) {
            let most = last.entry(model.parent()).or_default();
            *most = (*most).max(level);
        }
    }
    for (extras, children, model) in &nodes {
        let Some(level) = level_of(extras) else {
            continue;
        };
        let last = last.get(&model.parent()) == Some(&level);
        for &child in children {
            if meshes.contains(child) {
                commands.entity(child).insert(Lod { level, last });
            }
        }
    }
}

/// Meshes newly given their bounds, not levels of detail.
type NewBounds<'a> = (Entity, &'a Aabb, &'a GlobalTransform);
type NotLevels = (Added<Aabb>, With<Mesh3d>, Without<Lod>);

/// Tags each mesh from a model with its size (not the client's own shapes,
/// nor the simpler copies, whose level says when they're drawn).
pub fn tag_small(
    mut commands: Commands,
    new: Query<NewBounds, NotLevels>,
    parents: Query<&ChildOf>,
    roots: Query<(), With<WorldAssetRoot>>,
) {
    for (entity, aabb, at) in &new {
        if !parents.iter_ancestors(entity).any(|a| roots.contains(a)) {
            continue;
        }
        let size = (Vec3::from(aabb.half_extents) * at.scale()).max_element() * 2.0;
        if size < VAST {
            commands.entity(entity).insert(Sized(size));
        }
    }
}

/// Where each level of detail ends, in metres from the eye (the last goes
/// on to the view's end), and how many times its size a small thing is
/// drawn out to.
fn reach(detail: Level) -> ([f32; 3], f32) {
    match detail {
        Level::Off | Level::Low => ([15.0, 45.0, 90.0], 80.0),
        Level::Medium => ([25.0, 80.0, 180.0], 150.0),
        Level::High => ([40.0, 140.0, 300.0], 300.0),
    }
}

/// Sets how far each tagged mesh is drawn, when tagged or when the setting
/// changes.
pub fn ranges(
    mut commands: Commands,
    graphics: Res<Graphics>,
    levels: Query<(Entity, &Lod, Ref<Lod>)>,
    sized: Query<(Entity, &Sized, Ref<Sized>)>,
) {
    let changed = graphics.is_changed();
    let (ends, times) = reach(graphics.detail);
    let view = graphics.view as f32;
    let fade = |d: f32| {
        let d = d.min(view * 0.95);
        d..d * 1.05
    };
    for (entity, lod, tagged) in &levels {
        if !changed && !tagged.is_added() {
            continue;
        }
        let level = (lod.level as usize).min(ends.len());
        let start = if level == 0 {
            0.0..0.0
        } else {
            fade(ends[level - 1])
        };
        let end = if lod.last || level == ends.len() {
            fade(view)
        } else {
            fade(ends[level])
        };
        commands.entity(entity).insert(VisibilityRange {
            start_margin: start,
            end_margin: end,
            use_aabb: false,
        });
    }
    for (entity, size, tagged) in &sized {
        if !changed && !tagged.is_added() {
            continue;
        }
        let end = if size.0 < SMALL { size.0 * times } else { view };
        commands.entity(entity).insert(VisibilityRange {
            start_margin: 0.0..0.0,
            end_margin: fade(end),
            use_aabb: false,
        });
    }
}
