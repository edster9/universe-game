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

Mass and energy are conserved, and money rests on them (see [money.md](money.md)). Knowledge is not: copying a design costs nothing. That is why it is valuable, and why secrets, licenses, piracy, and encryption have meaning. EVE Online already sells the difference between an original blueprint and a limited copy.

A program is a design for behaviour, so "programs are cargo" from the [in-game computer](in-game-computer.md) is one case of this.

Copying a design is free, but building from it is not. Knowing how a validator works doesn't give you a drill, a scale, or a factory. See [production.md](production.md).

## Knowledge lives somewhere

There is no global recipe book, and the engine keeps no list of what has been invented. Every design exists only as physical copies:

- in a person's head, along with their skill;
- in a book, on a data chip, or in a computer's memory;
- in a factory, in its tuned machines, its programs, and its workers.

A copy can be carried, sold, locked up, or destroyed. **If every copy is destroyed, the knowledge is gone.** Suppose a planet holds the only datasheets and designs for something, has never shared them, and is destroyed. Nothing it made can be made again, because everything about it was on that planet. Items already built still exist elsewhere, and someone can try to reverse-engineer them. Until then it is lost technology.

Fiction has told this story many times:

- In Warhammer 40,000, the Standard Template Constructs are lost designs from a golden age, hunted for millennia.
- *A Canticle for Leibowitz* follows monks preserving scraps of knowledge after a collapse.
- In Asimov's *Foundation*, the Encyclopedia Galactica is an effort to back up civilisation.
- In *Dr. Stone*, one person rebuilds technology from the stone age using what he remembers.

What it gives the game:

- **Secret or safe.** A design kept secret keeps its value but can be lost. A design shared survives but is worth less. Every inventor chooses.
- **Archives, backups, and escrow.** An archive can hold a copy for a fee. It is a business, a target, and something worth defending. A copy stored on the network costs storage and lives wherever that storage physically is.
- **People are knowledge too.** Kill a planet's engineers and its intact factory may stop, because nobody knows how to run it.
- **Ruins and derelicts matter.** A wreck may hold the last copy of a design, or the last example of an item to take apart.
- **Collapse and recovery.** Losing a whole chain is a world event, and getting it back is the survivors' challenge. The climb in [production.md](production.md) explains why a route back always exists, however long it is.

## The player's head or the character's

This is the hardest open question here. Outer Wilds gates progress entirely by what the *player* knows. Rust, Subnautica, and EVE gate it by what the *character* has learned.

The working proposal is to have both:

- The player can always try to put things together. If an assembly obeys the laws, it works, so the player's own intelligence counts.
- The character's skill decides the success rate and the quality.
- Designs make big builds tractable. Nobody hand-assembles a ship from thousands of parts. Without a design, it is impossible in practice even if not in principle.

## Skills

A skill is a saved procedure for doing something, plus the practice to do it well. How skills are learned, saved, and used through a spoken interface is in [skills-and-interface.md](skills-and-interface.md). Knowing what things *are*, which shapes what a character perceives and whether they can be cheated in trade, is in [recognition.md](recognition.md).

## Machines that build

Later, a machine can carry out a design: a program that encodes the design, running on a computer wired to a fabricator and a robot arm. That is automation, and it is where the in-game computer meets building. The machine still needs the design, the power, the materials, and someone to have built it.

## In the slices

Knowledge is not a mechanic in the earliest slices, but the data model carries it from slice 1. Every process names the agent performing it, and checks whether that agent knows how. At first the check always passes. It is cheap to include now and painful to retrofit. It becomes real in slice 4 (see [slices.md](../slices.md)), including the loss of the only copy.
