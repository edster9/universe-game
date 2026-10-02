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

**Corrected by the owner, 2026-10-01: the AI is a translator, not an adviser.** You don't ask it how to make fire. **You describe the process as you know it, in your own words**, for example:

> find something small that burns first, find something we can rub together, dig a hole or build a ring of rocks, put things in there, rub things together, and make a fire

**It turns that into the literal commands, within the scope of what your character has:**
- the commands that exist;
- what your character knows and has words for;
- what they carry;
- what they can see.

Whatever falls outside that scope, it says so and doesn't invent. "Use a cigarette lighter to light grass and twigs" comes back as: grass and twigs, yes, but no idea what a cigarette lighter is.

1. **You describe a process**, in plain words, by typing or speaking.
2. **The translator reads what's in scope:**
   - the commands (`help`);
   - the character's words for things;
   - what they know how to make (designs, recipes);
   - what they carry (`backpack`);
   - what they see (`look`);
   - their saved skills.
   
   It's shown none of the engine's hidden truth.
3. **It maps each part of your description to commands:**
   - "something small that burns" becomes the dry grass in sight;
   - "a ring of rocks" becomes gathering five stones and assembling a fire ring;
   - "rub things together" becomes `rub wood against wood into ring`.
   
   The process is yours. The translator fills in only the literal detail of the steps you described, and only with what's in scope.
4. **It reports anything it can't map:** a word the character doesn't know, a tool they don't have, or a step with no command.
5. **You see the commands** and run them, change them, or drop them.
6. **The engine runs them** through the laws, as it does anything typed. They can fail.
7. **Save it as a skill**, good or bad.

**Your knowledge only goes as far as the world does.** You may know how to build a rocket, but until the island has what it takes (fuels, a combustion chamber, made one step at a time), the translator can't map "build a rocket" to anything. Each step up the ladder, once made and named, becomes something the next description can use. That's how the climb happens: "build a rocket" is a skill made of skills such as "build a combustion chamber".

The translator helps throughout the game: wherever your words and the literal command set don't meet, it bridges them.

## Layers (the owner, 2026-10-01)

What a player says or types goes up through layers, and stops at the first that understands it. The cloud is the last resort:

1. **The console's own parser.** "go forest" or "gather sticks x3" is understood as it is, with no model and no cost.
2. **Built-in translation, ours and local.** It takes a stab at what the parser can't take literally:
   - saved skills by name ("make fire");
   - other ways of saying a command ("pick up", "grab", "collect");
   - spoken forms (the voice shaping already built: "times three", "ten minutes");
   - plans that worked before. Each cloud translation that worked can be kept and reused when the same thing is said in the same situation, so this layer grows with play.
3. **The cloud translator** (Claude on Bedrock or Anthropic's API), for descriptions nothing below could map.

Each layer passes on only what it can't fully map, and says which layer answered, so we can measure how often the cloud is needed. That number is what the costs in [research/translator-costs.md](../research/translator-costs.md) rest on.

## Why it matters beyond the fire (the owner, 2026-10-01)

- **A selling point.** "A truly AI-driven human interface to a universe": you describe what you want to do in your own words, and the world's laws decide what happens. The owner knows of nothing like it, and counts it as one of the game's first marketability factors.
- **How it could be paid for:**
  - **Free credits for demo players:** a set amount to try it.
  - **A tracked budget:** translations, in tokens, counted per day and per month for each player.
  - **Paid plans get a much bigger slice**, as part of the subscription goes to the player's AI allowance.
  - **Bring your own key:** a player can give the app their own API key, and it's used behind the scenes for them, outside the game's budget.
  
  Numbers are in [research/translator-costs.md](../research/translator-costs.md).
- **The same bridge, for code.** As the game advances, things will run code: the gold tester's script, the "money validator" in [code-and-laws.md](code-and-laws.md), and later [computers inside the game](in-game-computer.md). A sphere needs no code; a rocket's guidance computer does. A player could describe what a program should do and have AI write it, in the in-game computer's own language and only with the sensors and devices actually wired to it. It's the same rule as the translator: AI proposes, the engine decides, and nothing outside the device's scope can be used.
- **One step at a time.** First, prove the translator on the fire.

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

## Answered by the owner (2026-10-01)

- **Your knowledge or your character's?** Yours, as the process you describe. But it can only be carried out with your character's: their words, their designs, their things, and the world's commands. What's out of scope is reported, not made up.
- **Still open:** whether a saved skill shows its commands before running, every time or only the first; and who pays for the translator in multiplayer.
