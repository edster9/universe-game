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
