# Memory: knowing what you know

Decided 2026-09-29, from the owner's direction ("Maps, and knowing what you know" in [requirements.md](../requirements.md)). It builds on exploring and seeing from [where am I?](../challenges/where-am-i.md), and comes before the rest of [living with the island](../challenges/living-with-the-island.md), because much will depend on it.

## The idea

The world always knows the truth. A person knows only what's in their memory, and their memory can be wrong. Everything a person knows is one of two things:

- **Certain:** they've seen it for themselves. They've been to a place, walked a way, or looked at what's there.
- **Possible:** they were told, or read it on a map. It's marked with where it came from, and it may be wrong.

Seeing for yourself turns a possible into a certain, or shows it was wrong. And what you saw can change after you've gone: when you come back and it's different, your memory is corrected.

## What a person remembers, for now

Kept to what the island needs:

- **Places:** that a place exists (been there, seen it from afar, or told).
- **Ways:** that you can get from one place to another.
- **What's at a place:** the fixed things and the creatures they saw there, and when they last looked.
- **Claims:** "there's a spring on the slopes", from a map or a person, until confirmed or refuted.

## How it changes

- **Arriving somewhere**, you see it: the place and the way you came become certain; you compare it with what you remember. Anything gone is noticed and forgotten; anything new is remembered. Every claim about the place is checked: the ones that hold become certain; the ones that don't are corrected, and you're told.
- **Reading a map** adds its claims to your memory as possible, marked as from that map.
- **Following a possible way**, you try. If the way is there, you find it, and it's certain. If not, you spend the time looking, find nothing, and correct your memory.
- **Surveying** from a height makes the landmarks you see certain.

## What a person can ask

`recall` lists what you know: what's certain, what's only possible and where it came from, and how many things you've had to correct. A person's datasheet counts them.

## Kept simple

- **A thing is remembered by what it is**, not what it's called; recognition later decides how you'd describe it.
- **Only fixed things and creatures** are remembered at a place: the loose sticks you drop come and go too often to be worth it.
- **Memory is only updated when you arrive somewhere, survey, or read.** Standing in one place, you don't notice things change.
- **Only persons remember.** Creatures acting on instinct know their range.
- **Claims can't be about places that don't exist.** A map can be wrong about ways and about what's at a place; a false place needs a way to name what isn't there, which can come later.

## Later

- Being **told** by another person, which will come with non-player characters.
- **Forgetting** over time, and how sure you are of something seen long ago.
- **Star charts**: the same model, far away. A chart claims a star is there; when you arrive, it might have exploded.

## Built, 2026-09-29

Every person now has one memory, replacing the separate "known ways" and "places seen" of the where-am-I? challenge. Creatures acting on instinct have none.

- **A map is an item that makes claims**, in data: that a place exists, that there's a way between two places, or that something by a name is at a place. `read map` adds them as possible, marked with where they came from. [where-am-i.toml](../../data/where-am-i.toml) has a map scratched on bark in the forest, partly right and partly wrong.
- **Arriving somewhere** makes the place and the way you came certain, confirms or corrects every claim about what's there ("Clay is here, as the map … showed"; "The map … showed a spring here, but there's none. You correct your memory."), and compares what's there with what you last saw ("Gone since you were last here: a boar, a boar, …").
- **A way you were told of can be followed** without exploring. If it's really there, you walk it and it's certain. If not, you spend the time a search takes looking for it and correct your memory.
- **`recall`** lists the places and ways you know, what's only possible and from where, and how many things you've confirmed and corrected. A person's datasheet counts them: "knows: 15 for certain, 2 possible; 2 confirmed, 2 corrected".

Proofs: [memory-1-map.txt](../../data/scripts/memory-1-map.txt) finds the map, reads it, and tests it: two claims confirmed, two corrected, two still possible. [memory-2-changes.txt](../../data/scripts/memory-2-changes.txt) sees the boars asleep by the stream at night, and is told they've gone in the morning.

**Sabotage checks:** claims never checked on arrival, told ways that can't be followed, gone things not noticed, and a false way refused instead of searched for each fail a proof.

Simplifications, as planned above, plus:

- **A map names places by their true names**, since a map can carry names; recognition will decide what a person can make of them.
- **Memory is only of places, ways, what's at a place, and claims about those.** Knowing what's in a container, or who owns what, comes when a story needs it.

