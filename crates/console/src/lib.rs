//! The text console for the early slices. It turns typed lines into engine
//! commands and engine results into text. All the rules live in the engine.

pub mod convert;
pub mod help;
pub mod live;
pub mod menu;
pub mod script;
pub mod session;

use std::path::Path;

use engine::world::World;

/// Reads a world's data file, and the libraries it uses from the same
/// folder, and builds the world.
/// The data ID of the first person in a world that nobody else plays: no
/// mind of their own, no instinct.
pub fn person_to_play(world: &World) -> Option<String> {
    world
        .entities()
        .find(|&id| world.is_agent(id) && world.mind(id).is_none() && world.instinct(id).is_none())
        .map(|id| world.key(id).to_string())
}

pub fn load_world_file(path: &Path) -> Result<World, String> {
    let name = path.display();
    let text = std::fs::read_to_string(path).map_err(|e| format!("can't read {name}: {e}"))?;
    let folder = path.parent().unwrap_or(Path::new("."));
    let mut libraries = Vec::new();
    for library in engine::data::libraries(&text).map_err(|e| format!("{name}: {e}"))? {
        let text = std::fs::read_to_string(folder.join(&library))
            .map_err(|e| format!("can't read {library}, which {name} uses: {e}"))?;
        libraries.push(text);
    }
    let libraries: Vec<&str> = libraries.iter().map(String::as_str).collect();
    engine::data::load_world_with(&text, &libraries).map_err(|e| format!("{name}: {e}"))
}
