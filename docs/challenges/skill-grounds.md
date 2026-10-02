# Skill grounds: a sandbox for training skills

Proposed 2026-10-01; **agreed the same day** ("exactly": the seven skills, the five grounds about 30 m apart, twig, stick, and log). The owner's words are "A sandbox for skills" in [requirements.md](../requirements.md).

**Why:** the translator works for simple descriptions and fails on multi-stage processes like a fed fire (see [ideas/ai-and-skills.md](../ideas/ai-and-skills.md)). Before any camp, community, or village, we stay in the skill-training zone and set the board for much more robust exercises: playable in real time, at x1, using the AI translator without speeding the clock up.

## The board

**Relaid 2026-10-02**, at the owner's word ("everything we need, all in a circle around the actor"): the first board had each ground as its own place, so from the start you saw nothing of them, and the game still opened on the companion's island.

`data/skill-grounds.toml`, now **the world the client starts in**, is **one open place**. The player starts in the middle, a clearing; **six zones lie in a ring about 25 m out**, twenty seconds' walk, each with a landmark (a big flat rock with the zone's name, standing just behind it) and **everything its own skills need, within reach of its middle**. Where two skills need the same thing, each zone has its own pile, so nothing is fetched from another zone.

```
                         the woodland
              trees, roots, flint, stones, sticks, grass,
              twigs, bushes        (the deep woods beyond: boars)
   the knapping ground                          the fibre grove
   flint, stones, sticks                        bushes, sticks, deadwood
                          the clearing
                            (start)
   the hearth                                   the stream bank
   grass, twigs, sticks,                        a pool, fish, mussels, roots,
   deadwood, stones                             flint, stones, sticks
                           the forge
         bog iron, clay, stones, flint, grass, twigs, sticks, deadwood
```

| Zone | For | Its kit |
| --- | --- | --- |
| The hearth | fire | dry grass, dry twigs, fallen sticks, deadwood (logs), loose stones |
| The forge | a knife (an axe head) from ore | bog iron, clay, stones (furnace, whetstone), flint (a flake to cut moulds and handles), and a fire's makings |
| The knapping ground | stone tools: a spear, a stone knife | flint, stones, sticks |
| The fibre grove | rope and shelter | fibrous bushes, sticks, deadwood (poles) |
| The stream bank | water and food | a pool of the stream, fish (for a spear), mussels, roots and nuts, and flint, stones, sticks for the spear |
| The woodland | felling, hunting, hide | standing trees (for an axe), roots and nuts, and flint, stones, sticks, grass, twigs, bushes for a spear, a fire to dry a hide, and rope for shoes |

**The boars live in the deep woods**, a second place beyond the woodland, with their own spring, roots, and trees: in one place with nowhere to run, a boar that flees people is cornered and charges, which it did, at once. They come out to the grounds, and hunting means going in.

