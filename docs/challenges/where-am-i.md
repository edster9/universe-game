# Challenge: where am I?

Proposed 2026-09-28 from the owner's direction ("More island scenarios before validation" and "Sleep, and later age and body types" in [requirements.md](../requirements.md)). A training challenge, like [stranded](stranded.md): it exists to find the laws we're missing. Its laws are recorded in [laws.md](../laws.md).

## The story

Someone wakes on a beach with nothing. Before building anything to leave, they want to know where they are. They may be on an island, or on a mainland a few kilometres from a village. There's no way to know if the island across the water is any better. So they explore, find the high ground, and climb it for a view, which may take days, with camps on the way.

It's played on the stranded island, grown: a mountain rises behind the forest.

## Stages

Each stage has a test: proofs with fixed luck that play the right steps and deliberate mistakes, and trials with real chance where survival is in play.

| # | Stage | What happens | New laws it needs |
| --- | --- | --- | --- |
| 1 | Day, night, and sleep | Get through the first nights: the air cools, it gets dark, and the survivor must sleep | The world has a time of day. Nights are colder and dark; searching needs light, and a fire gives it. A body grows tired while awake, is slower when tired, and falls asleep where it stands if it goes too long. Below its set point, a body burns more to keep warm (shivering) |
| 2 | An unknown island | The survivor knows only the beach, and finds the other places by exploring | Each person knows the places they've been or seen. A way out has to be found before it can be taken, by searching, as gathering does |
| 3 | Carrying water | Fire a clay pot, fill it at the stream, and drink from it later | A portable container holds liquid |
| 4 | The climb | Take two or three days to climb the mountain, camping on the way | Places have a height. Going up costs time and energy: lifting your weight and your load. The air cools with height |
| 5 | The view | From the summit, see what lies around: other islands, a coast, maybe smoke from a village | You see as far as the horizon, which grows with height. What you see becomes places you know exist |

## Simplifications planned from the start

- **No age or body types yet.** Every character is the same adult body. Age (18 to 55) and body types are recorded for later.
- **No weather.** Rain, wind, and storms belong with the harder crossings.
- **Sunlight for growth stays flat**, day and night, as in the stranded challenge.

## Results

### Stage 1: day, night, and sleep. Passed, 2026-09-28.

The island's day is 24 hours, with the sun up from 06:00 to 18:00, starting at 08:00. The air is warmest at midday (300 K) and coolest at midnight (292 K). `look` now says the day and time, and whether it's daylight, dark, or lit by fire.

- **Night is dark.** Searching for anything, whether shellfish, wood, or stones, is refused in the dark: "it's too dark to search: wait for daylight, or make a fire here". A fire lights the place, and searching works again. Walking and working in the dark are still allowed.
- **Sleep is a necessity.** A body stays awake 16 hours for 8 hours of sleep. `sleep` sleeps until rested; `sleep for 9 h` sleeps a set time. Tired, past 16 hours, a body works at two-thirds pace: a 400 m walk takes 8 min 17 s instead of 5 min 33 s. After 40 hours awake, it falls asleep where it stands ("You're exhausted, and fall asleep where you are") and can't act until it wakes. You can't sleep when you've barely been awake.
- **Shivering.** Below its set point, a body burns more to stay warm, up to its working power. At rest through a day and a night it now burns about 9 MJ, not 6.9, and holds 310 K all night.

Proofs: [where-1-sleep.txt](../../data/scripts/where-1-sleep.txt) covers tiredness, sleep, and collapse. [where-1-night-and-fire.txt](../../data/scripts/where-1-night-and-fire.txt) finds it too dark to search at 19:13, makes fire in the dark by rubbing sticks, and gathers driftwood by firelight.

**What the attempt found, and fixed:**

