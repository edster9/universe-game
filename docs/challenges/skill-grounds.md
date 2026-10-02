# Skill grounds: a sandbox for training skills

Proposed 2026-10-01. Not decided. The owner's words are "A sandbox for skills" in [requirements.md](../requirements.md).

**Why:** the translator works for simple descriptions and fails on multi-stage processes like a fed fire (see [ideas/ai-and-skills.md](../ideas/ai-and-skills.md)). Before any camp, community, or village, we stay in the skill-training zone and set the board for much more robust exercises: playable in real time, at x1, using the AI translator without speeding the clock up.

## The board

A new world, `data/skill-grounds.toml`, using the island library (`data/island-things.toml`) and the islanders' culture. The companion's island stays as it is, with its scripts.

The player starts in **a clearing**. Five grounds lie around it, each **about 30 m away: half a minute's walk**. Each ground has everything its skill needs close together, within a few steps of each other.

```
                 the woodland
                 (trees, boars)
                       |
  the knapping ground --- the clearing --- the fibre grove
  (flint, stones, sticks)  (start)        (bushes, sticks)
                    /             \
           the hearth            the stream bank
  (grass, twigs, sticks, logs,   (water, fish, clay,
   stones)                        bog iron)
```

All grounds are reachable from the clearing and from their neighbours. Gathering takes seconds there, as on the companion's island now. Long processes get playable times in this world's data (the furnace, firing clay), so the whole ladder can be played in real time in an evening.

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

## Steps

1. **The board:** the world file, the five grounds with their materials, playable times, and words for sizes.
2. **Fire, done properly:**
   - an open fire works but loses heat faster; a ring or a dug pit holds it in (the owner's rule);
   - a fire laid first and lit after catches;
   - an ember tipped into tinder catches;
   - logs are the last fuel.
3. **The scripts and bench descriptions,** one skill at a time, fixing what each finds.
4. **Saved skills:** "remember that as making fire", and replaying it with a kit check.
5. **The translator inside the client,** layered: the parser, our own translation, then the cloud.
