# Stylized Nature MegaKit, by Quaternius

**Not in git.** Download the pack and put its files here; the game draws these things with the shapes it makes itself until you do.

- **Get it:** [quaternius.com](https://quaternius.com/packs/stylizednaturemegakit.html) (also on itch.io). We use the free **Standard** version; the Pro version ($9.99) has more models and the same folders.
- **Licence:** CC0 1.0 (public domain), as the pack's `License_Standard.txt` says. We keep it out of git anyway, like every pack (see [../../README.md](../../README.md)).
- **Put here:** the zip's `glTF` folder, whole, so the models sit at `glTF/<name>.gltf` with their `.bin` files and textures beside them (about 48 MB). The models and textures can't be split up: each `.gltf` names its own `.bin` and the textures it uses, found in the same folder.

## What the game uses

44 models, named in `client/style.toml` (a client test checks that every model the style names is listed here):

- **Grass and plants:** `Bush_Common_Flowers`, `Clover_1`, `Clover_2`, `Fern_1`, `Grass_Common_Short`, `Grass_Common_Tall`, `Grass_Wispy_Short`, `Grass_Wispy_Tall`, `Plant_1_Big`, `Plant_7_Big`
- **Trees:** `CommonTree_1`, `CommonTree_2`, `CommonTree_3`, `CommonTree_4`, `CommonTree_5`, `DeadTree_2`, `Pine_1`, `Pine_2`, `Pine_3`, `Pine_4`, `Pine_5`, `TwistedTree_1`
- **Flowers and mushrooms:** `Flower_3_Group`, `Flower_3_Single`, `Flower_4_Group`, `Mushroom_Common`, `Mushroom_Laetiporus`, `Petal_2`
- **Rocks and stones:** `Pebble_Round_1`, `Pebble_Round_2`, `Pebble_Round_3`, `Pebble_Round_4`, `Pebble_Round_5`, `Pebble_Square_1`, `Pebble_Square_2`, `Pebble_Square_3`, `Pebble_Square_4`, `Pebble_Square_5`, `Pebble_Square_6`, `RockPath_Round_Wide`, `RockPath_Square_Wide`, `Rock_Medium_1`, `Rock_Medium_2`, `Rock_Medium_3`

Through them, these textures from the same folder: `Bark_DeadTree`, `Bark_NormalTree`, `Bark_TwistedTree` (each with its `_Normal` map), `Flowers`, `Grass`, `Leaf_Pine_C`, `Leaves`, `Leaves_NormalTree_C`, `Leaves_TwistedTree_C`, `Mushrooms`, `PathRocks_Diffuse`, and `Rocks_Diffuse` (`.png`), and each model's own `.bin`.

How they're used, and what we found trying them: "Quaternius' Stylized Nature MegaKit" in [docs/ideas/assets.md](../../../../docs/ideas/assets.md).
