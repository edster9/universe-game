# The context menu: what you can tell, and what to offer

Proposed 2026-10-01. **Decided the same day: the owner said "those are all good", adding that sizes must be automatic, measured by the engine for anything, invented ones too, and that knowing what something is depends on the viewer's knowledge. Built: see "Built" at the end.** The owner's words are "The context menu: what you can tell, and what to offer" in [requirements.md](../requirements.md). What's built is "Shortcuts, faster gathering, and clicking" in [challenges/first-steps.md](../challenges/first-steps.md).

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

## Built (2026-10-01)

- **Sight** (`crates/engine/src/sight.rs`). Something is made out within 500 times its size and seen within 5,000 times; both are ten times shorter in the dark. The numbers are the world's settings for now; they become each body's eyesight in data when kinds of eyes differ.
  - **Size is measured, never written:** across a patch, as long as a shape with a length, or else the side of a cube of its volume, from what it's made of. Anything made in play gets a size the same way. A fire ring of five stones measures bigger than one stone, so it's made out from further.
  - **Knowing what it is** comes from the viewer's own words, which were already built (`words.rs`). Something they've never learned is called by its look: its parts, or what it seems made of. A far-folk iron barb shows to the islander as "flake of dark metal". So a rocket engine, to a Stone Age islander, would be a description of its parts and materials however close they stood.
  - A 200 g stick (about 7 cm) is made out within about 35 m. A patch of sticks 20 m across is made out from kilometres away. A person is made out from about 200 m.
- **Where it applies:**
  - **What a person sees** (`look`): what's only seen is "something small" (under 30 cm), "something", "something large" (over 2 m), or "someone", with no mass. What's unseen isn't listed.
  - **What the client draws:** `view::scene` leaves out what's unseen.
  - **Hovering and the menu's title** show the same words.
  - **Names in commands:** a name only finds what's made out; a pointer ("#12") also finds what's only seen.
  - **Replies:** `laws::named` says "something small" too, so no refusal gives a name away.
  - **The menu:**
    - unseen gets no menu;
    - only seen gets just "walk up to it";
    - made out gets the actions, walking up first.
- **"Keep gathering":**
  - **The menu:** gathering offers "gather" and "keep gathering".
  - **The console:** the same is `gather sticks until full`. It goes on until the laws refuse (a full pack or a bare patch, said with the reason) or the player stops it.
  - **Moving stops it:** a WASD key stops it, or anything else being done in the place, and the console tells what it came to.
  - **Progress:** the screen's top line shows how it's going: "gather from fallen sticks until full: 5 so far, +1 kg (move or stop to stop)".
- **Proofs** (`crates/console/tests/sight.rs`):
  - a dropped stick across the forest is "something small", can't be named, offers only "walk up to it", and walking up makes it a lump of wood again;
  - a patch is made out from across the forest;
  - darkness turns a stick 10 m off into "something";
  - a fire ring made in play measures bigger than its stones.
  
  A sabotage that makes everything made out fails two of the three. Played in the client: "keep gathering" walked up and gathered until W stopped it, then told the total.
- **Cost:** none measurable once tuned. The month-long village takes 50.8 s against 50.7 s before sight, on the same machine. The first version measured a whole datasheet for every thing whenever anyone named something, which made it two and a half times slower; it now measures only volume and length, and only for the best matches of a name.
- **Trials, all as before:** the stranger alive after 30 days in 10 of 10, 60 of 60 boars, and every castaway (30 of 30 in each challenge).

**Simplified, and open:**
- Seeing is still within the place you're in. Seeing into the next place, or from a summit, keeps its own law (`survey`).
- Nothing blocks sight: no walls, trees, or hills between. A thing in a container is seen as well as the container, whether or not the container is closed.
- Which person someone is, rather than that it's someone, has no closer band yet.
- What's remembered of places you've left isn't filtered by how well it was seen.
