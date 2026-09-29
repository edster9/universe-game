# The skill learning paradigm

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
