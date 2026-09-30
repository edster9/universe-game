# The road ahead: ready for the village?

Claude's take, 2026-09-30. **Decided the same day**: see the end. The owner asked, before any village work, whether we're ready: how many primitives and laws exist, whether a smaller community should come first, how the rules engine becomes a real-time 3D game that still takes prompts, how much processing NPC minds need, and whether the text world and the 3D world can coexist ("Are we ready for the village?" in [requirements.md](../requirements.md)). The village design itself is in [village.md](village.md); this is the road to it. Research behind it: [research/npcs-and-real-time.md](../research/npcs-and-real-time.md).

## Short answers

- **Ready for a whole village? No.** The physical world is well along; the social world doesn't exist yet. A village needs minds for people nobody plays, trade, ownership, buildings you walk into, and food grown at scale, and none of them exist. **Start small**, as the owner suggests: one companion, then a camp, then a hamlet, then the village.
- **Are NPC minds a burden? No, if they're simple.** We measured it: a boar's decision costs about a ten-millionth of a second. The cost is in the world's physics and in how the engine checks itself, not in minds. Only one kind of mind would be a burden: an AI language model deciding every move. So NPCs get simple minds, and AI, if used at all, only for rare moments such as conversation.
- **Real time and 3D:** the engine is already real time, in text. 3D is a way of *showing* it, not a different engine. The engine sends "who starts what, when, and when it ends"; the screen animates in between, 60 times a second. Every button, click, and typed prompt becomes the same command a script types today.
- **Will the two worlds coexist? Yes, by design.** One engine, one channel, many windows onto it: the scripts (our tests, forever), the console, Claude's live channel, a browser page, and later the 3D client, all at once, in the same world.
- **But three things in the engine won't scale**, and they come first, before any new people.

## What we have, and what a village needs

**Already built** (see [laws.md](../laws.md)): matter, heat, and fire; mining, smelting, casting, and forging; shaping, assembling, and datasheets; floating and paddling; hunger, thirst, sleep, cold, wounds, cooking, and spoiling; wild food that grows back from sunlight; kinds of creature with an instinct; wearing and clothing; shelter and barriers; words in minds, and peoples with their own words; memory and maps; players' rules; actions with a start and an end, several people acting at once, and a live channel. Places have positions, heights, and paths with a length and a roughness, so **a road is already just a smooth path**: a number in data.

**Not built, and a village needs it:**

| Missing | Why it matters |
| --- | --- |
| A mind for people nobody plays | Only animals have instinct. A person not played does nothing. |
| Trade, and whose things are whose | `give` exists; agreeing a swap, prices, and ownership don't. |
| Buildings you walk into | A shelter is something you sleep in, not a place with a door. |
| Working together on one job | One person, one action. Raising a house takes several. |
| Food grown on purpose | Wild food regrows; nobody plants or tends. |
| Room to grow | See the measurements below. |

## The measurements

On the living-with-the-island world, with a player standing idle and the boars left to themselves:

| Boars | 30 game days took | Memory used |
| --- | --- | --- |
| 6 | 14 s | 0.6 GB |
| 60 | 108 s | 5.2 GB |
| 600 | stopped: on course for about 50 GB | |

One game day with 60 boars (95 things in the world) took 2 seconds, in about 1,800 steps: mostly calm, one-minute steps. Where the time went:

