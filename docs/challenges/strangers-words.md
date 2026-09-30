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

Not started.
