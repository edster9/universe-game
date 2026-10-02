//! Saving and resuming (`/save <name>`, `/load <name>`, `/saves`): the whole
//! world in a file, with its clock, every piece of matter, and every mind,
//! and who's being played. Single player only (docs/ideas/tools.md): a
//! server keeps its own saves. Saves live in a folder beside the data folder.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use engine::intent::Intent;
use engine::world::{EntityId, World};

use super::{Reply, Session};

/// The name of the save made when a game ends, and resumed by `/load` alone.
pub const LAST: &str = "last";

#[derive(serde::Serialize)]
struct Writing<'a> {
    player: &'a str,
    started: &'a BTreeMap<EntityId, Intent>,
    world: &'a World,
}

#[derive(serde::Deserialize)]
struct Reading {
    player: String,
    started: BTreeMap<EntityId, Intent>,
    world: World,
}

/// A save's file, if its name is one: letters, digits, '-' and '_'.
fn file(folder: &Path, name: &str) -> Result<PathBuf, String> {
    let fine = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if fine {
        Ok(folder.join(format!("{name}.json")))
    } else {
        Err(format!(
            "\"{name}\" can't be a save's name: use letters, digits, '-' and '_'"
        ))
    }
}

fn read(path: &Path) -> Result<Reading, String> {
    let text = std::fs::read_to_string(path).map_err(|_| {
        let name = path.file_stem().unwrap_or_default().to_string_lossy();
        format!("there's no save called \"{name}\"; /saves lists them")
    })?;
    serde_json::from_str(&text)
        .map_err(|e| format!("this save can't be read by this version of the game ({e})"))
}

impl Session {
    /// Keeps saves in `folder`, for `/save` and `/load`.
    pub fn with_saves(mut self, folder: PathBuf) -> Self {
        self.saves = Some(folder);
        self
    }

    /// Starts from a save in `folder`, keeping saves there.
    pub fn resume(folder: PathBuf, name: &str) -> Result<Session, String> {
        let saved = read(&file(&folder, name)?)?;
        let mut session = Session::new(saved.world, &saved.player)?;
        session.started = saved.started;
        session.settle();
        Ok(session.with_saves(folder))
    }

    /// Writes the whole world to the save called `name`.
    pub fn save(&mut self, name: &str) -> Result<String, String> {
        let folder = self.saves.clone().ok_or("saving isn't on here")?;
        let path = file(&folder, name)?;
        let text = serde_json::to_string(&Writing {
            player: self.world.key(self.player),
            started: &self.started,
            world: &self.world,
        })
        .map_err(|e| format!("the world couldn't be saved ({e})"))?;
        std::fs::create_dir_all(&folder)
            .and_then(|()| std::fs::write(&path, text))
            .map_err(|e| format!("can't write {}: {e}", path.display()))?;
        Ok(format!(
            "Saved as \"{name}\", at {}.",
            self.clock(self.world.tick())
        ))
    }

    /// Picks up the save called `name` exactly where it was.
    pub fn load(&mut self, name: &str) -> Result<String, String> {
        let folder = self.saves.clone().ok_or("loading isn't on here")?;
        let saved = read(&file(&folder, name)?)?;
        let player = saved
            .world
            .find_by_key(&saved.player)
            .filter(|&id| saved.world.is_agent(id))
            .ok_or("this save has no one to play")?;
        self.world = saved.world;
        self.player = player;
        self.started = saved.started;
        self.queue = None;
        self.spent = 0;
        self.settle();
        Ok(format!(
            "Loaded \"{name}\", at {}.\n{}",
            self.clock(self.world.tick()),
            self.look()
        ))
    }

    /// What a fresh start knows: a death already told, and whether asleep.
    fn settle(&mut self) {
        self.announced_death = self.world.life(self.player).and_then(|l| l.died_of.clone());
        self.was_asleep = self.world.is_asleep(self.player);
    }

    /// The saves there are, newest first.
    pub fn saves(&self) -> Vec<String> {
        let Some(folder) = &self.saves else {
            return Vec::new();
        };
        let mut found: Vec<(std::time::SystemTime, String)> = std::fs::read_dir(folder)
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                let name = path.file_stem()?.to_str()?.to_string();
                let when = entry.metadata().ok()?.modified().ok()?;
                (path.extension()? == "json").then_some((when, name))
            })
            .collect();
        found.sort_by(|a, b| b.cmp(a));
        found.into_iter().map(|(_, name)| name).collect()
    }

    /// `/save <name>`, `/load [name]`, and `/saves`.
    pub(super) fn saving(&mut self, tool: &str, name: &str) -> Reply {
        let done = match (tool, name) {
            ("/save", "") => Err("Give the save a name, like /save fire-lit".to_string()),
            ("/save", name) => self.save(name),
            ("/load", "") => self.load(LAST),
            ("/load", name) => self.load(name),
            _ => {
                let saves = self.saves();
                return Reply::say(if saves.is_empty() {
                    "There are no saves yet. /save <name> makes one.".to_string()
                } else {
                    format!("Saves, newest first: {}.", saves.join(", "))
                });
            }
        };
        match done {
            Ok(text) => Reply::say(text),
            Err(why) => Reply::refuse(super::sentence(&why)),
        }
    }
}
