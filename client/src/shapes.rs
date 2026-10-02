//! The shapes things are drawn with, made in code from a few primitives:
//! no model files, nothing to license, and every one scaled by the size the
//! style gives it. Each is about a metre at unit size, resting on the ground
//! at y = 0. Stones, flint, and ore come in a few variants, chosen by the
//! thing, so a pile of them isn't a pile of one stone. Until the game has
//! its own artists (docs/ideas/assets.md), these are the base shapes.

use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

/// How many variants a form has.
pub fn variants(form: &str) -> u64 {
    match form {
        "stone" | "shard" | "nugget" | "boulder" => 4,
        "stick" | "twig" | "tuft" | "bush" | "nuts" => 3,
        _ => 1,
    }
}

/// A form's mesh, in the given variant, if it's one of these.
pub fn build(form: &str, variant: u64) -> Option<Mesh> {
    let seed = variant.wrapping_mul(0x9E37_79B9) ^ hash_str(form);
    Some(match form {
        "twig" => twig(seed),
        "stick" => stick(seed),
        "log" => log(seed),
        "tuft" => tuft(seed),
        "stone" => lumpy(seed, 2, 0.28, Vec3::new(1.0, 0.6, 0.8), false),
        "nugget" => lumpy(seed, 2, 0.4, Vec3::new(1.0, 0.7, 0.9), false),
        "shard" => lumpy(seed, 0, 0.45, Vec3::new(1.0, 0.45, 0.6), true),
        "boulder" => lumpy(seed, 2, 0.18, Vec3::new(1.0, 0.35, 0.75), false),
        "shell" => shell(),
        "nuts" => nuts(seed),
        "bush" => bush(seed),
        "pine" => pine(),
        "ring" => ring(seed),
        "kiln" => kiln(),
        _ => return None,
    })
}

fn hash_str(s: &str) -> u64 {
    s.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3)
    })
}

/// A number from 0 to 1, the same for the same inputs.
fn rand(seed: u64, n: u64) -> f32 {
    let mut x = seed ^ n.wrapping_mul(0xD6E8_FEB8_6659_FD93);
    x ^= x >> 32;
    x = x.wrapping_mul(0xD6E8_FEB8_6659_FD93);
    x ^= x >> 29;
    (x % 10_000) as f32 / 10_000.0
}

/// Joins meshes into one.
fn join(parts: Vec<Mesh>) -> Mesh {
    let mut parts = parts.into_iter();
    let mut whole = parts.next().expect("a part");
    for part in parts {
        let _ = whole.merge(&part);
    }
    whole
}

/// A rod of radius `r` from `a` to `b`.
fn rod(a: Vec3, b: Vec3, r: f32) -> Mesh {
    let length = a.distance(b).max(1e-4);
    let along = (b - a) / length;
    Mesh::from(Cylinder::new(r, length)).transformed_by(
        Transform::from_translation((a + b) / 2.0)
            .with_rotation(Quat::from_rotation_arc(Vec3::Y, along)),
    )
}

/// A small twig lying on the ground, with a side shoot.
fn twig(seed: u64) -> Mesh {
    let r = 0.012;
    let a = Vec3::new(-0.5, r, 0.0);
    let bend = Vec3::new(0.0, r, (rand(seed, 1) - 0.5) * 0.12);
    let b = Vec3::new(0.5, r, (rand(seed, 2) - 0.5) * 0.08);
    let fork = a.lerp(bend, 0.7);
    let shoot = fork
        + Vec3::new(
            0.22,
            0.0,
            0.18 * if rand(seed, 3) > 0.5 { 1.0 } else { -1.0 },
        );
    join(vec![
        rod(a, bend, r),
        rod(bend, b, r * 0.8),
        rod(fork, shoot, r * 0.6),
    ])
}

/// A stick, a little crooked.
fn stick(seed: u64) -> Mesh {
    let r = 0.025;
    let points = [
        Vec3::new(-0.5, r, 0.0),
        Vec3::new(-0.15, r, (rand(seed, 1) - 0.5) * 0.1),
        Vec3::new(0.2, r, (rand(seed, 2) - 0.5) * 0.1),
        Vec3::new(0.5, r, (rand(seed, 3) - 0.5) * 0.06),
    ];
    let mut parts: Vec<Mesh> = points.windows(2).map(|w| rod(w[0], w[1], r)).collect();
    // A knot where a branch broke off.
    parts.push(rod(
        points[1],
        points[1] + Vec3::new(0.04, 0.03, 0.05),
        r * 0.6,
    ));
    join(parts)
}

/// A log lying on the ground, with the stub of a branch.
fn log(seed: u64) -> Mesh {
    let r = 0.13;
    let body = Mesh::from(Cylinder::new(r, 1.0).mesh().resolution(10).build()).transformed_by(
        Transform::from_translation(Vec3::new(0.0, r, 0.0))
            .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
    );
    let at = Vec3::new((rand(seed, 1) - 0.5) * 0.5, r * 1.5, 0.0);
    join(vec![
        body,
        rod(at, at + Vec3::new(0.05, 0.12, 0.08), r * 0.3),
    ])
}

