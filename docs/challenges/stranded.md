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

**Trial: 30 of 30 survivors were alive after 10 days.** The 300 island-days took about 21 seconds to compute. After ten days on seed 1, the shellfish bed was down from 60 kg to 15 kg, body fat from 9.8 to 9.1 kg, body water steady near 41 kg, and body temperature 310 K. Mass and energy were conserved in every run.

What the tests showed:

- **Thirst kills in about three and a half days** at rest, and in about 12 hours of hard work in the heat. The body stays where it fell, as matter, with whatever it was carrying. (The first data had death after losing 5 kg of water, which killed a hard worker in under 9 hours; it's now 7 kg, about 10% of body weight.)
- **Proofs:** four scripts. Ten days survived with the right routine; death by thirst with no water; the same when only trying the sea; and death within a day working hard without drinking.
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

### Stage 2: fire. Passed, 2026-09-28; simplified the same day.

**Fire from two sticks**, from general laws. In [stranded-2-fire.txt](../../data/scripts/stranded-2-fire.txt):

1. Rubbing two sticks for a minute wears off dust, and friction heats it to over 1,000 K, where it smoulders.
2. A tuft of dry grass on the ember catches.
3. Feed the fire one size at a time, once the last size has caught: more tufts, then twigs, then sticks.
4. The sticks burn at about 1,400 K for ten minutes and more, leaving glowing ash.

**No rule says "tinder, then kindling, then fuel".** That order comes out of size, surface, and heat.

The mistakes fail the way they would in reality, and nothing tells them to:

- **Everything piled on at once** before there's a flame: the ember's heat is spread too thin and nothing catches.
- **Skipping the twigs:** burning grass can't bring a 200 g stick up to its ignition point.
- **An ember against a stick:** the stick barely warms.
- **Rubbing two stones:** stone dust gets warm, but stone doesn't burn, so there's no flame. (Real stone fire-lighting strikes sparks from flint and iron pyrite, which isn't modelled.)

**Sabotage check:** switching off "a flame heats what it's piled with" makes the fire proof fail.

What the attempts found, and what changed:

1. **Heat loss depends on size**, in every world now. Objects lose heat through their surface (convection), plus radiation, which dominates once something glows: 100 cm² at 1,300 K sheds about 1.45 kW, hundreds of times what it sheds at 400 K. With the old flat rate, a speck of dust could never get hot, so no friction fire was possible. It also changed slice 1: a 3 kg iron casting now takes about two hours to cool enough to hold, which is realistic, where the flat rate said 17 minutes.
2. **A flame heats what it's piled with.** A burning piece passes half the heat it releases to the things held with it, split by their surfaces. That's how fire spreads.
3. **Rubbing is an activity that takes chosen time** (`rub … for 1 min`), advanced by nature each second. A living worker's effort becomes heat in the dust, and the dust passes heat to the two things being rubbed.
4. **Soft things can be divided by hand.** Anything harder than bare hands can pull apart (wood, stone) can't. No longer needed for fire, but kept: it's general and cheap.
5. **An assembly can hold things**: a ring of five stones is a fire ring. A design says whether what's built to it holds things, and a slot can take any piece of a material. The ring's own stones are parts, not things in it.
6. **Three engine bugs, found by the attempts and the random tests:** carbon dioxide made by burning briefly made dust's size unknown; a speck touching many things gave away heat until it reached 0 K; and a burning speck being rubbed could turn to gas and merge into the air while still in use. All fixed: a piece's volume counts the materials whose density is known; a piece never gives away more than would bring it to its coldest neighbour's temperature; and something in use can't be merged away.
7. **Naming:** "smallest grass" and "largest wood" pick by mass. "rub wood against wood" means two different pieces.

**Simplified after the first version** (see "Enough physics" in [world-engine.md](../ideas/world-engine.md)). The first working fire also passed heat by touch between everything in the ring. That made it fussy: grass had to be pulled into pinches, packed before rubbing, and fed in an exact order, and piling on cold fuel too early smothered it. Removing the touch law inside piles (flame spreads fire; touch only matters between dust and the sticks being rubbed), and gathering grass as 5 g tufts and twigs at 20 g, left the sturdy rhythm above.

Simplifications to revisit:

- **Each piece has one temperature throughout.**
- **No oxygen, blowing, wind, rain, or damp wood.** No chance is involved in fire yet.
- **Fire stays in its container**, and burns don't hurt the survivor.

### Stage 3: spear and fishing. Passed, 2026-09-28.

Built under the standing rule: nothing more detailed than the spirit needs.

**Making a spear uses laws that already existed.** A stone knocks a flake off flint, because stone is harder than flint on the engine's scale. The flake shapes a stick into a shaft, and flake and shaft assemble into a spear with a 5 mm flint edge.

**Fish need a spear, and luck.** A search of the shallows takes 15 minutes and finds a fish six times in ten while they're plentiful, less as they thin. Without a spear the engine refuses: "you need a spear".

**Living things grow back.** Fish and shellfish grow toward a limit, quickly while few and slowing as they fill up. They take their matter from the sea and their energy from sunlight, the one named way energy enters the world. The gate tracks exactly how much sunlight has come in, and conserves everything else.

- **Proofs:** with average luck, making a spear and fishing lands three fish in three searches, after the engine refused the first bare-handed try. With bad luck, every search comes up empty and the time is still spent.
- **Growth:** a shellfish bed thinned to under 45 kg grows back by more than 15 kg in a month, never past its limit. Over five days, sunlight brings in exactly the chemical energy the growing fish and shellfish store, to the microjoule.
- **Trials:** a survivor with a first real skill (make a spear, then fish when hungry) lived through **30 days on 30 of 30 islands**. On seed 1 the fish grew from 150 kg to 209 kg while being fished, so the life is sustainable. The shellfish survivor is still 30 of 30 over ten days.
- **Sabotage check:** removing the tool requirement fails the spear proof.

What the attempt found:

- **Naming things in hand first.** "work flint …" matched the flint *source* on the ground exactly by its ID, ahead of the flint in hand, and shaped the hillside's nodules into a flake. Working something, and measuring it, now look at what you're holding before what's around you.

Simplifications to revisit:

- **Sunlight is an unlimited named inflow.** A place doesn't yet have a sunlight budget that caps growth.
- **Fish are one population with a catch chance**, not individual animals.
- **Knapping is shaping:** flint is just a little softer than stone on the engine's scale, which isn't true to real hardness but keeps one law for all shaping.
- **Raw fish is eaten raw.** Cooking isn't modelled.

### Stage 4: furnace and axe. Passed, 2026-09-28.

**From bog iron to an axe.** [stranded-4-axe.txt](../../data/scripts/stranded-4-axe.txt) is the whole recipe, written as a script: in effect, a skill.

1. **Gather everything first:** sticks, grass, twigs, driftwood, clay, bog iron, flint, and twelve stones. Gathering takes hours, and a fire made first would be out before the furnace was ready.
2. Knock a flint flake and shape a lump of clay into an axe-head mould.
3. Build a fire ring and a furnace (six stones and two lumps of clay).
4. Make fire, and light the furnace from it.
5. The mould, in the furnace, fires hard at 900 K as the furnace heats. Wood takes the furnace to about 1,970 K, which melts the iron (1,811 K) out of its rock but not the rock itself (2,200 K).
6. Pour the molten iron into the mould and let it all cool for two hours. An iron axe head comes out with a 5 mm edge.
7. Hone the edge on a stone for 30 minutes, down to 2 mm, grinding off 18 g of iron. Fit a flint-shaped haft.

The mistakes fail on their own: **an unfired mould** can't take the iron (unfired clay slumps at 1,500 K), and **a furnace with no flame nearby** won't light.

**Sabotage check:** switching off clay firing fails the axe proof.

New laws, all data-driven:

- **A furnace is an assembly whose design encloses heat.** The design says so, with its burn rate and heat loss. The slice 1 hearth is the same thing, given whole in data.
- **A mould is a shaped piece whose shape casts.** Clay worked into an axe-head mould becomes a form.
- **Materials change at a temperature.** Clay becomes fired clay at 900 K.
- **Fire comes from fire.** Lighting a furnace or hearth needs something burning within reach. Until now `light` made fire from nothing; the slice 1 forge now has a campfire to light its hearth from.
- **Sharpening:** rubbing any shaped part against anything solid makes it finer, so an edge can be honed on a stone.
- **Pouring** can go into a form sitting inside another container, such as a mould in a furnace.

What the attempts found:

- **An engine bug:** a design slot asking for "any piece of clay" took the clay *mould*, which the gate then refused as a part. Such a slot now takes only a plain, unshaped lump.
- **Order matters, as in real life:** a fire made before hours of gathering has gone out by the time it's needed. And a big log or a mould put on a young fire takes its heat and smothers it.

Simplifications to revisit:

- **Iron melts out of its rock** rather than being reduced to a bloom and hammered, as a real bloomery works.
- **Metal poured into a form inside a furnace cools in the open**, not at the furnace's temperature.
- **Moving between places takes no time**, which makes gathering everything first cheaper than it should be.

### Stage 5: timber. Passed, 2026-09-28.

**Felling reuses the gathering law.** Standing trees are a source that needs an axe, and each 20 kg log is one piece of work. Two small additions made it work:

- **Work with a tool takes longer the blunter the tool.** A source can say how long a piece takes with a given edge (for trees, an hour with a 1 mm edge), and the time scales with the tool's actual edge width. The honed 2 mm axe fells a log in exactly **2 hours**; the same axe unhoned, with its 5 mm cast edge, takes **5 hours**. Honing now pays off in hours saved.
- **A tool must be harder than what it takes from.** An assembled tool cuts with its edge's hardness, as its datasheet measured it.

Proofs: [stranded-5-timber.txt](../../data/scripts/stranded-5-timber.txt) casts, hones, and hafts an axe, drinks, and fells two logs at two hours each. [stranded-5-blunt-axe.txt](../../data/scripts/stranded-5-blunt-axe.txt) skips the honing and takes five hours. Trees can't be felled by hand. **Sabotage check:** making the edge not matter fails both timing proofs.

**Scripts can now include recipes.** The axe-making steps live once, in [skills/cast-an-axe-head.txt](../../data/scripts/skills/cast-an-axe-head.txt) and [skills/hone-and-haft-an-axe.txt](../../data/scripts/skills/hone-and-haft-an-axe.txt), and the stage 4 and 5 proofs include them. This is a first, hand-written form of the skills in [skills-and-interface.md](../ideas/skills-and-interface.md).

Simplifications to revisit:

- **Effort doesn't change the time.** Only the edge does; a stronger or more skilled worker isn't faster yet.
- **No carrying limit, and walking takes no time.** A person could carry every log at once. These start to matter with the raft and the crossing.
- **Trees don't regrow**, and a felled log is a plain lump of wood with no shape.
- **The daily test run grew to about 24 seconds**, because several proofs replay the whole axe recipe from nothing.

### Walking and carrying. Added 2026-09-28.

Not a stage of its own, but needed from here on: hauling logs to the beach, and any character walking around a world.

- **Paths between places have lengths** in data. Walking takes the distance at the walker's speed (1.2 m/s here), and it's hard work, burning energy and water like any other.
- **A body has a carrying limit** (40 kg here). Taking, gathering, digging, or being given more is refused.
- **Load slows you down**, to half speed at a full load.

[stranded-walking-and-carrying.txt](../../data/scripts/stranded-walking-and-carrying.txt): 300 m takes 4 min 10 s unloaded and 8 min 20 s under 40 kg of driftwood, and a 21st log is refused. Worlds without distances, like the slice worlds, still walk instantly.

Simplification: the carried mass counts, not its bulk. Twenty driftwood logs weigh 40 kg; nothing yet says they're too awkward to hold at once.

## How it's tested

Decided 2026-09-28.

- **Proofs, every run.** Scripts in [data/scripts/](../../data/scripts/) play the island with luck fixed: average (every roll lands in the middle), good, or bad. The right steps succeed every time, and deliberate mistakes end as they should. Fixed luck, not a fixed seed, because a seed's luck shifts whenever the engine's history changes.
- **Trials, on demand.** A survivor plays many seeds with real chance, and the success rate and causes of death are reported here. Only getting stuck, or a conservation failure, fails a trial.

## Reporting

Each stage's test reports in [slices.md](../slices.md)-style: what was built, what passed, the pass rate over many seeds, how the failed runs ended, and what's simplified. After stage 8, a validation challenge follows, with the laws frozen.
