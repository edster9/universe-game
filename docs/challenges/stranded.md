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
| 1 | Stay alive | Drink from the stream, gather shellfish by hand, get hungry and thirsty, and die if neither is fixed | Bodies are matter and burn their stores to live; eating and drinking; limits of life; sunlight; living things grow; chance |
| 2 | Fire | Rub two dry sticks until one catches, then feed the fire | Work becomes heat; things catch fire above their ignition temperature, anywhere |
| 3 | Spear and fish | Knock a sharp edge on flint, cut a branch, shape a spear, fish | Taking material from living things; the chance of a catch depends on the tool and on how many fish there are |
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

## Reporting

Each stage's test reports in [slices.md](../slices.md)-style: what was built, what passed, the pass rate over many seeds, how the failed runs ended, and what's simplified. After stage 8, a validation challenge follows, with the laws frozen.
