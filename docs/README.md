# Universe game notes

Design notes for a small, networked space game that runs on a world engine of its own. Nothing is simulated from nowhere, things are made of things, and nothing gets built without someone who knows how.

Started 2026-09-28. No code yet. The first attempt is text-only slices of the engine.

## The idea in five lines

1. **Laws, not things.** The engine knows how heat, hardness, energy, and money behave. It does not know "sword" or "rocket".
2. **Conservation.** Mass, energy, and credits are never created from nothing. Shops, guards, and towns run on real sources.
3. **Datasheets.** Something built is tested once and becomes a part with a spec. That is how the world climbs from ore to microchips.
4. **Knowledge.** Making anything takes a design and the skill to use it, as well as materials and tools.
5. **Prehistory.** The universe opens in the space age because the space age was built inside the engine first, not hardcoded.

## Read in this order

| Doc | What it holds |
| --- | --- |
| [requirements.md](requirements.md) | What was asked for, in the words used |
| [ideas/world-engine.md](ideas/world-engine.md) | Laws, conservation, layers, datasheets, prehistory |
| [ideas/knowledge.md](ideas/knowledge.md) | Intelligence as the third input to making anything |
| [slices.md](slices.md) | The first attempt: small text-only slices, each with a test |
| [ideas/in-game-computer.md](ideas/in-game-computer.md) | Computers as parts, programs as knowledge |
| [ideas/game-concept.md](ideas/game-concept.md) | The space game that eventually runs on the engine |
| [research/prior-art.md](research/prior-art.md) | What other games have tried |
| [open-questions.md](open-questions.md) | Decisions not yet made |

## How to add to this

- Put intent in `requirements.md` only when it was actually asked for.
- Put proposals in `ideas/` and sourced surveys in `research/`.
- When a proposal becomes a decision, say so at the top of that file and give the date.
- When a slice is built, record its result in `slices.md`: whether it passed its test and what was learned.
