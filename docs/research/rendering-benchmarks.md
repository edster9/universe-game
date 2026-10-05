# Rendering benchmarks

The owner's request (2026-10-04, in [requirements.md](../requirements.md), "Rendering benchmarks, from small to massive"): benchmark from simple to massive. Stop where it breaks, optimize, and go on. Find the sweet spot of render settings, as games do. Make the settings console commands with presets. This is the first round of many. Each round adds its results here, so the next round starts from the last table.

## How it's measured

- **The forests.** `blender/make_forest.py <name> <trees> <half-width>` scatters the catalogue's trees and undergrowth over a square. It leaves a clearing in the middle for the player and puts nothing else there. It's the same every time for the same arguments. Everything in it is scenery.
  - Per tree, on average, it adds:
    - 1.5 grass tufts;
    - 0.3 bushes;
    - 0.3 ferns;
    - a few flowers, mushrooms, pebbles and rocks.
  - About 5% of the trees are dead or twisted.

  | Forest | Trees | Area | Objects in Blender |
  |---|---|---|---|
  | small | 200 | 100 m | 700 |
  | medium | 2,000 | 300 m | 7,000 |
  | large | 20,000 | 1 km | 70,000 |
  | huge | 100,000 | 2.2 km | not yet built |

  The forests are made, not built by hand, so they live only on the machine that made them: `client/bench.sh` makes any that are missing.
- **The route.** `client.exe --bench <metres>` waits for the world to load, then moves the camera along a fixed route. It holds at each stop to settle, then measures for 5 seconds. The four stops:
  1. inside the forest at eye height;
  2. walking through it;
  3. at its edge, looking in;
  4. high above, looking over it.
- **What's recorded.** For each stop, one line in `bench.csv` beside the client:
  - the average frame time;
  - the worst 1% of frames;
  - the worst frame;
  - the graphics card's state, from NVIDIA's own tool in the background: how busy, its power, its clock, its slowdown flags and its memory;
  - the meshes and triangles drawn;
  - the load time.

  Per-stage card timings go to `bench-stages.csv`. Bevy times the main pass, bloom and the other post-processing passes, but not the shadow passes.
