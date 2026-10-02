# What the translator might cost

Worked out 2026-10-01, at the owner's request, for the translator in [ideas/ai-and-skills.md](../ideas/ai-and-skills.md): a player's plain description turned into the game's commands. **Projections, not measurements.** The models' quality at this task is untested; the first exercise will measure it.

## Prices

Anthropic's list prices per million tokens (from Anthropic's model table, 2026-09-25). Bedrock's on-demand prices for Claude have matched these in the past, but Bedrock sets its own: **check the Bedrock pricing page before relying on them.** Bedrock may also charge more for a single region than for global routing.

| Model | Input | Output | Cached input (read) |
| --- | --- | --- | --- |
| Claude Haiku 4.5 | $1 | $5 | $0.10 |
| Claude Sonnet 5.5 | $2 | $10 | $0.20 |
| Claude Opus 5.5 | $4 | $20 | $0.20 |
| Claude Fable 5.1 | $10 | $50 | $0.25 |

## One translation

Measured on the companion's island: the instructions are about 450 tokens and the character's scope about 1,050. Assumed as the game grows:
- **3,000 tokens in.** About 2,200 of them change rarely: the instructions, the commands, the character's words and ways of making things. Those can be cached, and read back at a tenth of the price.
- **400 tokens out:** the commands and anything that couldn't be mapped.
- **400 more tokens out** where the model thinks first. Haiku doesn't.

| Model | Without caching | With caching |
| --- | --- | --- |
| Haiku 4.5 | $0.0050 | **$0.0030** |
| Sonnet 5.5 | $0.0100 (with thinking $0.0140) | **$0.0060** (with thinking $0.0100) |
| Opus 5.5 | $0.0200 (with thinking $0.0280) | **$0.0117** (with thinking $0.0197) |
| Fable 5.1 | $0.0500 | about $0.034 |

Caching costs a little extra when the cache is first written, and a cache lasts five minutes between uses. The instructions and commands are the same for every player, so they stay warm in a busy game.

## A player's month

How often a player describes something new matters most. **Running a saved skill costs nothing** (no model call), and **typed commands the console already understands** ("gather sticks") don't need the translator either. Only new descriptions do.

| Player | Translations an hour | Hours a month | A month |
| --- | --- | --- | --- |
| Light | 5 | 20 | 100 |
| Typical | 15 | 30 | 450 |
| Heavy | 40 | 60 | 2,400 |

Cost per player per month, with caching:

| | Haiku 4.5 | Sonnet 5.5 | Opus 5.5 |
| --- | --- | --- | --- |
| Light | $0.30 | $0.60 | $1.17 |
| Typical | $1.35 | $2.70 | $5.27 |
| Heavy | $7.20 | $14.40 | $28.08 |

(Sonnet with thinking is about 1.7 times its row; Opus with thinking about 1.7 times its row.)

## A game's month

Every player typical (450 translations a month), with caching:

| Players | Haiku 4.5 | Sonnet 5.5 | Opus 5.5 |
| --- | --- | --- | --- |
| 1,000 | $1,350 | $2,700 | $5,270 |
| 10,000 | $13,500 | $27,000 | $52,700 |
| 100,000 | $135,000 | $270,000 | $527,000 |

Against a $15 monthly subscription, a typical player's translations take about 9% of it on Haiku, 18% on Sonnet, and 35% on Opus. A heavy player on Haiku takes about half of it.

## What brings it down

1. **Saved skills.** Translate once, replay free. The more the game rewards keeping and trading skills, the fewer translations. This is the biggest lever, and it's already the design.
2. **The console first.** Lines the console already understands never reach a model. Only what it can't parse goes to the translator.
3. **Caching.** Already counted above.
4. **The smallest model that's good enough, measured.** If Haiku translates as well as Sonnet on our tests, it costs half as much. A step up to a bigger model only when the smaller one's plan fails is possible, but measure the simple choice first.
5. **Allowances.** A number of translations a day in the subscription, with more for heavy players or a higher tier.
6. **A local model** for offline single-player, at no cost per use, if one proves good enough.

## Next

Run the same set of descriptions through Haiku, Sonnet, and Opus, and compare:
- whether the commands are right;
- what each says can't be done;
- what happens when the game plays them;
- how long each takes, and the tokens it used.

Then choose the cheapest model that's good enough, and replace the projections above with measurements.