/// A tuft of grass: thin blades fanning out from the ground.
fn tuft(seed: u64) -> Mesh {
    let blades = 11;
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for i in 0..blades {
        let turn = TAU * i as f32 / blades as f32 + rand(seed, i) * 0.6;
        let lean = 0.15 + rand(seed, i + 100) * 0.35;
        let height = 0.6 + rand(seed, i + 200) * 0.4;
        let out = Vec3::new(turn.cos(), 0.0, turn.sin());
        let side = Vec3::new(-turn.sin(), 0.0, turn.cos()) * 0.025;
        let root = out * 0.05;
        let tip = root + out * lean + Vec3::Y * height;
        let normal = side.cross(tip - root).normalize_or_zero();
        // Both faces, so a blade is seen from either side.
        for (a, b, c, n) in [
            (root - side, root + side, tip, normal),
            (root + side, root - side, tip, -normal),
        ] {
            for p in [a, b, c] {
                positions.push(p.into());
                normals.push(n.into());
                uvs.push([0.0, 0.0]);
            }
        }
    }
    Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

/// A rock-like lump: a sphere pushed in and out, squashed to `shape`, sat on
/// the ground. Faceted, with flat faces, for something knapped like flint.
fn lumpy(seed: u64, detail: u32, rough: f32, shape: Vec3, faceted: bool) -> Mesh {
    let mut mesh = Sphere::new(0.5).mesh().ico(detail).expect("a sphere");
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for p in positions.iter_mut() {
            let v = Vec3::from(*p);
            // The same displacement for the same point, so shared corners
            // stay joined.
            let key = ((v.x * 1000.0).round() as i64 as u64).wrapping_mul(73_856_093)
                ^ ((v.y * 1000.0).round() as i64 as u64).wrapping_mul(19_349_663)
                ^ ((v.z * 1000.0).round() as i64 as u64).wrapping_mul(83_492_791);
            let push = 1.0 + (rand(seed, key) - 0.5) * 2.0 * rough;
            let v = v * push * shape;
            *p = (v + Vec3::Y * shape.y * 0.45).into();
        }
    }
    if faceted {
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();
    } else {
        mesh.compute_smooth_normals();
    }
    mesh
}

/// A mussel: two long shells, a little open.
fn shell() -> Mesh {
    let half = |turn: f32| {
        Sphere::new(0.5)
            .mesh()
            .ico(2)
            .expect("a sphere")
            .transformed_by(
                Transform::from_translation(Vec3::new(0.0, 0.12, 0.0))
                    .with_rotation(Quat::from_rotation_x(turn))
                    .with_scale(Vec3::new(1.0, 0.18, 0.45)),
            )
    };
    join(vec![half(0.25), half(-0.25)])
}

/// Roots and nuts: a few round nuts and a couple of thin roots.
fn nuts(seed: u64) -> Mesh {
    let mut parts = Vec::new();
    for i in 0..5 {
        let at = Vec3::new(
            (rand(seed, i) - 0.5) * 0.6,
            0.08,
            (rand(seed, i + 10) - 0.5) * 0.6,
        );
        parts.push(
            Sphere::new(0.08 + rand(seed, i + 20) * 0.05)
                .mesh()
                .ico(1)
                .expect("a sphere")
                .transformed_by(Transform::from_translation(at)),
        );
    }
    for i in 0..2 {
        let a = Vec3::new((rand(seed, i + 30) - 0.5) * 0.5, 0.03, -0.3);
        let b = a + Vec3::new((rand(seed, i + 40) - 0.5) * 0.4, 0.0, 0.7);
        parts.push(rod(a, b, 0.02));
    }
    join(parts)
}

/// A bush: a few rounded clumps together.
fn bush(seed: u64) -> Mesh {
    let mut parts = Vec::new();
    for i in 0..6 {
        let turn = TAU * i as f32 / 6.0 + rand(seed, i);
        let out = if i == 0 {
            0.0
        } else {
            0.25 + rand(seed, i + 10) * 0.1
        };
        let r = if i == 0 {
            0.35
        } else {
            0.2 + rand(seed, i + 20) * 0.1
        };
        let at = Vec3::new(
            turn.cos() * out,
            r * 0.9 + if i == 0 { 0.15 } else { 0.0 },
            turn.sin() * out,
        );
        parts.push(
            Sphere::new(r)
                .mesh()
                .ico(2)
                .expect("a sphere")
                .transformed_by(Transform::from_translation(at)),
        );
    }
    join(parts)
}

/// A tree: a trunk, and three tiers of foliage.
fn pine() -> Mesh {
    let mut parts = vec![rod(Vec3::ZERO, Vec3::Y * 0.45, 0.04)];
    for (y, r, h) in [(0.25, 0.32, 0.4), (0.48, 0.25, 0.35), (0.68, 0.17, 0.32)] {
        parts.push(
            Mesh::from(Cone::new(r, h))
                .transformed_by(Transform::from_translation(Vec3::Y * (y + h / 2.0))),
        );
    }
    join(parts)
}

/// A ring of stones laid on the ground.
fn ring(seed: u64) -> Mesh {
    let stones = 8;
    let parts = (0..stones)
        .map(|i| {
            let turn = TAU * i as f32 / stones as f32;
            lumpy(
                seed.wrapping_add(i),
                1,
                0.25,
                Vec3::new(0.32, 0.22, 0.26),
                false,
            )
            .transformed_by(
                Transform::from_translation(Vec3::new(turn.cos() * 0.38, 0.0, turn.sin() * 0.38))
                    .with_rotation(Quat::from_rotation_y(-turn)),
            )
        })
        .collect();
    join(parts)
}

/// A furnace: a squat clay dome over a stone base.
fn kiln() -> Mesh {
    let base = Mesh::from(Cylinder::new(0.45, 0.3))
        .transformed_by(Transform::from_translation(Vec3::Y * 0.15));
    let dome =
        Mesh::from(Cone::new(0.42, 0.6)).transformed_by(Transform::from_translation(Vec3::Y * 0.6));
    let flue = Mesh::from(Cylinder::new(0.1, 0.2))
        .transformed_by(Transform::from_translation(Vec3::Y * 0.95));
    join(vec![base, dome, flue])
}
