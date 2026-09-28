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

**Result, 2026-09-28: passed.**

- About 50,000 random commands per test run, from every entity (including things that aren't people) with real, misspelled, and nonexistent names and amounts up to the largest possible number. Mass and credits never changed, a refused command never changed anything, and the laws never let through something the gate had to catch. The same commands always produced the same world.
- A fixed run of 20,000 mostly sensible commands, with thousands accepted, conserved everything.
- Each cheat has its own test, and the gate refuses bad changes on its own, including a set where the first change is fine and the second isn't. Neither sticks.
- The tests were checked by removing the law against overpaying. The random test found `pay mara 51` with 50 credits within seconds, and the gate refused it independently.

What was learned:

- The laws propose changes and only the gate applies them. That split made conservation easy to guarantee and easy to test.
- Names resolve only among what the actor can perceive, so "No oracles" was straightforward from the start.
- Credits stay a plain counter until slice 3.

## Slice 1: Matter and processes

About ten materials in a data file, each with a handful of properties: density, melting point, hardness, speed of sound, and for ores, composition. Processes: dig, heat, cool, crush, and separate by melting. A furnace that burns fuel for energy. A tool can only work material softer than itself.

Dig ore, smelt it with fuel, get iron and slag, and cast an ingot or a crude blade. The data file includes one rare metal that doesn't corrode, for money later.

**Proves:** the engine can run a production chain it has no names for.

**Test:** the engine code contains no word "iron" or "sword". Swap the data file for a copper world and the same chain works. Mass in equals mass out. Heating costs fuel, and running out of fuel stops the furnace. A stone tool can't shape hardened steel.

**Result, 2026-09-28: passed.**

The worlds are [data/slice1-iron.toml](../data/slice1-iron.toml) and [data/slice1-copper.toml](../data/slice1-copper.toml). In the iron world a player can:

1. take the stone pick and dig 5 kg of ore;
2. carry it to the forge and put it in the hearth with a kilogram of charcoal;
3. light the hearth;
4. after about four minutes, see the iron run out of the rock as 3 kg of molten iron;
5. pour it into the clay mould, where it sets as an iron blade;
6. wait about 17 minutes for it to cool enough to pick up.

The same commands in the copper world make a copper blade.

- **No names:** a test reads every file of engine code and fails if any word naming a material, shape, or item from either world appears. It caught "pick", "rock", and "small" in early drafts, which were renamed. Even "furnace" and "mould" aren't in the engine: they're a *chamber* (burns fuel inside it and holds the heat) and a *form* (liquid setting inside it takes its shape).
- **Energy is conserved exactly**, alongside mass and credits. Each piece of matter stores heat energy as a whole number of microjoules; temperature is worked out from it. Burning 100 g of charcoal released exactly 3 MJ of chemical energy as exactly 3 MJ of heat.
- **Fuel:** with only 100 g of charcoal, the hearth burned for 50 seconds, went out, never melted the ore, and cooled afterwards.
- **Hardness:** the stone hammer can't shape cold hardened steel. Hardness falls as a material heats, so the same hammer shapes the steel once it's been in the hearth. Forging came out of one law, without a rule for it. A lump of charcoal can't dig ore; the stone pick can.
- **Random tests:** random commands and waits from a forge already burning never changed total mass, energy, or credits, never let the laws propose something the gate refused, never cooled anything below its surroundings, and always replayed identically.
- **Sabotage check:** making hardness ignore temperature was caught by the forging test.

Simplifications to revisit:

- **No chemistry yet.** Metal separates from rock by melting first, not by reduction with carbon. Air and oxygen aren't modelled, so burning products weigh the same as the fuel, and smoke collects in a place's air rather than blowing away.
- **Heat is simple.** Everything in a chamber shares one temperature; everything else cools at one open-air rate; the hearth itself doesn't heat up.
- **Hardness is a game scale**, and a tool's hardness is its main material's.
- **Nature logs every tick.** A long wait writes thousands of log entries. Logs will need trimming or summarising before worlds run for days.

## Slice 2: Parts and datasheets

Shape a material into a part: a blade, a rod, a plate, a wire. Put parts together into something new. The engine tests the result once at the layer below and writes its datasheet.

Two examples at different layers:

- **Mechanical:** a blade plus a handle, and its datasheet: sharpness, durability, weight.
- **Electrical:** a battery, a wire, a switch, and a lamp, and the circuit's datasheet: it lights, for how long.

**Proves:** a built thing can become a part without being simulated again, which is how the ladder climbs.

Parts carry a precision inherited from the tool that made them (see [ideas/production.md](ideas/production.md)). Precision can be improved by hand, slowly, the way three plates rubbed together get flatter than any tool.

**Test:** two blades from different ores get different datasheets. A composite is used as a part in something larger without the engine re-running its insides. A design that needs a tight fit fails with hand-filed parts, and succeeds after enough slow hand work. Nobody wrote a rule saying it needs a machine.

**Result, 2026-09-28: passed.**

The world is [data/slice2.toml](../data/slice2.toml): a workshop with a stone hammer, iron and copper stock, a wooden handle, a dry cell, copper wire, a tungsten filament, and two iron contact plates filed by hand to 2 mm. It has three designs: a knife (blade and handle), a switch (two contact plates), and a lamp (cell, wire, switch, and filament). The lamp uses the switch as a part.

- **Every entity in every world has a datasheet**, measured by the engine: what it's made of, mass, volume, state, temperature, melting point, hardness, and stored energy. Shaped parts add their tolerance and what their shape's role measures: edge width and hardness for cutting, resistance for conducting, voltage for a source of charge, surface roughness for touching. The hearth, the mould, places, and people have datasheets too. `datasheet <thing>` in the console shows one.
- **Two blades from different ores differ**: worked with the same hammer, iron and copper blades have the same 2 mm edge but edge hardness 4 and 3.
- **Precision is inherited**: a part is as fine as the tool that made it. The hammer is a 2 mm tool; a bare lump gives the world's rough 5 mm; the mould casts to 1 mm.
- **Assemblies are measured once, from datasheets alone.** The lamp's datasheet is exactly what the measuring law makes from its parts' datasheets, including the switch's stored one. Given a made-up switch that is only a datasheet, with no plates behind it, the law can't tell the difference: it never looks inside.
- **The tight fit, from physics:** where two surfaces touch, resistance grows with their roughness. With 2 mm hand-filed plates the switch adds 4 Ω, the filament reaches only 421 K, and the lamp gives no light. Rubbing the plates together makes both 20% finer per 10-minute session. After nine sessions the lamp still doesn't light; after ten (an hour and forty minutes) the filament passes 1000 K and it lights, running 3 h 11 min on its cell. No rule says it needs a machine.
- **Random tests** building, rubbing, assembling, and taking apart never changed mass, energy, or credits, and every assembly's datasheet matched what was inside it.
- **Sabotage check:** making touching surfaces add no resistance was caught by the lamp test.
- **The no-names scan** now covers the slice 2 data too, including design names. It caught "lump", used both as a data ID and as the engine's word for an unshaped piece; the data was renamed.

Simplifications to revisit:

- **Stored datasheets don't age.** An assembly's datasheet is its rating when it was put together. If a part inside is later damaged or heated, the stored sheet doesn't change.
- **Shapes have no size.** A blade is 20 cm long whether it weighs 300 g or 3 kg, and nothing checks whether the mass makes sense for the shape.
- **Rubbing needs only two parts**, not the real three-plate method, and removes no material.
- **Electricity is one series loop per assembly**, measured, not run: the lamp's cell doesn't drain yet.

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

A chip is a part whose datasheet sets its speed, memory, and ports. Programs are WebAssembly, run one tick at a time, and every instruction draws power (see [technology.md](technology.md)). The validator program is written in Rust for this slice; the player language comes in slice 6. Sensors and actuators are parts, and a program can only reach the ones wired to its chip (see [ideas/code-and-laws.md](ideas/code-and-laws.md)).

The scenario is a coin validator: a scale and a water tank wired to a chip, and a program that computes density and sorts coins into accept and reject.

**Proves:** the in-game computer is a part of the world, and programs know only what their sensors measure.

**Tests:**

- The validator catches plated lead.
- A coin with a core of a metal of nearly the same density fools it.
- Adding an ultrasound sensor, and a better program, catches that coin. Nothing in the engine knows what a counterfeit is.
- Copy the program to a second chip wired to a worse scale, and it misses fakes the first one catches.
- When the battery runs out, the validator stops.

## After that, roughly in order

- **Slice 6: The player language.** A small language that compiles to WebAssembly, with a compiler that runs in the browser, and a workbench to test a program against a local copy of a device. The workbench is published as a static page on AWS. Test: rewrite the slice 5 validator in it, and it behaves the same.
- **Slice 7: Prehistory tools.** Build a small catalog of designs inside the engine and check that each one obeys the laws. Check that the climb exists: every design must be reachable from raw materials, skill, and time, with no unbreakable cycle. This is the first test of starting further up the ladder without hardcoding it.
- **Slice 8: Two people.** A second player over a network, sharing the town, on one AWS server. Tests: each player's browser receives only what their character can perceive; browsers send intents, and a modified browser can't cheat; a player installs new firmware and the other player needs no update.
- **Slice 9: A second place and something that travels between them.** Space begins here, even as text. Messages travel too, which is when a shared ledger and network credits become possible. Test: kill a server in the middle of a ship's handoff, and the ship is neither duplicated nor lost.
- **Slice 10: Graphics.** Only once the text version is worth looking at.

## What we're really finding out

- Are a few laws enough to feel like a world, or does it collapse into a recipe list?
- Does the datasheet trick hold up across layers?
- Do production chains emerge from a few laws of manufacture, without a recipe list?
- Can a conserved economy stay stable without a designer propping it up?
- Is it fun before it's pretty?

## Choices needed before slice 0

- ~~Implementation language~~ and ~~data file format~~: decided 2026-09-28, Rust and TOML. See [technology.md](technology.md).
- **What a tick is.** How much game time passes per step.
