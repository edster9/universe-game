//! Smaller copies of every texture (mipmaps), made as it loads. The packs'
//! textures come as single large pictures (often 2048 pixels across), and
//! Bevy doesn't make the halves, quarters, and so on that a graphics card
//! reads for things far away; without them, every distant leaf reads the
//! full picture, which is slow and shimmers (measured in
//! docs/research/rendering-benchmarks.md). So they're made here, once, as
//! each picture arrives, from the pack's own files: nothing extra to keep.
//!
//! A leaf's cut-out edge comes from its picture's transparency, and
//! averaging thins it out in the smaller copies, so distant trees would go
//! bare; each copy's transparency is scaled to keep as much of it solid as
//! the full picture has.
//!
//! The `textures` setting drops the largest copies, for less of the graphics
//! card's memory: medium halves each picture over 1024 pixels, low quarters
//! each over 512. It applies to what loads after it changes.

use std::collections::HashSet;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::graphics::{Graphics, Level};

/// Gives each newly loaded picture its smaller copies.
pub fn mipmaps(
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    graphics: Res<Graphics>,
    mut done: Local<HashSet<AssetId<Image>>>,
) {
    for event in events.read() {
        let AssetEvent::Added { id } = event else {
            continue;
        };
        if !done.insert(*id) {
            continue;
        }
        let Some(mut image) = images.get_mut(*id) else {
            continue;
        };
        let descriptor = &image.texture_descriptor;
        let srgb = match descriptor.format {
            TextureFormat::Rgba8UnormSrgb => true,
            TextureFormat::Rgba8Unorm => false,
            _ => continue,
        };
        let size = descriptor.size;
        if descriptor.dimension != TextureDimension::D2
            || descriptor.mip_level_count != 1
            || size.depth_or_array_layers != 1
            || size.width < 64
            || size.height < 64
        {
            continue;
        }
        let Some(data) = image.data.take() else {
            continue;
        };
        let drop = match graphics.textures {
            Level::Off | Level::Low => smaller_than(size.width.max(size.height), 512),
            Level::Medium => smaller_than(size.width.max(size.height), 1_024),
            Level::High => 0,
        };
        let (chain, levels, width, height) = chain(data, size.width, size.height, srgb, drop);
        image.data = Some(chain);
        image.texture_descriptor.size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        image.texture_descriptor.mip_level_count = levels;
    }
}

/// How many halvings bring `size` down to `most`.
fn smaller_than(mut size: u32, most: u32) -> u32 {
    let mut halvings = 0;
    while size > most {
        size /= 2;
        halvings += 1;
    }
    halvings
}

/// From sRGB to linear light, for averaging, and back: colours average as
/// light does, not as their numbers do.
fn to_linear(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn to_srgb(c: f32) -> u8 {
    let c = if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (c * 255.0).round().clamp(0.0, 255.0) as u8
}

/// The cut-out's edge: what's more solid than this is drawn.
const CUTOFF: u8 = 128;

fn coverage(pixels: &[u8], scale: f32) -> f32 {
    let solid = pixels
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|p| p[3] as f32 * scale >= CUTOFF as f32)
        .count();
    solid as f32 / (pixels.len() / 4).max(1) as f32
}

