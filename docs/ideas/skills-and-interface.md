# Skills and the interface

Recorded 2026-09-28, prompted by building fire in the [stranded challenge](../challenges/stranded.md). A direction, not a decision. The requirement is "Skills, and talking to the game" in [requirements.md](../requirements.md).

## The problem fire exposed

Making fire took a precise sequence: gather tufts, twigs, and sticks; build a ring; rub for a minute; put a tuft on the ember; then feed the fire one size at a time within seconds of each catching. That is realistic, and it is exactly the kind of thing that is miserable to do through buttons and menus. Most of what the engine will simulate is like this: many small steps, in an order, with timing.

## The direction

**Players say what they want, and the game works out whether the world allows it.**

1. **An open microphone, or plain text.** "I want to make a fire."
2. **An AI model interprets it** into the engine's commands, planning steps the laws allow: gather tinder, build a ring, rub, feed the fire.
3. **The engine decides.** Every step goes through the laws and the gate like any other command. The AI never bypasses physics or conservation; it only proposes. This is the same rule as "AI proposes, the engine verifies" in [world-engine.md](world-engine.md).
4. **The outcome is told back** in plain language: the fire caught, or the tinder was too coarse and it went out.

## Skills

**When a character succeeds at something, the engine can save how they did it as a skill.**

- A skill is a recorded procedure: the steps that worked, generalised ("a tuft of tinder, then feed one size at a time", not "tuft #46").
- **Next time, "make a fire" runs the skill.** It still takes game time, uses real materials, and can still fail, from bad luck, damp wood, or missing parts.
- **Doing it again makes the character better at it:** faster, more reliable, better results. That's the know-how half of [knowledge.md](knowledge.md), and a skill's written steps are the know-what half.
- **A skill is knowledge, so it follows knowledge's rules.** Its steps can be written down, taught, sold, stolen, or lost with the last person who knew them. The practice behind it can't be copied, only earned.

**Where skills come from:**

- **Common sense.** A starting character already knows basic skills most people would: making a fire, finding water, simple fishing. A blank mind knows none and learns everything.
- **Doing.** Working something out step by step, with the AI's help, and succeeding.
- **Libraries and teachers.** Learning skills beyond common sense: smelting, boat building, circuitry.

## The interface grows with the player

The interface becomes reflective and adaptive. It shows what a character can do (their skills) and suggests what's within reach, and it grows as they learn. A new character's interface is simple because they know little. A master smith's has smithing at its fingertips.

## How it fits what's built

- **The engine doesn't change.** It still sees only commands, and the laws still decide. Skills and the AI sit in a layer above: interface, then skills, then commands, then laws.
- **Proof scripts are early skills.** [stranded-2-fire.txt](../../data/scripts/stranded-2-fire.txt) is a fire-making procedure written by hand. The survivor in the tests is an early version of an AI planning with skills.
- **Skills and programs are close cousins.** A skill is a procedure a person follows; a program is a procedure a computer follows ([in-game-computer.md](in-game-computer.md)). They may turn out to be the same thing, run by a body or by a chip.
- **Replays stay exact.** The AI's output is recorded as commands. Replaying a world replays the commands, not the AI, so determinism holds.
- **Clear outcomes matter now.** For an AI to plan well, every command must say plainly what happened or why it was refused. The engine already does this, and it should stay that way.

## Open

- How does a recorded sequence become a general skill that works with different materials in a different place?
- Does a failed attempt teach anything?
- Can skills be traded like designs, and does a copied skill need practice before it works well?
- The cost and speed of an AI model per command, especially for many players at once.
- Are skills and in-game programs one system?
- When does this get built? Probably alongside knowledge (slice 4) and the player language (slice 6).
