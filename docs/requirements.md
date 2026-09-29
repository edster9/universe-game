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

## Duplication

Just because something exists doesn't mean anybody can build another one, even if its design or code is common knowledge. You have to go through all the datasheets of the things in it and how they fit together. Some components are easy to find. Others, like a drill, can't be made by hand; they need a machine factory, and the factory is a much more complex thing that isn't born on its own.

- Everything depends on everything else. That is where the effort and the cost are: you have to find a drill, find a scale, find the rest, and have the skill to put it together, or have it built in a factory somewhere.
- As things are invented, there must be a way to reproduce them through a chain of things.
- Stranded on a desert island, you can find food and eat it. You can't build a computer in a day. Eventually, perhaps, but first you have to make metals, melt them, make batteries, wires, and motors to machine things, and so on.
- Don't get too carried away. This is the basic idea, not a demand for every real-world step.
- As the world is trained and templates exist, things solidify.

## Knowledge is located

Knowledge is very key. If a planet holds datasheets and has never shared them, and the planet is destroyed, everything is lost. It isn't backed up anywhere. Anything that was produced on that world can no longer be produced, because everything about it was destroyed.

## Loss is real, and so is the climb back

Clarified 2026-09-28. Information can be lost, and the last chip factory can be destroyed. Nothing prevents it.

- Training gets to the design of a chip factory layer by layer, under the laws of the universe and the evolution it took to get there. Once built, it is released: the bill of materials, the templates, the spec sheets, everything needed to make it.
- If it is lost, getting back to that point is the challenge of whoever survives.
- How far can you go from a desert island: to a chip fab, to teleportation, and beyond, to things we can't even think of? If the engine's rules allow it, maybe even time travel.
- That climb is one of the challenges of the game.

## Many universes

Not one universe but many, the way World of Warcraft runs many realms.

- Some universes have certain types of rules, for training. Others are free-for-all.
- If a civilisation decides to annihilate itself, that is the choice it made. The safeguard is having good people in a universe.
- Many universes will destroy themselves, as it is human nature to destroy when there is too much power. Some will survive and thrive. That is the ultimate experience.

## The world runs on code

The original idea is a virtual computer as the world itself. Things are not only created by code, they run through code. A spaceship is higher-level code made of many other operations. When ship A meets ship B, code is running for both. Code far away doesn't need to be considered, because it's outside the cone of influence.

This is the most important early design decision: whether the world is rules with templated ways to build things, or code that does things, and where the core engine ends. See [ideas/code-and-laws.md](ideas/code-and-laws.md).

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

## Training by challenges

Added 2026-09-28. Before modelling towns, run challenge exercises to see whether the engine has enough in it yet. For example: dropped on a desert island with nothing, what series of steps gets a person onto a raft and across to a nearby island? Food first (a sharp rock, branches made into spears, fishing), then fire (rubbing wood or stones), then an axe (finding iron, melting it, casting, sharpening with a rock), chopping trees, making rope, building the raft, a sail or paddle, and the crossing.

- Once the engine's primitives let a challenge like this succeed, inject another random scenario and see whether the engine covers it without inventing anything new.
- This sets the tone for how the system is trained: basic survival, boats, and weapons first, then mechanical things (a bicycle, a car), then simple rockets, until the system is trained up to the space age.
- **A stranded person can die**, and often will in early testing. The chance of success should be about what a real person's would be. Run a challenge 20 or 30 times: perhaps five succeed and the rest die. Dying doesn't fail the test. **The test fails only if it runs out of logical options.** Better skills and better ways to feed yourself improve the odds, but there's always a chance of dying.
- Time in tests doesn't run in real time. Days of activity should compute in seconds, and waiting an hour should happen instantly.

See [challenges/stranded.md](challenges/stranded.md).

### Keep the spirit, not every detail

Added 2026-09-28, after stage 2 (fire). It's fine to simplify laws if that makes things easier. We don't have to be extremely granular to mimic a real-world example. This kind of fine tuning would happen a lot, and we don't want to over-engineer: keep the spirit of something alive.

**This is a standing rule** for every design like this: don't overcomplicate.

## Skills, and talking to the game

Added 2026-09-28. Making fire showed a new direction for how the game works.

- **Skills.** The whole fire-making process (grab twigs and grass, a 20-second window to heat and light it) would be very hard to build a user interface for. When a character succeeds at making a fire, the engine can save that as a skill. As you learn things, they become additional skills.
- **Talking to the game.** A very likely interface is an open microphone: explain what you want to do, an AI model interprets it, and the game engine takes your prompt, checks whether it works with the rules of the world, and applies it.
- **An adaptive interface.** The user interface itself becomes reflective and adaptive as you learn.
- **Where skills come from.** A blank mind knows nothing and is trained. Characters start with skills most people would know, such as building a fire. You can go to a library to learn skills beyond basic common sense.

See [ideas/skills-and-interface.md](ideas/skills-and-interface.md).

## The first attempt

No graphics. Text only. Very simple slices of the engine, each proving one simple concept, to chart a course and see where it goes. See [slices.md](slices.md).

## Explicitly not decided

- The number of systems, planets, or ports.
- The language, instruction set, or API of the in-game computer.
- Whether player code may change rules for everyone.
- Engine, networking, art, perspective on foot, single-player offline play.
- The implementation language for the slices.
