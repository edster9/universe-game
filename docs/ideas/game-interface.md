# From text adventure to the game's interface

Recorded 2026-09-29 **as a placeholder for a discussion the owner has asked for**. Nothing here is decided. The requirement is "From text adventure to the game's interface" in [requirements.md](../requirements.md). It's close to [skills-and-interface.md](skills-and-interface.md) (skills, a spoken interface) and [time-away.md](time-away.md) (the world running without you).

## Where we are

The engine plays like a text adventure: a person types a command, the laws decide, and the world's clock moves forward by as long as the action took. That's been a very good proving ground. But it's turn-based at heart, and one person at a time:

- **Time moves when the player acts.** `go forest` jumps the clock by the walk; `wait 2 h` runs it. Between commands, nothing happens.
- **An action happens all at once, then time passes.** You arrive in the forest the moment you set off, and spend the walk there.
- **Creatures already work differently.** A boar is "busy until" its current action ends, and chooses again when it's free, while the world's clock runs on. That's much closer to how a real-time, shared world has to work.

## Questions for the discussion

- **The clock.** In a shared, real-time world the clock runs on its own, and every person, like every creature, is busy doing something until it ends. What does "real time" mean here: one game second per real second, or faster?
- **Actions as processes.** Walking, gathering, and felling become things in progress that can be seen, interrupted, and joined. Where is a person while they're walking?
- **What a player sees.** A described scene, as now, a map, a 2D or 3D view? The engine's view is already data (`view.rs`), so it can feed any of them.
- **Skills as buttons.** A recorded skill (a script, as the recipes are now) becomes something to press, and to repeat.
- **What stays text.** The console and scripts remain the developer's and tester's tools, and perhaps a way to play.
- **How it runs.** Browser and server, as planned in [technology.md](../technology.md).

## Claude's take, 2026-09-29, for the owner to refine

Superseded in part by the decision below: the world does not run at 12×. Interface and time are treated together, with [time away](time-away.md), because they're one question: how the world runs when nobody is typing.

### 1. The world runs on its own clock

The server keeps every universe's clock running. Each universe has a **speed**, in data: game time per real time. The recommendation is faster than real: **12×**, so a game day is two real hours. At 1×, felling a log takes two real hours and the climb a real day and a half; at 12×, a log takes 10 minutes, a 300 m walk 20 seconds, and a night's sleep 40 minutes. In a shared world there's no `wait 2 h`: long actions are real waiting, and a player does other things meanwhile. The console keeps `wait` for tests and for playing alone.

### 2. Everyone is busy until something ends

Starting an action means **doing it until a time**, and its result comes at the end: you arrive when the walk is over, and a search finds its fish when the search is done. Walking, you're on the path between two places, where you can be seen and met. An action can be interrupted by a charge, a wound, or your own change of mind. This is how boars already work, and how rubbing works: it becomes true of everyone.

