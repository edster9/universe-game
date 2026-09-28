# World engine

Working proposal, 2026-09-28. Not a decision. The requirement it answers is in [requirements.md](../requirements.md). How it gets built, slice by slice, is in [slices.md](../slices.md).

## The one idea: laws, not things

The engine knows **laws**. It never knows **things**.

It knows that heat raises temperature, that a material melts above its melting point, that a harder edge cuts a softer surface, that burning fuel releases a fixed amount of energy. It does not know "iron", "sword", "rocket", or "teleporter". Those are data, or they are built by someone under the laws.

The real universe works the same way. It has a small set of laws and no list of objects.

## Conservation is the core's job

Everything else can be content or player code. These cannot:

- **Mass.** Smelting ore yields metal plus slag, and the two add up to the ore.
- **Energy.** Heating costs fuel. Thrust costs propellant. A device that does work draws power from somewhere.
- **Money, indirectly.** The engine has no concept of money. Hard money is conserved because mass is. Ledger money changes only by its ledger's rules, and an issuer that prints too much causes inflation, not an exploit. No script, shop, or mission creates value by decree. See [money.md](money.md).
- **People.** A guard, a smith, or a trader is one agent in one place, with a wallet and needs.

Every change to the world passes through one place that checks these. That is the anti-exploit layer. Player code can combine anything, but it cannot make mass, energy, or money from nothing.

The one thing not conserved is **knowledge**: copying a design costs nothing. See [knowledge.md](knowledge.md).

### Recovery needs sources too

A real economy can be raided to death. Ultima Online cut its original ecology during beta because players killed everything faster than it regrew. A town that loses its shop, guards, and treasury must be able to recover, but only through sources that exist in the world: migrants arriving, traders importing goods from beyond the map at a price, an empire sending aid it paid for with taxes. Recovery is slow and costly, never free.

## Entities: label, attributes, behaviour

An entity is an ID with components attached. This is the Entity Component System (ECS) pattern.

- **Label:** what people call it. The engine doesn't care.
- **Attributes:** mass, temperature, composition, hardness, shape, charge, contents.
- **Behaviour:** what it does when the laws act on it, or code it runs if it has a computer.

Engine code is written against components ("has temperature", "is a container", "is an agent", "can be burned"), never against kinds of thing. If adding bronze needs a change to engine code, the design has failed.

## Layers, each with a few laws

A microchip cannot be simulated from atoms, and a rocket cannot be simulated from molecules. Nobody in the real world does that either. An engineer designing a bridge doesn't use quantum mechanics. Each level of technology has its own simplified physics and treats the level below as a spec.

| Layer | The engine knows (laws) | It does not know (things) |
| --- | --- | --- |
| Matter | Density, hardness, melting point, conductivity, chemical energy, semiconductor behaviour | "Iron", "silicon": these are data |
| Processes | Heat, cool, cut, crush, cast, alloy, dope, deposit: inputs, tools, conditions, energy | "Smelting", "chip fabrication": these are chains of processes |
| Components | Electrical, mechanical, thermal behaviour of a part | "Motor", "transistor", "nozzle" |
| Systems | Logic, power flow, thrust, heat budget | "Autopilot", "rocket", "computer" |

Perhaps a few dozen laws in total. The engine never has a type called "rocket".

## Datasheets: test once, then use as a part

This is how the ladder climbs without the engine knowing the destination.

When something new is built, the engine runs it once at the layer below and measures it. The result is a **datasheet**: a few numbers that say what it does. From then on the thing is a part with that datasheet, and the layer above only reads the datasheet.

- **A blade.** Material hardness and toughness plus shape give sharpness, durability, and weight.
- **A transistor.** Doped silicon, measured once, gives a switch with a speed and a power draw.
- **A chip.** Switches arranged into logic, tested at the logic layer, give an instruction set and a clock speed. From then on the [in-game computer](in-game-computer.md) runs it. The microchip and the in-game computer are the same idea.
- **A rocket engine.** Tank, pump, chamber, and nozzle, tested against chemistry and flow, give thrust, efficiency, and mass. The flight simulator only sees those numbers.

Datasheets come from actual inputs, so uniqueness is natural. Better ore or a better design gives a better datasheet. Wear and damage change a part, which means its datasheet has to be recomputed at some point (see open questions).

## Prehistory: starting in the space age

The universe opens in the space age, and the space age is built, not hardcoded.

Before the world opens, the space age is constructed **inside the engine, under the same laws players will use**. The team, with tools and possibly AI to help, plays the role of the civilisations that came before: designing furnaces, fabs, engines, and ships as processes and assemblies the engine verifies. Dwarf Fortress does a smaller version of this, simulating centuries of history before play begins.

What this buys:

- Nothing in the starting universe is special. Every ship has a real bill of materials. It can be taken apart, studied, reverse-engineered, or improved.
- The stone age is still there, underneath. A stranded crew can go down the ladder (scavenge, smelt, rebuild) because the ladder is real. They cannot make chips without a chip fab, and a fab is itself a thing that has to exist somewhere.

### AI proposes, the engine verifies

Infinite Craft (2024) lets a language model decide what "fire + water" makes. It feels limitless, but nothing is conserved, results aren't consistent, and there is no physics to reason with. AI should not be the live physics.

AI is useful as a builder: drafting the prehistory catalog, proposing designs the engine then checks against its laws, or playing NPC engineers and traders.

## What the engine must know in advance

- The laws of each layer.
- Where the layer boundaries sit, and how a datasheet is measured at each one.
- The conservation rules.

That is the "knowing some things in advance" that cannot be avoided. The test for any law: does it let people build things nobody planned, while conservation still holds?

The teleporter is decided here. It exists only if some layer has a law that allows moving mass across space at a cost (for example, energy in proportion to mass times distance, plus a rare material that is used up). The team decides whether that law exists. Players discover what can be built with it.

## Risks

- **Too few laws** and it becomes a recipe list, a crafting menu. **Too many** and nothing can run. Choosing the laws is the design work.
- **Players find the broken combination.** Emergent systems always have exploits. Conservation is the main defence.
- **Showing things nobody designed.** What does an invented device look like? Dwarf Fortress affords deep simulation by drawing with text. The more is simulated, the less visual polish is affordable. Text slices sidestep this for now.
- **Compute.** Deep simulation is expensive. The small universe is what makes it affordable.
- **Griefing.** Real consequences cut both ways.

## Related

- [code-and-laws.md](code-and-laws.md): laws, designs, and programs, and what each can touch.
- [production.md](production.md): why knowing a design is not being able to make it.
- [knowledge.md](knowledge.md): the third input to making anything, and where it lives.
- [in-game-computer.md](in-game-computer.md): computers as parts, programs as knowledge.
- [game-concept.md](game-concept.md): the space game that eventually runs on this.
