# Prior art

What has already been tried, compiled 2026-09-28. This file merges the earlier surveys of space games and programmable worlds, and adds the simulation, economy, and knowledge games behind the [world engine](../ideas/world-engine.md).

**Checked vs not checked.** The space-game and programmable-world entries were checked against the sources listed at the end around 28 September 2026. The entries under "Economies with real sources", "Building from materials", "Layers and datasheets", and "Knowledge as a mechanic" are from general knowledge and have **not been re-checked yet**. Verify them before relying on a detail.

## The space-game landscape

Each kind of universe game is already owned by a live game:

| Fantasy | Game | Note |
| --- | --- | --- |
| Seamless life, on foot and in the cockpit | Star Citizen | Still an alpha after more than a decade. The playable universe is a few systems, and Squadron 42 moved to Q2 2027. |
| A real galaxy you are small inside | Elite Dangerous | A 1:1 Milky Way that is more than 99% unvisited. Colonisation (2025) lets players extend the inhabited area. |
| An infinite sandbox | No Man's Sky | The Cosmos update (September 2026) spent a tenth anniversary on the space between planets. |
| You become the economy | X4: Foundations | Single-player. A mine in one sector changes a war in another. |
| One shared political universe | EVE Online | About 30,000 concurrent players. The meaning is the players and their shared economy. |

Lessons:

- **More places with the same verbs makes a thinner game.** Starfield's thousand planets were criticised by a former Bethesda artist on the project in August 2026.
- **Emptiness and sameness are one complaint.**
- **Games feel alive when distant places are connected by consequences**, as in X4, EVE, and Elite's colonisation.
- **Seamless everything is a funding model.** A smaller project that starts there gets a hangar.

## Economies with real sources

*Not yet re-checked.*

- **X4: Foundations.** NPC ships and stations are built from wares that factories produced. Cut a supply line and a faction stops rebuilding.
- **Mount & Blade: Bannerlord.** Garrisons cost wages. Towns have food, prosperity, and a treasury.
- **Kenshi.** You can rob shops, kill shopkeepers, and wipe out towns, and the world changes when you do. Shops still restock without a source.
- **Wurm Online.** Player settlements pay upkeep, and upkeep pays for guards.
- **Victoria 3, Dwarf Fortress.** Soldiers and workers are people with jobs and wages.
- **EVE Online.** Nearly everything is built by players from mined materials, and destroyed ships are gone.
- **Ultima Online.** The warning. Its original ecology of predators, prey, and respawns was cut in beta because players killed everything faster than it regrew.

## Building from materials

*Not yet re-checked.*

- **Dwarf Fortress.** Materials are defined in data files ("raws") with properties like melting point, density, and shear strength. The engine doesn't know iron; the data does.
- **Wurm Online, Vintage Story, TerraFirmaCraft.** Prospect, dig ore, smelt it, and forge the tool by hand.
- **Breath of the Wild.** A small "chemistry engine" (fire, electricity, wind, metal) where interactions come from rules rather than scripts.
- **Space Station 13.** Simulated chemistry and atmosphere. Players invent uses nobody planned.
- **Noita.** Every pixel is simulated material.
- **Star Wars Galaxies, Haven & Hearth.** Raw materials have stats that carry into crafted items, so crafted items differ.
- **Infinite Craft (2024).** A language model decides what a combination makes. It feels limitless, but it has no conservation and no consistent physics.

## Layers and datasheets

*Not yet re-checked.*

- **Minecraft redstone.** A handful of rules. Players built working computers from them.
- **Turing Complete, nandgame.** Build logic from NAND gates, then a CPU, then program it. A built layer becomes a part for the next one.
- **Kerbal Space Program.** Parts come with specs (thrust, efficiency, mass). A rocket's performance emerges from how they're assembled.

## Knowledge as a mechanic

*Not yet re-checked.*

