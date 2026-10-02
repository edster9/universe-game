//! The game's native client: one person's view of a world, drawn live from
//! the engine. The engine runs here, as it does in the tests; the client
//! draws only what the engine says the person pictures (`view::scene`), from
//! simple shapes and colours (see `style.toml`), and takes commands through
//! the console's own session. See docs/ideas/the-client.md and
//! docs/challenges/first-steps.md.
//!
//! Controls: hold the right mouse button and drag to look around the
//! islander, the wheel to come closer or go further; a left click on
//! something opens a menu of what to do with it, and on open ground walks
//! there; Enter to type a
//! command, Esc to stop, ` to resize the console; F to fly free (WASD, E/Q
//! up and down, Shift faster) and F again to snap back; B the backpack, V
//! the body (click it for everything measured); hold T and speak a command;
//! G shows a grid on the ground, for seeing movement; M is build mode (drag
//! things with the left button, lift them with the middle one);
//! Space pauses the
//! world, [ and ] slow it down and speed it up.
//!
//! Options:
//! - `--world <file>` a world in the data folder (skill-grounds.toml; the
//!   companion's island is companion.toml), `--as <id>` who to play (the
//!   first person no mind or instinct runs), `--speed <x>` game seconds a
//!   second (1).
//! - `--load <save>` picks up a save (`/save`, `/load`) from the `saves`
//!   folder beside the data folder; a game ends with a save called "last",
//!   except one that only takes a picture (`--shot`).
//! - `--shot <file.png> [--after <seconds>]` saves one frame and exits.
//! - `--yaw`, `--pitch` (degrees) and `--zoom` (metres) set the camera
//!   around the islander; `--from x,y,z --look x,y,z` start it flying.
//! - `--script <file> [--shots <folder>] [--step <seconds>]` plays a script
//!   through the console, saving a screenshot at each expectation, and exits
//!   when it's over (with an error if it failed).
//! - `--hear-script <file>` plays a script by voice, from recordings of
//!   its commands (see `client/voice-proof.sh`), and exits.
//! - `--open backpack,body,grid` (or `body-all`) opens those at the start.
//! - `--type "<command>; <command>"` types commands at the start, as the
//!   player would, for trying things without a keyboard.

use std::path::PathBuf;

use bevy::camera_controller::free_camera::FreeCameraPlugin;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use console::script::Playing;
use console::session::Session;
use engine::world::{EntityId, World as EngineWorld};

mod build;
mod camera;
mod draw;
mod grid;
mod menu;
mod panels;
mod terminal;
mod terrain;
mod tools;
mod voice;
mod walking;

use crate::terminal::{Console, Said};
use terrain::Land;

/// What's being played: a person, live, or a script.
pub enum Play {
    Live(Box<Session>),
    Script(Box<Playing>),
}

/// The engine's world, as the session holds it, and how fast it runs.
#[derive(Resource)]
pub struct Sim {
    pub play: Play,
    /// Game seconds per real second.
    speed: f32,
    paused: bool,
    /// Game time not yet run, in seconds.
    owed: f32,
}

impl Sim {
    pub fn session(&self) -> &Session {
        match &self.play {
            Play::Live(session) => session,
            Play::Script(playing) => playing.session(),
        }
    }

    pub fn world(&self) -> &EngineWorld {
        self.session().world()
    }

    /// Who's being played.
    pub fn me(&self) -> EntityId {
        self.session().player()
    }

    /// The world's time, with the fraction of a second not yet run.
    /// Game seconds a real second.
    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn now(&self) -> f32 {
        self.world().tick() as f32 + self.owed
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct Sun;

/// What the command line asked for.
#[derive(Resource)]
pub struct Options {
    shot: Option<String>,
    after: f32,
    pub from: Option<Vec3>,
    pub look: Option<Vec3>,
    pub yaw: Option<f32>,
    pub pitch: Option<f32>,
    pub zoom: Option<f32>,
    pub script: Option<String>,
    pub shots: String,
    pub step: f32,
    /// Commands typed at the start, separated by ";".
    pub typed: String,
    /// Windows open at the start: backpack, body, body-all.
    pub open: String,
}

fn point(text: Option<&String>) -> Option<Vec3> {
    let v: Vec<f32> = text
        .map(|t| t.split(',').filter_map(|n| n.trim().parse().ok()).collect())
        .unwrap_or_default();
    (v.len() == 3).then(|| Vec3::new(v[0], v[1], v[2]))
}

/// The data folder: beside the program, or the project's.
fn data_dir() -> PathBuf {
    let beside = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.join("data")));
    [
        beside,
        Some(PathBuf::from("data")),
        Some(PathBuf::from("../data")),
    ]
    .into_iter()
    .flatten()
    .find(|p| p.join("scripts").is_dir())
    .unwrap_or_else(|| PathBuf::from("data"))
}

