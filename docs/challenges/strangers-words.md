# A stranger's words

The vocabulary stage, decided 2026-09-29 as the next step (see "Decided" in [ideas/vocabulary.md](../ideas/vocabulary.md), and the order at the end of [ideas/game-interface.md](../ideas/game-interface.md)). It comes before living with the island stages 5 and 6.

## The story

A second castaway washes up on the islander's island, from a people who know metal and cloth but have never seen flint, flakes, or spears. Two people, two sets of words, one island. Scripts play each in turn.

## Stage 1: seeing and saying in your own words

- **Each people has a starting culture in data:** the materials, shapes, designs, and kinds of animal they know, and the word each uses. A person gets their people's culture.
- **Every thing has a look, in plain words from data:** each material has a phrase for how it looks to anyone ("grey stone", "wood", "raw flesh"); each shape a phrase for its form; each kind of animal a phrase for how it looks. The engine only puts them together, so no names enter it.
- **What you see goes through your own words.** The islander sees "a spear"; the stranger sees a long pole with a thin sharp flake of grey stone. An animal you don't know shows as the nearest kind you do know ("an animal"), or by its look.
- **What you can say goes through them too.** The stranger's "take the spear" is refused; "take the long pole" works. Replies speak your words, so the engine never lets slip a name you don't know.
- **Being told.** `tell stranger the spear is a spear` teaches the word, with that spear as an example. The stranger then recognises other spears by resemblance.
- **The same word for different things:** when a word fits several things you can see that differ, the engine asks which, naming the difference; alike things it just picks. See "The same word for different things" in [ideas/vocabulary.md](../ideas/vocabulary.md).

## Stage 2: inventing and naming

- **Joining parts without a design** (`join the flake and the shaft`): the engine measures the result from its parts, as it does any assembly.
- **If it resembles nothing you have a word for,** the engine says so and asks what you call it; `call it a stabber` gives you the word and the recipe, so "make a stabber" works next time.
- **If it resembles something you know,** it's that: a close match by its name, a different material as "a spear, but of metal", anything further as "something like a spear".
- **Resemblance, kept small:** the same jobs (cut, contain, push…), the same kinds of parts, and roughly the same size (half to double, by weight for now).
- **The same word twice is allowed:** naming a second invention "spear" adds another example and another recipe.

## What stays as it is

A world with no cultures in its data behaves exactly as before: everyone knows every name. Existing scripts are untouched; only this challenge uses cultures.

## Proofs planned

- The islander and the stranger see the same spear differently.
- The stranger can't use a word they haven't learned.
- Once told, they recognise a second spear they've never seen.
- Two different spears in hand: "take the spear" asks which, naming the difference; "take the metal one" works.
- An invention asks for a name, and the named recipe works next time.
- A metal-headed spear reads as "a spear, but of metal".
- Sabotage: ignore whose words are used, and the stranger sees "spear" untaught; make every look match, and nothing new is ever new. A test must fail each time.

## Simplified

Size is judged by weight. Places' and people's names stay as they are, governed by memory. Words are English. Told words can be lies, but nothing tests lying yet.

## Results

**Both stages passed, 2026-09-29.** World: `data/strangers-words.toml`, with two cultures, `islanders` and `far-folk`. Plain descriptions of materials, shapes, and kinds (`looks` and `form`) are in `data/island-things.toml`. Proofs: `data/scripts/words-1-two-peoples.txt`, `data/scripts/words-2-inventing.txt`, and `crates/engine/tests/words.rs`.

### Stage 1: seeing and saying in your own words

- The stranger carries an "iron barb" (their word for the shape the islander calls a flake). They see the boar as "a large bristly beast", and the flint as "glassy grey stone".
- The islander makes a spear and sees "spear of flint and wood". The stranger sees "barb of glassy grey stone joined to long straight pole of wood". "Take the spear" is refused ("you don't see the spear here"); "take the pole" works.
- `tell the stranger that the spear is a spear`: the stranger now sees "spear of glassy grey stone and wood", and takes it by that name. **A second spear they were never shown is a spear to them too.**
- A world with no cultures names everything exactly as before; every earlier script passes unchanged.

### Stage 2: inventing and naming

- The stranger joins their barb to a stick: *"You've made something new: iron barb joined to lump of wood. What do you call it?"* They call it a stabber, which keeps the way they made it: "make a stabber" works, and needs another barb.
- To the islander, the stabber is a "flake of dark metal joined to lump of wood".
- **Before anyone names it, the stranger sees the islander's spear as "something like a stabber"**: it does the same job at about the same size, from different parts. That's the owner's "I can see that they have something that resembles a spear". Told it's a spear, the closer word wins.
- The islander takes the barb out of the stabber and makes a spear with it. With two spears in hand, "drop the spear" asks: *"Which spear: the spear of flint and wood, or the spear of dark metal and wood?"*, and "drop the metal one" works.
- To the stranger, the metal-headed spear is a "spear of iron and wood": the same parts as the spear they learned, in another material.

### What the attempt found

- **A loose likeness mustn't beat a word you have.** Once told "spear", the stranger saw their own lone barb as "something like a spear": both cut, at about the same weight. Now a word for a thing's own shape comes before a likeness.
- **Names need an order of closeness.** "Gather flint" asked whether it meant the flint nodules or the spear of flint lying beside them, and "work wood" asked about the lump of wood or the spear of flint and wood. Now a name finds, in turn: what's called exactly that; what the name says a thing is ("flint nodules" before "spear of flint and wood"); a plain piece of the material named; and only then anything that mentions it. Asking "which?" happens only among the closest.
- **The same id can name a material and a kind** (the island's fish), so a culture that knows it learns both.

### Checks have teeth

Each sabotage made the proof scripts fail: everyone seeing the world's own names; everything looking like everything; and never asking which.

### Simplified

- **Size is judged by weight**, and a thing's form only by its parts and their jobs. Anything that cuts, at a similar weight, is "something like" a cutting thing you know.
- **Tools a source needs are still matched by the world's design**, not by look: fish in the shallows need a "spear", and an invented stabber won't do, even though it would work.
- Places and people keep their names. Things named in data, such as a source like "flint nodules", show that name to anyone who knows what they're made of; otherwise they're described.
- Things made in play can still be named by number ("#12"), which is how a client will point at something. Data ids can't be used by people with words of their own.
- Words are English. Told words can be lies, but nothing tests lying yet. "It" means only what you made last. A single thing can't yet have its own name ("Old Faithful").
