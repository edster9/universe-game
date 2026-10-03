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

## Next

The converter: reading the world (exported as glTF, with the tags) and writing the world's data for the engine and the scene for the client, checking every tag against the catalogue. Then the first test: the skill grounds' proofs, passing on the yard.
