# Open questions

Decisions nobody has made yet. When one closes, add the date and a one-line answer, and move the durable result into [requirements.md](requirements.md) or `ideas/`.

## Before slice 0

- What implementation language? See [slices.md](slices.md).
- What data file format?
- What is a tick, in game time?

## Code and laws

These are for [code-and-laws.md](ideas/code-and-laws.md).

- Which material properties to simulate. Each one decides what can ever be measured, and so what can be invented.
- How big a region is, and how fast influence travels between regions.
- Are floating-point maths and determinism compatible here, or do the laws use fixed-point numbers?
- The player VM: WebAssembly, or a small VM of our own?

## Production

These are for [production.md](ideas/production.md).

- How many steps deep should a chain be at each layer?
- How precise can hand work get, and how slowly?
- Which facilities exist at the start, and how many of each?
- How long should rebuilding a lost chain take, in game time?

## World engine

These are for [world-engine.md](ideas/world-engine.md).

- Which laws belong to each layer, and where do the layer boundaries sit?
- When is a datasheet recomputed: when a part wears, gets damaged, or gets repaired?
- What recovery sources exist for a raided town, and how slow should recovery be?
- Is there a law that permits a teleporter? It is a deliberate decision, not something to leave to chance.
- How physical is combat? A sword's datasheet against armour's, or something more abstract?

## Knowledge

These are for [knowledge.md](ideas/knowledge.md).

- How much does the player's own head count, compared with the character's skill?
- How does skill grow? Can it fade?
- How much of a design does reading a book give you, without the skill to understand it?
- How much of a design does taking an item apart reveal?
- Can an NPC be forced to share knowledge, and what does the world do about it?
- Does a stored copy of a design decay over time: a rotting book, a failing data chip?
- Who runs archives, and what stops them from reading what they store?

## Money

These are for [money.md](ideas/money.md).

- Which material is the hard money, before and after the space age?
- Can matter be transmuted into the money metal, and at what energy cost?
- How does information travel between systems: only with ships, through relays, or both? This decides what "universal" credits means.
- In the fiction, who wrote the network's rules, and why does everyone accept them?
- Confirm the recommendation: no real-money purchase or cash-out.

## The computer

These are for [in-game-computer.md](ideas/in-game-computer.md).

- One real language, or a tiny instruction set of our own?
- Do programs keep running while the owner is offline?
- Can a program be unique, as well as copied?
- Can player code change local rules (a town's taxes) through ownership?
- What is the first program a new character can run?

## The game on top

These are for [game-concept.md](ideas/game-concept.md).

- How small is small: one system, a handful, a few dozen ports?
- Are planets places you fly down to, or a port plus a point on the chart?
- Do systems connect by jump, by real minutes of flight, or both?
- What camera does the walking layer use: top-down, isometric, or first person?
- How big is a port in walking time?

## Networks

- One shared world, or many copies of the same small map?
- Does single-player exist, with NPCs standing in for other players?
- A small map only puts players together if there are players. How does the world work while few people are online?
- What persists: reputation, prices, programs, docked ships, towns' damage?

## Not yet in scope

Art style, audio, monetisation, and whether the eventual game runs in a browser or natively. Pick these once the text slices show the engine is worth building on.