fn main() {
    STARTED.get_or_init(std::time::Instant::now);
    // "--from x,y,z" and "--from=x,y,z" alike.
    let args: Vec<String> = std::env::args()
        .flat_map(|a| match a.split_once('=') {
            Some((k, v)) if a.starts_with("--") => vec![k.to_string(), v.to_string()],
            _ => vec![a],
        })
        .collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
    };
    let number = |name: &str| arg(name).and_then(|a| a.parse::<f32>().ok());
    let options = Options {
        shot: arg("--shot").cloned(),
        after: number("--after").unwrap_or(4.0),
        from: point(arg("--from")),
        look: point(arg("--look")),
        yaw: number("--yaw"),
        pitch: number("--pitch"),
        zoom: number("--zoom"),
        script: arg("--script").cloned(),
        shots: arg("--shots").cloned().unwrap_or("shots".into()),
        step: number("--step").unwrap_or(0.4),
        typed: arg("--type").cloned().unwrap_or_default(),
        open: arg("--open").cloned().unwrap_or_default(),
    };
    let data = data_dir();
    let play = match &options.script {
        Some(script) => {
            // A script's name, or its path.
            let path = [PathBuf::from(script), data.join("scripts").join(script)]
                .into_iter()
                .find(|p| p.is_file())
                .unwrap_or_else(|| fail(&format!("there's no script {script}")));
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| fail(&e.to_string()));
            let _ = std::fs::create_dir_all(&options.shots);
            Play::Script(Box::new(
                Playing::load(&text, &data).unwrap_or_else(|e| fail(&e)),
            ))
        }
        None => {
            let saves = data.parent().unwrap_or(&data).join("saves");
            let session = match arg("--load") {
                Some(name) => Session::resume(saves, name),
                None => {
                    let file = arg("--world").map_or("skill-grounds.toml", String::as_str);
                    console::load_world_file(&data.join(file)).and_then(|world| {
                        // Whoever nobody else plays, unless --as says.
                        let who = match arg("--as") {
                            Some(who) => who.clone(),
                            None => console::person_to_play(&world)
                                .ok_or("this world has no one to play")?,
                        };
                        let world = world.with_player_rules(&who)?;
                        Session::new(world, &who).map(|session| session.with_saves(saves))
                    })
                }
            }
            .unwrap_or_else(|e| fail(&e));
            Play::Live(Box::new(session.with_real_time()))
        }
    };
    let model = data
        .parent()
        .unwrap_or(&data)
        .join("models")
        .join("ggml-base.en.bin");
    if let Some(script) = arg("--hear-script") {
        std::process::exit(hear_script(script, &data, &model));
    }
    let style: draw::Style = toml::from_str(draw::STYLE).expect("the style file");
    let land = Land::of(play_world(&play));
    let speed = number("--speed").unwrap_or(1.0);

    keep_crashes();
    // DirectX 12 on Windows unless WGPU_BACKEND says otherwise: Vulkan on
    // this laptop's NVIDIA chip has lost the device now and then.
    let mut wgpu = bevy::render::settings::WgpuSettings::default();
    if cfg!(windows) && std::env::var("WGPU_BACKEND").is_err() {
        wgpu.backends = Some(bevy::render::settings::Backends::DX12);
    }
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(Box::new(wgpu)),
                ..default()
            })
            .set(bevy::log::LogPlugin {
                custom_layer: file_log,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Universe game".into(),
                    resolution: (1600, 900).into(),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(FreeCameraPlugin)
    .insert_resource(ClearColor(Color::srgb(0.55, 0.72, 0.9)))
    .insert_resource(Sim {
        play,
        speed,
        paused: false,
        owed: 0.0,
    })
    .insert_resource(style)
    .insert_resource(land)
    .insert_resource(options)
    .init_resource::<draw::Kit>()
    .init_resource::<Console>()
    .init_resource::<terminal::Shots>()
    .init_resource::<tools::Settings>()
    .init_resource::<draw::Drawn>()
    .init_resource::<build::Build>()
    .init_resource::<walking::ByKeys>()
    .insert_resource(voice::Voice::new(&model))
    .add_systems(
        Startup,
        (
            setup,
            camera::setup,
            terminal::setup,
            panels::setup,
            voice::announce,
            grid::setup,
        ),
    )
    .add_systems(
        Update,
        (
            // What the player does: keys, typing, speaking.
            (
                terminal::type_in,
                controls,
                panels::keys,
                build::toggle,
                menu::click,
                tools::run_typed,
                walking::walk_keys,
                grid::toggle,
                voice::push_to_talk,
                voice::run_heard,
            )
                .chain(),
            // Then the world, and drawing it.
            (
                run_world,
                draw::follow_me,
                terminal::play_script,
                draw::draw_scenery,
                draw::draw_movers,
                build::drag,
                camera::follow,
                camera::point,
                grid::draw,
                walking::draw_reach,
                day_and_night,
                hud,
                terminal::show,
                panels::show,
                terminal::take_shots,
                shot,
                frames,
            )
                .chain(),
        )
            .chain(),
    );
    app.add_systems(Last, save_at_end.after(bevy::window::ExitSystems));
    if std::env::args().any(|a| a == "--frames")
        && let Some(render) = app.get_sub_app_mut(bevy::render::RenderApp)
    {
        render.add_systems(bevy::render::Render, shaders_waiting);
    }
    use_system_font(&mut app);
    app.run();
}

/// A game ends with a save, and the next can start from it: in the frame
/// the game is told to end, unless it was only taking a picture.
fn save_at_end(
    mut exits: MessageReader<AppExit>,
    mut sim: ResMut<Sim>,
    options: Res<Options>,
    mut saved: Local<bool>,
) {
    if exits.read().next().is_none() || *saved || options.shot.is_some() {
        return;
    }
    *saved = true;
    if let Play::Live(session) = &mut sim.play {
        match session.save(console::session::LAST) {
            Ok(text) => info!("{text}"),
            Err(e) => warn!("{e}"),
        }
    }
}

/// Where the client keeps its log: beside the program.
fn log_path() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.join("client.log")))
        .unwrap_or_else(|| PathBuf::from("client.log"))
}