In the engine, starting an action stops also running the clock; the clock is run by whoever hosts the world (the server, or the console's `wait`), and it finishes actions as their times come. The log of who asked for what, and when, still replays exactly.

### 3. Skills are queued commands, and a button runs one

A person can have a queue of commands. A **skill** is a named, saved queue, like the recipes in `data/scripts/skills/`, with checks along the way; a **button** starts it, and pressing it again repeats it. An **instinct** is the same machinery choosing its own next command, and a player's **standing orders** are a small instinct they set for their own person.

### 4. What a player sees, first

A browser page with: the scene, as text that updates as things happen; a **map of places from memory**, certain ones solid and possible ones dashed; what you carry; your skills as buttons; and a command line, so the text adventure lives on inside the game. The engine's views are already data, so this is a new client, not a new engine.

**Keep the world made of places**, like rooms, joined by paths with lengths, rather than continuous space. A 3D walk-around view would need space inside places, a big decision better made later, once the game's shape is clear. Places, a map, and text suit this engine now.

### 5. Time away: standing orders

When a player leaves, **their person stays in the world, following standing orders** the player set: drink when thirsty, eat from their stores, sleep in their shelter, stay home. The orders run on the instinct machinery the boars use. Nothing comes from nowhere: an absent person lives on real food and water, and others can see them, visit them, and, in a lawless place, rob them.

The tension is the world's speed. At 12×, a real day away is twelve game days: a lot of food and water. So being away safely takes preparation (a home, stores, water near), and later other people: a household or colony feeding you. A universe could also choose to slow time for absent people; that's a rule a universe could pick, but it bends "the world keeps running".

### 6. A proof slice to settle it

1. Persons are busy until their action ends; results come at the end; a walker is on the way.
2. The console gets a live mode: the world runs at a set speed and events show as they happen. Scripts still drive the clock themselves, so tests stay exact.
3. A skill runs from one command, and can be repeated.
4. A person left with standing orders, and enough stored, lives through a week alone.

### For the owner to decide

- The world's speed: is 12× about right?
- Places and paths now, and continuous space later: agreed?
- Time away: standing orders on real stores, and preparation matters?
- The first client: text, a map, and buttons in the browser?

## Decided, 2026-09-29: time, sacrifices, and processes

Decided with the owner (their words are in "Sacrifices, and universes that are configurable" and "How time passes" in [requirements.md](../requirements.md)). This replaces the 12× proposal above.

- **One clock, one to one.** Everything that happens to a body, and how fast it moves, runs at real speed. So do fire burning its fuel, food spoiling, and the planet's day. Distances become design: the game's places (docked ships, cities) will be small enough to walk, with transport to cross further.
- **Time is negotiable; matter and energy are not.** A setting may change how long a process takes, never what it takes: iron still has to reach its melting point, with the heat to get it there.
- **A process's time is a number on the thing, not a global dial.** Work times are on sources and tools, furnace heating on its design, as they are now. Two small additions: a **hold time** on a change of material (clay fires, meat cooks, ore melts after being held at temperature for so long), and **how fast a thing gives up its heat** (a mould or a quench can set a casting in a minute while everything else cools at its real rate).
- **Realistic and playable are two sets of numbers.** A realistic library of things, used by every test and training challenge, keeps the laws proved against reality. A playable library has the same laws and things with game-friendly numbers. Small sacrifices, such as ten swords from one fire, are acceptable.
- **Rules are switches a universe's administrator sets**, per kind of mind. The norm: players don't need sleep or food; NPCs do, so a village sleeps at night and can be fed and traded with. The rules stay in the engine, for realistic universes and tutorials. Short-term fatigue, tiring from hard work and recovering with rest, is a separate law to add, whatever the sleep setting.
- **Sleep belongs to the species**, in its kind's data, not to the planet. On its home planet it usually lines up with night; a colonist elsewhere sleeps by their body. Instinct should sleep by its own rhythm, not "at night".

Still open: where a player's energy comes from when they don't need food. The proposal is **vitality**, a named inflow the universe grants to players, accounted like sunlight.

## Claude's take on vitality, 2026-09-29

**Not decided.** The problem: in the normal universe players don't need food, drink, or sleep, yet their bodies still walk, fight, and keep warm. "Nothing from nowhere" means that energy must come from somewhere.

### Vitality is a named inflow, like sunlight

The universe grants each player's body a steady flow of energy, **vitality**, at a set power, the way the sun grants the island its daylight. The gate accounts for it by name, exactly as it accounts for sunlight, so conservation still holds and the ledger shows where every joule came from. Its size is a universe setting, per kind of mind: NPCs get none and eat; players get enough to live on.

### What it covers, and what it doesn't

- **Living:** yes. Vitality pays the body's resting upkeep and keeps it warm in ordinary conditions. With the rule switches off, the laws that drain a player's food, water, and wakefulness don't run at all. Switching a law off creates nothing, so no matter comes from nowhere.
- **Hard work: only up to a point.** Vitality flows at a fixed rate. Work faster than that, running, fighting, hauling, and the difference comes out of a small reserve, **stamina**. When it's gone you're exhausted and must slow down or rest while vitality refills it. That *is* the short-term fatigue law already agreed: it falls out of a capped inflow, with no extra machinery.
- **Harsh places: no.** Vitality covers ordinary warmth, not a blizzard or vacuum. Extreme cold, heat, or no air still kill, so suits, shelter, and gear matter. That's what makes a space game's equipment worth having.
- **Healing: no, it needs matter.** A wound loses blood, which is matter, and vitality is only energy. So **healing needs something taken in**: food, or medicine, which a universe can make faster. Players who never need to eat still need to eat, or carry medkits, to recover. That keeps food and medicine worth trading, and keeps hunting and cooking meaningful, as the owner wants.

### What it costs to build

Small: a second named inflow in the gate beside sunlight, a stamina reserve on bodies, and the rule switches per kind of mind. Nothing on the island needs it (castaways eat), so it's built when players first differ from NPCs: at the village.

## Claude's take on what a player sees, 2026-09-29

**Not decided.** The owner has said the game will be 3D and real time, and that visuals carry much of recognition ([vocabulary](vocabulary.md)). This take revises "What a player sees, first" above.

### The end state

- **A 3D view in real time**, walking one to one.
- **Every thing drawn from what it's really made of:** each shape has a simple form in data (a rod of this length and thickness, a sheet, a bowl, a block), and each material a surface look (colour, sheen, grain). An assembly is drawn by putting its parts' forms together. So the engine can draw a thing nobody designed, a player's first spear or someone's odd ship, because it's drawing parts, not looking up a model. **The one "look" of a thing serves three uses:** the eyes (drawing it), recognition (comparing it with your words' examples), and text (describing it).
- **Labels from your own words** over what you see: "a spear", "something like a raft", or nothing.
- **Around the view:** what you carry; your skills as buttons; your map from memory, certain places solid, possible ones dashed; notifications; and a command line or microphone, where the text adventure lives on as the way you tell your character what to do.

### The principles

- **The engine sends only what your character perceives.** The client can't show what it wasn't sent, so a modified client can't cheat (slice 8's test).
- **The engine never draws.** It sends views as data; clients draw them. The text console, a browser page, and a 3D client are all clients of the same views.
- **Skills are buttons.** A skill is a saved queue of commands; a button starts it, you watch your character do it at the process's speed, and it can be interrupted.
- **Everyone is busy until their action ends**, as boars already are: results come at the end, and a walker is on the path, where they can be seen and met.

### The path there

1. **Now:** the text console, for tests and development. Unchanged.
2. **When the village needs it:** a simple browser page on the same views: the scene as text, the map from memory, what you carry, skill buttons, and the command line. Cheap, and enough for NPCs and two players.
3. **3D, when the world gets real space.** Today the world is places joined by distances. 3D, flight, and orbit all need positions within space. That's the one big engine change ahead. Recommendation: make it when the climb first needs it, at the latest at flight, or earlier if the owner wants to see things walk around sooner. Forms in data can start earlier, since text descriptions and recognition use them too.