- **Things never warmed back up.** Heat only ever flowed out: anything colder than the air stayed cold. With a constant 300 K this never showed. With nights, a stone cooled at night would stay cold forever. Heat now flows both ways, and the warmth comes from the heat the place's surroundings have taken in and, beyond that, from sunlight, which is what warms the air by day. The gate still balances energy exactly.
- **Friction fire was tuned on a knife edge.** It worked at exactly 300 K. In the cooler morning, one minute of rubbing no longer lit the tinder, and two minutes lit it and burned it up while still rubbing. Now rubbing into a fire ring stops by itself when something in it catches, as a person would stop, and takes only as long as that took (about 1 min 14 s). Fire works at any hour.
- **Nature crawled.** The sea lags the night air by several kelvin, and the carbon dioxide from breath stayed cold, so nature took one-second steps all day. The daily run jumped from 16 seconds to over a minute. Fixed sources no longer hold nature to small steps, and warming fixed the rest.
- **Every long proof had to learn to sleep.** The raft recipe now fells two logs on the first afternoon, sleeps twelve hours by the stream, and fells four more the next day. The ten-day routine sleeps nine hours a night. Working hard without water now kills the next day rather than the same day, because darkness ends the day's work.

**Sabotage checks:** switching off darkness, shivering, tiredness, falling asleep, or warming by day each fails a test. The trials are 30 of 30 alive, for 10 days on shellfish and 30 days fishing; the survivor now sleeps at night.

Simplifications to revisit:

- **Walking in the dark** is allowed at normal speed.
- **Sleep has no place or comfort.** Sleeping on bare sand is as good as a bed; shelter comes in the next challenge.
- **The temperature swing is a straight line** from midday to midnight, not a curve.
- **Sunlight for growth stays flat**, day and night.

### Stage 2: an unknown island. Passed, 2026-09-29.

The challenge now has its own world, [where-am-i.toml](../../data/where-am-i.toml): the stranded island, with a castaway who knows no way off the beach. The two worlds share their materials, shapes, and designs through a library, [island-things.toml](../../data/island-things.toml), so they can't drift apart; each keeps its own places, people, and items.

- **You know only the ways you've found or walked.** `look` lists "ways out you know", and going anywhere else is refused: "you don't know a way to the stream from here: try exploring". The refusal gives nothing away: it says the same whether or not such a way exists.
- **Exploring is a search**, like gathering. `explore` takes 30 minutes and, six times in ten, finds the nearest way out you don't know yet, so the forest (300 m) turns up before the stream (400 m). It needs light, like any search. When there's nothing left to find, a search just finds nothing, so the castaway can never be sure they've found every way.
- **Walking a way teaches the way back**, but not the other ways out of where you arrive.

Proofs: [where-2-explore.txt](../../data/scripts/where-2-explore.txt) explores the beach, the stream, the forest, and the hillside with average luck. [where-2-bad-luck.txt](../../data/scripts/where-2-bad-luck.txt) has every search fail: the castaway never finds the stream and dies of thirst on the beach within four days. That's an outcome, not a failure.

**Trial:** 30 of 30 castaways found water, explored as they needed, and were alive after 10 days. Finding water is easy on this island: the stream is 400 m away, usually two searches. A harder island would test exploring more.

**Sabotage checks:** ignoring the chance of a search lets the unlucky castaway find the stream and live, which fails the bad-luck proof; letting people use ways they don't know fails the explore proof.

Simplifications to revisit:

- **Exploring finds ways, not places.** Places you can see but haven't reached come with the view (stage 5).
- **Knowledge is only of ways.** Knowing what things are, and where things are found, comes with recognition and the knowledge slice.
- **Every search in a place has the same chance**, however far or well hidden a way is; only the order is nearest first.

### Stage 3: carrying water. Passed, 2026-09-29.

**Shape a clay pot, fire it hard in a furnace, fill it at the stream, and drink from it far from water.** [where-3-carry-water.txt](../../data/scripts/where-3-carry-water.txt) does the whole thing as the castaway, starting with exploring, in two shared recipes: [explore-the-island.txt](../../data/scripts/skills/explore-the-island.txt) and [fire-pots.txt](../../data/scripts/skills/fire-pots.txt).

