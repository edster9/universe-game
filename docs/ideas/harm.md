# Harm: how things cause damage

Recorded 2026-09-29 from the owner's direction ("How things cause harm" in [requirements.md](../requirements.md)). A structure to build toward, one piece at a time. Only the first way, the edge, is built.

## The idea

The engine doesn't know "weapons". It knows **ways harm is delivered**, each grounded in physics, and anything that delivers harm one of these ways does harm, whether it was made to or not. A weapon's class is data: which way it delivers harm, and its numbers.

| Way | What it is | Examples | Status |
| --- | --- | --- | --- |
| **Edge** | Force concentrated on a fine edge or point cuts or pierces. The finer the edge, the worse the wound; wounds bleed. | Spear, knife, axe, arrowhead, tusks, claws | Built (living 3) |
| **Blunt** | Force spread over a face bruises and breaks rather than cuts. The heavier and faster, the worse. | Hammer, club, a fall, a charging animal's head | Planned |
| **Projectile** | Something thrown or fired carries kinetic energy, ½·m·v², and delivers it through its own edge or face when it lands. Throwing speed comes from the thrower's effort; firing speed from stored or chemical energy. | A thrown rock, a spear, an arrow, a bullet | Planned |
| **Blast** | Chemical energy released almost at once, spreading outward and falling off with distance. It's the same energy the gate already accounts for, only released all at once. | Explosives | Planned |

Heat, cold, poison, and disease are harm too; heat and cold already kill through a body's limits. They'll fit alongside when stories need them.

## How it fits what's built

- **An edge is already measured.** A cutting shape's datasheet gives its edge width, and an assembly takes its edge from its part. A wound's bleeding rate comes from that width.
- **A kind's natural weapon is written by its way**, `weapon = { edge = "5 mm" }` for a boar's tusks, so a kind can later say `{ blunt = … }` for a charging head or a club-like tail without the data changing shape.
- **What a blow does is a change through the gate**, like any other: a wound today, a break or a bruise later.
- **Projectiles reuse the laws of motion that paddling already started**: speed from power against drag, and energy from mass and speed.

## Kept simple

One way at a time, each added when a story needs it, and each as a law about energy and contact rather than a list of weapons.
