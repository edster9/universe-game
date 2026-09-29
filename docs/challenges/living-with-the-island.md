# Challenge: living with the island

Proposed 2026-09-28 from the owner's direction ("More island scenarios before validation" in [requirements.md](../requirements.md)). A training challenge, after [where am I?](where-am-i.md), whose day, night, and sleep it builds on.

## The story

The island has wildlife: boars, and perhaps other creatures. They're a danger and a source of food, since a boar is a lot of meat. The survivor protects themselves from animals and from the elements: a shelter against the night cold, a barrier or tree house against animals, and shoes and clothing because anything they have wears out.

Animals are the first characters in the world that aren't the player, so this challenge rehearses the non-player characters that come after it: other people, friend or foe, then colonies and a town (slice 3).

## Stages

| # | Stage | What happens | New laws it needs |
| --- | --- | --- | --- |
| 1 | Shelter | Build a lean-to and sleep warmer inside it | An assembly built to shelter keeps in a share of a sleeper's body heat |
| 2 | Animals | Boars roam the forest, feed, breed, and charge when cornered | Animals are living bodies under the same laws as people, with a few rules of behaviour in data: wander, feed, flee, and charge |
| 3 | Injury | A tusk or a spear wounds | Wounds bleed, which is faster fluid loss; they heal slowly. Bleeding to death is dying of fluid loss, a limit that already exists |
| 4 | The hunt | Kill a boar, butcher it, cook the meat, and keep what's left | Butchering divides a body with a cutting tool into meat, fat, hide, and bone. Cooking is a change at a temperature, as clay firing is. Meat spoils with time unless it's dried or smoked |
| 5 | Hide and shoes | Scrape and dry a hide into leather, and make shoes and clothing | Walking wears what's between you and the ground: bare feet on rough ground are hurt and slowed, and shoes wear through instead. Worn clothing cuts a body's heat loss |
| 6 | A barrier or tree house | Keep boars out at night | A barrier blocks what can't get past it; height keeps out animals that can't climb |

## Left out, to keep it simple

Detailed combat moves and anatomy, and disease. Weather belongs with the harder crossings.

## Results

The challenge has its own world, [living.toml](../../data/living.toml): the where-am-I? island, mountain and view included, sharing [island-things.toml](../../data/island-things.toml), with an islander who already knows the way around. Wildlife will live here without disturbing the earlier challenges' proofs.

### Stage 1: shelter. Passed, 2026-09-29.

**Six sticks and four lumps of plant fibre make a lean-to**, which keeps in 60% of a sleeper's body heat. [living-1-shelter.txt](../../data/scripts/living-1-shelter.txt): build it, put it down (it can't be slept in while held), and `sleep in lean-to for 10 h`. A lump of wood "gives no shelter".

- **A design can shelter.** Its data says what share of a sleeper's body heat it keeps in, as a furnace's data says how well it holds heat. Its datasheet shows it: "keeps in: 60% of a sleeper's heat".
- **Sleeping in a shelter** cuts the body's heat loss by that share, for as long as the sleep lasts and the shelter is where the sleeper is.

An engine test compares the same night on the beach: sleeping in the open burns just over 4 MJ, because the body shivers against the cool air; sleeping in the lean-to burns 2.9 MJ, which is just a body's 80 W at rest. **Sabotage check:** a shelter that keeps nothing in fails it.

What the attempt found: the stage table first said "an assembly you can get inside". Being *inside* things, in general, would change how everything around a person is found, and nothing needs it yet. Sleeping in a shelter gives the spirit of it with one small law.

Simplifications to revisit:

- **A shelter helps only a sleeper.** Sitting awake in it does nothing.
- **Rain and wind don't exist yet**, so a shelter only saves food; it can't yet save a life.
- **A lean-to weighs 2.2 kg and can be carried** like any other assembly.
