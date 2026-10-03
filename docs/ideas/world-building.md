# World building: what a world is made of, and what can be touched

Proposed 2026-10-02 and **agreed the same day, all six points** (see "Decided" at the end). The owner's words are "World building, in parallel with skills" and "What the world is made of, and what can be touched" in [requirements.md](../requirements.md). It builds on "the one and the many" ([granularity.md](granularity.md), agreed 2026-09-30) and answers the parts that decision left open for this conversation: how the map changes when trees are felled or land is cleared, and how building changes the world.

## The three kinds, in plain words

Every object a world builder places in Blender is one of three kinds, and says which.

**Scenery** is everything in the world that you can see but never use. The ground under your feet, the cliffs, the roads, the outside walls of a city's buildings, a forest on a distant hillside, and the flowers and mushrooms placed for decoration are all scenery. The game draws scenery exactly as it was built, but the engine doesn't keep track of any of it piece by piece, because nothing can ever happen to it. The engine learns only three things from scenery: the shape of the ground, so people walk on it at the right height; later, which parts block the way, so people walk around a building rather than through it; and what the ground is made of, so you can gather sand on a beach. When you move the mouse over scenery, nothing happens: no name, no menu. If a builder wants part of the scenery to be usable, such as a way into a building, they place a door there as a separate thing.

**A source** is a place where you gather or work, made of many alike things that you take from but never single out: a stand of trees, a patch of grass, a pile of loose stones, a flint deposit, fish in the shallows. In Blender, the builder marks out the source's area, says what it is (what it's made of, how much there is, how big a piece you take, which tool you need, and whether it grows back), and scatters the models that show it, such as four hundred trees. To the engine, the whole source is one thing, however many trees are drawn. When you point at any tree in the stand, the whole stand answers by one name, "standing trees". When you fell a tree, the work takes its time; then the tree nearest you disappears and a log lies on the ground. The engine remembers which tree went, so every player sees the same gap and a save keeps it, and over time the trees grow back. Clearing ground for a town is felling every tree in that area.

**A thing** is anything that counts on its own: anything you can pick up, carry, use, open, or name; anything alive; and anything anyone has made. A spear on the ground, a door, a car, a chest, a named landmark boulder, a boar, a fire you lit, and a house you built are all things. In Blender, the builder places things from the catalogue, so each arrives with its meaning attached, and can adjust one, such as making a boulder heavier or giving it a name. The engine keeps every thing individually, with its own place and history, and things are fully interactive: pointing shows the name, clicking opens the menu, and build mode moves them. A thing made of parts, such as a house built from a kit of walls, a roof, and a door, is still one thing: you click the house, not a wall. Only a part that does something on its own, such as a door that opens or a chest that holds things, is a separate thing attached to it. Everything players make in play, from a fire to a town, is things, and it lives in the saves, not in the Blender file.

**The rule that sorts them:** if a player can't change it or take from it, it's scenery; if they take from it but it's one of many alike, it's part of a source; otherwise it's a thing.

In short:

| Kind | What it is | In the engine | Hover and click |
| --- | --- | --- | --- |
| **Scenery** | The ground, cliffs, roads, a city's baked-in buildings, distant forests, decoration | **Nothing**, except the shape of the ground and what blocks the way | Never |
| **Sources** | Where you work or gather: a stand of trees, a grass patch, a flint deposit, a fishing shallow | **One thing per source**, however many trees or tufts are drawn for it | As one: "standing trees", not tree #212 |
| **Things** | What you can pick up, use, open, or name: an item on the ground, a door, a car, a landmark, a creature, everything anyone makes | **One thing each** | Yes |

**The rule for which is which: if a player can't change it or take from it, it's scenery.** If they can take from it but it's one of many alike, it's part of a source. If it acts, can be carried, used, or opened, was made, or is named, it's a thing (the one-and-many rule, unchanged).

This is where the cost is saved. Scenery costs the engine nothing at all, and the client draws it in batches, which is cheap. A forest of 400 trees is one source to the engine. Only things cost memory and attention, and those are what people touch: the cone of influence, applied to the map.

## Scenery

**What it is:** everything the player only sees and walks on or around. The ground, rock faces, a city's buildings (their outsides, baked into the scene), roads, fences, far-off forests, and decoration such as the showcase's flowers and mushrooms.