- **A shape can hold things.** A shape whose role is containing says how much it holds (a pot, 2 kg), and a piece worked into it becomes a container. Its datasheet says "can hold: 2 kg".
- **`fill pot from water`** takes liquid from a source into a container, up to what it holds and what you can carry. Drinking reaches into a container you carry.
- **What a container holds is carried too.** A full pot weighs 3 kg against your carrying limit and your walking speed, and `inventory` shows what's inside.
- **Some materials soften in some liquids.** Unfired clay softens in water, so an unfired pot can't be filled ("the clay pot would soften in the stream's water"). Fired, it holds water. This is data on the material, and the engine applies it to any container and any liquid, when filling or pouring.

The mistakes, in [where-3-unfired-pot.txt](../../data/scripts/where-3-unfired-pot.txt): an unfired pot, and a plain lump of clay, which can't hold anything.

**What the attempt found:**

- **Carried containers didn't weigh what they held.** The carrying limit and walking speed counted only the pot itself. Fixed: things are weighed with their contents.
- **An open fire can't be built up to big logs.** Sticks burn out before a 2 kg piece of driftwood catches, so a campfire never gets a pot anywhere near 900 K. A furnace does, as a kiln would, so the pot is fired there. Real people fired pots in bonfires; this is a gap in the fire laws, recorded rather than fixed now.

**Sabotage checks:** a containing shape that holds nothing, clay that doesn't soften, and contents that weigh nothing each fail a proof.

Simplifications to revisit:

- **A pot's capacity is a mass**, not a volume.
- **Drinking leaves the last milligram** in a pot, as it does in any source.
- **Water in a pot doesn't spill, evaporate, or go stale.**

### Stage 4: the climb. Passed, 2026-09-29.

**A mountain rises behind the hillside**: wooded slopes 5 km away at 600 m, a bare ridge 5 km further at 1,200 m, and the summit 3 km beyond at 1,800 m. There's no water above the stream and no wood above the slopes. [where-4-climb.txt](../../data/scripts/where-4-climb.txt) plays two and a half days:

1. **Day 1:** explore the island, fire two pots, and fill them at the stream: 4 kg of water.
2. **Day 2:** sleep by the stream, gather tinder, twigs, and sticks at dawn, and climb, finding each way up by exploring. With 15 kg carried: 2 h to the slopes, 2 h 15 min to the ridge (where the castaway picks up stones for a fire ring), and 1 h 50 min to the summit, reached at 15:16.
3. **The night on top:** a fire, a drink from a pot, and twelve hours' sleep in air that falls to about 280 K.
4. **Day 3:** down again: 52 min back to the ridge, where going up had taken 1 h 50 min.

The new laws:

- **Places have a height, and climbing lifts your weight and your load.** A share of the climber's working power (45%) goes into lifting mass × gravity × height, so climbing takes time on top of the walk, and more with a heavier load. It's hard work, so it burns food and water. Going down costs nothing extra. For 70 kg with a light load, that comes to about 600 m an hour, the old hikers' rule of thumb.
- **The air cools with height**, 6.5 K per km. A place's temperatures are given as at zero height, and its height does the rest. The summit is 286 K on a mid-afternoon when the beach is 298 K.

[where-4-heavy-load.txt](../../data/scripts/where-4-heavy-load.txt): the same 5 km and 450 m climb takes 1 h 46 min empty-handed, 1 h 9 min coming back down, and 2 h 30 min going up again with 25 kg of stones.

**What the attempt found:**

- **The first mountain was too small for camps.** At 1,600 m and 5 km away, the castaway was on the ridge by 11:00 on day 2 and could have been back by dark. Making it the size of a real island's peak, a long approach and 1,800 m, made the night on top a necessity: exactly the owner's "it might take days".
- **Shivering holds the body a kelvin below its set point** in 280 K air: 309 K by morning. It's alive and warm enough, but it's burning about 230 W to stay there.
- **"Fill the pot" filled the full one.** Filling now picks a container with room in it first.

**Sabotage checks:** free climbing fails the timings; air that doesn't cool with height fails the summit's temperature.

Simplifications to revisit:

- **All ground is equally easy.** Scree, jungle, and cliffs walk like a beach; only distance and height matter.
- **Wind** doesn't add to the cold at height.
- **The summit night isn't dangerous** for a healthy, fed, watered body. It costs food, but it can't kill yet without rain or wind.

