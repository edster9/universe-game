# Hands, and how things are joined

Proposed 2026-10-02. **Not decided.** The owner's words are "First play of the shortcuts" in [requirements.md](../requirements.md): a lit lantern isn't backpack-friendly and is carried as if equipped in a hand, like a spear, a sword, or a gun; and a fire can be scattered, but a chair can't simply be taken apart.

## What's built already (2026-10-02)

The bug the owner hit was a fire ring taken whole into the backpack, still burning there. Two small fixes close it without a rule about fire:

- **Nothing too hot to touch goes in among your things**, and neither does anything holding something that hot. This is the existing "cool enough to hold" law, extended to what's inside. So a ring with a fire in it can't be taken, and the click menu stops offering "take", because the menu asks the laws. A lit lantern will be fine, once it can be carried in hand (below).
- **Taken apart, a thing's parts stay where it stood**, along with whatever it held. A burning ring taken apart leaves its fire burning on the ground, among hot stones: scattered, as the owner put it, and now an open fire.

## Hands: carrying something ready to use

**The idea:** what you carry is either **packed** (the backpack, as now) or **in hand**. A body has two hands (a number in the body's data).

| | Packed | In hand |
| --- | --- | --- |
| A cold lantern, a candle | yes | yes |
| A lit lantern, a burning torch | no: too hot inside | yes, held by its cool part |
| A spear, a sword, a gun | yes (later: too long to pack, slung instead) | yes: ready to use |
| A tool you're working with | yes | yes, taken in hand when you use it |

**The laws, kept broad:**
1. **Something goes in your pack only if nothing in it is too hot to touch** (built). Holding it in hand needs only that **some part of it is cool enough to hold**: the lantern's frame, the torch's far end, the ring's stones if they're cool (a ring isn't really liftable; see "joins" below).
2. **What you use, you use from your hands.** Striking with a spear, cutting with a knife, rubbing two sticks: if it's packed, you take it in hand first, which takes a moment. For now, that happens on its own when you use something; later, being ready (sword drawn) matters in a fight.
3. **Two hands are two hands.** Holding a lit lantern and a spear leaves none free to gather. That's a real constraint, and it's what makes "equip" meaningful.

**Commands:** `hold <thing>` (equip: take it in hand), `pack <thing>` (put it away), and the backpack panel shows "in hand" above what's packed. `drop` works on either.

**When:** after the knife (the next skill), because the knife, the spear, and a torch to see by at night are the first things you'd hold. A small step: two hands, hold and pack, the hot rule moved to the pack only, and tools taken in hand when used.

## Joins: what can be taken apart

Today, anything put together can be taken apart into its parts, instantly. The owner's point: a fire can be scattered, but a chair isn't simply taken apart. **The difference is how the parts are joined**, which belongs in each design's data:

| Joined | Like | Taken apart |
| --- | --- | --- |
| **Laid** | a ring of stones, a cairn, a woodpile | By hand, at once: it falls apart. You can't lift it whole either; you'd carry the stones |
| **Lashed** | a spear (flake bound to a shaft), a raft, a lean-to | By hand, taking a while to untie; the lashing comes back |
| **Fixed** | a chair (pegged), a fired furnace, anything glued, riveted, or welded | Not by hand. Only broken, with a tool and force, losing some of it. That needs the blunt-harm law ([harm.md](harm.md)) |

The click menu would then offer "take apart" for laid and lashed things, and nothing (later "break") for fixed ones. **When:** with the axe (the first lashed tool that matters), or sooner if the owner wants the fire ring to stop being carried whole. One field in a design (`joined = "laid"`), and the lift law refusing laid things.

**One consequence to accept:** several old scripts carry an assembled fire ring from one place to another. With laid rings, you'd carry the stones and build the ring where you want the fire, as people do. Those scripts would change.

## The graphics layer: DirectX, Metal, Vulkan

The owner asked whether we use DirectX on Windows rather than Vulkan, and what macOS needs. **The game's drawing code is written once, against wgpu (through Bevy), which translates to each platform's own graphics system:**
- **Windows:** DirectX 12, the native one. Vulkan also works there; we switched to DirectX on 2026-09-30 after Vulkan lost the graphics device once on this laptop.
- **macOS:** Metal, Apple's own. Apple doesn't support Vulkan; wgpu uses Metal directly, so nothing changes in our code.
- **Linux, later:** Vulkan.

So it's the same game on both, and only the bottom layer differs, as the owner said: whatever works. **The slow start was DirectX's old shader compiler:** without Microsoft's newer one (two DLLs beside the program) it spent about 4 seconds compiling about 50 shaders before drawing the ground; with them, about 1.2 seconds, a little faster than Vulkan's 2. `client/run.sh` now fetches them once, like the voice model. `client.exe --frames` prints the shader count as they compile.
