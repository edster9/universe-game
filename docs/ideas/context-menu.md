# The context menu: what you can tell, and what to offer

Proposed 2026-10-01. Not decided. The owner's words are "The context menu: what you can tell, and what to offer" in [requirements.md](../requirements.md). What's built is "Shortcuts, faster gathering, and clicking" in [challenges/first-steps.md](../challenges/first-steps.md).

The owner raised three questions. This proposes an answer to each.

## 1. What hovering tells you, near and far

This is already decided in principle. "Distance limits what you can see" in [vocabulary.md](vocabulary.md) says that at a distance you see less of a thing's look. Today, everyone sees everything in the place by its name, at any distance. That's an oracle.

**Proposal: three bands, by how big the thing is and how far away it is.** It's one law in the engine's view of the scene, so the client can't leak what the islander couldn't know.

| Band | When | What hovering says |
| --- | --- | --- |
| **Made out** | Within about 300 times its size | Your word for it: "a flint knife", "fallen sticks" |
| **Seen** | Within about 3,000 times its size | Only what anyone can see from there: "something small", "something tall" |
| **Not seen** | Further | Nothing: it isn't drawn |

For example, with those factors:
- a 10 cm knife is made out within 30 m, and seen as "something small" up to 300 m;
- a 1 m bush is made out within 300 m;
- a tree is made out within kilometres;
- a person is made out as "someone" from hundreds of metres. Telling which person it is would be a closer band, later.

The two factors are numbers in a body's data (its eyesight), and darkness would shorten them. A "something" can still be pointed at and walked to, but not named in a command: you can't ask for the knife until you can see that it's a knife.

## 2. What the menu offers

| What you clicked | The menu |
| --- | --- |
| **Something you can't make out** | Only "walk up to it". You don't know what it is, so you can't know what to do with it. This is the owner's idea. |
| **Something you've made out, out of reach** | The things you could do there, each walking you up first, with the distance in the menu's title ("fallen sticks, 12 m"). |
| **Something within reach** | What you can do now. |

**Why made-out things keep their actions from afar.** The owner's biggest complaint was too many steps. Clicking a tree to chop it, and walking over to do it, is one click in most games that use clicking (The Sims, RuneScape, Diablo). Offering only "walk to" means two clicks every time, for the commonest case. People with minds of their own already work this way: they walk up first.

**No ghosted choices, for now.** The list of things you can't do is long and mostly noise: you can't eat a stone, or wear a fire. Only what would work is shown. One exception may earn its place later: a hint when a tool would make it work ("gather, needs something sharp"), since trying it tells you that anyway.

New actions, like kicking or pushing, appear in the menu by themselves once their laws exist.

## 3. Doing something more than once

The canned "x3" and "x10" are gone from the menu (2026-10-01, at the owner's request). Gathering is offered once. Typing "gather sticks x3" or "gather wood 500 g" in the console still works. The options:

1. **Keep going until you stop:**
   - a choice (or Shift-click) gathers again and again until you move, press stop, the pack is full, or the patch is bare;
   - a small line on the screen shows how it's going ("gathering sticks: 3, 600 g");
   - most survival games do this.
2. **A count in the menu row:** "gather [-] 1 [+]", with the mouse wheel changing the number. This is like splitting a stack in an inventory.
3. **Leave counting to skills:** a saved "start fire" knows its kit and gathers just that, so counting by hand becomes rare.

**Recommended:** 1 now, and 3 when skills come. A plain click gathers once. Shift-click, or a "keep gathering" choice, keeps going until you stop. Skills take care of exact amounts later. The console keeps x3 and 500 g for when you know exactly what you want.
