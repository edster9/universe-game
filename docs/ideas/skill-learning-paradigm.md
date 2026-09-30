# The skill learning paradigm

> **Decided 2026-09-29** (see "Decided" at the end): the ladder is the production chain; the finer rules of learning are settled in practice.

Recorded 2026-09-28 **for a major design conversation later.** Nothing here is decided. The requirement is "The skill learning paradigm" in [requirements.md](../requirements.md). It builds on [skills-and-interface.md](skills-and-interface.md), [recognition.md](recognition.md), and [knowledge.md](knowledge.md).

## The question

A character learns skills by talking to someone, going to a library, or working things out alone. But the *player*, sitting at the computer, knows things too. I know how to build a fire, and my starting character may not. So I tell it: find some dry grass, take a stick, start rubbing; you'll see smoke. It succeeds, and that becomes a skill.

**If I can teach my character to make fire from what's in my head, why can't I teach it to build a rocket to the Moon?** That isn't practical, and it would break the game.

And the opposite worry: **someone mustn't be able to download a script from the internet, paste it into the prompt, and have their character instantly know everything.**

## Directions to explore

**Skills are hierarchical.** You crawl before you walk, and walk before you run. A skill can only be taught if the character already has the skills beneath it. You can teach fire to someone who knows what sticks and grass are, but not a rocket to someone who has never smelted metal. What a player can pass from their own head to their character is limited by where the character already is on the ladder.

Some mechanisms that may already give much of this for free:

- **Recognition limits what can even be said.** The interpreter speaks the character's language ([recognition.md](recognition.md)). "Machine a turbopump from aluminium" means nothing to a character who has never seen aluminium or a turbopump. The words map to nothing they know, so the instruction can't be carried out. A pasted script full of unknown words fails the same way.
- **Every taught step is still performed under the laws.** Teaching isn't downloading. The character has to actually do each step, in game time, with real materials, and it can fail. A rocket can't be taught faster than it can be built, and it can't be built without every tool and material beneath it existing somewhere in the world.
- **Practice is earned, not pasted.** Knowing the steps (know-what) can be told or copied. Doing them well (know-how) comes only from doing them ([knowledge.md](knowledge.md)). A pasted procedure runs at a novice's speed and success rate.
- **Learning takes time and may have limits.** A character might only absorb so much new knowledge in a day, and more easily next to what they already know.

## Questions for the conversation

- How are skills arranged into a hierarchy? Is the hierarchy written in data, or does it emerge from the production chains (you can't learn casting without knowing fire, because casting needs fire)?
- What exactly can a player teach their character from their own head, and how does the engine decide the character understood?
- How is a player's teaching different from a teacher in the game, a book, or working it out alone?
- How do we stop pasted scripts, without punishing players who simply know a lot?
- Does learning cost anything: time, energy, money, a teacher's wages?
- Do skills fade without use?
- How does this work across many universes and generations: a people who never learned iron, or a universe that lost it?

## A take

Claude's take, 2026-09-29, after the decisions on time and on vocabulary. **Not decided.**

### Most of the protection is already decided

The two worries, teaching a rocket from your own head and pasting a script from the internet, are mostly answered by rules we've already agreed:

- **You can only say what you have words for** ([vocabulary](vocabulary.md)). "Machine a turbopump from aluminium" means nothing to a character with no word for either. And you can't invent those words early: you name a thing when you've made it or been shown it, so to have a word for aluminium, aluminium has to be in front of you.
- **Every step runs under the laws, at real speed** ([time](game-interface.md)). A taught step isn't downloaded; the character does it, with real materials, taking real time, and it can fail. A rocket can't be taught faster than it can be built.
- **Everything beneath must physically exist.** To make aluminium you need its ore, heat, and power; for those, tools; for the tools, fire and metal. The world's production chain is the ladder.

### Recommendation: the ladder is the production chain, not a skill tree

No list of skills with prerequisites, written in data. **The hierarchy emerges from what things need.** Casting needs a fire and a mould, a mould needs fired clay, fired clay needs a kiln or a fire. You crawl before you walk because the things you'd need to run don't exist yet. Nothing forbids trying something out of order; it just won't have what it needs.

So the player's own head **should** count, and that's a feature. A player who knows chemistry climbs faster than one who doesn't, the way *Dr. Stone*'s hero rebuilds civilisation from memory. What their knowledge can't skip is the materials, the time, and the practice.

### A skill is two halves

- **The recipe (know-what):** the steps, written in the character's own words. Because words match by look, a recipe written as "take a tuft, rub a stick on a board" works with any tuft and any stick that look right to the character, in any place. That's how a recorded sequence becomes a general skill: the vocabulary decision gives it for free.
- **Practice (know-how):** how quickly and how reliably the character does it, and how good the result is. It grows only by doing. A first attempt is slow and fails more often; a practised hand is fast and sure. How much practice changes, per kind of process, is numbers in data.

### Where skills come from

| Source | Gives the recipe | Gives practice |
| --- | --- | --- |
| **Your starting culture** | Yes | Some: what your people do every day |
| **Working it out** (you, the player, directing each step) | Yes, when it succeeds | From that attempt |
| **Being taught** by someone who has it, in words and by showing | Yes, if you know (or are shown) the things it names | A little, if you do it alongside them |
| **Reading** a book or manual | Yes, if you know most of its words | None |

Reading and teaching are gated by vocabulary too: a smelting manual is useless to someone with no words for ore, bellows, or bloom. That's "you can't understand chip-fab without knowing metalwork", without writing it down anywhere.

### What about a pasted script?

It's the same as the player typing each command, which is allowed. Each command still has to make sense in the character's words, still runs at real speed with real materials, and runs at a novice's practice. If it works, the character has earned the recipe by doing it. The thing to watch isn't knowledge; it's automation: a script playing for someone while they're away. That's already covered by [standing orders](time-away.md), which an absent character follows.

### Saving a skill

- **Inventing something saves its recipe.** "What do you call it?" also names the recipe that made it: "make a spear".
- **Anything else can be kept on request:** "remember how I did that as *making fire*". The engine keeps the commands that led to the success, in the character's words.
- A person can hold several recipes for the same thing (fire by rubbing, fire by striking flint), and can pick one by name.

### Questions for the owner

1. Is the ladder the production chain itself, with no skill tree in data?
2. Should practice make first attempts slower and less reliable, with the numbers in data per kind of process?
3. Should practice fade without use? (Suggestion: practice fades slowly; the recipe doesn't.)
4. Is a pasted script acceptable as long as it plays by the same rules as typing?

## Decided

**2026-09-29.** The owner answered by explaining the purpose of the whole climb ("Why we climb from the Stone Age" in [requirements.md](../requirements.md)): it's how we build and refine the engine, and the released game will most likely start at the space age, with everything already existing. So:

- **The ladder is the production chain, and our own climb is its proof.** There's no skill tree in data. Every rung we build under the laws shows that the route exists.
- **The player's own knowledge counts.** What it can't skip is materials, time, and practice.
- **A skill is a recipe in the character's own words, plus practice.** Recipes come from a starting culture, from working things out, from being taught, and from reading, and reading and teaching are gated by vocabulary.
- **The finer rules are for the released game, and are settled in practice:** how much practice helps, whether it fades, and how pasted scripts are treated. The take's suggestions stand as defaults until then: first attempts slower and less reliable, practice fading slowly while recipes stay, and a pasted script treated like typed commands. During the climb, build only what a stage needs: words and recipes now, and practice as a number when a challenge needs it.
