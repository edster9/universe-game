//! The rendering benchmark (`--bench <metres>`, docs/research/
//! rendering-benchmarks.md): once the world is loaded, the camera goes
//! round a fixed route, the same every run, so runs can be compared: inside
//! the forest at eye height, walking through it, at its edge looking in,
//! and high above looking over all of it. `<metres>` is how far the forest
//! reaches from the middle. At each stop it holds a moment for the scene to
//! settle, then measures: frame times (the average, the worst 1%, the
//! worst), the graphics card (how busy, its power and clock, whether it
//! slowed itself, its memory), and how many meshes were drawn and their
//! triangles. With vsync on (the default), frames are capped at the
//! screen's rate, so how busy the card is shows the room left. Each stop is a line in `bench.csv` beside the program,
//! and the game ends after the last. `--bench-label <text>` names the run
//! (the drawing settings are set as ever, by `--type "/quality low"`).

use std::io::Write;
use std::time::Instant;

use bevy::prelude::*;
use bevy::world_serialization::WorldAssetRoot;

/// Seconds at each stop before measuring, and measuring.
const SETTLE: f32 = 2.0;
const MEASURE: f32 = 5.0;

#[derive(Resource)]
pub struct Bench {
    reach: f32,
    label: String,
    world: String,
    began: Instant,
    /// Seconds from the start until the world was loaded and drawn.
    loaded: Option<f32>,
    /// Meshes, and since when there have been that many: loading is done
    /// when the count holds still.
    settling: (usize, f32),
    stop: usize,
    since: f32,
    frames: Vec<f32>,
    /// The graphics card, watched while measuring.
    watching: Option<std::thread::JoinHandle<Option<Card>>>,
}

impl Bench {
    /// The benchmark, if asked for.
    pub fn wanted(world: &str) -> Option<Bench> {
        let args: Vec<String> = std::env::args().collect();
        let after = |name: &str| {
            args.iter()
                .position(|a| a == name)
                .and_then(|i| args.get(i + 1))
        };
        let reach = after("--bench")?.parse().ok()?;
        Some(Bench {
            reach,
            label: after("--bench-label").cloned().unwrap_or_default(),
            world: world.trim_end_matches(".toml").to_string(),
            began: Instant::now(),
            loaded: None,
            settling: (0, 0.0),
            stop: 0,
            since: 0.0,
            frames: Vec::new(),
            watching: None,
        })
    }
}

/// A stop on the route: where the camera goes (from, to, while measuring),
/// and what it looks at. North is -z.
struct Stop {
    name: &'static str,
    from: Vec3,
    to: Vec3,
    look: Vec3,
}

fn route(r: f32) -> [Stop; 4] {
    let eye = 1.7;
    [
        Stop {
            name: "inside",
            from: Vec3::new(1.5, eye, 1.5),
            to: Vec3::new(1.5, eye, 1.5),
            look: Vec3::new(1.5, 1.5, -30.0),
        },
        Stop {
            name: "walking",
            from: Vec3::new(0.0, eye, r * 0.5),
            to: Vec3::new(0.0, eye, r * 0.5 - 10.0),
            look: Vec3::new(0.0, 1.5, -r),
        },
        Stop {
            name: "edge",
            from: Vec3::new(0.0, eye, r + 10.0),
            to: Vec3::new(0.0, eye, r + 10.0),
            look: Vec3::new(0.0, 1.5, 0.0),
        },
        Stop {
            name: "above",
            from: Vec3::new(-r * 0.8, r * 0.5 + 20.0, r + 20.0),
            to: Vec3::new(-r * 0.8, r * 0.5 + 20.0, r + 20.0),
            look: Vec3::ZERO,
        },
    ]
}

/// Starts the window maximized, as with `--maximized`.
pub fn maximize(mut windows: Query<&mut Window>) {
    if std::env::args().any(|a| a == "--maximized") {
        for mut window in &mut windows {
            window.set_maximized(true);
        }
    }
}

