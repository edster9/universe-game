//! Seeing at a distance. How well something can be seen depends on how big
//! it is and how far away it is, measured, never written in data: anything
//! made in play has a size like anything else, so whoever sees it from afar
//! sees it by the same law. Near enough, it's made out, and called by the
//! viewer's own word for it (which, for something they've never learned,
//! is only its look: see `words.rs`). Further off, it's seen but not made
//! out: "something small". Further still, it isn't seen at all. Darkness
//! shortens both. See docs/ideas/context-menu.md.

use crate::datasheet;
use crate::matter::State;
use crate::world::{EntityId, World};

/// How well a viewer sees something, from worst to best.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sight {
    Unseen,
    /// Seen, but too far to make out what it is.
    Seen,
    MadeOut,
}

/// What a viewer sees with, worked out once for many things.
pub struct Eyes {
    viewer: EntityId,
    place: Option<EntityId>,
    /// How many times its own size away something can be and still be made
    /// out, and still be seen.
    made_out: u64,
    seen: u64,
}

impl Eyes {
    pub fn of(world: &World, viewer: EntityId) -> Eyes {
        let place = world.place_of(viewer);
        let settings = world.settings();
        let dark = place.is_some_and(|p| world.is_dark(p));
        let shorter = if dark {
            settings.sight_in_dark.max(1)
        } else {
            1
        };
        Eyes {
            viewer,
            place,
            made_out: settings.sight_made_out / shorter,
            seen: settings.sight_seen / shorter,
        }
    }

    /// How well they see `id`.
    pub fn sight(&self, world: &World, id: EntityId) -> Sight {
        // What's held or inside something is seen as well as what holds it;
        // what the viewer carries, they see.
        let mut outer = id;
        loop {
            if outer == self.viewer {
                return Sight::MadeOut;
            }
            match world.location(outer) {
                Some(holder) if !world.is_place(holder) => outer = holder,
                _ => break,
            }
        }
        if world.is_place(outer) {
            return Sight::MadeOut;
        }
        if world.place_of(outer) != self.place || self.place.is_none() {
            return Sight::Unseen;
        }
        // The air isn't looked at from afar: it's all around.
        if world.is_all(outer, State::Gas) {
            return Sight::MadeOut;
        }
        let gap = u128::from(world.gap(self.viewer, outer));
        // Right beside it, there's nothing to measure.
        if gap == 0 {
            return Sight::MadeOut;
        }
        let size = u128::from(size(world, outer).max(1));
        if gap <= size * u128::from(self.made_out) {
            Sight::MadeOut
        } else if gap <= size * u128::from(self.seen) {
            Sight::Seen
        } else {
            Sight::Unseen
        }
    }

    /// What they call `id`: their word for it when made out, or only how big
    /// it looks.
    pub fn label(&self, world: &World, id: EntityId) -> String {
        match self.sight(world, id) {
            Sight::MadeOut => world.label_for(self.viewer, id),
            _ => something(world, id),
        }
    }
}

/// How well `viewer` sees `id`.
pub fn sight(world: &World, viewer: EntityId, id: EntityId) -> Sight {
    Eyes::of(world, viewer).sight(world, id)
}

/// What `viewer` calls `id`, as far as they can see it.
pub fn label(world: &World, viewer: EntityId, id: EntityId) -> String {
    Eyes::of(world, viewer).label(world, id)
}

/// Something seen but not made out, by how big it looks.
pub fn something(world: &World, id: EntityId) -> String {
    if world.is_agent(id) && world.instinct(id).is_none() {
        return "someone".into();
    }
    match size(world, id) {
        s if s < 300_000 => "something small".into(),
        s if s < 2_000_000 => "something".into(),
        _ => "something large".into(),
    }
}

/// How big something is, in µm, measured: across a patch, as long as a
/// shape that has a length, or else the side of a cube of its volume, from
/// what it's made of (or, for something put together, its datasheet).
pub fn size(world: &World, id: EntityId) -> u64 {
    let spread = world.spread(id);
    if spread > 0 {
        return spread.saturating_mul(2);
    }
    let volume = match world.composition(id) {
        Some(composition) => crate::matter::volume(world.materials(), composition),
        None => datasheet::volume(world, id),
    };
    // Nothing known of its volume: as small as things go.
    let side = volume.map_or(10_000, cube_side);
    let length = world
        .shape(id)
        .and_then(|shape| world.shapes().get(shape))
        .and_then(|def| def.length)
        .unwrap_or(0);
    side.max(length)
}

/// The side of a cube of volume `n`, in whole units: the largest whole
/// number whose cube is at most `n`.
fn cube_side(n: u128) -> u64 {
    let (mut low, mut high) = (0u128, 1u128 << 43);
    while low < high {
        let mid = (low + high).div_ceil(2);
        if mid.checked_pow(3).is_some_and(|c| c <= n) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    u64::try_from(low).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::cube_side;

    #[test]
    fn cubes() {
        assert_eq!(cube_side(0), 0);
        assert_eq!(cube_side(26), 2);
        assert_eq!(cube_side(27), 3);
        assert_eq!(cube_side(1_000_000_000_000_000_000), 1_000_000);
    }
}
