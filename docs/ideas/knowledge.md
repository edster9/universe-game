# Knowledge

Working proposal, 2026-09-28. Not a decision. The requirement is "Intelligence" in [requirements.md](../requirements.md).

## Why it is an input

Materials and parts are not enough. A stranded crew standing over every part of a ship still cannot build one without knowing how. Knowledge is the third input to making anything, alongside materials and tools.

It also makes people matter. An engineer is worth hiring, a teacher is worth finding, a library is worth raiding, and a blueprint is worth stealing.

## Two kinds

**Designs (know-what).** An exact plan: which parts, which processes, in what order, under what conditions. A design is information. It can be written down, copied, sold, stolen, encrypted, or lost. It lives in a book, on a data chip, in a blueprint, or in a program.

**Skill (know-how).** Lives in a person. It grows with practice and teaching. It cannot be copied, only passed on slowly. It decides:

- whether you can carry out a design at all,
- how long it takes,
- how good the result is (the datasheet it gets),
- whether you can make sense of a design you've never seen.

Making something takes:

> design + skill + materials + tools or facility + labour + time

## Ways to get it

- **Have it.** A character starts with some skills and some designs from their background.
- **Read it.** A book or manual carries a design. Understanding it may need skill in that layer. A chip-fab manual is useless to someone who has never worked metal.
- **Be taught, or hire it.** An NPC engineer has skill and charges wages. This connects to the economy: someone has to pay the engineer, just as someone pays the guard.
- **Collaborate.** Several people combine skills on one build. A ship needs specialists, the way the stranded raiding party does.
- **Take it apart.** Reverse engineering a built item yields part of its design, more with more skill. This works because every item in the world has a real bill of materials (see [world-engine.md](world-engine.md)).
- **Experiment.** Combine things and see what happens. The engine's laws answer. A successful experiment becomes a new design, which is how things nobody planned enter the world.

## The one thing that isn't conserved

Mass, energy, and credits are conserved. Knowledge is not: copying a design costs nothing. That is why it is valuable, and why secrets, licenses, piracy, and encryption have meaning. EVE Online already sells the difference between an original blueprint and a limited copy.

A program is a design for behaviour, so "programs are cargo" from the [in-game computer](in-game-computer.md) is one case of this.

## The player's head or the character's

This is the hardest open question here. Outer Wilds gates progress entirely by what the *player* knows. Rust, Subnautica, and EVE gate it by what the *character* has learned.

The working proposal is to have both:

- The player can always try to put things together. If an assembly obeys the laws, it works, so the player's own intelligence counts.
- The character's skill decides the success rate and the quality.
- Designs make big builds tractable. Nobody hand-assembles a ship from thousands of parts. Without a design, it is impossible in practice even if not in principle.

## Machines that build

Later, a machine can carry out a design: a program that encodes the design, running on a computer wired to a fabricator and a robot arm. That is automation, and it is where the in-game computer meets building. The machine still needs the design, the power, the materials, and someone to have built it.

## In the slices

Knowledge is not a mechanic in the earliest slices, but the data model carries it from slice 1. Every process names the agent performing it, and checks whether that agent knows how. At first the check always passes. It is cheap to include now and painful to retrofit. It becomes real in slice 4 (see [slices.md](../slices.md)).
