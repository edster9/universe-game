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

Not decided. Interface and time are treated together, with [time away](time-away.md), because they're one question: how the world runs when nobody is typing.

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
