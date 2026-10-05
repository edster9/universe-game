//! How the scene is drawn, for speed against looks: the settings games
//! usually have, set one by one or all at once by a preset (`/quality low`,
//! `medium`, `high`, `ultra`). They're the person's own machine's, not the
//! world's, so they're kept between games, in `graphics.toml` beside the
//! program, and always allowed (docs/ideas/tools.md). What each costs is
//! measured in docs/research/rendering-benchmarks.md.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use bevy::anti_alias::fxaa::Fxaa;
use bevy::anti_alias::smaa::Smaa;
use bevy::camera::Hdr;
use bevy::light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::window::PresentMode;
use serde::{Deserialize, Serialize};

/// A preset, or "custom" once a setting has been changed on its own.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    Medium,
    High,
    Ultra,
    Custom,
}

/// How much of something: shadows can also be off.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Off,
    Low,
    Medium,
    High,
}

/// How jagged edges are smoothed: FXAA and SMAA blur them after drawing,
/// cheaply; MSAA draws edges four times over, sharper and dearer, above
/// all in the graphics card's memory.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Smoothing {
    Off,
    Fxaa,
    Smaa,
    Msaa,
}

/// Every drawing setting.
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Graphics {
    pub quality: Quality,
    /// The sun's shadows: off, or how far they reach and how sharp they are.
    pub shadows: Level,
    pub smoothing: Smoothing,
    /// A glow round flames and bright things (bloom, with the wider colour
    /// range it needs).
    pub bloom: bool,
    /// How far anything is drawn, in metres.
    pub view: u32,
    /// Haze thickening towards the edge of the view, which hides it.
    pub fog: bool,
    /// Wait for the screen, so no frame is drawn that isn't shown.
    pub vsync: bool,
    /// At most this many frames a second, or 0 for no cap: saves power
    /// and heat on a laptop.
    pub fps_cap: u32,
    /// Changes are written to `graphics.toml`; not when measuring, or
    /// taking pictures, which shouldn't change the player's own choices.
    #[serde(skip)]
    pub kept: bool,
}

impl Default for Graphics {
    fn default() -> Self {
        Graphics::preset(Quality::High)
    }
}

/// The drawing settings: name, values, and what each does.
pub const SETTINGS: &[(&str, &str, &str)] = &[
    (
        "quality",
        "low, medium, high, ultra",
        "sets all the drawing settings below at once (custom once one is changed)",
    ),
    (
        "shadows",
        "off, low, medium, high",
        "the sun's shadows: how far they reach and how sharp they are",
    ),
    (
        "smoothing",
        "off, fxaa, smaa, msaa",
        "smoothed edges: fxaa and smaa are cheap blurs, msaa sharper and dearer",
    ),
    ("bloom", "on, off", "a glow round flames and bright things"),
    ("view", "metres", "how far anything is drawn"),
    (
        "fog",
        "on, off",
        "haze thickening towards the edge of the view",
    ),
    (
        "vsync",
        "on, off",
        "wait for the screen, drawing no frame it won't show",
    ),
    (
        "fps-cap",
        "a number, or off",
        "at most this many frames a second, to save power and heat",
    ),
];

fn on(b: bool) -> String {
    if b { "on" } else { "off" }.to_string()
}

fn named<T: Serialize>(value: T) -> String {
    toml::Value::try_from(value)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default()
}

fn parse<T: for<'de> Deserialize<'de>>(text: &str) -> Option<T> {
    T::deserialize(toml::Value::String(text.to_lowercase())).ok()
}

fn flag(name: &str, now: bool, value: Option<&str>) -> Result<bool, String> {
    match value {
        None => Ok(!now),
        Some("on" | "yes" | "true" | "1") => Ok(true),
        Some("off" | "no" | "false" | "0") => Ok(false),
        Some(other) => Err(format!("{name} is on or off, not \"{other}\"")),
    }
}

impl Graphics {
    /// A preset's settings. Display choices (vsync, the cap) aren't part
    /// of quality, so a preset leaves them as they were.
    pub fn preset(quality: Quality) -> Graphics {
        let (shadows, smoothing, bloom, view) = match quality {
            Quality::Low => (Level::Off, Smoothing::Fxaa, false, 250),
            Quality::Medium => (Level::Low, Smoothing::Smaa, true, 500),
            Quality::High | Quality::Custom => (Level::Medium, Smoothing::Msaa, true, 1_000),
            Quality::Ultra => (Level::High, Smoothing::Msaa, true, 2_000),
        };
        Graphics {
            quality,
            shadows,
            smoothing,
            bloom,
            view,
            fog: true,
            vsync: true,
            fps_cap: 0,
            kept: true,
        }
    }

