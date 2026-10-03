# Assets

**Placeholder for a later conversation**, raised by the owner on 2026-09-30 ("The first scene, and assets for later" in [requirements.md](../requirements.md)).

Where the game's 3D assets come from: bushes, trees, rocks, sticks, people, animals, building pieces, and everything else. Options to weigh: making our own, free game asset libraries, paid asset stores, and procedural generation from the engine's own data (a thing's parts, forms, and materials, as decided in [game-interface.md](game-interface.md): things are drawn from their parts' forms and materials). Something to work with until the game has its own artists.

Until then, the first scene ([the-client.md](the-client.md)) draws everything from simple shapes and colours.

## Base shapes made in code (built 2026-10-02)

At the owner's request ("we need some more shapes"), the client draws things with shapes it makes itself (`client/src/shapes.rs`), still from forms and materials, chosen in `client/style.toml`: a **twig** with a side shoot, a crooked **stick** with a knot, a **log** with a branch stub, a **tuft** of grass blades, irregular **stones**, faceted **shards** for flint, lumpy **nuggets** for ore, flat **boulders** (the skill grounds' landmarks), **shells**, **roots and nuts**, a **bush** of clumps, a tiered **pine**, a **ring** of stones (a fire ring), and a **kiln** (a furnace). Stones, flint, and ore come in four variants each, so a pile isn't one stone repeated.

**The size picks the shape:** a material can have other forms for lighter pieces, by weight. Wood under 50 g is drawn as a twig and under 2 kg as a stick, matching the words twig, stick, and log; a stock is drawn by the weight of its pieces, so "dry twigs" shows twigs. Things built to a design are drawn by design (`[design.fire-ring]`). A client test checks that every form the style names is drawn.

## Asset packs: the owner's offer (2026-10-02)

The owner would buy a pack, or use a free one. Claude's take:

- **Free (CC0):**
  - **Kenney's Nature Kit**: 330+ low-poly nature objects (trees, rocks, stones, plants, terrain pieces, camping things).
  - **Quaternius' Stylized Nature MegaKit**: 110+ models (40 trees, 35 plants and flowers, 27 rocks, grass, bushes); 68 in the free standard version.

  Both come in formats Bevy loads (glTF), and CC0 means no conditions. (Even so, no pack goes in git: see below.)
- **Paid packs** (Synty's POLYGON range is the best known) look more finished, but **check the licence before buying**: store licences usually allow use in a game but not sharing the raw files. **Our repository is public**, so paid models live outside git.
- **Recommendation:** try a free CC0 pack first, for the things our own shapes do worst: trees, bushes, and the landmarks. A thing would be drawn by a model when the style names one for its form, falling back to our shapes; the engine is untouched. Keep our shapes for anything whose size or make-up matters (twig, stick, log, a piece of flint), since they scale from what the thing is. Decide after seeing one in the scene.

## Quaternius' Stylized Nature MegaKit: what's in it (2026-10-02)

The owner downloaded the free Standard version; it's kept in `assets/third-party/quaternius/stylized-nature-megakit/` (git ignores the packs; see `assets/third-party/README.md`). **CC0**. 68 models, each as glTF (which Bevy loads directly), FBX, and OBJ, sharing 20 textures. The full kit (116 models, the autumn and purple trees, more rocks and bushes) is the paid Pro version.

- **Trees, 25:** pines (5, 7 to 10 m), common leafy trees (5, 7 to 9 m), dead trees (5, 10 to 16 m), twisted trees (5, 16 to 19 m). 1,600 to 10,000 triangles each.
- **Rocks and stones, 24:** medium rocks (3, about 3 m), pebbles (11, round and square), rock-path pieces (10, flat stepping-stone patches).
- **Plants, 19:** grass (4: short and tall, common and wispy), bushes (2, plain and flowering), fern, clover (2), plants (4), flowers (4, single and group), mushrooms (2, one a shelf fungus), petals (5, ground scatter).
- **Not in it:** twigs, sticks, logs, stumps, flint, ore, shells. Those stay our own shapes.
- **Sizes are the artist's, not real ones** (grass 1.3 to 1.9 m, a "pebble" half a metre), which doesn't matter: the style scales each form to the size it gives.
- **Looks:** the previews are rendered in Unreal with Quaternius' own stylized shader (in the paid Source version). Bevy's standard lighting will look plainer, with the same shapes and textures. Leaves use cut-out textures (alpha mask), which Bevy supports.
- **Weight:** 48 MB for the glTF set, mostly six 2048-pixel bark textures (4 to 5 MB each); we'd shrink those when adopting.

### Tried in the scene (2026-10-02)

At the owner's request ("render a few items into our current scene... so I can see their quality before I buy the Pro pack"), the client now draws things with models where the style names them (`models`, `scale`, `most`, `over` in `client/style.toml`), falling back to our shapes when the files aren't there; `client/run.sh` copies `assets/` beside the program. On trial: standing trees (common trees and pines), bushes (the flowering one), loose stones (pebbles), grass, and the zone landmarks (medium rocks). The rest is set out by **the showcase** (`/showcase`): fern, plants, flowers, clover, mushrooms, petals, stepping stones, a twisted tree, and a dead tree. Twigs, sticks, logs, flint, ore, and shells stay our shapes.

What we saw: the models load and look like the pack's previews in shape and texture, under plainer light. **The plain bush is red**: it borrows the twisted tree's red autumn leaf texture, which Quaternius' own shader would recolour; we use the green flowering bush instead. The grass is green even where the world says dry grass; a colour tint per thing would fix that later. Big models are capped (four trees, three bushes, one landmark rock) so a stock's many pieces don't pile into one blob. Wide mounds (the forest floor) are now drawn low, at most 30 cm, instead of hiding what lies on them.

## No packs in git; a README for each (decided 2026-10-02)

The owner's decision ("Asset packs stay out of git" in [requirements.md](../requirements.md)): **no third-party asset files go in the repository, whatever their licence**, for their size and because licences differ (packs we buy soon may forbid sharing). What's committed is a **README in each pack's folder** under `assets/third-party/<maker>/<pack>/`: where to get the pack, the version, its licence, where its files go, and which files the game uses, so someone who forks the project knows what to download. A client test checks that every model `client/style.toml` names is listed in its pack's README. Without a pack, the client draws its own shapes in place of its models.
