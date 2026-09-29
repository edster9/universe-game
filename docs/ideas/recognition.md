# Recognition: knowing what things are

Recorded 2026-09-28. A direction, not a decision. The requirement is "Knowing what things are" in [requirements.md](../requirements.md). It extends [knowledge.md](knowledge.md) and [skills-and-interface.md](skills-and-interface.md).

## The idea

Knowing *how* to do something is a skill. Knowing *what* something is, is knowledge too. A character who has never seen a coconut doesn't see "a coconut": they see a hard, round, brown thing about the size of a head, which might be food. An early human who sees an aeroplane overhead sees a large, loud thing crossing the sky. Someone who finds an abandoned bow and arrows sees a bent stick with a cord and some thin straight sticks.

You recognise a thing because you've seen it before, handled it, built it yourself, been taught about it, or read about it.

## Why it matters

- **Getting anywhere depends on it.** A stranded person with no knowledge might work out how to fish, but will never get off the island, at least not in their generation. Someone who can tell them what fire is, or that iron comes out of a certain rock, changes everything. Starting knowledge and learning are what move a person, and a people, up the ladder.
- **Trade has no honour system.** Most games trust the shop: ask for a gun, get a gun. Here, if you don't know what a gun is, the seller can hand you a stick, call it a gun, and take your money. The same goes for a table, a medicine, or a chip. You can only check what you can recognise, or measure with instruments you know how to use. This ties straight into the money validator in [code-and-laws.md](code-and-laws.md): recognising a real coin is knowledge, and a better validator is better knowledge built into a device.
- **Rumour and lies become possible.** Being told what something is can be wrong, by mistake or on purpose.

## What a character perceives

The engine always knows the truth: every piece's materials, shape, and datasheet. What a character is *shown* is that truth filtered by what they know.

| They know… | They see |
| --- | --- |
| Nothing about it | Its look: size, weight in the hand, rough shape, whether it seems like metal, wood, stone, or flesh, whether it's hot. "A heavy dark metal thing with an edge, about a forearm long." |
| The kind of thing | Its name: "an axe". |
| The kind, and the craft behind it | Its quality: a smith sees the edge width and the hardness; a novice sees "an axe". |

The same filter applies to commands. You can't ask for "the coconut" if you don't know what a coconut is; you'd ask for "the round brown thing".

## Where recognition comes from

- **Starting knowledge.** A character's background: a stone-age islander knows fire, fishing, and the local plants; a space-age engineer knows circuits, and may never have made fire from sticks. A blank mind knows nothing, and is rare because it would have to be taught everything.
- **Building it yourself.** Make an axe and you know what an axe is.
- **Seeing and handling it.** Enough time with something makes it familiar.
- **Being taught or told**, which can be wrong.
- **Reading**, in a library or from a manual.

## How it fits what's built

- **It's "no oracles", applied to people.** Programs only know what their sensors measure. People only know what their senses show them and what they've learned. Today every character sees every thing's true name; that is the one oracle the engine still has for people.
- **Datasheets stay the truth.** The `datasheet` command is a testing tool that shows the engine's view. A character's view of a datasheet would be filtered by their knowledge.
- **The interpreter speaks the character's language.** A spoken request uses the character's own vocabulary, which grows with what they recognise.

## Keeping it simple

Under the standing rule: to start, recognition is simply knowing a *kind* (a material, a shape, or a design) or not. An unrecognised thing is described from what anyone can observe. That needs one small piece of data: what a material looks like to the untrained eye (metal, wood, stone, flesh, fibre). Degrees of familiarity and craft-level insight can come later if a challenge needs them.

## When

With knowledge, in slice 4 (see [slices.md](../slices.md)). The stranded challenge assumes the survivor already knows everything it uses.