/// Everything the client logs also goes to `client.log`, so a freeze or a
/// crash leaves its reasons behind, even when it was started by a double
/// click with nowhere to print them.
fn file_log(_: &mut App) -> Option<bevy::log::BoxedLayer> {
    use bevy::log::tracing_subscriber::{self, Layer};
    let file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path())
        .ok()?;
    Some(
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(std::sync::Mutex::new(file))
            .boxed(),
    )
}

/// Panics go to `client.log` too, after a line saying when the client
/// started, so each run can be told apart.
fn keep_crashes() {
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path())
    {
        let _ = writeln!(
            file,
            "--- the client starts: {:?}",
            std::env::args().collect::<Vec<_>>()
        );
    }
    let usual = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(log_path()) {
            let _ = writeln!(file, "PANIC: {info}");
        }
        usual(info);
    }));
}

/// Windows' own Consolas, when it's there, for the text: it has every sign
/// the engine writes (µ among them), which the built-in font lacks.
fn use_system_font(app: &mut App) {
    let Ok(bytes) = std::fs::read("C:\\Windows\\Fonts\\consola.ttf") else {
        return;
    };
    let mut fonts = app.world_mut().resource_mut::<Assets<Font>>();
    let _ = fonts.insert(AssetId::default(), Font::from_bytes(bytes));
}

