# Challenge: stranded

Proposed 2026-09-28. The first training challenge: someone is dropped on an island with nothing, and has to get to the next island on a raft they built. It trains the engine on survival, fire, metal, wood, rope, and boats. Its laws are recorded in [laws.md](../laws.md).

## Why challenges, and how they're judged

A challenge is a story the engine should be able to play out from its laws alone. The first challenge is **training**: it shows which laws are missing, and we add them, stated as generally as possible. Later challenges are **validation**: no new laws are allowed, and anything the engine can't do is recorded as a finding, not patched on the spot. Validation shows whether the laws we added are general or were props for one story.

**Dying is an outcome, not a failure.** The challenge is played many times with different luck (different seeds), and should succeed about as often as a real person would. The report says how many runs reached the other island, and how the others ended: hunger, thirst, cold, drowning.

**The challenge fails only if the survivor runs out of logical options**: a step with no way to do it under the laws. That is a missing law.

**The survivor** is a decision-maker that plays the island through ordinary commands, as a player would. It lives in the tests, not the engine. Improving it is improving survival skill. The engine's laws don't change for it.

## The island

- **The beach.** Shellfish on the rocks, fish in the shallows, driftwood, and a view of the next island about 2 km across open water.
- **The forest.** Standing trees, fallen branches, and bushes with fibrous stems.
- **The stream.** Fresh water, clay in the bank, and lumps of bog iron in the streambed. Bog iron is a real historical source of iron on land with no mines.
- **The hillside.** Loose stone, and flint that breaks with a sharp edge.

The person arrives with nothing but their body, and, until knowledge exists in slice 4, knows how to do everything.

## Stages

Each stage has a test. A stage passes when the survivor can do it through commands, the laws decide the outcome, and mass and energy stay conserved.

| # | Stage | What happens | New laws it needs |
| --- | --- | --- | --- |
| 1 | Stay alive | Drink from the stream, gather shellfish by hand, get hungry and thirsty, and die if neither is fixed | Bodies are matter and burn their stores to live; eating and drinking; limits of life; gathering by hand; chance |
| 2 | Fire | Rub two dry sticks until one catches, then feed the fire | Work becomes heat; things catch fire above their ignition temperature, anywhere |
| 3 | Spear and fish | Knock a sharp edge on flint, cut a branch, shape a spear, fish | Taking material from living things; the chance of a catch depends on the tool and on how many fish there are; sunlight; living things grow |
| 4 | Furnace and axe | Gather bog iron, build a furnace from stone and clay, shape and fire a clay mould, smelt, cast an axe head, fit a handle, sharpen it | An assembly that encloses heat acts as a chamber; a shape that casts is a form; materials change at a temperature (clay fires hard) |
| 5 | Timber | Fell trees and cut logs, taking hours with a crude axe | Work takes time, set by the tool's edge and the effort |
| 6 | Rope | Strip fibre from bushes and twist it into rope | Tension: a material's strength when pulled, and a pulling role that's measured |
| 7 | Raft | Lash logs together with rope | Buoyancy: it floats if it's less dense than the water it displaces, and its datasheet says what load it carries |
| 8 | The crossing | Shape a paddle and cross 2 km of open water, with some risk | Propulsion against drag gives speed; some routes can only be crossed in something that floats, taking distance divided by speed |

Stages 1 to 3 teach the engine what the stone age is. Stage 4 reuses the smelting chain from slice 1. Stages 7 and 8 introduce vessels and routes, which is the pattern space travel will use later.

## The body

People become matter, like everything else, so the same laws apply to them:

- A body is made of materials: flesh, fat, and water.
- **Living burns stored energy.** A body burns its fat at a resting rate, and faster when working, like fuel burning slowly. The heat keeps it warm, and it loses heat to its surroundings like anything else. At rest in mild air, the two balance near body temperature, so cold places cause hypothermia on their own.
- **Water is lost** through breath and skin at a steady rate, faster when working or hot.
- **Eating and drinking** add food and water to the body. Food's stored energy is what the body then burns.
- **Life has limits.** Below a share of body water, or with no stored energy left, the person dies, and their body stays where they fell, as matter.

## Energy from outside

Until now, nothing entered or left the world. Living things that regrow need energy, and conservation says it has to come from somewhere. The one named source is **sunlight**: each place receives energy at a known rate, and it's accounted for like everything else. This is the same pattern money follows: things enter only through named sources.

## Chance

Whether a fish bites, a stick catches, or a wave swamps a raft is partly luck. The world holds a **seed**, and every chance event draws from it in a fixed order, so the same seed and the same commands still replay exactly. Different seeds are different luck.

## Time

The island takes days. When nothing fast is happening (no fire, nothing hot, nobody travelling), nature takes bigger steps, so days of game time pass in seconds of real time, and still replay exactly.

## Results

### Stage 1: stay alive. Passed, 2026-09-28.

The island is [data/stranded.toml](../../data/stranded.toml). The survivor is in [crates/engine/tests/stranded.rs](../../crates/engine/tests/stranded.rs).

**30 of 30 survivors were alive after 10 days.** The 300 island-days took about 21 seconds to compute. After ten days on seed 1, the shellfish bed was down from 60 kg to 15 kg, body fat from 9.8 to 9.1 kg, body water steady near 41 kg, and body temperature 310 K. Mass and energy were conserved in every run.

What the tests showed:

- **Thirst kills in about three days.** A person who never drinks dies of thirst after 48 to 84 hours. Their body stays where it fell, as matter, with whatever it was carrying.
- **The sea can't be drunk and the stream can.** Nothing names either: you can drink something only if it's nothing but the fluid your body needs.
- **Eating keeps what the body digests** and leaves the shells in your hand.
- **A resting body holds 310 K** because 80 W of burned fat balances the heat it loses to 300 K air. No rule sets body temperature.
- **Hard work makes you thirstier:** twice the fluid loss, plus sweat. Sweat holds a working body within a kelvin of its set point.
- **Take the stream away** and the survivor tries the sea, is refused, and dies of thirst. That's an outcome, not a failure.

What the first runs found, and what changed:

1. **An engine bug.** When someone died while carrying something, the world's structure check rejected the death, because a dead body couldn't hold things. Now a body keeps holding what it carried.
2. **A law that was wrong.** Gathering made the survivor search until it found something. As the shellfish thinned, one search ran seven hours in the heat with no break to drink, and two survivors died of thirst mid-search. The law is now: **a search takes a fixed time, and the thinner the source, the less likely it finds anything.**
3. **A survival-skill mistake.** The survivor judged hunger by body fat, which eating shellfish doesn't restore, so it gathered nonstop and stripped the shellfish bed in two days. It now eats again once its last meal is used up.
4. **A law with no test.** Switching sweating off didn't fail anything, because two hours of work only warms an unsweating body 5 K. A test now checks sweat holds a working body near its set point, and the sabotage fails it.

Simplifications to revisit:

- **Walking is instant**, and there is no day and night: the air is always 300 K.
- **Shellfish don't regrow yet.** On this island the food runs out in two to three weeks. Growth and sunlight arrive with fish in stage 3.
- **Energy is stored in whole microjoules per holder**, which can't hold an ocean's heat. The sea here is 1,000 t. Planet-sized bodies will need bigger numbers.
- **The body is simple:** no stomach or digestion time, no shivering, and no illness.

## Reporting

Each stage's test reports in [slices.md](../slices.md)-style: what was built, what passed, the pass rate over many seeds, how the failed runs ended, and what's simplified. After stage 8, a validation challenge follows, with the laws frozen.
