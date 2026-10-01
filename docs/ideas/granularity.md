# The one and the many: how fine-grained things are

Proposed 2026-09-30, from the owner's question in [requirements.md](../requirements.md) ("Being near things, agreed; and how fine-grained things are"): what's counted one by one, what's counted in bulk, and what's limited.

## What the engine does today, without having named it

- **Counted one by one:** people and creatures (six boars are six things; kill one and five are left), everything made (a fire ring, a spear, a raft), and every piece taken from anywhere (a 200 g lump of wood, a 1 kg stone).
- **Counted in bulk:** sources, each a mass with a piece size and a time to find one: dry grass (20 kg, 5 g a tuft), loose stones (500 kg, 1 kg each), bog iron (200 kg), standing trees (20 t, 20 kg a piece, with an axe), fish in the shallows (150 kg, 500 g a fish, with a spear). Taking a piece takes its mass from the source, so a source can run out.
- **Some sources grow back**, up to a limit, from a named source: fish and shellfish from the sea (5% a day, up to 300 kg and 600 kg). Nothing else does yet.
- **Nothing is unlimited.** Nothing from nowhere is the first rule: the sea is 1,000 t, not infinite.

## The ruling, proposed

**1. Nothing is unlimited; things are renewable or vast.** What feels unlimited in other games is, here, one of two things: **renewable**, growing back from a named source up to a limit (grass, trees, fish, shellfish), so over-harvest it and it's gone for a while; or **vast**, too big to use up in practice (the sea, the ground's dirt, a mountain's rock). Conservation stays exact, and scarcity is real but rarely final.

**2. Two forms of things: the one and the many.**
- **One (an individual):** counted one by one, with its own spot, and can be pointed at, named, owned, remembered, moved, broken.
- **Many (a stock):** counted by mass, spread over part of a place; taking from it gives you individual pieces; it may grow back.

**3. The boundary, as a rule. Something is one, not part of a many, if any of these is true:**
- **it acts on its own**: a person, a boar, anything with instinct or a mind;
- **someone made or changed it**: a house, a fire ring, a felled log, a half-cut tree, a dug pit;
- **someone took it out of a stock**: the tuft in your hand, the fish you caught;
- **the world's data names it as one**: a landmark, the great oak at the crossroads, a wreck on the reef.

Everything else is part of a stock. **This is the cone of influence applied to things:** the world stays coarse until someone touches it, and touching it makes history, an individual with its own story. The engine only spends memory on what people have touched, which is what lets a world hold many islands and many players.

**4. Your examples, ruled:**

| Thing | Form | When you take from it |
| --- | --- | --- |
| Dirt, sand, the ground | A vast stock | You get a piece; the ground's shape doesn't change |
| Ore, clay, flint | A stock at a spot, finite, not growing | It shrinks and can run out; mine it out and it's gone |
| Grass, sticks, twigs | A renewable stock, spread over an area | It thins, and grows back over days |
| A forest's trees | A renewable stock: the standing trees | Felling one gives you a log (one); the forest thins |
| A landmark tree | One, named in data | Fell it and it's gone for good, and stays remembered |
| Fish in the shallows | A renewable stock | A catch is one fish (one); the shallows refill |
| Boars | Each one is one | Kill one and one fewer is left |
| A house | One (made) | Destroyed, it becomes its parts: rubble, beams, each one |

**5. How the land shows a stock.** The client draws a stock by its mass, as it already does: a forest of 20 t as many trees, fewer as it's cut; a grass patch as tufts, thinner as it's gathered; a deposit as a mound, smaller as it's dug. *Which* tree goes is the drawing's choice (with stage 6's spots, the one nearest the axe). **The ground's shape never changes** from taking: no holes. If digging the land itself matters later (pits, moats, mines, foundations), a dug pit becomes one, by rule 3, "someone changed it".

**6. What follows for gameplay.** Harvesting is real but forgiving: renewables come back, vast stocks don't run out, finite deposits are worth finding and fighting over (which matters for money and trade later). What people build and change persists: the log you cut, the house you built, the cleared corner of the forest. Not every blade of grass is remembered, but everything anyone made is.

## Questions for the owner

1. **The rule of the one and the many** (one if it acts, was made or changed, was taken, or is named in data): agree?
2. **The ground's shape doesn't change from digging**; a dug pit becomes one only when digging the land itself matters: agree?
3. **Forests are a renewable stock**, with landmark trees as ones, rather than every tree being one: agree? (Every tree being one would mean tens of thousands of things per island, most of them never touched.)
4. **Renewal** is a law: grass, trees, and fish grow back from sunlight or the sea, up to a limit, as fish and shellfish already do. And creatures breed back up to what their range can feed, a law not yet built, without which the island's boars only ever dwindle. When should breeding come: with the camp, or now?