- **One command.** `client/bench.sh [small medium large]` runs every preset in each forest, maximized.
  - `QUALITIES="high"` limits the presets.
  - `EXTRA="/shadows off"` adds a setting on top of each preset.
  - `PROFILE=1 client/run.sh` builds `client-profile.exe`, which writes a trace of every frame (Chrome's format). A trace can reach gigabytes, so keep runs short.
- **The machine.** A laptop with an RTX 4050 (6 GB, about 80 W at most).
  - Windows scales the screen at 175%.
  - Maximized, the game draws 3200×1876 pixels.
  - The screen is 60 Hz and is driven by the laptop's other graphics chip (Intel Arc). That matters, as shown below.
  - The laptop was plugged in.

**How to read the card's figures.** The card slows its own clock when it has room: from 2,670 MHz down to as little as 800 MHz. When it does, it reports itself as busier. So read "busy" together with the clock. Busy at a high clock means no room left; busy at a low clock means plenty.

The card's slowdown flags (thermal, power brake) came up in most samples once the laptop had warmed up, even at low load and with frames perfectly steady. They don't predict a freeze; the worst frames are the measure that matters.

## Round 1 (2026-10-04)

### The freezes: a laptop card run flat out

The small forest (200 trees) already froze, for 50 to 250 ms at a time, at every preset. The card's own timings showed these weren't our work being slow. Even FXAA, a 2 ms stage, sometimes took 130 ms. Sampling the card five times a second showed why. Running flat out at about 80 W, it hit the laptop's power limit, and its clock was cut from 2,650 to 210 MHz for a moment, flagged as "software power cap" and "thermal slowdown".

It ran flat out because **screen sync didn't hold the game back**. On this laptop the screen belongs to the other graphics chip, so the NVIDIA card's frames are handed across, and waiting for the screen doesn't block. The game drew 70 to 214 frames a second for a 60 Hz screen, so most frames were never shown.

**Fix:** with vsync on (the default), the game also caps itself at the screen's refresh rate (`graphics::cap`).

| Small forest, high, maximized | Frame rate | Worst 1% | Card |
|---|---|---|---|
| Before (no cap) | 51–83 fps | 50–250 ms | 99%, about 78 W, repeated slowdowns |
| Capped at 60 | 59 fps | 20–29 ms | about 70%, about 60 W, no slowdowns |

This is probably common to gaming laptops with two graphics chips. An `fps-cap` setting is there for anyone who wants less, for battery or heat.

### Textures: smaller copies (mipmaps)

The pack's textures are single large pictures, mostly 2048 pixels across. Bevy makes no smaller copies of them for things far away. Without those, a distant leaf reads the full picture, which is slow and makes distant things shimmer.

`client/src/textures.rs` makes the copies as each picture loads, from the pack's own files, with nothing extra to keep. It keeps leaf cut-outs as solid in the small copies as in the full picture, so distant trees don't go bare. The `textures` setting leaves out the largest copies:
- medium halves every picture over 1024 pixels;
- low quarters every picture over 512.

This wasn't the cause of the freezes, but it's needed anyway, and the setting saves card memory. NVIDIA's memory figure moves in large blocks, so it doesn't show the saving.

### Simpler trees far away (levels of detail)

The medium forest (2,000 trees) drew 2 to 11 million triangles a frame. About 80% of those were trees, at 3,500 to 6,300 triangles each. Even low quality couldn't hold 60 fps. The pack has no simpler models, so we make them.

- **In the catalogue.** An entry's `lods = [0.25, 0.06]` makes copies with about those shares of the triangles, tagged `ug_lod` 1 and 2 (the full model is 0). The trees, dead trees and twisted trees have them. `blender/preview_lods.py` renders them side by side for judging.
- **Reducing each material separately.** Reduced as one mesh, the trees lost every leaf at the lowest level. Bark is many small triangles and leaves are fewer, larger cards, and Blender's reducer collapsed the cards first. So:
  - solid parts (bark) are reduced with Blender's Decimate;
  - foliage, a part made of many small separate cards, is thinned whole cards at a time. It keeps the square root of the share, and the remaining cards grow a little to fill the gaps.

  No material is named; the rule only looks at the mesh's shape.
- **Shading kept.** The pack's own normals (soft shading on the leaves) and vertex colours (shading baked in) don't survive the rebuild. They're copied back from the full model, normals from the nearest surface.
- **Shared materials.** Every model imported used to bring its own copy of the pack's materials (`Bark_NormalTree.001` and so on), which the game draws separately. Now each material is shared: an export has 14 materials where it had 58.
- **In the game.** `client/src/detail.rs` shows one level of each tree by distance, cross-fading between them with Bevy's `VisibilityRange`. The `detail` setting says how soon:

  | Detail | Full model to | Middle level to | Small things fade out at |
  |---|---|---|---|
  | low | 15 m | 45 m | 80× their size |
  | medium | 25 m | 80 m | 150× their size |
  | high | 40 m | 140 m | 300× their size |

  "Small" means under 3 m, so grass fades out long before a bush does. Sight works the same way in the engine.

Medium forest, maximized, capped at 60, before → after:

| Preset | Before | After |
|---|---|---|
| low | 47–59 fps, worst 1% up to 115 ms | 59 fps, worst 1% 18–23 ms |
| medium | 45–56 fps, freezes to 250 ms | 59 fps, worst 1% 21–31 ms |
| high | 35–42 fps | 52–59 fps, worst 1% 20–43 ms |
| ultra | 19–26 fps | 38–59 fps |

### The view distance didn't cut anything off

Bevy's culling ignores the camera's far plane: it checks the frustum with the far plane turned off. So at low, with a 250 m view, all 20,000 trees of the large forest were drawn to the horizon, hidden only by the haze.

**Fix:** every scenery mesh smaller than 50 m gets a visibility range ending at the `view` setting. That covers the last detail level and everything without levels. The ground is bigger, so it's always drawn.

### Where things stand: large forest (20,000 trees, 1 km)

Maximized, capped at 60:

| Preset | Frame rate | Worst 1% | Drawn | Note |
|---|---|---|---|---|
| low | 59 | 19–21 ms | 1–2 thousand meshes, about 1 M triangles | plenty of room |
| medium | 59 | 20–30 ms | about 7 thousand meshes, 4 M triangles | one freeze at the first stop, still settling in |
| high | 43–50 | 34–41 ms | 12–28 thousand meshes, 8–16 M triangles | the card is full |
| high, shadows off | 53–59 | 21–33 ms | the same | shadows cost 8–15 fps |
| ultra | 34–40 | 35–37 ms | up to 34 thousand meshes, 19 M triangles | the card is full |

High's 1 km view takes in most of the forest, and the lowest detail level is still about 800 to 1,600 triangles a tree. The main drawing pass alone takes about 20 ms. **Next: cards (impostors).** Beyond a few hundred metres, a tree becomes two or three flat pictures of itself, rendered in Blender from the full model, at about 8 triangles instead of 1,000.

## The presets so far

These are set in `client/src/graphics.rs`. The benchmarks will refine them.

| Setting | low | medium | high (default) | ultra |
|---|---|---|---|---|
| shadows | off | low: 1 layer, 40 m, 1024 px | medium: 2 layers, 100 m, 2048 px | high: 3 layers, 200 m, 4096 px |
| smoothing | FXAA | SMAA | MSAA ×4 | MSAA ×4 |
| bloom | off | on | on | on |
| view | 250 m | 500 m | 1 km | 2 km |
| detail | low | medium | high | high |
| textures | low | medium | high | high |

Vsync and the frame cap aren't part of quality, so a preset leaves them as they are.
