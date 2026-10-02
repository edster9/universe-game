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

**Changed 2026-10-01** (skill grounds, step 4, in [challenges/skill-grounds.md](../challenges/skill-grounds.md)): an open fire works, slower to catch; everything piled in, then rubbed, catches; an ember tipped into tinder teased fine catches (onto a whole tuft it still dies, as it would); fed all at once, it climbs to the sticks; and "wait until <thing> is burning" shows when each size has caught.

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

## First trial (2026-10-01)

Through Amazon Bedrock on the owner's account, with `prototypes/translator/translate.py`. The islander had been to the forest and the hillside and stood in the forest. The owner's description:

> find something small that could burn first, find something we can rub together, dig a hole or build a rock circle, put things in there, and go ahead and rub things together and make a fire

| Model | Time | Plan | In the game |
| --- | --- | --- | --- |
| **Opus 5.5** | 10 s, 2,800 tokens in, 780 out (about 3¢) | Right: grass, two sticks, five remembered stones from the hillside, the ring, grass in it, rub into it; "dig a hole" reported as out of scope (no tool) | **A fire.** Told "put the fire ring down first", it repaired itself in 5 s (drop the ring, rub), and the grass caught |
| **Sonnet 5.5** | 3 to 4 s, about 1¢ | Right on one run, but missed walking up to the stones; on two other runs it stopped after gathering, calling the trip to the hillside "not asked for" | One run was stopped by the stones being out of reach (before repairs existed); the others did only the gathering |
| **Haiku 4.5** | 4 to 5 s, under half a cent | Muddled: invented rubbing flint against stone, wrote commands the game doesn't take, gathered what wasn't needed, and its repairs went round in circles | No fire |

**What the trial found in our own game:**
- **Twelve commands were missing from `help`**, so no translator could know them: gather, eat, drink, fill, explore, attack, read, survey, offer, ask, butcher, divide.
- **`rub`'s help said it only makes parts finer.** It now says the work turns to heat, wearing off hot dust that can smoulder into an ember.
- **"go to the hillside" didn't go there:** only "go hillside" did. It does now.
- **Rubbing into a fire ring still in your hands said "you don't see fire ring here".** It now says "put the fire ring down first". Both are proved by `data/scripts/words-for-places-and-fires.txt`.
- **The scope needed memory:** what was seen at places the character knows, or no plan could fetch stones from the hillside.

**What it says about the approach:** there's real hope. The strongest model turned a plain description into a working fire, reported the one part outside the character's world, and fixed its own mistake from the game's refusal. The instructions matter a great deal: two changes to them took Sonnet from one mapped step to a whole plan, though not every time. The help text matters just as much: a translator knows only what the help tells it.

**Next:** many more descriptions (the cigarette lighter, the variations, places not yet seen), several runs each, to measure how often each model gets it right, and tune the instructions until a cheaper model is good enough.

## The test bench (2026-10-01)

`prototypes/translator/bench.py` plays six plain descriptions several times on each model, and scores each run from what happens in the game:
- **the owner's fire:** is something burning;
- **a cigarette lighter:** reported as out of scope, and never put in a command;
- **grab three sticks:** three carried;
- **stones never seen:** no trip to a place the character doesn't know has stones, and the stones reported;
- **a fire, fed:** wood burning a minute after the plan ends;
- **put it all down:** the backpack empty.

The owner chose to drop Grok and Opus for their turnaround (Grok 4.7 took 27 s direct and 145 s through Bedrock; Opus 10 to 13 s) and focus on Sonnet 5.5, with Haiku 4.5 as the cheap comparison. Three runs of each, straight from Anthropic, about 5 seconds a plan:

| | Owner's fire | Lighter | Three sticks | Stones unseen | Fire, fed | Put down | Total |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Sonnet 5.5 (best run) | 3/3 | 3/3 | 3/3 | 3/3 | 0/3 | 3/3 | 15/18 |
| Haiku 4.5 (best run) | 0/3 | 3/3 | 3/3 | 2/3 | 0/3 | 3/3 | 11/18 |

**The simple cases pass every time on both:** the lighter is always reported, three sticks are gathered, everything is put down, and stones never seen are never fetched. **Fires are where they fail**, and nearly every failure traced back to our game, not the model's understanding:
- **"Once the grass is burning" couldn't be said.** Added: `wait until <thing> is burning`. It waits only for something by that name lying here, not already alight, and not carried, and counts a container as burning when something in it is ("wait until the fire ring is burning"). It gives up after three minutes. `data/scripts/wait-until-burning.txt` proves it: the twigs catch 14 s after going on, and the sticks 1 min 47 s after theirs.
- **The help didn't say a fire is made on the ground,** so the ring was rubbed into while still held, and the repairs were spent on that. It does now.
- **Sizes have no words.** Twigs and sticks are both "lump of wood" to the islander, so "put the twigs on" comes out as "put wood in fire ring", which can put the sticks on too early, and "wait until twigs is burning" names nothing. "smallest" and "largest" exist, and the scope now says so, but the real fix is the vocabulary conversation owed on words for sizes (twig, stick, log).
- **Now and then Sonnet's reply isn't a valid plan,** twice in a row on one description, though it worked when tried alone. The bench now keeps the reply to find out why.

**Where it stands:** viable for simple and medium descriptions with Sonnet at about 5 seconds and a cent each. Multi-stage processes like a fed fire need the game's words to catch up: words for sizes, and perhaps "feed the fire" as a saved skill. The owner, 2026-10-01: we're in the beginning phases of fine-tuning this.
