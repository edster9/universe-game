# Assets

**Placeholder for a later conversation**, raised by the owner on 2026-09-30 ("The first scene, and assets for later" in [requirements.md](../requirements.md)).

Where the game's 3D assets come from: bushes, trees, rocks, sticks, people, animals, building pieces, and everything else. Options to weigh: making our own, free game asset libraries, paid asset stores, and procedural generation from the engine's own data (a thing's parts, forms, and materials, as decided in [game-interface.md](game-interface.md): things are drawn from their parts' forms and materials). Something to work with until the game has its own artists.

Until then, the first scene ([the-client.md](the-client.md)) draws everything from simple shapes and colours.

## Base shapes made in code (built 2026-10-02)

At the owner's request ("we need some more shapes"), the client draws things with shapes it makes itself (`client/src/shapes.rs`), still from forms and materials, chosen in `client/style.toml`: a **twig** with a side shoot, a crooked **stick** with a knot, a **log** with a branch stub, a **tuft** of grass blades, irregular **stones**, faceted **shards** for flint, lumpy **nuggets** for ore, flat **boulders** (the skill grounds' landmarks), **shells**, **roots and nuts**, a **bush** of clumps, a tiered **pine**, a **ring** of stones (a fire ring), and a **kiln** (a furnace). Stones, flint, and ore come in four variants each, so a pile isn't one stone repeated.

**The size picks the shape:** a material can have other forms for lighter pieces, by weight. Wood under 50 g is drawn as a twig and under 2 kg as a stick, matching the words twig, stick, and log; a stock is drawn by the weight of its pieces, so "dry twigs" shows twigs. Things built to a design are drawn by design (`[design.fire-ring]`). A client test checks that every form the style names is drawn.

## Asset packs: the owner's offer (2026-10-02)

The owner would buy a pack, or use a free one. Claude's take:

- **Free, and free to put in our public repository (CC0):**
  - **Kenney's Nature Kit**: 330+ low-poly nature objects (trees, rocks, stones, plants, terrain pieces, camping things).
  - **Quaternius' Stylized Nature MegaKit**: 110+ models (40 trees, 35 plants and flowers, 27 rocks, grass, bushes); 68 in the free standard version.

  Both come in formats Bevy loads (glTF), and CC0 means no conditions.
- **Paid packs** (Synty's POLYGON range is the best known) look more finished, but **check the licence before buying**: store licences usually allow use in a game but not sharing the raw files. **Our repository is public**, so paid models would have to live outside git (a folder the build copies, ignored by git), not in the repo.
- **Recommendation:** try a free CC0 pack first, for the things our own shapes do worst: trees, bushes, and the landmarks. A thing would be drawn by a model when the style names one for its form, falling back to our shapes; the engine is untouched. Keep our shapes for anything whose size or make-up matters (twig, stick, log, a piece of flint), since they scale from what the thing is. Decide after seeing one in the scene.
