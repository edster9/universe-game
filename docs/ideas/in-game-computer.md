# In-game computer

Working proposal, rewritten 2026-09-28 to sit on top of the [world engine](world-engine.md). Not a decision.

## The requirement

Players write code in a sandbox inside the game, and that code can reach the core. The examples were a better autopilot, and a programmatic way to create objects that can be traded. The game should grow because someone inside it made something.

## What changed from the first proposal

The first version split the terminal into two machines. One was a ship machine that flies the ship. The other was a world machine that let player code publish new commodities into a shared catalog, each with a price function its author wrote.

The world engine replaces the world machine. New objects enter the world by being **built** under the laws, not by being published into a catalog. Prices come from what people actually pay. That removes the catalog, the review queue, and an exploit: an author who writes a commodity's price function can price it cheap where they buy and dear where they sell, which prints money.

The ship machine survives, generalised to any computer.

## A computer is a part

A computer is built like anything else (see datasheets in [world-engine.md](world-engine.md)). A chip's datasheet is an instruction set, a clock speed, and memory. The virtual machine runs whatever the datasheet says. A better chip is a faster or bigger computer. A salvaged chip might run an older instruction set.

Running code costs cycles, and cycles cost power. Metering is physical, not a quota: a program that runs all night drains a battery.

## Power comes from wiring, not permissions

A program can touch only the devices wired to the computer it runs on.

- An autopilot flies because its computer is wired to sensors and thrusters.
- A furnace controller holds a temperature because it is wired to a thermometer and a fuel valve.
- A program cannot move money unless there is a device for it: a link to a bank that the bank chose to offer, on the bank's terms.

The blast radius of bad code is whatever it is wired to. This is the same boundary the prior art kept (see [prior-art.md](../research/prior-art.md)), expressed as hardware instead of as a permission system.

## Programs are knowledge

A program is a design for behaviour (see [knowledge.md](knowledge.md)). It can be copied for free, sold, licensed, stolen, or hidden inside something else. Running someone else's program is a risk the player chooses, and hackmud's trust ladder is the reference for making that risk part of play.

A program that encodes a design, running on a computer wired to a fabricator, is automation: machines that build.

## Rules for everyone

Player code cannot change the laws. That is what keeps conservation intact.

Player code may be able to change *local* rules, such as a town's taxes, tariffs, or door locks, if its owner controls that town. That is governance, not physics. Whether this belongs in the game is open.

## Still open

- **The language.** Options:
  - A tiny instruction set of our own. It has charm, and 0x10c's DCPU-16 grew a community before the game existed.
  - WebAssembly with metering. Players can use real languages.
  - Lua.
  
  hackmud's lesson is that sandboxing JavaScript is harder than it looks.
- **Offline ships.** Do programs keep running while the owner is offline? Power budgets make that physical, but the server still pays for it.
- **Unique or copied.** Can a program be unique (a stolen original) as well as copied (a license)?
- **The first program.** What can a new character run before they can write one?

## In the slices

Slice 5 in [slices.md](../slices.md): a chip with an instruction set, a tiny virtual machine, and a program that controls one device from slice 1.
