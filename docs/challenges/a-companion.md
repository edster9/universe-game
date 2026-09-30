# Challenge: a companion

Step 2 of the road to the village ([ideas/the-road-ahead.md](../ideas/the-road-ahead.md)), started 2026-09-30. The smallest community: the islander and the stranger, living on the same island. The stranger lives by the realistic rules (eats, drinks, sleeps) and has a mind of their own: scoped, with a temperament and standing orders, as decided in [ideas/npc-minds.md](../ideas/npc-minds.md). Nobody plays them.

The world is `data/companion.toml`: the island of [living with the island](living-with-the-island.md), with the stranger, whose people are the far-folk ([a stranger's words](strangers-words.md)), so everything the two do passes through two peoples' words.

## Stages

| Stage | Name | What it proves | Laws it needs |
| --- | --- | --- | --- |
| 1 | A mind of their own | The stranger looks after themselves for 30 days by their standing orders, with nobody commanding them. Everyone, people and creatures, acts through the same path: checked when they start, carried out when they finish. The shopkeeper test: take away what their orders need, and a *confined* mind stands and waits while a *resident* one finds another way to live | Minds: scope, standing orders in the person's own words, instinct underneath; creatures act at the end of what they do |
| 2 | Temperament | A boar turns on the stranger. Fight, defend, flee, or give in, set in data, decide what they do | Temperament as a setting for people as for creatures |
| 3 | Asking | The islander asks the stranger to do something ("ask the stranger to gather sticks"), in words the stranger knows. Within their scope they take it on; outside it, or in words they don't know, they don't | Asking: a request becomes a one-off order |
| 4 | Barter, and whose things are whose | Fish for meat, agreed by both, in one step. The stranger's orders say what they'll give for what, and a bad offer is refused. What they gathered or made is theirs in their mind: take it unseen and nothing happens; take it while they watch, and they remember | Exchange (both agree, one step through the gate); ownership lives in minds |
| 5 | Watching | The first browser page: the time, where you are, who's here and what they're doing, what you carry, the places you know, a command box. Playwright screenshots | None: a window onto the live channel |

## How minds work (stage 1)

A person with a mind is given, in data:

- **A scope** (`mind = "confined"` or `"resident"`; `"free"` comes later): how far they think.
- **Standing orders** (`orders = [...]`): lines like `"thirsty, at the stream: drink water"`. Before the colon, conditions they can tell for themselves: the time of day, whether they're hungry, thirsty, or tired, where they are, what they carry, what they see. After it, a command in their own words, exactly as a player would type it.

When they're free, awake, and not in danger, they take the **first order whose conditions hold and whose command the laws allow**, and start it. Underneath every scope is the body's **instinct**: sleep at night when tired, eat what they carry when hungry, drink if there's water where they stand. When no order applies:

- **confined**: they wait where they are;
- **resident**: they look after themselves, going for water or food at places they remember seeing it, and otherwise wait.

A mind knows only what its person perceives and remembers: no oracles. It only ever proposes commands, which the laws check like anyone's.

## Stage 1 result, 2026-09-30: passed

**The stranger lives on their own.** In `data/companion.toml`, they're a *resident* mind with seven standing orders in far-folk words: eat pale flesh when hungry; drop the shells once the flesh is gone; drink at the stream when thirsty, going there if they aren't; gather shellfish at the beach when hungry, going there if they aren't; and go back to the beach at night. They remember the beach, stream, forest, and hillside, having lived there a while (`remembers` in data).

- **Proofs** (`crates/engine/tests/companion.rs`): the stranger looks after themselves for 10 days, within a few kilos of where they started, with mass conserved; a **confined** stranger with the same orders lives by them, and is back on the beach by night; and **the shopkeeper test**: their orders send them for water to a spring that isn't there. Confined, they stand on the beach and die of thirst within six days; resident, they remember the stream, go there, drink, and live. Orders that make no sense (no colon, a condition nobody can tell, no command) are refused when the world loads.
- **The story** (`data/scripts/companion-1-a-mind-of-their-own.txt`): the islander watches the stranger's day: gathering on the beach, shells piling up on the sand, coming to the stream for water, back on the beach for the night, and alive ten days on.
- **The trial** (on demand): 10 of 10 strangers alive after 30 days with real chance, each about a kilo heavier.
- **Sabotage checks:** a resident mind that can't look after itself, a mind that ignores its orders, and a mind that never feels thirst each fail a proof.
- **Creatures now act at the end of what they do**, like people: a boar's walk, search, or charge is checked when it starts and carried out when it's due. Two stories changed: in the boar story played by players' rules, the herd, back at the stream for a morning drink, was caught mid-way out, so the check now looks ten minutes later; and the hunt brought down a boar whose hide carried 14 mg more rotting flesh.

**What the attempt found:**

- **Orders can be foolish, and the mind follows them.** The first orders put "carrying shell: drop shell" first; a fresh catch is shell *and* pale flesh, so every catch went straight back on the sand, and the stranger slowly starved. Order matters, as it does for any written routine.
- **The living island's shellfish bed can't feed a person on its own.** 60 kg growing back 5% a day gives a few hundred grams of flesh a day, against about 2.5 kg a person needs. Once stripped, searches mostly found nothing. The companion's island has a long rocky shore instead (400 kg, up to 600 kg), a choice recorded in its data.
- **A needed tool is matched by its design's name, not its look.** Fish need a spear, and only something assembled *as* a spear counts; a barb joined to a shaft by someone who doesn't know the word wouldn't, though it's the same thing. That goes against "things are recognised by their look". Noted for the stages where the stranger makes or trades tools.

**Simplified:** a mind has no temperament yet (stage 2); free minds, which plan their own way, come later; a mind decides only among what its orders, its body, and its memory offer, and remembers places only as it last saw them.
