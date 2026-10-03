//! Land, shaped from what the engine knows: every place's position and
//! height, and the paths between them. There's land around each place and
//! along each path over ground; heights blend between places; the coast falls
//! away into the sea. Nothing is invented but the shape between the places.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use engine::world::{EntityId, World as EngineWorld};

/// How far land reaches around a place at sea level, and either side of a
/// path, in metres. Higher places reach further: a mountain's base is wider
/// than a hill's, about `SPREAD` times its height.
const AROUND_A_PLACE: f32 = 300.0;
const ALONG_A_PATH: f32 = 180.0;
const SPREAD: f32 = 3.0;
/// How deep the sea floor lies off the coast.
const SEA_FLOOR: f32 = -8.0;
/// Grid spacing of the land's mesh, in metres.
const CELL: f32 = 25.0;

/// The land's shape, as a function anyone can ask: how high the ground is
/// at a point.
#[derive(Resource, Clone, PartialEq)]
pub struct Land {
    /// Each place: where, how high, and how far its land reaches.
    places: Vec<(Vec2, f32, f32)>,
    /// Each path over land: its ends, and how far land reaches at each end.
    paths: Vec<(Vec2, Vec2, f32, f32)>,
    /// Places joined by paths over land, drawn as one piece of land.
    islands: Vec<Vec<usize>>,
    /// A world built in Blender brings its own ground, level at nought, so
    /// the land here is flat: things stand on Blender's ground, not above it.
    /// (Until Blender's ground heights come across with the world.)
    flat: bool,
}

fn reach(height: f32, base: f32) -> f32 {
    base.max(height * SPREAD)
}

impl Land {
    pub fn of(world: &EngineWorld) -> Land {
        let at = |p: EntityId| {
            let (east, north) = world.position(p).unwrap_or((0, 0));
            Vec2::new(east as f32 / 1e6, -(north as f32 / 1e6))
        };
        let height = |p: EntityId| world.height(p) as f32 / 1e6;
        let ids: Vec<EntityId> = world.entities().filter(|&e| world.is_place(e)).collect();
        let index = |p: EntityId| ids.iter().position(|&q| q == p);
        let mut paths = Vec::new();
        // Which island each place is on: joined by paths over land.
        let mut island: Vec<usize> = (0..ids.len()).collect();
        fn root(island: &mut [usize], i: usize) -> usize {
            let mut r = i;
            while island[r] != r {
                r = island[r];
            }
            island[i] = r;
            r
        }
        for (i, &a) in ids.iter().enumerate() {
            for &b in world.exits(a) {
                // Crossings are over water, not land.
                if a < b && world.crossing(a, b).is_none() {
                    paths.push((
                        at(a),
                        at(b),
                        reach(height(a), ALONG_A_PATH),
                        reach(height(b), ALONG_A_PATH),
                    ));
                    if let Some(j) = index(b) {
                        let (ra, rb) = (root(&mut island, i), root(&mut island, j));
                        island[ra] = rb;
                    }
                }
            }
        }
        let mut islands: Vec<Vec<usize>> = Vec::new();
        let mut which = std::collections::BTreeMap::new();
        for i in 0..ids.len() {
            let r = root(&mut island, i);
            let k = *which.entry(r).or_insert_with(|| {
                islands.push(Vec::new());
                islands.len() - 1
            });
            islands[k].push(i);
        }
        Land {
            places: ids
                .iter()
                .map(|&p| (at(p), height(p), reach(height(p), AROUND_A_PLACE)))
                .collect(),
            paths,
            islands,
            flat: false,
        }
    }

    pub fn is_flat(&self) -> bool {
        self.flat
    }

    /// The same land, flat at nought if the world was built in Blender.
    pub fn flat_if(mut self, built: bool) -> Land {
        self.flat = built;
        self
    }

    /// The middle of all the places, on the level.
    pub fn middle(&self) -> Vec2 {
        let n = self.places.len().max(1) as f32;
        self.places.iter().map(|&(at, _, _)| at).sum::<Vec2>() / n
    }

    /// Ground height at a point, in metres above the sea.
    pub fn height(&self, p: Vec2) -> f32 {
        if self.flat {
            return 0.0;
        }
        // Heights blend between places, nearest counting most.
        let (mut sum, mut weight) = (0.0, 0.0);
        let mut nearest = f32::MAX;
        for &(at, h, r) in &self.places {
            let d = at.distance(p);
            // Smooth even at the place itself: no step where it's reached.
            let w = 1.0 / (d * d + 1.0);
            sum += h * w;
            weight += w;
            nearest = nearest.min(d / r);
        }
        for &(a, b, ra, rb) in &self.paths {
            let (d, t) = distance_to_segment(p, a, b);
            nearest = nearest.min(d / (ra + (rb - ra) * t));
        }
        let blended = sum / weight;
        // Land slopes away to the sea floor towards the edge of its reach.
        let land = 1.0 - smoothstep(0.15, 1.0, nearest);
        // A little roughness, so ground isn't flat as a table.
        let bumps = ((p.x * 0.013).sin() * (p.y * 0.017).cos()) * 3.0 * land;
        SEA_FLOOR + (blended.max(2.0) - SEA_FLOOR) * land + bumps
    }

