# Worlds built in Blender

Worlds are built in Blender and converted for the game (decided 2026-10-02; see [docs/ideas/world-building.md](../docs/ideas/world-building.md)). Every object placed in a world is one of three kinds: **scenery** (seen, never used), a **source** (many alike, taken from, one thing to the engine), or a **thing** (counts on its own, fully interactive).

## What's here

| File | What it is | In git |
| --- | --- | --- |
| `catalogue.toml` | The catalogue: what each model means to the game (scenery, source, or thing; material, mass, pieces, and so on) and which models show it | Yes |
| `make_catalogue.py` | Builds `catalogue.blend` from the catalogue and the downloaded packs | Yes |
| `catalogue.blend` | The models, one collection per model of each entry (`standing-trees/3`), marked as assets | **No**: it holds the packs' models. Make it with the script |
| `skill-yard.blend` | The first world: the skill grounds as a walled yard 100 m across. **The master copy**, edited by hand | Yes: it links to the catalogue, never copies from it, so it holds no pack models |
| `make_skill_yard.py` | Made the first version of `skill-yard.blend`; not run again unless we start over | Yes |
| `blender.sh` | Runs a script in Windows' Blender from WSL, without its window | Yes |

## Which command makes what

| Command | Reads | Makes | In git |
| --- | --- | --- | --- |
| `blender/blender.sh make_catalogue.py` | `catalogue.toml`, the packs | `blender/catalogue.blend` | No |
| `blender/blender.sh make_skill_yard.py` | `catalogue.blend` | `blender/skill-yard.blend` (the first version only; it's edited by hand since) | Yes |
| `blender/convert.sh skill-yard` | `skill-yard.world.toml` (the manifest, written by hand), `skill-yard.blend`, `catalogue.toml` | `assets/worlds/skill-yard/`: the export (`skill-yard.gltf`, `skill-yard.bin`, and the textures it uses), which the client draws; and `data/skill-yard.toml`, the world's data for the engine | The export no (it holds the packs' models); the data yes |

`blender.sh` only runs a script in Blender; which files it makes depends on the script. Blender also leaves a `.blend1` beside a file it saves: the version before, as a backup (git ignores it).

## Getting started (or after a fresh clone)

1. Download the packs listed in [assets/third-party](../assets/third-party/README.md) and put their files where each README says. Without them, the catalogue draws simple shapes instead.
2. Make the catalogue: `blender/blender.sh make_catalogue.py` from WSL, or `blender --background --factory-startup --python blender/make_catalogue.py` anywhere else. Run it again whenever `catalogue.toml` changes or a pack is added.
3. Open `skill-yard.blend` in Blender. From Windows, the repository in WSL is at `\\wsl.localhost\Ubuntu-24.04\home\edster\projects\universe-game`.

## What you'll see in skill-yard.blend

The outliner (top right) has four collections:
- **places**: two circles: the yard (50 m out from the middle) and the deep woods in the north-east corner, where the boars live.
- **things**: the player in the middle, six landmark rocks, one behind each zone, and two boars.
- **sources**: one circle for each source, named `source: woodland-trees` and so on, with the models that show it as its children. The woodland's stand of trees, its piles of grass, stones, sticks, and flint, and the stream's water and fish are all here.
- **scenery**: the ground, the four walls, the stepping stones, and the decoration (flowers, ferns, clover, mushrooms, a dead tree, a twisted tree).

Two cameras: **overview**, from the south-west corner, and **from the clearing**, looking north to the woodland. Select one and press Numpad 0 to look through it.

Blender's axes are the world's: **x is east, y is north, z is up, in metres**, and the yard's middle is 0, 0.

## The tags

Each object's tags are custom properties: select it, then **Object Properties** (the orange square in the properties panel), **Custom Properties**.

| Property | On | Meaning |
| --- | --- | --- |
| `ug_entry` | sources and things | The catalogue entry it is (`standing-trees`, `landmark-rock`) |
| `ug_id` | sources and things | Its own name in the world, unique (`woodland-trees`, `hearth`) |
| `ug_label` | any, optional | Its name in the world's words, instead of the entry's (`the hearth`) |
| `ug_mass` | sources, optional | How much there is, for a source whose models don't each stand for an amount (`20 t` of water) |
| `ug_place` | places | The place's id; with `ug_label`, `ug_size`, `ug_temperature`, `ug_night` |

**Anything without `ug_entry` is scenery**, and so is an instance of a scenery entry. Things are linked to the place whose circle they're in.

## Working on the world

- **Move a thing or a source:** select it and press G. Moving a source's circle moves all its models with it.
- **More or less of a source:** duplicate one of its models (Shift+D) or delete one. A source whose models each stand for an amount (trees at 1 t each, stones at 50 kg) holds that much more or less.
- **Another thing:** duplicate one (Shift+D) and **give it a new `ug_id`**: two things with one id is a mistake the converter will refuse.
- **Something new from the catalogue:** add the `blender` folder as an asset library once (Edit, Preferences, File Paths, Asset Libraries, +), then drag entries in from the Asset Browser. A dragged-in model comes without tags: add `ug_entry` and `ug_id` for a thing, or parent it to a source's circle (Ctrl+P) to make it part of that source.
- **Save** (Ctrl+S). Blender keeps a `.blend1` backup beside it, which git ignores.

## Converting a world for the game

    blender/convert.sh skill-yard

This exports each Blender file the manifest (`skill-yard.world.toml`) lists, as glTF with the tags, into `assets/worlds/` (not in git: it holds the packs' models), then runs the converter (`cargo run -p console -- --convert blender/skill-yard.world.toml`), which checks every tag against the catalogue and writes **`data/skill-yard.toml`**, the world's data file. That file is in git, so the proofs run without Blender; it says at its top that it's made, not written. Run the converter again after every change in Blender. It refuses, naming each problem, a tag the catalogue doesn't know, a thing or source without a `ug_id`, two with the same one, a source with no models, and anything outside every place's circle.

How the converter reads the world:
- **Places** in one file all lead to each other, as far apart as their middles. A thing is in the smallest place whose circle it's in.
- **A source's mass** is its models times the entry's `each` (12 trees at 1 t each make 12 t), unless `ug_mass` says otherwise or the entry has no `each`. **Its spread** is how far its models stand from its circle's middle.
- **A creature** keeps to the places of its file. **Something that grows back** draws on the nearest source of the entry its catalogue entry names, in the same place.
- **Scenery** isn't written at all: the engine never hears of it.

Proofs on the converted world: `data/scripts/skill-yard-0-the-board.txt` and `skill-yard-1-fire-variations.txt`, the skill grounds' own, played on the yard. Moving things around in Blender can change how far walks take, which those proofs check in places.

## In the game

The client opens the skill yard by default and draws it from the export (`client/src/built.rs`): scenery just as it was placed; each source by its own models, where they stand in Blender; each thing by its own model, following the engine, so build mode (M) moves it (and `/save` keeps where). People and creatures, and anything made in play, the client draws itself. Build mode leaves sources alone: they move in Blender. After changing the world in Blender, convert it again and restart the client (`client/run.sh` copies the export beside it).

## Next

A source's models known to the engine, so felling takes the tree nearest you and gathering thins a pile; ground heights from Blender.