/// Runs the route, after the drawing has moved the camera, so the route
/// wins.
#[allow(clippy::too_many_arguments)]
pub fn run(
    time: Res<Time>,
    bench: Option<ResMut<Bench>>,
    assets: Res<AssetServer>,
    roots: Query<&WorldAssetRoot>,
    drawn: Query<(&Mesh3d, &ViewVisibility)>,
    meshes: Res<Assets<Mesh>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    windows: Query<&Window>,
    mut exit: MessageWriter<AppExit>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    let Some(mut bench) = bench else {
        return;
    };
    let dt = time.delta_secs();
    if bench.loaded.is_none() {
        // Loaded: every model's files are in, and the meshes have stopped
        // multiplying for a second.
        let count = drawn.iter().count();
        let all_in = roots
            .iter()
            .all(|root| assets.is_loaded_with_dependencies(root.0.id()));
        if count != bench.settling.0 || !all_in {
            bench.settling = (count, 0.0);
        } else {
            bench.settling.1 += dt;
        }
        if bench.settling.1 >= 1.0 {
            bench.loaded = Some(bench.began.elapsed().as_secs_f32() - 1.0);
        }
        return;
    }
    let route = route(bench.reach);
    let stop = &route[bench.stop];
    bench.since += dt;
    // Longer at the first stop, while the graphics card's programs for
    // what's drawn are still being made.
    let settle = if bench.stop == 0 {
        SETTLE + 3.0
    } else {
        SETTLE
    };
    let measuring = bench.since > settle;
    if measuring && bench.watching.is_none() {
        bench.watching = Some(watch(MEASURE));
    }
    if measuring {
        bench.frames.push(dt * 1000.0);
        if dt > 0.05 {
            let (stop, since) = (bench.stop, bench.since);
            // What the graphics card took lately, stage by stage: the
            // worst of the last 20 frames.
            let stages: Vec<String> = diagnostics
                .iter()
                .filter(|d| d.path().as_str().ends_with("elapsed_gpu"))
                .map(|d| {
                    let most = d.values().copied().fold(0.0, f64::max);
                    let stage = d.path().as_str().trim_start_matches("render/");
                    (stage.to_string(), most)
                })
                .filter(|(_, most)| *most > 2.0)
                .map(|(stage, most)| format!("{stage} {most:.0}"))
                .collect();
            info!(
                "BENCH slow frame: {:.0} ms at stop {stop}, {since:.2} s in; card: {}",
                dt * 1000.0,
                stages.join(", ")
            );
        }
    }
    let along = ((bench.since - settle) / MEASURE).clamp(0.0, 1.0);
    let at = stop.from.lerp(stop.to, along);
    if let Ok(mut eye) = camera.single_mut() {
        *eye = Transform::from_translation(at).looking_at(stop.look, Vec3::Y);
    }
    if bench.since < settle + MEASURE {
        return;
    }

    // What was drawn, as the stop ends.
    let (mut shown, mut triangles) = (0usize, 0usize);
    for (mesh, visible) in &drawn {
        if !visible.get() {
            continue;
        }
        shown += 1;
        if let Some(mesh) = meshes.get(&mesh.0) {
            triangles += mesh.indices().map_or(mesh.count_vertices(), |i| i.len()) / 3;
        }
    }
    let mut frames = std::mem::take(&mut bench.frames);
    frames.sort_by(f32::total_cmp);
    let n = frames.len().max(1);
    let average = frames.iter().sum::<f32>() / n as f32;
    let worst_1 = frames[((n as f32 * 0.99).ceil() as usize).clamp(1, n) - 1];
    let worst = frames.last().copied().unwrap_or(0.0);
    let window = windows
        .iter()
        .next()
        .map(|w| format!("{}x{}", w.physical_width(), w.physical_height()))
        .unwrap_or_default();
    let card = bench
        .watching
        .take()
        .and_then(|w| w.join().ok().flatten())
        .map_or(",,,,".to_string(), |c| {
            format!(
                "{:.0},{:.0},{:.0},{},{:.0}",
                c.busy, c.watts, c.mhz, c.slowed, c.mb
            )
        });
    let line = format!(
        "{},{},{},{},{},{:.2},{:.0},{:.2},{:.2},{card},{},{},{:.1}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()),
        bench.world,
        bench.label,
        window,
        stop.name,
        average,
        1000.0 / average,
        worst_1,
        worst,
        shown,
        triangles / 1000,
        bench.loaded.unwrap_or(0.0),
    );
    println!("BENCH {line}");
    keep("bench.csv", BENCH_HEADER, &line);
    // What each stage of drawing took on the graphics card, where it can
    // say (`RenderDiagnosticsPlugin`: DirectX 12 and Vulkan).
    let mut stages: Vec<(String, f64)> = diagnostics
        .iter()
        .filter(|d| d.path().as_str().ends_with("elapsed_gpu"))
        .filter_map(|d| Some((d.path().as_str().to_string(), d.average()?)))
        .collect();
    stages.sort_by(|a, b| b.1.total_cmp(&a.1));
    for (stage, ms) in stages {
        let stage = stage
            .trim_start_matches("render/")
            .trim_end_matches("/elapsed_gpu");
        keep(
            "bench-stages.csv",
            "world,label,stop,stage,gpu_ms",
            &format!(
                "{},{},{},{stage},{ms:.3}",
                bench.world, bench.label, stop.name
            ),
        );
    }
    bench.since = 0.0;
    bench.stop += 1;
    if bench.stop == route.len() {
        exit.write(AppExit::Success);
    }
}

