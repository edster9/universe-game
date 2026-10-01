# Being near things: space within places

Proposed and **agreed 2026-09-30** ("all those are fine") for stage 6 of [first-steps.md](../challenges/first-steps.md), from the owner's words in [requirements.md](../requirements.md) ("Played from the actor's perspective", "Moving by keys, and by commands"). A design to agree before any code.

## Where we are

The world is places joined by paths. Being "at the forest" means everything in the forest is at hand: every law that needs a thing within reach asks one function in the engine (`Reach::of`), and it answers "everything in the same place". Walking between places takes the path's length at the walker's pace. The client scatters things around a place's middle only for drawing; the engine has no idea where in a place anything is.

## The proposal, in short

**Places stay the unit of the world; inside each, things get a spot.** Walking within a place is free movement; leaving a place is walking a path, as now. One new law (reach), one new action (walking within a place), and some data.

### 1. Spots

Everything at a place (people, creatures, loose things, sources) has a spot: east and north in whole micrometres, in the same frame as the places' own positions. The place stays where it is in every other law: what you see, the air, the weather, darkness, memory.

- **Sources spread.** Dry grass, fallen sticks, and loose stones aren't at a point: data gives a source a spot and a spread ("over 40 m around a point 20 m north of the middle"). You can gather it anywhere within its spread. A source with no spot in data covers the whole place, so older worlds and scripts are unchanged.
- **A place has a size** in data (how far from its middle you can walk), 50 m by default.
- **What you drop lands at your feet.** What you make stays where you made it.

### 2. Reach

**A person reaches 2.5 m** (a number on the kind in data, so a boar's is its own). Laws that need a thing at hand (take, put, give, gather, dig, light, rub, strike, eat from a source) need it within reach, or within reach of its spread. Otherwise they refuse, saying how far and which way: "The dry grass is too far away: 14 m north-east."

- **Names still find anything you can see** in the place (no oracles, unchanged): you can name the grass across the clearing; you just can't pick it from here.
- **Talking** (ask, tell, offer) reaches anyone in the place, for now: shouting distance is a later refinement.
- **Seeing** stays the whole place, for now: line of sight is later.
- **Creatures and minds walk to what they act on** before acting: a boar closes in before it strikes; the stranger walks to the shellfish before gathering. They're programs, so they can plan two steps.

### 3. Walking within a place

A new action, **walking towards a spot or a thing**, at the walker's pace (the same pace and load rules as paths):

- `go to the grass` (or `walk to the fire ring`) walks to within reach of it. It's a commanded walk, like `go forest`, and can be cut short.
- **WASD walks the islander** in the client, relative to the camera's direction, as in any third-person game. To the engine it's the same action, a walk towards a point a little ahead, renewed while the key is held.
- **Pressing a walking key during a commanded walk takes over**, as the owner described: the walk is cut short where the islander is, and the keys move them from there.
- You can't walk beyond the place's size. Walking to its edge in the direction of a path sets off along the path, as `go <place>` does.
- **Arriving at a place** puts you at its middle, as now. So the old scripts, where everything is at hand at the middle, still pass.

### 4. Between places (unchanged for now)

A walk between places stays one action, as today. **Taking over with WASD works within a place, not on a path between places:** there, the walk finishes and you arrive. Making paths into space too (being somewhere along the way, turning back halfway) is a later step, once nearness within places plays right.

### 5. Keys in the client

The owner's idea: **WASD always walks the islander, and the arrow keys move the camera**, so both work at once, whether orbiting or flying. F still switches between orbiting and flying; in flight the arrows fly, E and Q go up and down, and WASD still walks the islander.

### 6. What the client shows

- `look` adds how far things are and which way: "dry grass (12 m north-east), the fire ring (at hand)".
- Things are drawn at their spots, sources across their spread, and a faint circle at the islander's feet shows their reach (a debugging setting, `/reach`).
- The grid shows the walking clearly.

## How it's proved

- Scripts with spots: too far ("too far away: 14 m"), walk closer (`go to the grass`), done; and the fire made by walking up to each source in turn.
- Every older script passes unchanged (sources with no spot cover their place; arrivals are at the middle).
- A creature closes in before it strikes; the stranger walks to the shellfish to eat.
- Sabotage: reach ignored (the too-far scripts fail); arrivals somewhere other than the middle (older scripts fail).

## Questions for the owner (all agreed, 2026-09-30)

1. **Places stay, with spots inside them**, rather than one continuous world where a place is just a named area. Agree? (A continuous world would need the engine to know the land's shape between places, which only the client invents today.)
2. **Too far is refused, with the distance and the direction**, and `go to <thing>` walks there; or should "gather grass" walk there by itself and then gather?
3. **WASD walks, the arrows move the camera**: agree?
4. **Taking over a walk works within a place first**, and paths become space later: agree?
5. Talking and seeing stay place-wide for now: agree?