/// The picture and its halves, each half the size of the last down to a
/// pixel, all one after the other, less the first `drop` of them: the
/// data, how many there are, and the size of the first.
fn chain(
    data: Vec<u8>,
    width: u32,
    height: u32,
    srgb: bool,
    drop: u32,
) -> (Vec<u8>, u32, u32, u32) {
    let decode = |c: u8| if srgb { to_linear(c) } else { c as f32 / 255.0 };
    let encode = |c: f32| {
        if srgb {
            to_srgb(c)
        } else {
            (c * 255.0).round().clamp(0.0, 255.0) as u8
        }
    };
    let solid = coverage(&data, 1.0);
    let cut_out = solid > 0.0 && solid < 0.999;
    let mut levels: Vec<(Vec<u8>, u32, u32)> = vec![(data, width, height)];
    loop {
        let (last, w, h) = levels.last().expect("the full picture");
        if *w == 1 && *h == 1 {
            break;
        }
        let (nw, nh) = ((*w / 2).max(1), (*h / 2).max(1));
        let mut next = vec![0u8; (nw * nh * 4) as usize];
        for y in 0..nh {
            for x in 0..nw {
                let mut sum = [0f32; 4];
                for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let sx = (x * 2 + dx).min(w - 1);
                    let sy = (y * 2 + dy).min(h - 1);
                    let i = ((sy * w + sx) * 4) as usize;
                    for c in 0..3 {
                        sum[c] += decode(last[i + c]);
                    }
                    sum[3] += last[i + 3] as f32;
                }
                let o = ((y * nw + x) * 4) as usize;
                for c in 0..3 {
                    next[o + c] = encode(sum[c] / 4.0);
                }
                next[o + 3] = (sum[3] / 4.0).round() as u8;
            }
        }
        if cut_out {
            keep_coverage(&mut next, solid);
        }
        levels.push((next, nw, nh));
    }
    let drop = (drop as usize).min(levels.len() - 1);
    let (_, w, h) = levels[drop];
    let count = (levels.len() - drop) as u32;
    let all = levels
        .into_iter()
        .skip(drop)
        .flat_map(|(d, ..)| d)
        .collect();
    (all, count, w, h)
}

/// Scales a copy's transparency so as much of it is solid as in the full
/// picture.
fn keep_coverage(pixels: &mut [u8], solid: f32) {
    let (mut low, mut high) = (0.25f32, 8.0f32);
    for _ in 0..12 {
        let mid = (low + high) / 2.0;
        if coverage(pixels, mid) < solid {
            low = mid;
        } else {
            high = mid;
        }
    }
    let scale = (low + high) / 2.0;
    for p in pixels.as_chunks_mut::<4>().0 {
        p[3] = (p[3] as f32 * scale).round().clamp(0.0, 255.0) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn picture(w: u32, h: u32, alpha: impl Fn(u32, u32) -> u8) -> Vec<u8> {
        let alpha = &alpha;
        (0..h)
            .flat_map(|y| (0..w).flat_map(move |x| [200, 100, 50, alpha(x, y)]))
            .collect()
    }

    #[test]
    fn every_half_down_to_a_pixel_follows_the_picture() {
        let (data, levels, w, h) = chain(picture(256, 64, |_, _| 255), 256, 64, true, 0);
        assert_eq!((levels, w, h), (9, 256, 64));
        // 256x64, 128x32, 64x16, 32x8, 16x4, 8x2, 4x1, 2x1, 1x1.
        let pixels: u32 = [256 * 64, 128 * 32, 64 * 16, 32 * 8, 16 * 4, 8 * 2, 4, 2, 1]
            .iter()
            .sum();
        assert_eq!(data.len() as u32, pixels * 4);
        // A flat colour stays that colour.
        assert_eq!(&data[data.len() - 4..], &[200, 100, 50, 255]);
    }

    #[test]
    fn a_lower_texture_setting_leaves_out_the_largest() {
        let (_, levels, w, h) = chain(picture(2048, 2048, |_, _| 255), 2048, 2048, false, 2);
        assert_eq!((levels, w, h), (10, 512, 512));
        assert_eq!(smaller_than(2048, 512), 2);
        assert_eq!(smaller_than(512, 512), 0);
    }

    #[test]
    fn a_cut_out_stays_solid_far_away() {
        // Specks, one pixel in nine: averaged plainly, every copy would fall
        // below the cut-off, and the leaves would vanish at a distance.
        let full = picture(
            96,
            96,
            |x, y| if x % 3 == 0 && y % 3 == 0 { 255 } else { 0 },
        );
        let (data, ..) = chain(full, 96, 96, true, 0);
        let mut start = 96 * 96 * 4;
        for size in [48, 24, 12] {
            let copy = &data[start..start + size * size * 4];
            assert!(
                coverage(copy, 1.0) > 0.05,
                "{size}: {}",
                coverage(copy, 1.0)
            );
            start += size * size * 4;
        }
    }
}
