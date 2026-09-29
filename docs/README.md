# Universe game notes

Design notes for a small, networked space game that runs on a world engine of its own. Nothing is simulated from nowhere, things are made of things, and nothing gets built without someone who knows how.

Started 2026-09-28. The first attempt is text-only slices of the engine, in Rust. Slices 0, 1, and 2 are built; see [slices.md](slices.md).

## The idea in eight lines

1. **Laws, not things.** The engine knows how heat, hardness, energy, and money behave. It does not know "sword" or "rocket".
2. **Conservation.** Mass and energy are never created from nothing, and money rests on them. Shops, guards, and towns run on real sources.
3. **Datasheets.** Something built is tested once and becomes a part with a spec. That is how the world climbs from ore to microchips.
4. **Knowledge.** Making anything takes a design and the skill to use it, as well as materials and tools. Knowledge lives somewhere, and can be lost.
5. **Chains.** Knowing a design is not being able to make it. Tools make tools, and factories don't appear on their own.
6. **Code sees through sensors.** Laws are the engine's. Designs are data. Programs run on in-game computers and know only what their sensors measure.
7. **Prehistory.** The universe opens in the space age because the space age was built inside the engine first, not hardcoded.
8. **Many universes.** Each has its own rules and history. Anything can be lost, and some universes will destroy themselves. The climb from a desert island to a chip fab and beyond is always possible, and the universes that survive are the point.

## Read in this order

| Doc | What it holds |
| --- | --- |
| [requirements.md](requirements.md) | What was asked for, in the words used |
| [ideas/world-engine.md](ideas/world-engine.md) | Laws, conservation, layers, datasheets, prehistory |
| [ideas/code-and-laws.md](ideas/code-and-laws.md) | The foundational choice: laws, designs, and programs, and what each can touch |
| [ideas/production.md](ideas/production.md) | Why duplicating something needs a chain of tools and factories |
| [ideas/knowledge.md](ideas/knowledge.md) | Intelligence as the third input to making anything, and where knowledge lives |
| [ideas/skills-and-interface.md](ideas/skills-and-interface.md) | Talking to the game, and successes saved as skills |
| [ideas/recognition.md](ideas/recognition.md) | Knowing what things are: perception and trade depend on what you've learned |
| [ideas/skill-learning-paradigm.md](ideas/skill-learning-paradigm.md) | For a later conversation: what a player can teach their character, and what stops a pasted rocket |
| [ideas/kinds.md](ideas/kinds.md) | Decided: how living things are classified, humans and future aliens included, and what drives them |
| [ideas/memory.md](ideas/memory.md) | Decided: what a person knows, certain or only possible, and how maps and seeing for yourself correct it |
| [ideas/harm.md](ideas/harm.md) | How things cause harm: edge (built), blunt, projectile, and blast, as ways of delivering energy rather than kinds of weapon |
| [ideas/time-away.md](ideas/time-away.md) | For a discussion soon: what happens to a player's person while they're away |
| [ideas/vocabulary.md](ideas/vocabulary.md) | For a conversation after the next two challenges: how new vocabulary emerges as the engine trains |
| [ideas/universes.md](ideas/universes.md) | Many universes, different rules, the climb, and universes that die |
| [ideas/money.md](ideas/money.md) | What money is, before and after the space age, and how it's validated |
| [slices.md](slices.md) | The first attempt: small text-only slices, each with a test |
| [challenges/stranded.md](challenges/stranded.md) | The first training challenge: from a desert island to the next island on a raft |
| [challenges/where-am-i.md](challenges/where-am-i.md) | The second training challenge: explore the island and climb its mountain to see where you are |
| [challenges/living-with-the-island.md](challenges/living-with-the-island.md) | The third training challenge: wildlife, hunting and cooking, shelter, shoes and clothing |
| [laws.md](laws.md) | Every law in the engine, stated generally, and what uses it |
| [technology.md](technology.md) | Rust, the browser, whole-number units, and how player code runs |
| [ideas/in-game-computer.md](ideas/in-game-computer.md) | Computers as parts, programs as knowledge |
| [ideas/game-concept.md](ideas/game-concept.md) | The space game that eventually runs on the engine |
| [research/prior-art.md](research/prior-art.md) | What other games have tried |
| [open-questions.md](open-questions.md) | Decisions not yet made |

## How to add to this

- Put intent in `requirements.md` only when it was actually asked for.
- Put proposals in `ideas/` and sourced surveys in `research/`.
- When a proposal becomes a decision, say so at the top of that file and give the date.
- When a slice is built, record its result in `slices.md`: whether it passed its test and what was learned.
