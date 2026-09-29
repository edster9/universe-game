# Challenge: living with the island

Proposed 2026-09-28 from the owner's direction ("More island scenarios before validation" in [requirements.md](../requirements.md)). A training challenge, after [where am I?](where-am-i.md), whose day, night, and sleep it builds on.

## The story

The island has wildlife: boars, and perhaps other creatures. They're a danger and a source of food, since a boar is a lot of meat. The survivor protects themselves from animals and from the elements: a shelter against the night cold, a barrier or tree house against animals, and shoes and clothing because anything they have wears out.

Animals are the first characters in the world that aren't the player, so this challenge rehearses the non-player characters that come after it: other people, friend or foe, then colonies and a town (slice 3).

## Stages

| # | Stage | What happens | New laws it needs |
| --- | --- | --- | --- |
| 1 | Shelter | Build a lean-to and sleep warmer inside it | An assembly built to shelter keeps in a share of a sleeper's body heat |
| 2 | Animals | Boars roam the forest, feed, and flee people | Kinds of living things; animals are living bodies under the same laws as people, with an instinct in data: flee, keep away, feed, drink, sleep, wander |
| 3 | Injury | A tusk or a spear wounds; a cornered boar charges | Wounds bleed, which is faster fluid loss; they heal slowly. Bleeding to death is dying of fluid loss, a limit that already exists |
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

### Stage 2: animals. Passed, 2026-09-29.

**Six boars live on the island**, keeping to the forest, the stream, the hillside, and the slopes, and looking after themselves. [living-2-boars.txt](../../data/scripts/living-2-boars.txt): the islander finds the herd asleep by the stream at 23:07 and walks right up to it; by morning it has gone.

**Kinds.** At the owner's request, living things are classified generically, humans and future aliens included: see [kinds.md](../ideas/kinds.md). Kinds form a tree in data (living thing, animal, mammal, human, boar; fish, shellfish; plant, tree, shrub). A kind gives its members a body and a mind. Datasheets show it: "kind: human (mammal, animal, living thing)". Populations, such as the fish in the shallows, have kinds too. The engine knows only "is this a kind of that?"; the no-names scan now covers kinds, so "boar" and "human" can't appear in the engine.

**Instinct is a mind.** A person acts on commands; a boar acts on its kind's instinct, which only ever proposes ordinary commands (go, drink, gather, eat, sleep), and the laws decide, exactly as for a person. In order:

1. **Flee** anything of a kind it fears at the same place, and **keep away** from that place a while (three hours for boars).
2. **Sleep** at night. Asleep, it can't flee.
3. **Eat** what it holds.
4. **Drink** when thirsty, going to water within its range if there's none here.
5. **Forage** when hungry, but only where a search has a fair chance, going elsewhere when a patch is picked over.
6. Otherwise **rest**, or sometimes **wander**.

**A general law came with it: bodies store surplus food as fat.** Until now a body burned food, then fat, and nothing ever turned food back into fat, so every creature, people included, could only get thinner. Now food beyond half a day's needs becomes reserve again, at up to the body's resting power and 75% efficiency (data), until the reserve is back where it began; the rest leaves as heat and carbon dioxide, and the gate balances it all.

**Trial:** 60 of 60 boars alive after 30 days, over 10 islands. On seed 1, the herd holds its weight for twenty days, eats the forest's roots and nuts down faster than they grow back, and migrates to the slopes, losing a little weight on the long walks.

**What the attempts found:**

- **Boars died of heat.** Working at 350 W, a boar that barely sweats made more heat than it could shed. Real pigs wallow in mud for this reason; the numbers were adjusted (250 W, better heat loss).
- **They starved beside food**, three times over: they only ate when their gut was empty, so every twelve-hour night ran on fat; nothing turned food back into fat; and a rule to skip picked-over patches ruled out every patch, because patches were measured against a growth limit I'd raised. Each fix is above.
- **A boar went back and forth all night** between the stream, where the islander was, and the forest, where it was thirsty. Real animals remember where they were scared, hence keeping away.

**Sabotage checks:** no storing fails the fat test; no fleeing fails the fleeing test; no keeping away fails the boar proof.

Simplifications to revisit:

- **Boars don't breed or age yet**, and the herd can only shrink.
- **A creature arrives the moment it sets off**, so boars hear a person coming the whole walk.
- **Instinct knows its range**: where water and food are within the places it keeps to, without having to find them.
- **Everyone sees true kinds**, until recognition.
- **No charging yet**: a cornered boar comes with injury, in stage 3.

### Stage 3: injury. Passed, 2026-09-29.

**Wounds bleed, and bleeding kills the way thirst does.** A wound drains the body's fluid, fast at first and half as fast for every clotting time (ten minutes for people and boars, in data). Below the body's minimum, it dies, "of bleeding" rather than of thirst.

- **An edge makes a wound, and a sharper edge a worse one.** `attack boar with spear`: the wound bleeds at a rate set by the edge's width (20 g a second for a 1 mm edge, in data). A crude flint spear with a 5 mm edge wounds at 4 g a second, and the wound clots before a boar loses enough to die. Hone the flake on a stone for half an hour, down to 2 mm, and the same spear wounds at 10 g a second, which kills. Bare hands have no edge: "you have nothing with an edge to wound with".
- **Someone asleep can't dodge**; a blow at someone awake lands six times in ten.
- **A wound wakes a sleeper** and interrupts whatever a creature was doing.
- **Tusks are a natural weapon**, an edge the boar's kind is born with (5 mm).
- **A cornered or hurt boar charges.** Its instinct turns on what it fears instead of fleeing: once when hurt, then it runs; again and again only if it's truly trapped.

[living-3-injury.txt](../../data/scripts/living-3-injury.txt): the islander spears a sleeping boar with a crude spear at night; the boar wakes, gores the islander (4 g a second), and runs; both wounds clot, and the islander drinks and lives. [living-3-a-killing-blow.txt](../../data/scripts/living-3-a-killing-blow.txt): with a honed spear, the boar still gores the islander and runs, but dies of bleeding by the stream, where it lies: "a boar (73 kg, dead)". Finding it and butchering it is the next stage.

**What the attempts found:**

- **A boar gored a person to death, again and again.** The herd had fled the forest and was keeping away from it, so at the stream a boar had nowhere to run and charged every few seconds. Now fear of what's in front of it beats wariness of a place (it will flee somewhere it's avoiding if it must), and a hurt creature charges once and then runs.
- **A wounded boar stood still.** The wound woke it, but it was still "busy" until its night's sleep would have ended. A wound now interrupts.
- **A tusk wound is serious.** Gored after a thirsty day, the islander's water fell to 36.4 kg, just above the 35 kg that kills.

**Sabotage checks:** an edge that doesn't matter, wounds that don't bleed, and boars that never charge each fail a proof.

Simplifications to revisit:

- **Bleeding drains the same fluid as thirst**, with the same limit: a body can lose about 7 kg of it, far more blood than a real one could.
- **A wound only bleeds.** It doesn't slow or weaken anyone, and it doesn't need tending or get infected.
- **A blow is a blow.** Where it lands, how hard, and armour don't count yet.

