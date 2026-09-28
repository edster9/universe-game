# Universes

Working proposal, 2026-09-28. The requirement is "Many universes" in [requirements.md](../requirements.md).

## Many universes, not one

The game runs many universes side by side, the way World of Warcraft runs many realms. Each one is a separate instance of the [world engine](world-engine.md), with its own state, its own people, and its own history.

This fits the small map. Each universe is small enough that players meet, and more universes open as more players arrive. It also fits the engine: universes don't influence each other, so each runs independently on its own servers.

## Different rules for different universes

The same engine, with a different set of rules per universe. Kinds suggested so far:

- **Training universes.** Rules set up for training. Prehistory runs here: the climb from raw materials up the ladder, layer by layer, that produces the designs the other universes start with (see [production.md](production.md)). Training universes might also be where new players learn. That second use is a proposal, not a decision.
- **Free-for-all universes.** No protections. If a civilisation decides to annihilate itself, that is the choice it made.
- **Others in between**, such as universes with protected zones or different starting points.

What a universe's rules might set:

- which epoch it starts in: a desert island, or the space age with its designs already released;
- what protections exist, if any;
- how fast things happen: travel, recovery, how long skills take to learn.

Whether the laws themselves can differ between universes is an open question. If they do, a design from one universe might not work in another.

## Universes can die

In a free-for-all universe, nothing in the engine stops self-destruction. The safeguard is people: having good people in a universe, who build, defend, and keep copies of what matters.

Many universes will destroy themselves. It is human nature to destroy when there is too much power. Some will survive and thrive, and that is the ultimate experience of this game: a universe that lasted, and climbed further than any other.

Minecraft's anarchy servers are the nearest existing example. 2b2t has run since 2010 with no rules and no resets, and its history of destruction and rebuilding is its whole identity.

## The climb

Every universe has the same challenge in front of it, whether it starts there or falls back to it: how far can you get? From a desert island to metals, then batteries, wires, and motors, then machine tools, then a chip fab. Then teleportation, and beyond, to things nobody has thought of, if the laws allow them. Perhaps even time travel.

A universe that loses its chip fab and every copy of the design isn't finished. It is back on the climb. See [production.md](production.md).

## Why determinism helps here

The engine is deterministic: the same starting state and the same inputs give the same result. For universes that means:

- a universe's whole history can be replayed from its log;
- a universe that has died can be archived and studied, like ruins;
- a training universe's climb can be re-run after the laws change, to see whether the same designs still work.

## Open

- Can anything cross between universes: characters, designs, money? If nothing does, each universe's knowledge is its own, and a lost design is truly lost there. The one thing that always crosses is what a player has learned in their own head.
- Do the laws ever differ between universes, or only the social rules?
- What happens to a universe that has destroyed itself? Does it close, stay open as ruins, or restart?
- How many players per universe, and when does a new one open?
- Are released designs the same in every universe, or does each universe start from its own training run?
