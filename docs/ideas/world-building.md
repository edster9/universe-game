# World building: what a world is made of, and what can be touched

Proposed 2026-10-02. **Not decided.** The owner's words are "World building, in parallel with skills" and "What the world is made of, and what can be touched" in [requirements.md](../requirements.md). It builds on "the one and the many" ([granularity.md](granularity.md), agreed 2026-09-30) and answers the parts that decision left open for this conversation: how the map changes when trees are felled or land is cleared, and how building changes the world.

## The short version

A world built in Blender is made of **three kinds of things**, and each placed object says which kind it is:

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

## Questions for the owner

1. **Three kinds** (scenery, sources, things), with the rule "if a player can't change it or take from it, it's scenery": agree?
2. **The house is one thing**; a part is its own thing only if it does something (a door, a chest): agree?
3. **Felling** takes the tree nearest you from the stand, recorded by the engine so everyone sees the same tree gone, and trees grow back one by one: agree? (The engine keeps the list because in multiplayer every player must see the same forest.)
4. **Building** goes footprint, then site, then time, then the house, with materials at the site and the ground clear first: agree?
5. **Blender is the master** for base worlds, and everything made in play lives in saves, never written back to Blender: agree?
6. **The first test**, once agreed: the skill grounds rebuilt in Blender with all three kinds (ground and decoration as scenery; the woodland stand, grass, and stones as sources; landmarks and a kit as things), converted, and the existing proofs passing on it.
