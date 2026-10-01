//! Voice: hold T and speak; let go, and the words are heard on this machine
//! (Whisper, in the background) and run in the console as if typed. It only
//! ever makes text: the same commands, checked by the same laws.
//!
//! Whisper is primed with the console's command words and the names of what
//! the islander can see, in their own words, so it hears "driftwood", not
//! "dripped wood". The model is `models/ggml-base.en.bin` beside the
//! program; without it, voice is off and the console says so.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use bevy::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use engine::world::{EntityId, World as EngineWorld};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::terminal::{Console, Said};
use crate::{Play, Sim};

/// Whisper hears 16,000 samples a second, one channel.
const RATE: u32 = 16_000;

/// The words the console understands, to prime Whisper with.
const COMMANDS: &str = "look, inventory, backpack, body, go, take, drop, put, give, pay, \
    gather, dig, light, pour, work, rub wood against wood, assemble, make, disassemble, \
    wear, join, call, tell, ask, offer, wait, start, sleep";

/// What was heard, and how long hearing took.
pub struct Heard {
    pub text: String,
    pub seconds: f32,
}

#[derive(Resource)]
pub struct Voice {
    model: Option<Arc<WhisperContext>>,
    /// Why there's no voice, if there isn't.
    trouble: Option<String>,
    /// While T is held, the microphone's samples go here.
    listening: Arc<AtomicBool>,
    samples: Arc<Mutex<Vec<f32>>>,
    /// The microphone's rate and channels.
    format: Arc<Mutex<(u32, u16)>>,
    heard: Mutex<Receiver<Heard>>,
    send: Sender<Heard>,
    /// Hearing in the background.
    pub busy: bool,
    /// The microphone's name, once open.
    microphone: String,
}

/// Loads the model, if it's there.
pub fn load_model(path: &std::path::Path) -> Result<WhisperContext, String> {
    if !path.is_file() {
        return Err(format!("no voice model at {}", path.display()));
    }
    WhisperContext::new_with_params(path, WhisperContextParameters::default())
        .map_err(|e| format!("the voice model wouldn't load: {e}"))
}

/// Hears speech: 16 kHz mono samples to text, primed with `prompt`.
pub fn hear(model: &WhisperContext, audio: &[f32], prompt: &str) -> Result<String, String> {
    let mut state = model.create_state().map_err(|e| e.to_string())?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));
    params.set_n_threads(8);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_print_special(false);
    params.set_suppress_blank(true);
    params.set_initial_prompt(prompt);
    // Whisper needs at least a second; pad short speech with silence.
    let mut audio = audio.to_vec();
    audio.resize(audio.len().max(RATE as usize + RATE as usize / 10), 0.0);
    state.full(params, &audio).map_err(|e| e.to_string())?;
    let mut text = String::new();
    for segment in state.as_iter() {
        text.push_str(&segment.to_string());
    }
    Ok(command(&text))
}

/// Spoken words as a command: lower case, without the full stop and commas
/// speech-to-text adds.
pub fn command(text: &str) -> String {
    let text = text.trim().to_lowercase();
    let text: String = text
        .chars()
        .filter(|c| !matches!(c, ',' | '!' | '?'))
        .collect();
    clock_times(&split_run_together(text.trim_end_matches('.').trim()))
}

/// Whisper sometimes runs a command into the next word ("dropwood", "rubwood
/// against wood"): a first word that isn't a command but starts with one is
/// split.
fn split_run_together(text: &str) -> String {
    let verbs: Vec<&str> = COMMANDS
        .split(", ")
        .filter_map(|c| c.split_whitespace().next())
        .collect();
    let (first, rest) = text.split_once(' ').unwrap_or((text, ""));
    if verbs.contains(&first) {
        return text.to_string();
    }
    let split = verbs
        .iter()
        .filter(|v| first.len() > v.len() + 1 && first.starts_with(**v))
        .max_by_key(|v| v.len());
    match split {
        Some(verb) => format!("{verb} {} {rest}", &first[verb.len()..])
            .trim()
            .to_string(),
        None => text.to_string(),
    }
}

/// Times of day as the console writes them: "1805", "18.05", and "6:05
/// pm" are all "18:05".
fn clock_times(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < words.len() {
        let word = words[i];
        let digits: String = word.chars().filter(char::is_ascii_digit).collect();
        let (hours, minutes) = match word.split_once([':', '.']) {
            Some((h, m)) if !h.is_empty() && m.len() == 2 => (h.parse().ok(), m.parse().ok()),
            // Bare numbers are times only after "until": "wait 600" is seconds.
            _ if i > 0
                && words[i - 1] == "until"
                && digits.len() == word.len()
                && (3..=4).contains(&word.len()) =>
            {
                (
                    word[..word.len() - 2].parse::<u32>().ok(),
                    word[word.len() - 2..].parse::<u32>().ok(),
                )
            }
            _ => (None, None),
        };
        match (hours, minutes) {
            (Some(mut h), Some(m)) if h < 24 && m < 60 => {
                let after = words.get(i + 1).map(|w| w.replace('.', ""));
                match after.as_deref() {
                    Some("pm") if h < 12 => {
                        h += 12;
                        i += 1;
                    }
                    // "13:00 hours" is 13:00.
                    Some("am") | Some("pm") | Some("hours") | Some("hrs") => i += 1,
                    _ => {}
                }
                out.push(format!("{h:02}:{m:02}"));
            }
            _ => out.push(word.to_string()),
        }
        i += 1;
    }
    out.join(" ")
}