**What the engine knows of it:** only what physics needs, in bulk:
- **the shape of the ground**, as a grid of heights, so walking, sight, and the drawing agree (today the land is guessed from a few places' heights; a Blender world gives the real shape);
- **what blocks the way**, later, as simple outlines (a building's footprint, a cliff), for walking around things;
- **what the ground is made of**, by area (sand, soil, rock), a vast stock as decided: "gather sand" works on the beach without anything to click.

**Hover:** none. The pointer passes through scenery to whatever is behind it. That can feel strange at first, as the owner says; a later UX choice (a subtle highlight on what can be touched) makes it read naturally. Every game with baked cities does this.

**Can scenery ever become touchable?** Not by itself. If a building should be enterable, its **door** is a thing placed in the scenery; if a boulder should be breakable, it's a thing, not scenery. The world builder decides when placing it. (Destroying a baked city is the open "earthworks and cities" question, later.)

## Sources

**What it is:** an area you work in, where many alike things are one stock: trees, grass, sticks, stones, flint, ore, fish, roots. As decided, counted by mass, with a piece size, a time to take one, maybe a tool, and maybe growing back.

**In Blender:** a marked area, with the source's meaning (material, mass, piece size, tool, regrowth), and the objects drawn for it inside the area: the 400 trees, scattered with Blender's own tools.

**In the engine:** one thing, as now, plus **the spots of what's drawn for it** (the 400 trees' positions: a short list of numbers). That list is new, and it's what answers the owner's question.

**Felling a tree, the answer:** you walk up to the stand, or to one tree in it, and choose "fell a tree" (with an axe). The action takes its time, with a work animation; when it ends, **the tree nearest you is gone and a log lies there**. The engine records which drawn tree was taken (by its place in the list), so every player sees the same tree gone, and a save keeps it. The stand's mass drops by the tree's share. When it grows back, trees come back, one at a time. Not literal, as the owner allows: you don't pick which branch, but you do see a tree fall out of the forest you're standing in.

**Clearing land** is the same thing many times: fell every tree in an area and it's clear; the source's area shrinks to what's left. That's how a city's ground gets made in a forest, the open question from 2026-09-30, answered without a new law.

**Hover:** the whole source answers as one, by its name in your words ("standing trees", "dry grass"), wherever on it you point. Its outline can highlight as a whole.

## Things

**What it is:** each one counts on its own: anything you can pick up, use, open, or name; anything that acts; anything made.

**In Blender:** an entry from the **catalogue**, placed, which brings its meaning with it (see below).

**In the engine:** one thing each, with a spot, as now. Fully interactive: hover, menu, carry, build mode.

### Assemblies: the house from a kit

A house made from a kit (walls, roof, door) is **one thing**, an assembly, as the engine already treats anything made: it's measured once from its parts' datasheets and never looks inside them. **You click the house, not its walls.** The kit's parts are how it's built and drawn, not things in the world.

**A part is its own thing only if it does something by itself:** the door opens and locks, a chest holds things, a lever moves. Then it's a thing **fixed to** the house (the "fixed" join in [hands-and-joins.md](hands-and-joins.md)), clickable on its own. Everything else about the house is one.

### Building: the site, the time, the house

The owner's picture, step by step, in the engine's terms:
1. **Choose where.** A footprint on the ground (2×2, 4×4, 6×6 metres), shown as a ghost outline. The ground must be clear: no scenery blocking it, no source in the way (fell or clear it first), no things on it.
2. **Start building.** The materials must be at hand or stacked at the site (nothing from nowhere). The site becomes a **thing at once, a building site**: someone made it, so it's one. It holds the materials as they go in.
3. **The work takes time**, by the design's numbers (process times on things, as decided), shown as scaffolding growing, a progress bar, and a work animation. Others can help or interrupt; leaving it pauses it.
4. **Done:** the site becomes the house (an assembly to its design), drawn by the design's model, and from then on clickable like any thing.

The raft, the hut, a bigger house, and a town are all this, with bigger designs and longer times. It's the same law throughout: making something with a time, from materials, to a design. Only the scale changes.

## The catalogue: where a model gets its meaning

A model of a rock isn't a rock: the owner's point. The **catalogue** joins the two. Each entry is:
- **a model** (from a pack, or our own shapes);
- **its kind of thing**: scenery, source, or thing;
- **its meaning**, in names from our data files: material, mass (or mass per size), kind of creature, design, and, for sources, piece size, tool, and regrowth.

Examples: "granite boulder: thing, rock, 2 t, fixed"; "pine stand: source, wood, 20 kg pieces, needs an axe, grows back"; "cliff face: scenery"; "house, small: thing, design small-house".

World builders place catalogue entries from Blender's asset library, so the meaning comes along, and adjust one placement when needed (a heavier boulder, a name for a landmark). **The catalogue is where the two tracks meet:** the skills track adds materials, kinds, and designs; the world track adds models and placements. The in-game world builder's palette, later, is the same catalogue.

## From Blender to the game

1. **Blender is the master copy of a base world**: its ground, scenery, sources, things, and places (marked areas, like the clearing and the deep woods).
2. **Export to glTF**; each object carries its catalogue entry and any changes, as Blender's custom properties.
3. **Our converter** (in the console, not the engine, which never reads files) checks every entry against the catalogue and our data files, refuses anything unknown, and writes two things: **the world's data** for the engine (places, ground heights, sources with their drawn spots, things) and **the scene** for the client (the scenery, drawn as is).
4. **Play begins**, and the world moves on from the base: everything anyone fells, builds, drops, or kills lives in saves (and later the server), never back in Blender.

So the owner's sequence holds: built in Blender, loaded, and you're in it; from then on, the world grows by play, and new things come in from the catalogue (the designer's `/make`, or the in-game builder later).

## What changes in the engine, and what doesn't

- **No new law** for any of this. The one and the many, making with time, assemblies, and joins already cover it.
- **New data:** a source's list of drawn spots, and which have been taken; the ground's height grid; later, what blocks the way.
- **Sight and names** already work on things and sources only; scenery simply isn't in the engine, so it can't be named or pointed at. (Navigation by landmarks still works: a landmark is a thing.)

## Questions for the owner (all agreed, 2026-10-02)

1. **Three kinds** (scenery, sources, things), with the rule "if a player can't change it or take from it, it's scenery": agree?
2. **The house is one thing**; a part is its own thing only if it does something (a door, a chest): agree?
3. **Felling** takes the tree nearest you from the stand, recorded by the engine so everyone sees the same tree gone, and trees grow back one by one: agree? (The engine keeps the list because in multiplayer every player must see the same forest.)
4. **Building** goes footprint, then site, then time, then the house, with materials at the site and the ground clear first: agree?
5. **Blender is the master** for base worlds, and everything made in play lives in saves, never written back to Blender: agree?
6. **The first test**, once agreed: the skill grounds rebuilt in Blender with all three kinds (ground and decoration as scenery; the woodland stand, grass, and stones as sources; landmarks and a kit as things), converted, and the existing proofs passing on it.

## Decided, 2026-10-02

The owner: "I want to agree on everything." All six points above are decided: the three kinds and the rule that sorts them; the house as one thing, with only working parts separate; felling the tree nearest you, recorded by the engine, growing back; building by footprint, site, time, and house; Blender as the master for base worlds, with play kept in saves; and the first test, the skill grounds rebuilt in Blender. The plain-words section above was rewritten at the owner's request, for reading aloud.

**Build mode moves things, not sources** (the owner, 2026-10-02): a source is moved in Blender and converted again. Exceptions may come; none yet.

## The first world in Blender (built 2026-10-02)

At the owner's request ("one big square area with walls"), Claude built it by script in the owner's own Blender (5.0, run without its window from WSL), so no MCP server was needed. Everything is in `blender/` (see its README): **`catalogue.toml`**, the catalogue as text (25 entries: 3 things, 15 sources, 7 kinds of decoration); **`catalogue.blend`**, the models, made from the catalogue and the packs by `make_catalogue.py` and kept out of git, as it holds pack models; and **`skill-yard.blend`**, the skill grounds as a walled yard 100 m across, which **links** to the catalogue rather than copying from it, so it holds no pack models and is kept in git. It has two places (the yard and the deep woods), 9 things, 39 sources drawn with 214 models, and the ground, walls, paths, and decoration as scenery. From now on the .blend file is the master, edited by hand; the script made only its first version.

The tags are Blender custom properties (`ug_entry`, `ug_id`, `ug_label`, `ug_mass` on things and sources, `ug_place` on places); anything without `ug_entry` is scenery. A source is an empty marking its area with its models as children, so moving the empty moves the source, and adding or removing children changes how much it holds.

## Many files, many hands (proposed 2026-10-02)

The owner asked whether a universe is many Blender files, a town each and an interior each, so that different people can work on different parts. **Yes, and it fits the engine as it is: a file is a place, or a few.**

- **A region file** for each stretch of the open world: a town, a coast, a forest. It holds its ground, scenery, sources, and things, and marks its places.
- **An interior file** for each inside: the pub, the library, a ship's hold. An interior is its own place in the engine, as the deep woods is now. **A door is a thing in the region file whose way leads to the interior's place**, exactly as places already have exits. Walking through it takes you there; the interior is loaded when someone's inside, and not before (the cone of influence again).
- **Library files** hold what's reused: the catalogues (one per pack or theme, as `catalogue.blend` is), and **prefabs**, such as a whole house assembled from a kit, or a pub interior, made once and placed in many towns. A region links to them and never copies, so a fix to the house fixes every town that uses it.
- **A manifest, in text**, says which files make a world and where each sits: the region files and their positions, and which interior each door leads to. Blender files can't be merged, so **one person works on one file at a time**, and the manifest, being text, can be merged as usual. That's what lets many people work at once without stepping on each other.
- **Names are kept apart by file**: an id is unique within its file, and the converter prefixes it with the file's own (`harbour-town/pub-door`), so two builders can each have a `door-1`.
- **Edges must agree**: where two regions meet, the ground must meet. The converter checks each shared edge and refuses a seam.
- **Bigger scales are the same pattern, nested**: a universe's manifest lists its star systems, a system's its planets, a planet's its regions. Each level is a text file listing the files below it.

## The converter (built 2026-10-02)

`blender/convert.sh <world>` exports each Blender file a world's manifest lists (`blender/<world>.world.toml`: the files, the catalogue, the libraries it uses, and the world's settings) as glTF with its tags, then runs the converter (`console --convert`, `crates/console/src/convert.rs`), which writes the world's data file, `data/<world>.toml`, kept in git so the proofs run without Blender. It refuses unknown tags, missing or repeated ids, sources with no models, and things outside every place. A source's mass is its models' count times its entry's `each`, and its spread is how far its models stand. **First result:** `data/skill-yard.toml`, 2 places, the player and two boars, 45 things and sources; the skill grounds' board and fire proofs pass on it, the board's only change being one walk that's longer in the yard's layout. Tests: `crates/console/tests/convert.rs` (a small export written by hand: a place, a rock, three stones as a source, decoration; and the refusals), checked by sabotage (a source's mass ignoring its models fails it).

**The client draws the world as built** (the same day, after the owner saw that the client didn't look like Blender): it loads the export, draws scenery as placed, sources by their own models where they stand, and things by their own models following the engine (build mode moves them, not sources); the skill yard became the client's default world. **Not yet:** a source's drawn models known to the engine (their spots, and which are taken), for felling the nearest tree; ground heights from Blender.

## Where world files live (proposed 2026-10-02)

The owner: a sample .blend belongs in git, for a new clone to have something to open and play; the evolving world map doesn't, being many binary files that change often.

- **In git: samples.** Small worlds, changed rarely: the skill yard, and later one sample of each kind (a region, an interior, a prefab), so a clone can open, convert, and play them, and the proofs have worlds to run on. With them, everything that's text: the scripts, the catalogue, and the samples' manifests and converted data.
- **Outside git: the world.** The real, evolving map (its region and interior files, its manifests, its prefab libraries, and its converted data), kept together, **in an S3 bucket with versioning** (AWS first, as decided for hosting): every save of every file is kept, and any one can be got back. A script fetches it into a folder git ignores (`worlds/`), and sends back what's changed, file by file. Converted data is made from what's fetched, so it never drifts from it.
- **One person per file:** Blender files can't be merged, so the script **takes a file out** (a small lock in the bucket, naming who and when) before it's edited, and gives it back when it's sent. Someone else asking for it is told who has it.
- **Later, for many people:** a game server reads its world from the same bucket, by version, so a release is a chosen version of every file.