- **Minds: small.** Deciding what all 60 boars do takes about 4.5 millionths of a second a round, about 75 billionths per boar. Deciding *and* carrying out their actions was about a fifth of the day's cost.
- **The gate: half the cost.** To prove nothing was created or destroyed, the gate weighs and counts the energy of the *whole world* before and after every set of changes. Fine for a hundred things; for a village of thousands of things, the work grows with the square of its size.
- **Memory: the first thing that would break.** The world keeps every change it has ever made. A month of 60 boars fills 5 GB. A server running for weeks would run out.
- **One clock for the whole world.** The world steps each second whenever anything anywhere is busy (a fire, a rub, someone's hard work), and each minute otherwise. A village smithy's fire burning all day would make every villager, boar, and stone be stepped every second: about 50 times the work.

In fairness: at 2 seconds a game day, the engine runs about 40,000 times faster than real time, so one village live in real time would probably keep up even today. It wouldn't keep up with a town of thousands, with many villages on one server, or with our tests, which run months in seconds. And memory would run out regardless.

## Minds: how much thinking, and when

Three levels, cheapest first:

1. **Instinct: the body.** Eat, drink, sleep, flee, fight. What boars have. About a ten-millionth of a second a decision.
2. **Standing orders: the routine** (proposed in [village.md](village.md)). "At dawn go to the shore and fish; when carrying fish, go to market." A rule lookup, about as cheap as instinct. It's also how a player's person behaves while they're away ([time-away.md](time-away.md)).
3. **An AI model: conversation and rare plans.** Seconds and real money per decision, so never for routine, and never every step. Like everything else, it only proposes commands.

**When a mind thinks matters more than how.** A mind should decide only when something changes for it: its action ends, it gets hungry, night falls, someone arrives, it's hurt. Between those moments, a villager at work costs nothing to think about. Instinct already works this way: a boar decides when it's free, and is then busy, or resting half an hour, until its action ends; a wound cuts that short. Standing orders will do the same.

**A player's mind costs the engine nothing: the player is the mind.** NPCs "at a player's level" would mean an AI model, the one that would be a burden. So NPCs get simpler minds, as the owner suspected.

**People nobody is watching live by the same laws**, only in bigger steps where nothing fast is happening. Many games keep two simulations, a detailed one near the player and a rough one elsewhere, and have to reconcile them when the player arrives. With one set of laws at different step sizes, nothing needs reconciling, and conservation stays honest.

## From text to real time to 3D

**The engine already works in real time.** Actions start, take their time, and end; the clock runs for everyone; `--real-time` ties it to the wall clock. What a 3D client adds is drawing.

**Two clocks.** The world steps once a second (or less often when calm); the screen draws 60 times a second. The engine tells the client "the islander walks from the gate to the well, from 08:00:00 to 08:00:40", and the client animates the walk. It never asks the engine where the islander is at each frame. This is how online games have long worked. It's also why actions with a start and an end, built in the real-time stage, were the right foundation: they're exactly what a renderer needs.

**Moving yourself in 3D** (keys or a gamepad) becomes a stream of short walks: "walk toward here". The client moves you at once on your screen, and the engine checks each step against the laws (your speed, your load, the ground) and corrects the client if it disagrees. You feel instant response; the engine still decides.

**Prompts in 3D.** Clicking a tree and choosing "cut" sends "cut the tree". A skill button sends its recipe's commands. A typed or spoken prompt goes through the interpreter to the same commands. The console and the 3D client are two windows onto one set of commands.

**What 3D needs from the engine** (none of it drawing):

- **Space within a place:** positions in metres, so someone can stand by the well, reach the shelf, or be met on the path. Reach already is a length.
- **Perhaps finer time for fast things:** a spear thrust takes half a second, and the clock counts whole seconds. Walking and working don't need it: EVE Online's server ticks once a second, RuneScape's every 0.6 s. Fights may.
- **Motion with a start and an end,** so a walker is on the path between places, not still at the start (a known follow-up).
- **Seeing from where you stand:** what's in view, not only what's in the same place.

**Coexistence.** Scripts stay the proving ground forever: every law is still proved in text, and a 3D build can't change what a script proves. The browser page grows in steps (panels of text, then a map of places with people moving along paths, then 3D), each reading the same live channel. The map step is a small rehearsal of the 3D client: it animates motion from start and end times, and it's where we'll find out whether the channel carries enough.

## What other games do

From [research/npcs-and-real-time.md](../research/npcs-and-real-time.md):

- **Minds are rarely the main cost.** In Dwarf Fortress, the costs its wiki names are units checking who can see whom (which grows with the square of the population), temperature, and item counts, not deciding. RimWorld's people re-think only when a job ends, with a quick emergency check every half second. **For us, perception is the cost to watch** once the village has many people.
- **Places nobody watches** are frozen (Minecraft), stepped in big strides (RimWorld moves distant people six game hours at a time), or replaced by rough "outcome" rules, which drift: X4 players report stations fight better when nobody's watching. Same laws, bigger steps, is the safe choice, and our gate lets us prove that a long step and many short ones agree, which none of the surveyed games described doing.
- **Real-time games are tick games underneath**, drawn smooth by the client: EVE Online at one tick a second, RuneScape every 0.6 s, Minecraft 20 a second. World of Warcraft's server sends monsters' moves as a path and a duration, which the client animates. **Our one-second clock is fast enough.**
- **Typing and clicking have coexisted since Ultima Online**, where you can say "bank" or use the menu. In The Sims, a player's order and a Sim's own choice run the same behaviour.

## The road, step by step

| Step | What | Proves |
| --- | --- | --- |
| **1. Room to grow** | The gate checks the changes themselves (each set must balance), not the whole world; a full recount stays as a test-time audit. Memory keeps recent history and snapshots, not everything. Creatures act at the end of what they do, like people. | A benchmark proof: a village's worth of bodies (60 people, 60 animals) for 30 days, in under a minute, with memory flat. Every existing proof unchanged. |
| **2. A companion** | The smallest community: the stranger stays on the living island as a realistic person, run by standing orders (fish, gather, keep the fire, sleep). Asking them to do something. Barter: the exchange law, and ownership in minds. **The first browser page.** | The companion lives 30 days on their own; meat is traded for fish; a bad offer is refused; a theft seen is remembered, one unseen isn't. Playwright screenshots. |
| **3. A camp** | Four to six people. A hut you walk into (a place inside a place). Several hands on one job. A shared store. Planting, or tending what grows. The browser page gains a map. | The camp stays fed for 30 days; the hut keeps the night's cold out; the store empties if nobody fishes. |
| **4. A hamlet** | Ten to fifteen people, each with a craft: the smith's chain run by villagers, a barter market, then coins as the thing everyone accepts. Roads as smooth paths. | Money goes round without being printed; a shelf stays empty until someone makes and brings more. |
| **5. The village** | The old slice 3: guard, tax, raids, and traders and migrants from beyond the map. | The old plan's tests. |

Then, as already decided: the rendering conversation, then space within places, then 3D.

Step 1 is engine work with nothing new to play. It comes first because every later step adds people, and each one makes the three problems worse. Local time (each place at its own pace) waits until the benchmark shows we need it: don't overcomplicate.

## Refined by the owner, 2026-09-30

Two follow-up conversations, both open: **scoped minds** ([npc-minds.md](npc-minds.md)), a ladder of how far an NPC thinks, and **the cone of influence** ([cone-of-influence.md](cone-of-influence.md)), which makes the world exact only where a player could know the difference, and becomes part of step 1.

## Questions for the owner

1. **Room to grow first,** before any new people: agree?
2. **The smallest community is the stranger staying on the island** as your companion, in the world we already have, so the far-folk's words pay off. Or would you rather start somewhere new?
3. **Simple minds:** instinct plus standing orders, deciding only when something changes, and AI kept for conversation later. Agree?
4. **The browser page at step 2, growing a map at step 3,** as the rehearsal for 3D: agree?

## Decided, 2026-09-30

The owner agreed with the direction ("I like where you're going with this") and asked to go with Claude's best recommendations on every open question:

1. **Room to grow comes first**, before any new people: the gate checks changes rather than the whole world; memory keeps recent history and snapshots; then places keep their own time by the cone of influence (exact, coarse, asleep). Creatures act at the end of what they do, like people. The proof: a benchmark of a village's worth of bodies for 30 days, fast and with flat memory, which also reports how much headroom the world has; and the same world watched and unwatched agreeing.
2. **The first community is the stranger staying on the island** as a companion, with scoped minds ([npc-minds.md](npc-minds.md)), barter, and ownership in minds.
3. **Simple minds** for most NPCs, deciding only when something changes; AI for conversation, later.
4. **The browser page at step 2, gaining a map at step 3**, as the rehearsal for 3D.

Then a camp, a hamlet, and the village ([village.md](village.md)); then the rendering conversation, space within places, and 3D.
