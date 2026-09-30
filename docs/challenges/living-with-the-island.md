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
| 4 | The hunt | Kill a boar, butcher it, cook the meat, and keep what's left | Butchering divides a body with a cutting tool into meat, fat, and hide, leaving the bones. Cooking is a change at a temperature, as clay firing is. Meat spoils with time, cooked meat more slowly |
| 5 | Hide and shoes | Scrape and dry a hide into leather, and make shoes and clothing | Walking wears what's between you and the ground: bare feet on rough ground are hurt and slowed, and shoes wear through instead. Worn clothing cuts a body's heat loss |
| 6 | A barrier or tree house | Keep boars out at night (became: keep boars out of your stores; see its result) | A barrier blocks what can't get past it; height keeps out animals that can't climb |

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

### Stage 4: the hunt. Passed, 2026-09-29.

[living-4-the-hunt.txt](../../data/scripts/living-4-the-hunt.txt) plays a day and a night:

1. **By day:** gather tinder, twigs, and sticks, stones for a fire ring, and flint; knap a flake, hone it, and make a spear.
2. **At night:** spear a sleeping boar. It gores the islander, runs, and dies of bleeding by the stream.
3. **Butcher it** with the spear: six cuts of raw meat of about 4.6 kg, three of fat, a hide, and what was in its gut. The bones and body water stay behind.
4. **Cook two portions** over a fire in the ring. They're too hot to touch for a while; then eat one.
5. **Two days later**, the raw cuts are mostly rotting flesh; the cooked portion is still mostly good.

The new laws:

- **Butchering** (`butcher boar with spear`) needs a dead body and something with an edge, takes half an hour, and cuts everything but the frame (the hardest part: bone) and the body's water into pieces no heavier than a cut (5 kg, data).
- **Eating takes in the water in food** along with what the body digests; before, a food's water was left in the hand.
- **Spoiling:** a material can spoil into another at a share a day, keeping its energy. Raw meat spoils into rotting flesh at half a day, cooked meat at 15%, a raw hide at 20%; only what's no longer alive spoils. Rotting flesh isn't food.
- **Cooking** reuses the law that fires clay: raw meat becomes cooked meat at 340 K.
- **A boar's body** is now water, meat, fat, bone, and hide; meat is taken as meat with its water in it, about 6 MJ a kilo. People can eat meat, raw or cooked.

**What the attempts found:**

- **A rotting carcass heated itself to 331 K.** The first spoiling law released the meat's energy as heat, like slow burning, which at half a day is over a kilowatt inside a body: it cooked the meat and melted the fat. Real spoiling is food becoming unsafe, not vanishing, so it's now a change of material that keeps its energy.
- **Butchering put all the body's water into the cuts**, where it drained out and pooled with melted fat. The water now stays with the bones.
- **The recipe made its spear shaft from a twig** ("the smallest wood"), leaving too few twigs for the fire.
- **Fire at half past midnight needed more tinder.** At 292 K, two tufts burned out before the twigs caught; five caught them. The fire laws' margins are still narrow.

**Sabotage checks:** no spoiling and no cooking each fail the hunt.

Simplifications to revisit:

- **Cold doesn't slow spoiling**, and there's no smoking or drying yet.
- **Raw meat is safe to eat**; rotting flesh just isn't food. Sickness would come with disease.
- **Butchering always yields the same parts**, whoever does it.

### Stage 5: hide and shoes. Passed, 2026-09-29.