/// Plays a script by voice: each command, spoken into
/// `voice/<command>.wav` beside the program (by `client/voice-proof.sh`), is
/// heard as the islander would hear it there and then, and run as heard.
/// Expectations are checked as ever. Returns the exit code.
fn hear_script(script: &str, data: &std::path::Path, model: &std::path::Path) -> i32 {
    let path = [PathBuf::from(script), data.join("scripts").join(script)]
        .into_iter()
        .find(|p| p.is_file())
        .unwrap_or_else(|| fail(&format!("there's no script {script}")));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| fail(&e.to_string()));
    let mut playing = Playing::load(&text, data).unwrap_or_else(|e| fail(&e));
    let model = voice::load_model(model).unwrap_or_else(|e| fail(&e));
    let recordings = data.parent().unwrap_or(data).join("voice");
    let (mut spoken, mut wrong) = (0, 0);
    while let Some(line) = playing.peek().map(str::to_string) {
        if !line.starts_with("expect ") {
            let (try_, said) = match line.strip_prefix("try ") {
                Some(rest) => ("try ", rest.trim().to_string()),
                None => ("", line.clone()),
            };
            let wav = recordings.join(format!("{}.wav", voice::file_name(&said)));
            let audio = voice::read_wav(&wav).unwrap_or_else(|e| fail(&e));
            let started = std::time::Instant::now();
            let session = playing.session();
            let prompt = voice::prompt(session.world(), session.player());
            let heard = voice::hear(&model, &audio, &prompt).unwrap_or_else(|e| fail(&e));
            spoken += 1;
            let same = heard == said;
            if !same {
                wrong += 1;
            }
            println!(
                "{} said {said:?}, heard {heard:?} in {:.2} s",
                if same { "  " } else { "!!" },
                started.elapsed().as_secs_f32()
            );
            playing.rewrite_next(format!("{try_}{heard}"));
        }
        match playing.step() {
            Some(Ok(_)) => {}
            Some(Err(why)) => {
                println!("FAILED: {why}");
                println!("{spoken} commands spoken, {wrong} heard differently");
                return 1;
            }
            None => break,
        }
    }
    println!("PASSED: {spoken} commands spoken, {wrong} heard differently");
    0
}

fn play_world(play: &Play) -> &EngineWorld {
    match play {
        Play::Live(session) => session.world(),
        Play::Script(playing) => playing.session().world(),
    }
}

fn fail(why: &str) -> ! {
    eprintln!("{why}");
    std::process::exit(2)
}

fn setup(
    mut commands: Commands,
    land: Res<Land>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // The sea, all round, and the land: scenery everywhere, as anyone would
    // see it from where they stand.
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200_000.0, 200_000.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.13, 0.33, 0.52),
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::default(),
    ));
    let ground = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.95,
        ..default()
    });
    for mesh in land.mesh() {
        commands.spawn((Mesh3d(meshes.add(mesh)), MeshMaterial3d(ground.clone())));
    }

    // The sun, turned by `day_and_night`.
    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::default(),
        Sun,
    ));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        ..default()
    });

    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: bevy::text::FontSize::Px(17.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(12.0),
            padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)),
        Hud,
    ));
}

fn controls(keys: Res<ButtonInput<KeyCode>>, mut sim: ResMut<Sim>, mut console: ResMut<Console>) {
    if console.typing {
        return;
    }
    if keys.just_pressed(KeyCode::Space) {
        sim.paused = !sim.paused;
        let now = if sim.paused { "paused" } else { "running" };
        console.say(Said::Debug, &format!("The world is {now}."));
    }
    let faster = keys.just_pressed(KeyCode::BracketRight);
    if faster || keys.just_pressed(KeyCode::BracketLeft) {
        sim.speed = if faster {
            (sim.speed * 4.0).min(16_384.0)
        } else {
            (sim.speed / 4.0).max(1.0)
        };
        let speed = sim.speed;
        console.say(Said::Debug, &format!("The world runs {speed}x."));
    }
}

/// Runs the engine's world on, a whole number of game seconds at a time,
/// and tells the player what they hear of as it happens.
/// With the `snap` setting, the clock goes back to real speed when what
/// the actor was doing is done.
fn run_world(
    time: Res<Time>,
    settings: Res<tools::Settings>,
    mut sim: ResMut<Sim>,
    mut console: ResMut<Console>,
    by_keys: Res<walking::ByKeys>,
    mut was_busy: Local<bool>,
) {
    if sim.paused {
        return;
    }
    let owed = sim.owed + time.delta_secs() * sim.speed;
    let Play::Live(session) = &mut sim.play else {
        return;
    };
    let whole = owed.floor();
    if whole >= 1.0 {
        session.advance(whole as u64);
    }
    let news = session.catch_up();
    // A step the keys took needs no word.
    let news: String = news
        .lines()
        .filter(|line| !(by_keys.0 && line.starts_with("You walk to ")))
        .collect::<Vec<_>>()
        .join("\n");
    if !news.is_empty() {
        console.say(Said::News, &news);
    }
    let busy = session.world().pending(session.player()).is_some() || session.queue_busy();
    sim.owed = owed - whole;
    if *was_busy && !busy && settings.snap && sim.speed > 1.0 {
        sim.speed = 1.0;
        console.say(
            Said::Debug,
            "Done: back to real speed (/snap off to stay fast).",
        );
    }
    *was_busy = busy;
}