    /// A drawing setting's value as said, or None if it isn't one.
    pub fn value(&self, name: &str) -> Option<String> {
        Some(match name {
            "quality" => named(self.quality),
            "shadows" => named(self.shadows),
            "smoothing" => named(self.smoothing),
            "bloom" => on(self.bloom),
            "view" => format!("{} m", self.view),
            "fog" => on(self.fog),
            "vsync" => on(self.vsync),
            "fps-cap" if self.fps_cap == 0 => "off".to_string(),
            "fps-cap" => self.fps_cap.to_string(),
            _ => return None,
        })
    }

    /// Sets a drawing setting, or flips one that's on or off; None if
    /// `name` isn't one. A preset sets them all; changing one on its own
    /// makes the quality custom.
    pub fn set(&mut self, name: &str, value: Option<&str>) -> Option<Result<(), String>> {
        let values = SETTINGS.iter().find(|(n, ..)| *n == name)?.1;
        let wrong = || format!("{name} is one of {values}");
        let result = match name {
            "quality" => {
                let quality = value
                    .and_then(parse::<Quality>)
                    .filter(|q| *q != Quality::Custom)
                    .ok_or_else(wrong);
                return Some(quality.map(|quality| {
                    *self = Graphics {
                        vsync: self.vsync,
                        fps_cap: self.fps_cap,
                        kept: self.kept,
                        ..Graphics::preset(quality)
                    }
                }));
            }
            "shadows" => value
                .and_then(parse)
                .map(|l| self.shadows = l)
                .ok_or_else(wrong),
            "smoothing" => value
                .and_then(parse)
                .map(|s| self.smoothing = s)
                .ok_or_else(wrong),
            "bloom" => flag(name, self.bloom, value).map(|b| self.bloom = b),
            "fog" => flag(name, self.fog, value).map(|b| self.fog = b),
            "vsync" => flag(name, self.vsync, value).map(|b| self.vsync = b),
            "view" => value
                .and_then(|v| v.trim_end_matches('m').parse().ok())
                .filter(|v| (20..=20_000).contains(v))
                .map(|v| self.view = v)
                .ok_or_else(|| "view is a distance in metres, from 20 to 20000".to_string()),
            "fps-cap" => match value {
                Some("off" | "0") => Ok(0),
                _ => value
                    .and_then(|v| v.parse().ok())
                    .filter(|v| (10..=1_000).contains(v))
                    .ok_or_else(|| "fps-cap is off, or a number from 10 to 1000".to_string()),
            }
            .map(|cap| self.fps_cap = cap),
            _ => Err(wrong()),
        };
        if result.is_ok() && !matches!(name, "vsync" | "fps-cap") {
            self.quality = Quality::Custom;
        }
        Some(result)
    }

    /// What a preset sets, as said.
    pub fn describe(quality: Quality) -> String {
        let g = Graphics::preset(quality);
        SETTINGS
            .iter()
            .filter(|(name, ..)| !matches!(*name, "quality" | "vsync" | "fps-cap"))
            .map(|(name, ..)| format!("{name} {}", g.value(name).unwrap_or_default()))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The kept settings, or a preset's if there are none.
    pub fn load() -> Graphics {
        std::fs::read_to_string(path())
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }

    /// Keeps the settings for the next game, unless told not to.
    pub fn keep(&self) {
        if !self.kept {
            return;
        }
        if let Ok(text) = toml::to_string(self)
            && let Err(e) = std::fs::write(path(), text)
        {
            warn!("Couldn't keep the drawing settings: {e}");
        }
    }
}

/// Where the settings are kept: beside the program.
fn path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.join("graphics.toml")))
        .unwrap_or_else(|| PathBuf::from("graphics.toml"))
}

/// The sun's shadows for each level: how many layers (cascades, finer near
/// the eye), how far, and how many pixels each layer's map has across.
fn shadow_reach(level: Level) -> (usize, f32, usize) {
    match level {
        Level::Off | Level::Low => (1, 40.0, 1_024),
        Level::Medium => (2, 100.0, 2_048),
        Level::High => (3, 200.0, 4_096),
    }
}

