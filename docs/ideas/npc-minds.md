# Scoped minds

Claude's take, 2026-09-30. **Decided the same day**: see the end. The owner's words are in "Scoped NPC minds" in [requirements.md](../requirements.md). This builds on the minds in [the-road-ahead.md](the-road-ahead.md) (instinct plus standing orders) and [village.md](village.md).

## The idea

Every mind has a **scope**: what it's allowed to want. A shopkeeper's scope is the shop. A player's is everything. The owner's test case is the right one: **the shop burns down; what does the shopkeeper do now?** The answer is set by the scope, not improvised by the engine.

## Two settings on every mind

The owner named two things: what a mind intends to do, and its survival skills. They're independent, so I'd make them two settings in data:

**1. Scope: how far it thinks.** A ladder, cheapest first:

| Scope | What it does | When its orders run out | Example |
| --- | --- | --- | --- |
| **Instinct** | The body only: eat, drink, sleep, flee, fight | Rests | A boar |
| **Confined** | Its standing orders, nothing else | Stays put and waits | The shopkeeper who stands in the ashes of the shop |
| **Resident** | Its orders, plus looking after itself | Finds food, shelter, and sleep for itself, and takes up another job it knows | A villager who, the shop gone, goes fishing |
| **Free** | Goals, not orders: works out its own way from the recipes it knows, as a player would | Picks a new goal | A wandering trader; a rival; a character who could pass for a player |

**2. Temperament: how it survives.** When threatened: fight, defend (fight only when cornered or struck, as the boars do now), flee, or give in. Instinct already has fear and a charge when cornered; this makes the choice a setting, for people as well as animals.

So the owner's shopkeeper is **confined** with a **defend** temperament: attacked, it fights back; the shop destroyed, it stands there. Make it **resident** and it goes home and finds other work. The same person, one word different in data.

Instinct stays underneath every scope. Even a confined shopkeeper eats, sleeps, and flinches from a blade, because those are the body's, not the job's. (Where hunger isn't the realistic kind, as under players' rules, that layer simply has less to do.)

## What each costs

- **Instinct, confined, and resident** are rule lookups: millionths of a second a decision, and a mind decides only when its action ends or something happens to it. Thousands are affordable.
- **Free** minds plan: "I want food; I know a way to fish; for that I need a spear; for that, a shaft and a flake..." That's a search through the recipes the character knows, so it costs more, perhaps thousandths of a second a plan. Still fine for dozens, and they re-plan only when a plan fails or a goal is met. Not for thousands.
- **An AI model** can sit on top of any scope for talking, and on top of a free mind for choosing goals. Seconds and money a call, so rarely, and it still only proposes commands.

So a village is mostly confined and resident people, with a few free ones, which is also what makes a place feel alive.

## Why this fits what's already decided

- **A player who's away is a person under a scope** ([time-away.md](time-away.md)): "keep a low profile" is confined with flee; "guard my camp" is confined with defend. The same settings, chosen by the player.
- **Laws, not things.** The engine knows only the ladder and the temperaments. "Shopkeeper", "guard", and "trader" are orders in data.
- **No oracles.** A mind at any scope knows only what its person perceives and remembers. A free mind is no smarter about the world than a player; it just never logs off.
- **AI proposes, the engine decides.** Every scope ends in commands, checked by the same laws.
- **Indistinguishable from players**, as the owner wants of the most intelligent: a free mind uses exactly a player's commands, and the world can't tell the difference.

## Later, not now

- **Changing scope as a story event:** a confined shopkeeper who, after losing everything, becomes free. It's just changing a setting, but deciding *when* is a story question.
- **Learning:** whether a resident villager's list of jobs grows with practice belongs with the [skill learning paradigm](skill-learning-paradigm.md).

## Questions for the owner

1. **Two settings, scope and temperament:** agree?
2. **Four scopes (instinct, confined, resident, free):** the right ladder, or would you split or merge any?
3. **Free minds plan from the recipes they know**, with AI only helping them choose goals and talk: agree?

## Decided, 2026-09-30

The owner: "go with Claude's best recommendations".

- **Two settings on every mind:** scope (instinct, confined, resident, free) and temperament (fight, defend, flee, give in). Instinct stays underneath every scope.
- **Free minds plan from the recipes they know**; an AI model, if used, helps with talking and choosing goals, rarely, and only proposes commands.
- **Free minds cast a cone of influence**, as players do: they can make choices that matter, even, as the owner put it, blowing up a planet ([cone-of-influence.md](cone-of-influence.md)).
- Built from step 2 of [the-road-ahead.md](the-road-ahead.md): the companion is the first person with a scope other than instinct.
