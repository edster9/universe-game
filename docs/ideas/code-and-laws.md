# Code and laws

Working proposal, 2026-09-28. Not a decision, but the most foundational choice in these docs, because it shapes the engine and the technology.

## The question

The original idea was a virtual computer as the world itself. Things are not only created by code, they run on code. A spaceship is a higher-level program made of many other operations. When ship A meets ship B, both ships' code is running.

So is the world a set of rules with templated ways of building things, or is everything code?

## The answer: everything is code, with different powers

What matters is who wrote a piece of code and what it is allowed to touch. An operating system is the model:

| | OS analogy | In this world | Written by | Can touch |
| --- | --- | --- | --- | --- |
| **Laws** | The kernel | Conservation, heat, hardness, energy, motion, how information travels | The team, in the engine | Everything |
| **Matter and designs** | Files | Materials, parts, assemblies, designs | Data, verified by the engine | Nothing; laws act on them |
| **Programs** | User processes | Autopilots, validators, controllers, ledgers | Players and NPCs, in the in-game VM | Only the devices wired to their computer |

- **A sword is matter.** Steel shaped into a blade and a handle. It never runs code. When it hits armour, the laws decide what happens. Running a program inside every rock would cost a great deal and decide nothing.
- **A spaceship is all three at once.** It is matter: thousands of parts, each with a datasheet. The laws turn its thrust into motion. And it runs programs: a reactor controller, a thruster controller, and an autopilot passing messages over the ship's wiring. That is the "higher-level code of many operations".

## Programs see through sensors and act through actuators

**Code can only know what its sensors measure, and can only do what its actuators allow.** Only the laws see everything.

There is no function that asks the engine what something *is*. A program cannot call `isCounterfeit(bar)` or `isGold(bar)`. It reads a scale, a probe, or a camera, each a physical part with a datasheet that includes its precision. It drives a motor, a valve, or a display, each also a part.

This is what makes invention real. A program is as smart as its author and as well-informed as its sensors.

### The money validator

- **A cheap validator** has a scale and a water tank. Its program computes density and compares it with the density of gold, a value it has to contain; that is knowledge. A bar with a tungsten core passes, because the densities almost match.
- **A smarter validator** adds an ultrasound probe, because sound travels at different speeds through tungsten and gold. Or it drills a core sample and runs it through a spectrometer. That costs time and energy, and removes a little metal, which is conserved as shavings.
- **The inventor can sell three things:**
  - the program: knowledge, copyable for free, so it can be pirated;
  - the device: a physical item;
  - the service: checking bars at a stall in port, for a fee.

### What can be invented is bounded by what the laws track

If materials only have density, no one can ever build a validator that catches tungsten. For that to be inventable, materials need a speed of sound, a conductivity, and a composition that a spectrometer can read. **The richness of the materials data sets the depth of invention.** Choosing which properties to simulate is choosing what can ever be discovered.

## Designs are data, and can carry programs

Building works through **designs**: which parts, how they connect, and the process steps (see [production.md](production.md)).

- **A design is data, not code**, because the engine has to reason about it without running it. It verifies conservation, computes mass and cost, measures datasheets, and supports reverse engineering. You can't take apart something that exists only as a running program.
- **A design includes firmware** for any computer parts in it. A validator design is sensors, a chip, wiring, and the validator program.
- **Code can generate designs.** A program can output a design, such as a family of hulls in different sizes. The output is still data, checked by the engine before anything is built. This is the original requirement's "programmatic way to create objects that can be traded", without giving code the power to create matter.

So building is templated (designs are data) and behaviour is code (programs run on computers).

## Why the laws are not player-editable code

The laws could in principle run inside the same VM as player programs, just with more privilege. They shouldn't:

- **Speed.** The laws touch every entity every tick and must be fast native engine code. Programs are few, and each only decides for its own device.
- **Trust.** If the laws were editable, conservation would be editable. Keeping them in the engine is what keeps "nothing from nowhere".
- **Precedent.** 0x10c planned to emulate every player's CPU in a persistent universe, priced in a subscription for it, and was cancelled.

New laws, like a teleporter law, can be added by the team as engine modules. Players discover laws through experiment. They never write them.

## The cone of influence

If influence has a maximum speed, regions beyond each other's reach can run independently and in parallel. This is the same law as "how information travels" in [money.md](money.md). One law is both physics and the plan for scaling.

- **Each region has one authority:** a server process that runs the laws and the programs in that region. EVE Online works this way, with solar systems as regions and jumps moving you between servers.
- **When a ship crosses into another region**, its state moves with it, including its running programs.
- **When ships A and B meet**, both players' machines can run the same simulation to make it look smooth, but the region's authority decides what actually happened. Because the simulation is deterministic (the same inputs always give the same result), anyone can re-run it to check, and a cheater shows up as a mismatch.
- **Player programs run on the authority, never on another player's machine.** If A's autopilot ran on B's client, B could read it and steal it. Code is valuable because it can be kept secret.

Improbable's SpatialOS tried to split one seamless world across many servers, and the games built on it shut down. Regions with explicit handoffs, EVE's approach, are the proven path. The small universe makes that easy.

## What this demands of the technology

Whatever language is chosen, it must support:

1. **Determinism.** Laws and VM give the same result on every machine. This constrains how floating-point maths is used.
2. **A sandboxed, metered VM for programs.** Every instruction costs in-game power, which is the metering.
3. **Programs that can be paused, saved, and moved mid-run**, so a ship's autopilot can cross between regions.
4. **Designs as data**, with a verifier in the engine.
5. **Code that never leaves the authority** except to its owner.

Current lean, not a decision:

- **Engine:** Rust, for determinism and speed, and because it has good tools for hosting sandboxed code.
- **Player VM:** WebAssembly with metering, with a friendlier in-game language compiled to it.
- **Slice 5:** a tiny custom VM is enough to prove the idea.

## Related

- [world-engine.md](world-engine.md): the laws.
- [production.md](production.md): how designs become things, and why duplication is hard.
- [knowledge.md](knowledge.md): designs and programs as knowledge.
- [in-game-computer.md](in-game-computer.md): the computer as a part.