- **Outer Wilds.** Progress is only what the *player* knows.
- **Kenshi.** Research needs books and AI cores found in ruins.
- **Rust.** Destroying an item at a research table teaches its blueprint: reverse engineering.
- **Subnautica.** Scanning fragments of a device teaches how to build it.
- **EVE Online.** An original blueprint can make copies that have limited runs. Knowledge as a license.

## Programmable worlds

Every game in this list kept a core that players cannot rewrite. The ones that offered a general machine over the whole simulation cancelled (0x10c), stalled (Starbase), or shut the official world down (Dual Universe).

- **0x10c** (Mojang, 2012–2013, cancelled). An Elite-like game built around the DCPU-16, a fully specified 16-bit CPU that could run the whole ship. Programs were to be traded on floppy disks, and malware was expected. Computers were to keep running while owners were offline, which is why a subscription was planned. Players built assemblers and operating systems before the game existed.
- **Starbase** (Frozenbyte). YOLOL, a small language on chips wired to ship devices. Players wrote real navigation stacks. The game around it has stalled.
- **Space Engineers.** C# on programmable blocks. A script belongs to one ship or station.
- **Stormworks, Stationeers, kOS.** Lua, IC10 assembly, and a Kerbal autopilot mod: powerful on the vehicle, silent about the wider world.
- **Oolite.** The descendant of classic Elite, scripted in JavaScript. Expansions add commodities that become real cargo, and market scripts adjust prices. Installed locally, not pushed into a shared universe.
- **Second Life, LambdaMOO.** The longest-running versions of building a shared world from the inside. LambdaMOO showed that permission becomes the real design problem.
- **Screeps.** JavaScript running server-side every tick, whether or not you are logged in, with a CPU budget per player. It shows a metered API can last a decade, and that its audience wants to write the bot, not play by hand. A flaw that let another player's code reach a client was reported in August 2026.
- **hackmud.** Players publish scripts that others run, and the economy is those scripts, including scams. A trust ladder runs from "can't reach anything dangerous" down to "can move money". Its author has said JavaScript was hard to sandbox.
- **ComputerCraft** (Minecraft mod, *not yet re-checked*). Lua computers inside a world of materials. The closest existing mix of "a computer is a block you built" and a crafted world.

Patterns kept:

1. **Separate nouns from verbs.** Materials and parts are data. Behaviour is code.
2. **Meter the runtime.** Here, through power.
3. **Make trust part of the fiction.** Running someone else's code is a risk you choose.
4. **The core stays a core.** Here, the core is the laws and conservation.

## Tools that fit

*Not yet re-checked.*

- **Entity Component System libraries.** Bevy (Rust) and Flecs (C). An entity is an ID with components, which matches "label, attributes, behaviour".
- **SpacetimeDB.** The game's logic runs inside the database. It was built for BitCraft, an online game whose economy is entirely player-crafted. A candidate for the networked stage, not for the first slices.

## Sources for the checked entries

- Hello Games, "No Man's Sky: Cosmos," 9 September 2026.
- Chris Roberts, Letter from the Chairman, 27 August 2026 (Squadron 42 to Q2 2027); PC Gamer, 27 August 2026.
- Space.com, "Why space games still struggle with the scale of the universe," 12 July 2026.
- Egosoft, X4: Foundations 9.00 Empire update, 10 June 2026.
- EVE Offline, Tranquility count, 28 September 2026.
- Reporting on Nate Purkeypile's Starfield comments, August 2026.
- Wikipedia, "0x10c"; Rock Paper Shotgun, 4 April 2012; archived 0x10c.com, May 2012.
- Starbase wiki, "YOLOL"; Frozenbyte Steam post, January 2026.
- Elite Wiki: "Scripting Oolite with JavaScript," "Oolite JavaScript Reference: Market Scripts."
- Screeps, https://screeps.com/; "Screeps: How A Game About Programming Sold Its Players a Remote Access Trojan," 7 August 2026.
- hackmud wiki; Sean Gubelman interview, MMOs.com, 22 September 2016.
- LambdaMOO programmer's manual (Pavel Curtis).
- Massively Overpowered, Dual Universe sunset coverage, July–August 2025.