    /// The land's mesh over the area its places cover: coloured sand, grass,
    /// rock, and snow by height.
    pub fn mesh(&self) -> Vec<Mesh> {
        // One piece of land for each island: every place on it, and its reach.
        self.islands
            .iter()
            .map(|members| {
                let (mut lo, mut hi) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
                for &i in members {
                    let (at, _, r) = self.places[i];
                    lo = lo.min(at - Vec2::splat(r * 1.05));
                    hi = hi.max(at + Vec2::splat(r * 1.05));
                }
                self.grid(lo, hi)
            })
            .collect()
    }

    fn grid(&self, lo: Vec2, hi: Vec2) -> Mesh {
        // Finer for small islands, coarser for big ones: at most ~700 cells
        // a side.
        let cell = CELL.max((hi - lo).max_element() / 700.0);
        let nx = ((hi.x - lo.x) / cell).ceil() as u32 + 1;
        let nz = ((hi.y - lo.y) / cell).ceil() as u32 + 1;
        let mut positions = Vec::with_capacity((nx * nz) as usize);
        let mut colours = Vec::with_capacity((nx * nz) as usize);
        for j in 0..nz {
            for i in 0..nx {
                let p = Vec2::new(lo.x + i as f32 * cell, lo.y + j as f32 * cell);
                let h = self.height(p);
                positions.push([p.x, h, p.y]);
                colours.push(ground_colour(h));
            }
        }
        let mut indices = Vec::with_capacity(((nx - 1) * (nz - 1) * 6) as usize);
        for j in 0..nz - 1 {
            for i in 0..nx - 1 {
                let a = j * nx + i;
                let (b, c, d) = (a + 1, a + nx, a + nx + 1);
                indices.extend([a, c, b, b, c, d]);
            }
        }
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
        .with_inserted_indices(Indices::U32(indices));
        mesh.compute_smooth_normals();
        mesh
    }
}

fn ground_colour(h: f32) -> [f32; 4] {
    // Colours as they look on screen, blended, then given to the renderer
    // as the light values it works in.
    let lerp = |a: [f32; 3], b: [f32; 3], t: f32| {
        let t = t.clamp(0.0, 1.0);
        let c = Color::srgb(
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        )
        .to_linear();
        [c.red, c.green, c.blue, 1.0]
    };
    let (sea_floor, sand, grass, rock, snow) = (
        [0.35, 0.33, 0.25],
        [0.85, 0.77, 0.54],
        [0.36, 0.55, 0.26],
        [0.52, 0.49, 0.43],
        [0.93, 0.93, 0.96],
    );
    match h {
        h if h < 0.0 => lerp(sea_floor, sand, (h - SEA_FLOOR) / -SEA_FLOOR),
        h if h < 6.0 => lerp(sand, grass, (h - 3.0) / 3.0),
        h if h < 700.0 => lerp(grass, rock, (h - 400.0) / 300.0),
        h => lerp(rock, snow, (h - 1400.0) / 300.0),
    }
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// How far a point is from a segment, and how far along it the nearest
/// point lies (0 at `a`, 1 at `b`).
fn distance_to_segment(p: Vec2, a: Vec2, b: Vec2) -> (f32, f32) {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared().max(1e-6)).clamp(0.0, 1.0);
    (p.distance(a + ab * t), t)
}

#[cfg(test)]
mod tests {
    use super::Land;
    use bevy::math::Vec2;

    /// In a world built in Blender, things stand on Blender's ground, level
    /// at nought; the land made from places stands at least 2 m above it,
    /// which once left the islander and the landmark rocks in the air.
    #[test]
    fn a_world_built_in_blender_stands_on_its_own_ground() {
        let data = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../data"));
        let world = console::load_world_file(&data.join("skill-yard.toml")).unwrap();
        let island = Land::of(&world);
        let built = Land::of(&world).flat_if(true);
        for at in [Vec2::ZERO, Vec2::new(0.0, -30.5), Vec2::new(36.0, -36.0)] {
            assert!(island.height(at) >= 2.0 - 3.0, "{at}");
            assert_eq!(built.height(at), 0.0, "{at}");
        }
        assert!(island.height(Vec2::ZERO) > 1.0);
    }
}
