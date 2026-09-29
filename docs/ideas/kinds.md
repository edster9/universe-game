# Kinds: classifying living things

Decided 2026-09-29, from the owner's direction ("Classifying living things" in [requirements.md](../requirements.md)). Built with the animals stage of [living with the island](../challenges/living-with-the-island.md).

## The idea

Every living thing belongs to a **kind**, and kinds form a tree, written in data:

```
living thing
├── animal
│   ├── mammal
│   │   ├── human
│   │   └── boar
│   ├── fish
│   └── shellfish
└── plant
    ├── tree
    └── shrub
```

A human is a mammal, a mammal is an animal, an animal is a living thing. An alien species, later, is just another branch: a kind under "animal", or under a new branch of "living thing", with nothing special in the engine.

## What a kind carries

- **A label and a parent.** That's the taxonomy.
- **Optionally, a body:** what members are made of and the figures of their life (resting power, fluid, sleep, walking speed, and so on). Members inherit it from their nearest kind that has one, so a herd of six boars is written once.
- **Optionally, a mind**, which is what drives a member:
  - **A person** acts on commands. Today that's the player; later it's a non-player character, or an intelligent alien. Intelligence is a kind of mind, not a branch of the tree, so an alien person is exactly as much a person as a human.
  - **Instinct** acts on a few rules its kind gives in data, such as what it flees, and otherwise looks after its own body: drinking when thirsty, foraging when hungry, sleeping at night, wandering its range. An instinct produces ordinary commands, and the laws decide what happens, as for any person ("AI proposes, the engine decides").

## What anything can have a kind

- **An individual**: a person, a boar.
- **A population**: fish in the shallows, shellfish on the rocks, standing trees. These stay one source that grows back, as now, but they're classified too.

## What the engine knows

Only "is this a kind of that?", and what a mind is. It never names a kind: "boar" and "human" are data, like "iron". A rule such as "boars flee humans" is written in the boar's data.

## Kept simple

- **One parent per kind.** No cross-cutting groups (predator, domesticated) until a story needs them.
- **Kinds don't carry laws.** Laws stay general; a kind carries numbers and rules for its instinct.
- **Everyone sees true kinds for now.** Knowing what something is will come with [recognition](recognition.md): a castaway who has never seen a boar sees "a large, bristly animal".