/// The sun's angle and strength from the world's time of day.
fn day_and_night(
    sim: Res<Sim>,
    mut sun: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut sky: ResMut<ClearColor>,
) {
    let settings = sim.world().settings();
    let Some(of_day) = sim.world().time_of_day() else {
        return;
    };
    let (rise, set) = (settings.sunrise as f32, settings.sunset as f32);
    // 0 at sunrise, 1 at sunset; outside that, night.
    let arc = (of_day as f32 - rise) / (set - rise);
    let up = if (0.0..=1.0).contains(&arc) {
        (arc * std::f32::consts::PI).sin()
    } else {
        0.0
    };
    for (mut transform, mut light) in &mut sun {
        let angle = arc.clamp(0.0, 1.0) * std::f32::consts::PI;
        let direction = Vec3::new(-angle.cos(), -angle.sin().max(0.05), -0.3);
        *transform = Transform::default().looking_to(direction, Vec3::Y);
        light.illuminance = 15_000.0 * up.sqrt();
    }
    ambient.brightness = 80.0 + 900.0 * up.sqrt();
    // Night's dark blue to a clear day's pale blue, quickly after dawn.
    let day = up.sqrt();
    sky.0 = Color::srgb(0.03 + 0.52 * day, 0.05 + 0.68 * day, 0.12 + 0.8 * day);
}

fn hud(sim: Res<Sim>, eye: Res<camera::Eye>, mut text: Query<&mut Text, With<Hud>>) {
    let world = sim.world();
    let me = sim.me();
    let clock = world.time_of_day().map_or(String::new(), |t| {
        format!(
            "Day {}, {:02}:{:02}",
            world.tick() / 86_400 + 1,
            t / 3600,
            t / 60 % 60
        )
    });
    let state = match (&sim.play, sim.paused) {
        (Play::Script(_), _) => "playing a script".to_string(),
        (_, true) => "paused".to_string(),
        (_, false) => format!("x{}", sim.speed),
    };
    let here = world
        .place_of(me)
        .map_or(String::new(), |p| world.label_for(me, p));
    let doing = match world.pending(me) {
        Some(p) => format!(", busy until {}", sim.session().clock(p.until)),
        None => String::new(),
    };
    // Something kept going: how it's going.
    let doing = match &sim.play {
        Play::Live(session) => match session.queue_progress() {
            Some(progress) => format!("{doing}; {progress} (move or stop to stop)"),
            None => doing,
        },
        Play::Script(_) => doing,
    };
    let view = if eye.flying {
        "flying free: arrows, E/Q, Shift; F to go back; WASD still walks"
    } else {
        "WASD walk, right-drag or arrows to look, wheel to zoom, F fly, B backpack, V body, T speak, G grid"
    };
    for mut text in &mut text {
        text.0 =
            format!("{clock}   {state}\n{here}{doing}\n{view}; Space pause, [ ] slower/faster");
    }
}

/// With `--frames`, prints how many frames were drawn every two seconds,
/// for finding where time goes.
fn frames(time: Res<Time>, mut count: Local<(u32, f32)>) {
    if !std::env::args().any(|a| a == "--frames") {
        return;
    }
    count.0 += 1;
    let now = time.elapsed_secs();
    if now - count.1 >= 2.0 {
        println!("{now:.1} s: {} frames", count.0);
        count.0 = 0;
        count.1 = now;
    }
}

/// With `--frames`, prints how many shaders are still being compiled
/// whenever that changes: nothing is drawn with a shader until it's ready.
fn shaders_waiting(
    cache: Res<bevy::render::render_resource::PipelineCache>,
    mut last: Local<Option<usize>>,
) {
    let waiting = cache.waiting_pipelines().count();
    if *last != Some(waiting) {
        println!(
            "{:.1} s: {waiting} shaders compiling",
            STARTED
                .get_or_init(std::time::Instant::now)
                .elapsed()
                .as_secs_f32()
        );
        *last = Some(waiting);
    }
}

/// When the program started, for `--frames`.
static STARTED: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

/// With `--shot`, saves a frame after a few seconds, then exits.
fn shot(
    mut commands: Commands,
    time: Res<Time>,
    options: Res<Options>,
    mut done: Local<Option<f32>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = &options.shot else { return };
    let now = time.elapsed_secs();
    match *done {
        None if now >= options.after => {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path.clone()));
            *done = Some(now);
        }
        Some(at) if now > at + 2.0 => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
