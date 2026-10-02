# Saying what you want: an AI that writes commands, and skills you keep

Proposed 2026-10-01. Not decided. The owner's words are "The fire, and a fork in the road" in [requirements.md](../requirements.md). It builds on [skills-and-interface.md](skills-and-interface.md), which said the design shouldn't depend on an AI; the owner now proposes leaning on one. Also see [skill-learning-paradigm.md](skill-learning-paradigm.md) (decided: a skill is a recipe in the character's words plus practice) and standing rule 6, "AI proposes, the engine decides".

## What the fire showed

Eleven variations, played against the laws on 2026-10-01:

| Tried | Result |
| --- | --- |
| As scripted, by day instead of night | Fire |
| No ring: grass on the ground | Impossible: fire has to be in something |
| A ring, no grass | No fire: an ember can't light a twig |
| One tuft, never fed | Out within ten minutes |
| Grass straight onto sticks, no twigs | No fire |
| Everything piled in, then rubbed | No fire: the ember's heat spread too thin |
| Rubbed for only ten seconds | No fire |
| The ember first, then the grass | No fire: the ember cools first |
| The script without its short waits | Out: each size has to be burning before the next |
| No sticks to finish | Out within ten minutes |
| Four stones | No ring: its design takes five |

The engine is a physics sandbox, not a recipe book: any sequence that gets the physics right works. Most failures are true to life. Four are not, and should change:
- an open fire should be possible;
- a fire laid first and lit after should work;
- an ember tipped into tinder should catch;
- feeding a fire shouldn't depend on timing nobody can see.

## The direction

**You say what you want; an AI turns it into the literal commands; the engine decides.**

1. **You ask, in your own words:** "make a fire here, with a ring of stones".
2. **The AI is shown only what your character could know** (no oracles):
   - what they see (`look`), what they carry, and what they know how to do;
   - their words for things;
   - the commands that exist (`help`);
   - their saved skills.
   
   It's never shown the engine's truth: no ids, no hidden numbers, no catalogue of designs.
3. **It writes a plan as ordinary console commands,** using the queue already built (`go to sticks; gather sticks x2; …`). It brings your own knowledge, as a player would: you know how fire is made, so it does too.
4. **You see the plan** and run it, change it, or drop it. The AI doesn't decide for you.
5. **The engine runs it, step by step, through the laws.** Gathering takes time, materials are real, and the fire can fail.
6. **If a step is refused, the AI can read the refusal and propose a fix**, shown to you the same way.
7. **Save it as a skill:** "remember that as making fire". It's kept whether it worked or not.

## Skills

- **A skill is a saved plan, in the character's words:** its steps, what it needs (a kit), and conditions such as "wait until it's burning" in place of fixed waits.
- **Running a skill needs no AI.** "Make fire" checks the kit is there, then plays the steps. This keeps it cheap, instant, the same every time, and fair in multiplayer. The AI is only for writing a skill or fixing one.
- **Good and bad skills.** The engine measures how a skill does each time it runs: whether it worked, how long it took, and what it used. A better skill is one that works more often, faster, or with less. Eleven ways to make fire are eleven skills.
- **Skills can be traded and taught.** A skill is knowledge: it can be given, sold, or taught to another character, in that character's words. (The decided paradigm adds practice: doing it often makes it faster and surer. That comes later.)

## What it needs, in order

1. **Fix what the fire showed:**
   - an open fire works but loses heat faster; a ring or a dug pit shelters it (the owner's rule);
   - a fire laid and then lit catches;
   - an ember tipped into tinder catches;
   - `wait until <thing> is burning`, so no plan depends on invisible timing.
2. **Skills without AI:**
   - "remember that as …" saves the last run of commands;
   - "make fire" checks the kit and plays them.
   
   Typing a plan by hand works from day one, and this is the shortcut queue grown up.
3. **The AI writer** (Bedrock, the standing AWS choice, running Claude; or the same model's API directly in single-player). It reads what the character perceives and the commands, and writes a plan. It works in the console first, and the client's console gets it for free, as does voice: what's spoken is just text.
4. **Measuring skills, teaching and trading them.**

## Questions for the owner

- **Should the AI know what you know, or what your character knows?** The owner's lean: yours ("you're applying your own skills"). The world still limits it, because only what your character can see, carry, and name can be used. A Stone Age islander asked for a rocket engine gets a plan the engine refuses at the first step.
- **Should a plan be shown before it runs, always,** or only the first time, with a saved skill running straight away?
- **Cost:** each written plan is one call to a model, perhaps a few with fixes. Replaying saved skills costs nothing. Who pays in multiplayer is for later.
