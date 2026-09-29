# How new vocabulary emerges

Recorded 2026-09-29 **for a conversation the owner has asked for**, after the [where am I?](../challenges/where-am-i.md) and [living with the island](../challenges/living-with-the-island.md) challenges and before any unplanned scenarios. The requirement is "How new vocabulary emerges" in [requirements.md](../requirements.md). Nothing here is decided; it's a starting point.

## What's happened so far

Every training stage has added words. They're of three kinds, and they live in different places:

| Kind | Where it lives | Added in training so far |
| --- | --- | --- |
| **Player words:** commands a person can say | The command parser (`intent.rs`) | `explore`, `sleep`, `fill`, `go … on …`, `divide`, `rub … until it catches` |
| **Engine words:** the roles, measurements, and properties the laws understand | Engine code (`world.rs`, `datasheet.rs`, `laws.rs`) | Roles: pulling, pushing, containing, casting. Measurements: buoyancy, holds up to, pushes with, can hold, awake for. Properties: softens in, becomes, tensile strength, ignition point |
| **World words:** the names of things | Data files (`data/*.toml`) | Rope, raft, paddle, pot, fired clay, logs, the next island |

The standing rules already keep the third kind out of the engine: a pot is data, and the engine knows only "a shape whose role is to contain". But the first two kinds grow with every stage, and each new one so far was added by hand, in code, when a story needed it.

## Questions for the conversation

- When a story needs something new, how do we tell a new **law** from a new **word** for an existing law? A paddle turned out to need a new role (pushing); a pot needed another (containing). Could they have been one broader idea?
- Should roles and properties be data rather than code, so a new world can bring its own vocabulary without changing the engine? What would the engine still have to know?
- Where does a *player's* vocabulary come from? It ties to [recognition](recognition.md) (you can only name what you know) and to the [skill learning paradigm](skill-learning-paradigm.md) (a skill is a named procedure).
- How does the vocabulary stay small as the engine climbs toward chips and rockets? What's the test that a new word is earning its place?
- Does vocabulary differ between universes, or peoples within one?
