# Rendering: how the game is drawn

Recorded 2026-09-29 **as a placeholder for a discussion the owner has asked for**. Nothing here is decided. The requirement is "Vitality, and moving to real time and 3D" in [requirements.md](../requirements.md).

## The owner's starting points

- Something like a Grand Theft Auto engine is very good for walking around, driving a car, and flying an airplane.
- There may be several modes: on foot; in a cockpit when flying a spacecraft; a third-person view of your own ship.

## What's already decided that bears on it

- **The engine never draws.** It sends views as data, holding only what a character perceives; clients draw them ([game-interface.md](game-interface.md)). A renderer is a client, and can be swapped.
- **Things are drawn from what they're made of:** each shape has a simple form in data and each material a surface look, and an assembly is drawn by putting its parts together ([vocabulary.md](vocabulary.md)). So the renderer must be able to draw things nobody modelled by hand.
- **Real time, one to one** for bodies and movement ([game-interface.md](game-interface.md)).
- **The world needs real space first.** Today it's places joined by distances. 3D, flight, and orbit need positions within space.

## Questions for the discussion

- Which engine or library: a full game engine (Unreal, Unity, Godot), or a web renderer (three.js, Babylon.js), given the plan for browsers and AWS ([technology.md](../technology.md))?
- Which modes, and when: on foot, vehicles, cockpit, third-person ship, a map view.
- How much detail comes from the engine's forms, and how much from artists?
- How does it stay in step with a server that owns the truth: what's predicted on the client, what waits for the server?