The first stage after [a stranger's words](strangers-words.md): the islander now speaks their people's words (the `islanders` culture, shared in [island-things.toml](../../data/island-things.toml)). Their people know hide, but not leather, and have no shoes. [living-5-shoes.txt](../../data/scripts/living-5-shoes.txt) plays it:

1. **Hunt a boar** (the stage 4 hunt, now the recipe [hunt-a-boar.txt](../../data/scripts/skills/hunt-a-boar.txt)) and take its hide, 3.2 kg.
2. **Dry it by a fire** in the ring. At 330 K a hide becomes leather, which doesn't rot. The islander has no word for it: they see "lump of stiff dried skin", and name it (`call the stiff dried skin leather`).
3. **Cut it**, by hand, into a cloak's worth (1.6 kg), a spare (0.8 kg), and two pairs' worth (0.4 kg each). Twist bush fibre into cord.
4. **Put a piece of leather and the cord together:** *"You've made something new: lump of leather and rotting flesh joined to plant fibre rope. What do you call it?"* They call it shoes, and wear them on their feet. They put on the cloak.
5. **Up the stony paths to the ridge and back.** The shoes take the wear, and the feet aren't cut. On the way down, *"Your shoes … wore through"*, and the last stretch cuts the feet.
6. **Another pair, made the same way:** "make shoes" follows the recipe from the first pair.

[living-5-bare-feet.txt](../../data/scripts/living-5-bare-feet.txt) walks the same paths barefoot: sand, the forest floor, and the hillside are fine; the slopes and the ridge cut the feet and halve the walking pace.

The new laws:

- **Ground has a roughness**, in data per place: how much of a sole a km of walking wears away. The living island's beach is smooth, the forest and stream 1 g/km, the hillside 4, the wooded slopes 10, the ridge and summit 20. A walk counts the rougher end.
- **Wearing** (`wear shoes on your feet`, `wear cloak`, `take off …`): only something soft enough for bare hands to shape can be worn, on the feet or about the body. What's worn stays among what you carry.
- **What's on your feet takes the wear.** The part of it that was heaviest when put on loses the ground's roughness for every km, and falls as dust where you arrive. **Half of it gone, it's worn through**, and as good as bare feet.
- **Bare feet on ground rough enough to hurt** (8 g/km here) bleed, 20 mg/s for every km (so a 5 km climb to the slopes bleeds 100 mg/s, which clots), and walk at half pace. Creatures acting on instinct are born with feet for their ground.
- **What you wear keeps in body heat:** its main material's share (leather and raw hide 40%), in proportion to how much of a body it covers (2 kg covers a whole one). With a shelter, it keeps in a share of what gets past the shelter. A 2 kg leather cloak saves about a fifth of what a person burns keeping warm through an evening and night: 8.7 MJ against 11.3 MJ.

Tests in [living.rs](../../crates/engine/tests/living.rs) measure it exactly: 400 g of leather on the feet loses 2.3 g from the beach to the hillside and 50 g more to the slopes; bare feet bleed 100 mg/s there and take over half as long again; 200 g shoes wear through by the ridge; a stone is too stiff to wear.

**What the attempts found:**

- **Which part of a shoe takes the wear has to be fixed when it's put on.** At first it was whichever part was heaviest at the time; once the leather wore thinner than the cord, the cord became the sole and the shoes never wore through. Now it's decided when they're put on.
- **A recipe mustn't take what you're wearing.** "Make shoes" first took the biggest leather within reach: the cloak on the islander's back, making shoes too big to look like shoes.
- **Words, again.** "Drink water" asked whether the islander meant the stream's water or a boar, whose body is mostly water; living things no longer count as a piece of their material. "Work fibre" asked about the cord or the shoes; now a name can be part of a material's name ("fibre" for plant fibre). And "call it shoes" first answered "You call it a shoes": the name is now echoed as said.
- **Long days need water.** The first try climbed to the summit and back and died of thirst; the story goes to the ridge, and drinks at the stream before climbing again.
- **Giving the islander words changed stages 1 to 4 only in wording:** a lean-to is now "the lean-to of wood and plant fibre", and "sleep in wood" had to say which wood, the driftwood or the lump. Every time and outcome stayed the same.

**Sabotage checks:** walking that wears and cuts nothing, clothing that keeps in nothing, and bare feet at full pace each fail a proof.

Simplified:

- **One number for the ground.** Roughness wears soles and, above a limit, cuts bare feet; mud, thorns, and heat underfoot aren't separate.
- **A covering is a covering.** Shoes need no particular shape, and a cloak isn't cut to fit; how much of a body something covers is only its weight.
- **Drying leather is just heat.** No scraping, and at 330 K it's instant, like cooking; hold times come with the processes decided in the time conversation.
- **Worn-through shoes are only half used up**, and staying on your feet they neither slip nor fall off.

### Stage 6: keeping boars out. Passed, 2026-09-29.

**The stage changed on the way.** The plan was to keep boars away from a sleeper at night, but in this world boars flee people and only charge when cornered or hurt, so a sleeper was never in danger. What they would go for, and didn't yet, is food. So the stage became keeping boars out of your stores, which is also the "safety is made in the world" rule for time away: a camp is raided while its owner is gone.

[living-6-cache.txt](../../data/scripts/living-6-cache.txt) plays it:

1. **Hunt a boar and butcher it by the stream**: three lumps of fat among the cuts.
2. **At daylight, build a raised cache**: six sticks and two lumps of bush fibre, poles lashed into a platform. Put two lumps of fat in it, and leave one on the ground.
3. **Spend the night away**, asleep on the beach.
4. **Come back**: the fat on the ground is gone; "take fat" finds only the fat *in the raised cache*, and the islander takes it down.

The new laws:

- **Raiding**: a hungry creature goes first for food lying where it can get at it: something loose on the ground, or in something it can reach into, all of it food for its body. Food that needs no searching is worth going out of its way for, within its range. Boars eat fat and roots; they flee a person, so they raid only when nobody's there. Six hungry boars find 2 kg of fat left in their forest within the hour, whatever the luck.
- **A barrier keeps out what can't get over it**: a design can hold what's in it at a height (the raised cache: 2 m), and every body has a reach, how high it can climb or jump (a person 2.5 m, a boar 1 m). Nothing can take from or put into what it can't reach.

Tests in [living.rs](../../crates/engine/tests/living.rs): fat left in the forest is eaten within a day; fat in a raised cache there survives the day, and the islander takes it back.

**Sabotage checks:** a barrier that keeps nothing out, and boars that never raid, each fail the proofs.

Simplified:

- **The cache is a design the islanders already know**, with its height in data, like the lean-to's shelter. Measuring a barrier's height from its parts would need shapes to have heights; nothing needed that yet.
- **No fence or pen**: a barrier is something you put things in. Keeping creatures out of a place, rather than out of a container, isn't built.
- **Nothing threatens a sleeper.** A creature that hunts people would be a new instinct rule, if a later challenge wants one.
- **A person always gets things down.** Reach is a single height; a cache too high for a person would need something to stand on.

**This completes the challenge.** Stages 1 to 6 have passed.