**Found and fixed while laying it out:**
- **Names mean the nearest of things alike:** with dry grass in three zones, "go to grass" goes to the nearest patch (only among things on the ground; what's carried keeps the precedence each command gives it, or the castaway in `where-4-climb.txt` drank his pots dry at the stream and died on the summit).
- **Gathering from something just out of reach says so** ("Out of reach: the roots and nuts, 3.1 m south-west"), instead of "you don't see roots" when something of that name was in hand.

**What it leaves:**
- **Approaching a zone from a neighbouring zone** stops you on that side of its landmark, and a far patch can be a step out of reach (the refusal says how far and which way). From the clearing ("walk to 0 0", then "go to the forge") everything is in reach.
- **The islanders don't know metalworking:** no furnace, mould, axe, or iron in their culture. So the forge's kit is all there, but "assemble furnace" is refused, and the trees need "a part put together". How the player comes to know it is the first question of step 5.
- `data/scripts/skill-grounds-0-the-board.txt` walks to each zone from the clearing and gathers and uses its kit (a spear at the knapping ground, rope at the grove, a fire at the hearth). Switching "the nearest is meant" off fails it.

## The skills

**With fire** (the owner's goals):

1. **Make fire** (the hearth). Tinder, kindling and fuel, in a ring of stones or a dug pit, from rubbing sticks, fed until logs burn.
2. **Make a knife** (stream bank, hearth, knapping ground):
   - dig bog iron and clay at the stream bank;
   - build a furnace from stones and clay, and fire a clay mould;
   - light the furnace from the hearth's fire, smelt, and cast a blade;
   - give it a handle;
   - sharpen it on a stone.
   
   A sword is the same, longer.
3. **Make an axe and fell a tree** (all grounds). The knife's chain with an axe head and a haft, then chopping a tree in the woodland into logs, which feed the next fire.

**Without fire** (suggested):

4. **Stone tools** (the knapping ground): knap a flint into a flake, shape a shaft from a stick, and bind them into a spear or a stone knife. That's the first edge, and the first tool that makes other tools. The spear's recipe exists already.
5. **Rope and shelter** (the fibre grove):
   - pull fibre from bushes and twist it into rope;
   - lash poles into a lean-to, or a raised cache that keeps food from boars.
   
   Rope is what binds every later tool.
6. **Water and food** (the stream bank): drink, spear a fish, gather shellfish, and forage roots and nuts. Gathering, chance, and the body's needs, for when players' rules are switched back on.
7. **Hunting and hide** (the woodland): spear a boar, butcher it with a flint edge, and make shoes from the hide. That's the first invention the islanders don't know, and it needs naming. These laws were built in "living with the island".

Every skill uses the others: the knife needs fire, the axe needs rope, and the fire is fed by the axe's logs. That's the ladder in miniature.

## Words for sizes (agreed 2026-10-01)

**Twig, stick, and log**, so a process can be said and translated without "smallest wood":
- **A word can mean a material within a range of sizes,** measured, never written: wood under 50 g is a twig, 50 g to 2 kg a stick, and over 2 kg a log.
- **The ranges live in the culture's data,** not the engine.
- **The most specific word wins:** a 20 g piece of wood is "a twig", while "wood" still names any of them.

This settles "words for sizes" from the conversations owed.

## How it's proved

- **A script per skill,** played by hand, as before.
- **The translator's bench** gets a description per skill, in plain words, three runs each on Sonnet 5.5, scored from the game.
- **Played in the client at x1,** with the translator inside it, as the target: the owner says what to do, and the islander does it.

## Shortcuts for development (the owner, 2026-10-01)

"We'll master things and save them as skills, and next time we start, we just execute our skills." To spend development time on what's being trained, not on getting back to it:

- **Saving and resuming.** `/save <name>` writes the whole world to a file, and `/load <name>` picks it up exactly where it was: the clock, every piece of matter, every mind. A session ends with a save, and the next starts from it. A fire already burning is just a save made beside one.
- **Making things on demand.** `/make <thing>` puts something from the world's data in front of you (a fire ring, a log, a pot), and `/make fire` a fire already burning in a ring. Later, `/make <kind>` makes a creature (a boar; a horse, once there are horses). It goes through the gate like everything else, with **the designer as a named source**, as sunlight is. Nothing comes from nowhere, and the world records what the designer gave.
- **Executing saved skills:** "make fire" plays a saved skill once its kit is checked (step 4 below).

Layers ([ideas/tools.md](../ideas/tools.md)): saving and loading change the whole world and its clock, so they're single player only. In multiplayer, the server keeps its own saves. Making things is the server's choice, like god mode.

## Steps

1. **Saving and resuming:** `/save`, `/load`, and a save on quit. Every step after this starts from a save instead of from the beginning. **Done** (2026-10-01): `/save`, `/load`, `/saves`, a save called "last" when the game ends, and `--load <name>` to start from one. Proved by `crates/console/tests/saves.rs`: a fire saved while burning, loaded, and fed comes out exactly as the fire never saved; leaving heat out of the save fails it.
2. **Making things on demand:** `/make <thing>`, and `/make fire`, with the designer as a named source. **Done** (2026-10-01): `/make` an amount of a material, shaped or not, a design, or a kit from data (`/make fire`, burning in a ring), and `/light`. Proved by `data/scripts/make-on-demand.txt`; the gate's audit catches the designer's matter left uncounted. Creatures wait. See "Shortcuts for development" in [ideas/tools.md](../ideas/tools.md).
3. **The board:** the world file, the five grounds with their materials, playable times, and words for sizes. **Done** (2026-10-01), **relaid 2026-10-02** as one place with six zones round the player (see "The board" above); the client now starts there. Proved by `data/scripts/skill-grounds-0-the-board.txt`. What the first version found:
   - **Twig, stick, and log** are in the islanders' culture (the library), so they apply on the companion's island too: a piece reads "wood stick", or "stick of wood and ash" once burning; gathering says "You find 200 g of wood, a wood stick."; `scope` gives each size word's range, for the translator. "wood" still names any piece of wood, without asking which. Scripts that expected "lump of wood" were updated. Switching the size words off fails five proofs.
   - **A bug in "wait until … is burning":** with a patch of dry grass in the same place as the fire, it watched the patch (which never burns) instead of the tuft in the ring. It now watches loose pieces first.
   - **A log needs a real fire under it:** one burning stick never lights a 3 kg log; five sticks light it in about two and a half minutes. That's right in spirit; step 4 looks at it again.
   - Times as played: 25 s between grounds, gathering 3 to 10 s a piece, a fire from nothing to a burning log in about 15 minutes.
   - Not yet: firing clay and the furnace at playable times, which the knife (step 5) will measure.
4. **Fire, done properly:**
   - an open fire works but loses heat faster; a ring or a dug pit holds it in (the owner's rule);
   - a fire laid first and lit after catches;
   - an ember tipped into tinder catches;
   - logs are the last fuel.

   **Done** (2026-10-01), proved by `data/scripts/skill-grounds-1-fire-variations.txt`:
   - **One change did most of it: a flame heats what it reaches, the finest first.** Before, a burning piece shared its heat with everything in the ring by surface, so a tiny ember among sticks gave the grass almost nothing. Now its flame reaches four times its own surface's worth of what's with it, finest first, skipping what's already alight. An ember nestles into the tinder; burning tinder reaches the twigs; burning sticks reach everything. With that, **a fire laid first and lit after catches**, and so does **a fire fed all at once**. **A log on a tuft alone stays unlit** while the tuft burns away; logs light from about five burning sticks. The reach is a number in data (`flame_reach`); twice too little or twice too much both fail, so it's measured, not guessed.
   - **An open fire works:** grass at your feet, rubbed over, catches (rubbed dust now falls at the worker's feet, and rubbing stops when the pile catches); twigs and sticks dropped on it catch in turn. Only a quarter of a flame's heat reaches the pile in the open, against half in a ring, so it takes about four and a half minutes of rubbing instead of one. No pit yet: digging a hole needs its own small step.
   - **An ember tipped into tinder catches when the tinder is fine.** A new verb, `tip` (or `pour`), moves loose pieces from one container to another, however hot. A fresh ember tipped onto a whole 5 g tuft warms it to about 470 K and dies, short of grass's 500 K; teased fine first (`divide grass` twice, 1.25 g), it catches in ten seconds. True to life, and no new law needed: it rewards skill.
   - **Fixed along the way:** the summit fire in `where-4-climb.txt` had never lit its wood; now it burns, and the castaway carries a lighter ring down (49 min, not 52). `stranded-2-everything-at-once.txt` proved the old failure, and now proves a laid fire works. An old test that "pour rock into mould" is refused now tips the rock out of the furnace; it checks a solid on the ground instead.
   - **Teeth:** sharing by whole surfaces again, no flame heat in the open, or heating what's already alight each fail a proof.
   - Waiting for each size to catch still matters for a fire built a piece at a time; "wait until <thing> is burning" makes that visible.
5. **The scripts and bench descriptions,** one skill at a time, fixing what each finds.
6. **Saved skills:** "remember that as making fire", and replaying it with a kit check.
7. **The translator inside the client,** layered: the parser, our own translation, then the cloud. The goal: the owner says what to do, at x1, and the islander does it.