#[cfg(test)]
mod tests {
    use super::command;

    #[test]
    fn spoken_words_become_commands() {
        assert_eq!(command(" Go forest."), "go forest");
        assert_eq!(
            command("Rub wood against wood, into ring!"),
            "rub wood against wood into ring"
        );
        assert_eq!(command("Wait until 1805."), "wait until 18:05");
        assert_eq!(command("Wait until 18.05"), "wait until 18:05");
        assert_eq!(command("wait until 6:05 p.m."), "wait until 18:05");
        assert_eq!(command("wait until 8:30 am"), "wait until 08:30");
        assert_eq!(command("wait 10 min"), "wait 10 min");
        assert_eq!(command("wait 600"), "wait 600");
        assert_eq!(command("Wait until 13:00 hours."), "wait until 13:00");
        assert_eq!(command("Dropwood."), "drop wood");
        assert_eq!(
            command("Rubwood against wood into ring."),
            "rub wood against wood into ring"
        );
        assert_eq!(command("gather driftwood"), "gather driftwood");
        assert_eq!(command("takeoff shoes"), "take off shoes");
    }
}

/// The words to prime Whisper with: the commands, and the names of what
/// `me` can see and where they can go, in their own words. Nothing they
/// couldn't say.
pub fn prompt(world: &EngineWorld, me: EntityId) -> String {
    let mut words: Vec<String> = Vec::new();
    if let Some(look) = engine::view::look(world, me) {
        words.push(look.place);
        words.extend(look.exits);
        words.extend(look.people);
        words.extend(look.things.iter().map(|t| t.label.clone()));
    }
    words.extend(
        engine::view::inventory(world, me)
            .things
            .iter()
            .map(|t| t.label.clone()),
    );
    words.sort();
    words.dedup();
    format!("Commands: {COMMANDS}. Names: {}.", words.join(", "))
}

/// Linear resampling of interleaved samples to 16 kHz mono.
fn to_whisper(samples: &[f32], rate: u32, channels: u16) -> Vec<f32> {
    let channels = channels.max(1) as usize;
    let mono: Vec<f32> = samples
        .chunks(channels)
        .map(|frame| frame.iter().sum::<f32>() / frame.len() as f32)
        .collect();
    if rate == RATE || mono.is_empty() {
        return mono;
    }
    let step = rate as f64 / RATE as f64;
    let n = (mono.len() as f64 / step) as usize;
    (0..n)
        .map(|i| {
            let at = i as f64 * step;
            let j = at as usize;
            let t = (at - j as f64) as f32;
            let next = mono.get(j + 1).copied().unwrap_or(mono[j]);
            mono[j] * (1.0 - t) + next * t
        })
        .collect()
}

/// Opens the microphone on a thread of its own (the stream must stay where
/// it was made), which keeps it open for good.
fn open_microphone(
    listening: Arc<AtomicBool>,
    samples: Arc<Mutex<Vec<f32>>>,
    format: Arc<Mutex<(u32, u16)>>,
) -> Result<String, String> {
    let (ready, result) = channel();
    std::thread::spawn(move || {
        let opened = (|| -> Result<(cpal::Stream, String), String> {
            let device = cpal::default_host()
                .default_input_device()
                .ok_or("there's no microphone")?;
            let name = device
                .description()
                .map(|d| d.name().to_string())
                .unwrap_or_else(|_| "the microphone".into());
            let supported = device.default_input_config().map_err(|e| e.to_string())?;
            *format.lock().unwrap() = (supported.sample_rate(), supported.channels());
            let config = supported.config();
            let error = |e| eprintln!("microphone: {e}");
            let stream = match supported.sample_format() {
                cpal::SampleFormat::F32 => device.build_input_stream(
                    config,
                    move |data: &[f32], _: &_| {
                        if listening.load(Ordering::Relaxed) {
                            samples.lock().unwrap().extend_from_slice(data);
                        }
                    },
                    error,
                    None,
                ),
                cpal::SampleFormat::I16 => device.build_input_stream(
                    config,
                    move |data: &[i16], _: &_| {
                        if listening.load(Ordering::Relaxed) {
                            samples
                                .lock()
                                .unwrap()
                                .extend(data.iter().map(|&s| s as f32 / 32_768.0));
                        }
                    },
                    error,
                    None,
                ),
                other => return Err(format!("the microphone gives {other:?} samples")),
            }
            .map_err(|e| e.to_string())?;
            stream.play().map_err(|e| e.to_string())?;
            Ok((stream, name))
        })();
        match opened {
            Ok((stream, name)) => {
                let _ = ready.send(Ok(name));
                // Keep it open while the game runs.
                let _keep = stream;
                loop {
                    std::thread::park();
                }
            }
            Err(why) => {
                let _ = ready.send(Err(why));
            }
        }
    });
    result
        .recv()
        .unwrap_or_else(|_| Err("the microphone thread stopped".into()))
}