/// The graphics card while measuring, from NVIDIA's own tool: how busy it
/// was (%), its power (W), its clock (MHz), how many times in five a
/// second it slowed itself (for heat or power: what makes a freeze), and
/// its memory in use (MB).
struct Card {
    busy: f32,
    watts: f32,
    mhz: f32,
    slowed: usize,
    mb: f32,
}

/// Watches the card for `seconds`, in the background.
fn watch(seconds: f32) -> std::thread::JoinHandle<Option<Card>> {
    std::thread::spawn(move || {
        use std::io::BufRead;
        let mut tool = std::process::Command::new("nvidia-smi")
            .args([
                "--query-gpu=utilization.gpu,power.draw,clocks.gr,clocks_event_reasons.active,\
                 memory.used",
                "--format=csv,noheader,nounits",
                "-lms",
                "200",
            ])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .ok()?;
        let until = Instant::now() + std::time::Duration::from_secs_f32(seconds);
        let mut samples: Vec<Vec<String>> = Vec::new();
        for line in std::io::BufReader::new(tool.stdout.take()?).lines() {
            let Ok(line) = line else { break };
            samples.push(line.split(',').map(|f| f.trim().to_string()).collect());
            if Instant::now() > until {
                break;
            }
        }
        let _ = tool.kill();
        let _ = tool.wait();
        let n = samples.len().max(1) as f32;
        let mean = |i: usize| {
            samples
                .iter()
                .filter_map(|s| s.get(i)?.parse::<f32>().ok())
                .sum::<f32>()
                / n
        };
        // Slowed for heat or power, as a laptop does: thermal, hardware,
        // or power-brake slowdown (not just running at its power limit).
        let slowed = samples
            .iter()
            .filter_map(|s| u64::from_str_radix(s.get(3)?.trim_start_matches("0x"), 16).ok())
            .filter(|reasons| reasons & (0x8 | 0x20 | 0x40 | 0x80) != 0)
            .count();
        Some(Card {
            busy: mean(0),
            watts: mean(1),
            mhz: mean(2),
            slowed,
            mb: mean(4),
        })
    })
}

const BENCH_HEADER: &str = "when,world,label,window,stop,average_ms,fps,worst_1pc_ms,worst_ms,\
     card_busy_pc,card_watts,card_mhz,card_slowed,graphics_mb,meshes_drawn,thousand_triangles,load_s";

/// Adds a line to a file beside the program, with a header if it's new.
fn keep(file: &str, header: &str, line: &str) {
    let path = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.join(file)))
        .unwrap_or_else(|| file.into());
    let new = !path.exists();
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
    {
        if new {
            let _ = writeln!(file, "{header}");
        }
        let _ = writeln!(file, "{line}");
    }
}
