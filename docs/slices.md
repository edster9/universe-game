# Slices

The first attempt, planned 2026-09-28. Text only, no graphics. Each slice is small, proves one idea, and ends with a test that says whether it worked. If a slice fails its test, stop and rethink before building the next one.

The design these slices test is in [ideas/world-engine.md](ideas/world-engine.md), [ideas/code-and-laws.md](ideas/code-and-laws.md), [ideas/production.md](ideas/production.md), and [ideas/knowledge.md](ideas/knowledge.md).

## Rules for every slice

- **Headless.** World state plus a text console. A renderer can come much later and read the same state.
- **Laws in code, things in data.** Materials, processes, and designs live in data files. If adding a new material needs a code change, the slice has failed.
- **One gate for every change.** Every change to the world goes through one function that checks conservation and writes a log entry.
- **Deterministic.** The same seed and the same commands give the same world. This makes tests and replays possible now, and networking possible later.
- **Every action has an actor.** From slice 1 on, each process names who performs it and asks whether they know how. The answer is always yes until slice 4.
- **Real units.** Kilograms, joules, kelvin, seconds. This keeps the laws honest.
- **No oracles.** Nothing outside the laws asks what a thing *is*. Agents and programs find out by measuring, with tools that have a precision.

## Slice 0: The ledger

Entities with components, a place to be, inventories, and credits. Console commands: `look`, `take`, `drop`, `give`, `pay`. Credits here are a plain counter standing in for money until slice 3 (see [ideas/money.md](ideas/money.md)).

**Proves:** nothing is created or destroyed except through the gate.

**Test:** run thousands of random commands. Total mass and total credits never change. Every attempt to cheat through the console (negative amounts, giving what you don't have, taking from yourself) is refused.

## Slice 1: Matter and processes

About ten materials in a data file, each with a handful of properties: density, melting point, hardness, speed of sound, and for ores, composition. Processes: dig, heat, cool, crush, and separate by melting. A furnace that burns fuel for energy. A tool can only work material softer than itself.

Dig ore, smelt it with fuel, get iron and slag, and cast an ingot or a crude blade. The data file includes one rare metal that doesn't corrode, for money later.

**Proves:** the engine can run a production chain it has no names for.

**Test:** the engine code contains no word "iron" or "sword". Swap the data file for a copper world and the same chain works. Mass in equals mass out. Heating costs fuel, and running out of fuel stops the furnace. A stone tool can't shape hardened steel.

## Slice 2: Parts and datasheets

Shape a material into a part: a blade, a rod, a plate, a wire. Put parts together into something new. The engine tests the result once at the layer below and writes its datasheet.

Two examples at different layers:

- **Mechanical:** a blade plus a handle, and its datasheet: sharpness, durability, weight.
- **Electrical:** a battery, a wire, a switch, and a lamp, and the circuit's datasheet: it lights, for how long.

**Proves:** a built thing can become a part without being simulated again, which is how the ladder climbs.

Parts carry a precision inherited from the tool that made them (see [ideas/production.md](ideas/production.md)). Precision can be improved by hand, slowly, the way three plates rubbed together get flatter than any tool.

**Test:** two blades from different ores get different datasheets. A composite is used as a part in something larger without the engine re-running its insides. A design that needs a tight fit fails with hand-filed parts, and succeeds after enough slow hand work. Nobody wrote a rule saying it needs a machine.

## Slice 3: A town with a budget

Agents with needs, wallets, and jobs. A mine, a smith, a shop, a guard, and a tax. The world runs in ticks. The shop buys from the smith, the smith buys ore from the mine, the guard is paid from taxes, and taxes come from trade.

The player can work, trade, and steal.

Money becomes real here, replacing the slice 0 counter. The town has coins minted from slice 1's rare metal, and it pays the guard and collects tax in them. Coins can be tested by density, and a fake can be caught or missed.

**Proves:** nothing comes from nowhere, and the town is still stable.

**Tests:**

- Stop the tax, and the guard leaves after going unpaid.
- Rob the shop, and it has nothing to sell until the supply chain restocks it.
- Run ten thousand ticks with no player. The town neither collapses nor prints money.
- Recovery after a raid happens only through named sources: a migrant arrives, or a trader brings goods from beyond the map at a price.

This is also the first **fun check**: is watching and poking this town interesting as text? If it isn't, graphics won't save it.

## Slice 4: Knowledge

Designs are items: a book, a note, a data chip. Skill lives in agents and grows with practice. The "does the actor know how" check from slice 1 becomes real.

**Proves:** parts plus materials are not enough without knowledge.

**Test:** the stranded scenario in miniature. Every part of a simple machine is on the ground. Without the design, the build fails. With a book, it succeeds, slowly and badly. With a skilled NPC you hired, it succeeds well, and the wages came out of your purse.

A second test: knowledge lives somewhere. Burn the only book, and with nobody left who knows the design, the machine can't be built again. Taking apart an existing one recovers part of the design, more with more skill.

## Slice 5: A tiny computer, and the money validator

A chip is a part whose datasheet is an instruction set and a clock speed. A tiny virtual machine runs it. Every cycle draws power. Sensors and actuators are parts, and a program can only reach the ones wired to its chip (see [ideas/code-and-laws.md](ideas/code-and-laws.md)).

The scenario is a coin validator: a scale and a water tank wired to a chip, and a program that computes density and sorts coins into accept and reject.

**Proves:** the in-game computer is a part of the world, and programs know only what their sensors measure.

**Tests:**

- The validator catches plated lead.
- A coin with a core of a metal of nearly the same density fools it.
- Adding an ultrasound sensor, and a better program, catches that coin. Nothing in the engine knows what a counterfeit is.
- Copy the program to a second chip wired to a worse scale, and it misses fakes the first one catches.
- When the battery runs out, the validator stops.

## After that, roughly in order

- **Slice 6: Prehistory tools.** Build a small catalog of designs inside the engine and check that each one obeys the laws. Check the whole production graph for dead ends: every design must be reachable from raw materials, skill, and time. This is the first test of starting further up the ladder without hardcoding it.
- **Slice 7: Two people.** A second player over a network, sharing the town.
- **Slice 8: A second place and something that travels between them.** Space begins here, even as text. Messages travel too, which is when a shared ledger and network credits become possible.
- **Slice 9: Graphics.** Only once the text version is worth looking at.

## What we're really finding out

- Are a few laws enough to feel like a world, or does it collapse into a recipe list?
- Does the datasheet trick hold up across layers?
- Do production chains emerge from a few laws of manufacture, without a recipe list?
- Can a conserved economy stay stable without a designer propping it up?
- Is it fun before it's pretty?

## Choices needed before slice 0

- **Implementation language.** Most of the code will be written by AI, so the choice follows the constraints in [ideas/code-and-laws.md](ideas/code-and-laws.md): determinism, a sandboxed and metered VM, and programs that can be saved and moved mid-run.
- **Data file format.** For example TOML, JSON, or YAML.
- **What a tick is.** How much game time passes per step.