impl Voice {
    pub fn new(model_path: &std::path::Path) -> Voice {
        let (send, heard) = channel();
        let listening = Arc::new(AtomicBool::new(false));
        let samples = Arc::new(Mutex::new(Vec::new()));
        let format = Arc::new(Mutex::new((RATE, 1)));
        let model = load_model(model_path).map(Arc::new);
        let (trouble, microphone) = match &model {
            Err(why) => (Some(why.clone()), String::new()),
            Ok(_) => match open_microphone(listening.clone(), samples.clone(), format.clone()) {
                Ok(name) => (None, name),
                Err(why) => (Some(why), String::new()),
            },
        };
        Voice {
            model: model.ok(),
            trouble,
            listening,
            samples,
            format,
            heard: Mutex::new(heard),
            send,
            busy: false,
            microphone,
        }
    }

    /// Whether voice is ready, for the console.
    pub fn status(&self) -> String {
        match &self.trouble {
            Some(why) => format!("No voice: {why}."),
            None => format!(
                "Voice ready: hold T and speak a command ({}).",
                self.microphone
            ),
        }
    }

    /// Whether T is held down, recording.
    pub fn listening(&self) -> bool {
        self.listening.load(Ordering::Relaxed)
    }
}

/// Holding T listens; letting go hears it, in the background.
pub fn push_to_talk(
    keys: Res<ButtonInput<KeyCode>>,
    sim: Res<Sim>,
    mut voice: ResMut<Voice>,
    mut console: ResMut<Console>,
) {
    if console.typing || matches!(sim.play, Play::Script(_)) {
        return;
    }
    if keys.just_pressed(KeyCode::KeyT) {
        if let Some(why) = &voice.trouble {
            console.say(Said::Debug, &format!("No voice: {why}."));
            return;
        }
        voice.samples.lock().unwrap().clear();
        voice.listening.store(true, Ordering::Relaxed);
    }
    if keys.just_released(KeyCode::KeyT) && voice.listening() {
        voice.listening.store(false, Ordering::Relaxed);
        let samples = std::mem::take(&mut *voice.samples.lock().unwrap());
        let (rate, channels) = *voice.format.lock().unwrap();
        let audio = to_whisper(&samples, rate, channels);
        let Some(model) = voice.model.clone() else {
            return;
        };
        if audio.len() < RATE as usize / 4 {
            console.say(Said::Debug, "Too short to hear: hold T while you speak.");
            return;
        }
        let prompt = prompt(sim.world(), sim.me());
        let send = voice.send.clone();
        voice.busy = true;
        std::thread::spawn(move || {
            let started = Instant::now();
            let text = hear(&model, &audio, &prompt).unwrap_or_default();
            let _ = send.send(Heard {
                text,
                seconds: started.elapsed().as_secs_f32(),
            });
        });
    }
}

/// What was heard runs in the console, as if typed.
pub fn run_heard(
    mut voice: ResMut<Voice>,
    mut console: ResMut<Console>,
    mut sim: ResMut<Sim>,
    mut exit: MessageWriter<AppExit>,
) {
    let heard: Vec<Heard> = voice.heard.lock().unwrap().try_iter().collect();
    for Heard { text, seconds } in heard {
        voice.busy = false;
        if text.is_empty() {
            console.say(Said::Debug, "Heard nothing.");
            continue;
        }
        console.say(Said::Debug, &format!("Heard in {seconds:.1} s:"));
        crate::terminal::send(&mut console, &mut sim, &text, &mut exit);
    }
}

/// A recording's file name for a command: its words joined by dashes.
pub fn file_name(command: &str) -> String {
    command
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// A recording: 16-bit PCM, 16 kHz, mono, as Whisper hears it.
pub fn read_wav(path: &std::path::Path) -> Result<Vec<f32>, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("can't read {}: {e}", path.display()))?;
    let data = bytes
        .windows(4)
        .position(|w| w == b"data")
        .ok_or("not a wav file")?
        + 8;
    Ok(bytes[data..]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|&s| i16::from_le_bytes(s) as f32 / 32_768.0)
        .collect())
}

/// Says at the start whether voice is ready.
pub fn announce(voice: Res<Voice>, mut console: ResMut<Console>) {
    let status = voice.status();
    console.say(Said::Debug, &status);
}
