# Requirements

Captured 2026-09-28 across the opening conversations. These are what was asked for, kept close to the words used. Everything else in `docs/` is a proposal or research.

## The world is an engine

The world is a virtual engine of its own. The game runs on top of it. This is the foundation, and the first attempt builds it before anything else.

### Nothing comes from nowhere

The flaw this project starts from: you land, walk into a town, and the shop sells an unlimited stock of weapons. Nobody made them, nobody brought them, and they never run out. The town is protected, but nobody pays its soldiers. Things that would be possible in real life are not possible.

In this world:

- A shop has stock because someone made it and brought it there. When it is sold, it is gone.
- A guard works because someone pays them. A town pays from taxes, trade, or an empire that is itself funded. When the money stops, the service stops.
- Everything depends on everything. The world is balanced by its own dependencies, not by rules that forbid things.

### Consequences are physical

What is possible in real life should be possible here, with the consequences real life would bring. A gang can raid a town. If the raiders blow up their own ships in the process, they are stranded. They are not stuck forever, but leaving means building a ship from whatever they can find, or from scratch.

### Things are made of things

Like Minecraft, A + B makes C. The building blocks of everything are defined by the engine's rules and by code, not by a list of finished items.

- Digging a hole, melting the rock to extract iron, and forging a sword should be possible without the engine containing "sword". The engine knows what rock and iron are and how they behave. The sword is something someone makes.
- An item is a label, attributes, and behaviour. The engine does not know what things *are*; it knows how they interact.
- Because items are built, an item can be unique in the universe.
- The ceiling is open. Reaching things nobody planned is the ultimate destination. The example given was a teleporter.
- Not everything has to be code. The world itself has to be the engine.

### Start in the space age, without hardcoding it

The universe does not start in the stone age and evolve. It starts in the space age. But the space age should exist because it was built up and taught to the engine, not hardcoded into it. The engine must still know the stone age, because the space age is built on it.

## Intelligence

Things don't build themselves; people do. Materials and parts are not enough. Stranded next to a wrecked ship with every part you need, you still cannot build a ship if you don't know how.

Knowledge is a third input to making anything. You must already have it, find a book and read it, or talk to others and collaborate to put things together. Maybe someday computers will build things too.

This is integral to the overall design. It may not be in the very first stages.

## Money

What makes money valuable should be answered, not assumed. Science fiction says "I have credits". Credits towards what: a computer system, like a crypto that controls it, or an element like gold that is rare in the universe and cannot be duplicated? At the end of the day, money is something rare that people are willing to use.

- There should be money before the space age, and money in the networked space age.
- Space-age money has to be validated somehow.
- Either way, money has to rest on the physics and primitive rules of the core universe.

## The in-game computer

A virtual computing engine inside the game. Players write code in a sandbox, and that code can reach the core of the game. The examples given: a better autopilot, and a programmatic way to create objects that can be traded. The game should grow from the inside, by what players make, rather than only by patches from outside.

This was called out as the distinctive part of the original concept.

## The game on top

The eventual game, once the engine can carry it:

- An Elite-style career. You start with a ship. Trade between worlds, fight in space, dock, take missions, and make money.
- The play is a journey: land on a planet, leave the ship, and continue on foot.
- Flight is a 3D simulator. On the ground, in cities and ports, play becomes an Ultima-style walk: streets, shops, bars, mission boards, shipyards. The two modes share one character, one purse, and one set of obligations.
- Money is credits, described as crypto: the one money that works everywhere in the universe. What stands behind credits is in [ideas/money.md](ideas/money.md).
- The universe is small on purpose, so that a networked game puts players near each other. The point of the size is more encounters with real people.

## The first attempt

No graphics. Text only. Very simple slices of the engine, each proving one simple concept, to chart a course and see where it goes. See [slices.md](slices.md).

## Explicitly not decided

- The number of systems, planets, or ports.
- The language, instruction set, or API of the in-game computer.
- Whether player code may change rules for everyone.
- Engine, networking, art, perspective on foot, single-player offline play.
- The implementation language for the slices.