/// Draws the scene as the settings say, when they change; the haze takes
/// the sky's colour, so the edge of the view fades into it, day or night.
#[allow(clippy::too_many_arguments)]
pub fn apply(
    mut commands: Commands,
    graphics: Res<Graphics>,
    sky: Res<ClearColor>,
    mut camera: Query<(Entity, &mut Projection, Option<&mut DistanceFog>), With<Camera3d>>,
    mut sun: Query<(Entity, &mut DirectionalLight), With<crate::Sun>>,
    mut windows: Query<&mut Window>,
) {
    let Ok((camera, mut projection, haze)) = camera.single_mut() else {
        return;
    };
    let view = graphics.view as f32;
    let fog = || DistanceFog {
        color: sky.0,
        falloff: FogFalloff::Linear {
            start: view * 0.5,
            end: view,
        },
        ..default()
    };
    if !graphics.is_changed() {
        if sky.is_changed()
            && let Some(mut haze) = haze
        {
            haze.color = sky.0;
        }
        return;
    }
    let mut camera = commands.entity(camera);
    match graphics.smoothing {
        Smoothing::Msaa => {
            camera.insert(Msaa::Sample4).remove::<(Fxaa, Smaa)>();
        }
        Smoothing::Fxaa => {
            camera.insert((Msaa::Off, Fxaa::default())).remove::<Smaa>();
        }
        Smoothing::Smaa => {
            camera.insert((Msaa::Off, Smaa::default())).remove::<Fxaa>();
        }
        Smoothing::Off => {
            camera.insert(Msaa::Off).remove::<(Fxaa, Smaa)>();
        }
    }
    if graphics.bloom {
        camera.insert(Bloom::NATURAL);
    } else {
        camera.remove::<(Bloom, Hdr)>();
    }
    if graphics.fog {
        camera.insert(fog());
    } else {
        camera.remove::<DistanceFog>();
    }
    if let Projection::Perspective(perspective) = &mut *projection {
        perspective.far = view;
    }
    let (layers, reach, pixels) = shadow_reach(graphics.shadows);
    commands.insert_resource(DirectionalLightShadowMap { size: pixels });
    for (sun, mut light) in &mut sun {
        light.shadow_maps_enabled = graphics.shadows != Level::Off;
        commands.entity(sun).insert(
            CascadeShadowConfigBuilder {
                num_cascades: layers,
                first_cascade_far_bound: (reach * 0.15).min(15.0),
                maximum_distance: reach.min(view),
                ..default()
            }
            .build(),
        );
    }
    for mut window in &mut windows {
        window.present_mode = if graphics.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };
    }
}

/// Holds each frame back to the cap, if there is one.
pub fn cap(graphics: Res<Graphics>, mut last: Local<Option<Instant>>) {
    if graphics.fps_cap == 0 {
        *last = None;
        return;
    }
    let frame = Duration::from_secs_f64(1.0 / graphics.fps_cap as f64);
    if let Some(then) = *last {
        let wait = (then + frame).saturating_duration_since(Instant::now());
        std::thread::sleep(wait);
    }
    *last = Some(Instant::now());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_preset_sets_everything_and_one_change_makes_it_custom() {
        let mut g = Graphics::preset(Quality::Ultra);
        assert_eq!(g.set("quality", Some("low")), Some(Ok(())));
        assert_eq!(g, Graphics::preset(Quality::Low));
        assert_eq!(g.set("shadows", Some("high")), Some(Ok(())));
        assert_eq!(g.quality, Quality::Custom);
        assert_eq!(g.value("shadows").as_deref(), Some("high"));
        // Display choices aren't quality, and a preset keeps them.
        g.set("quality", Some("medium"));
        g.set("fps-cap", Some("60"));
        g.set("vsync", Some("off"));
        assert_eq!(g.quality, Quality::Medium);
        g.set("quality", Some("high"));
        assert_eq!((g.fps_cap, g.vsync), (60, false));
    }

    #[test]
    fn wrong_values_are_refused_and_other_names_left_alone() {
        let mut g = Graphics::default();
        assert!(matches!(g.set("shadows", Some("lots")), Some(Err(_))));
        assert!(matches!(g.set("quality", Some("custom")), Some(Err(_))));
        assert!(matches!(g.set("view", Some("5")), Some(Err(_))));
        assert_eq!(g.set("view", Some("300m")), Some(Ok(())));
        assert_eq!(g.value("view").as_deref(), Some("300 m"));
        assert_eq!(g.set("grid", None), None);
        assert_eq!(g.set("bloom", None), Some(Ok(())));
        assert!(!g.bloom);
    }

    #[test]
    fn settings_are_kept_as_written_and_read_back() {
        let mut g = Graphics::preset(Quality::Medium);
        g.set("smoothing", Some("fxaa"));
        let text = toml::to_string(&g).unwrap();
        assert!(text.contains("smoothing = \"fxaa\""), "{text}");
        let back: Graphics = toml::from_str(&text).unwrap();
        assert_eq!(back.smoothing, Smoothing::Fxaa);
        assert_eq!(back.quality, Quality::Custom);
        // An old or partial file still reads, the rest from the default.
        let partial: Graphics = toml::from_str("shadows = \"off\"").unwrap();
        assert_eq!(partial.shadows, Level::Off);
        assert_eq!(partial.view, Graphics::default().view);
    }
}
