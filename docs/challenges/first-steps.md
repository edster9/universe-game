# Challenge: first steps, as the islander

Started 2026-09-30, from the owner's first milestones ("The first milestones: camera, console, fire, backpack, voice" in [requirements.md](../requirements.md)): play the companion's island in the native client ([ideas/the-client.md](../ideas/the-client.md)) as one actor, seen from behind, and make a fire. Small challenges, as before, each with its proofs. It finishes the companion challenge's last stage, and it's the base the camp, other players, and towns grow from.

The world is `data/companion.toml`; the islander lives by players' rules.

## Stages

| Stage | Name | What it gives | How it's proved |
| --- | --- | --- | --- |
| 1 | The actor's view | The camera follows the islander from above and behind. The mouse orbits it, and the angle stays while they walk (sideways, behind, in front); up and down, and zoom on the wheel. A key breaks out into free flying (a developer's tool), and snaps back. The client draws only what the engine says the islander perceives: the land and sea are scenery everywhere; people, creatures, and things appear once perceived. Pointing at something shows its name, in the islander's words | Screenshots from the islander's camera at set angles; a sabotage check that the client draws nothing the islander can't perceive |
| 2 | The console | Translucent, at the bottom left, where most online games keep their chat: an input line, and a log where replies, news ("the stranger goes for you…"), and grey debugging messages scroll. Enter to type, Esc to stop, a key to make it bigger or smaller. Every command the console takes today works, through the console's own session, so every reply and refusal is the one the scripts prove. `go forest` walks the islander along the path, the camera following | The client plays a script file through its own console and saves a screenshot at each expectation; the scripts still pass unchanged |
| 3 | A fire | **The first challenge:** from the beach, walk to the forest, gather dry grass, twigs, and sticks, and rub up a fire, using the laws built in the stranded challenge. Burning things are drawn burning: flames, light on their surroundings, smoke | `stranded-2-fire` played as the islander in the client, with a screenshot of the fire; the islander's view at night lit by it |
| 4 | The backpack and the body | A key opens and closes a simple backpack: what the islander carries, as labels with their masses, updated as they pick things up, drop them, eat them, or burn them. Another opens the body: everything the engine measures about the islander, compact until expanded: fluid and thirst, food and stored energy, stamina (under players' rules), tiredness and time awake, body temperature, wounds and bleeding, and what they're wearing. All measured, never written: the same numbers `datasheet me` shows | Screenshots of the backpack before and after gathering, and of the body before and after a hard day's work |
| 5 | Voice | Hold a key and speak; the words appear in the console and run as if typed. Speech to text runs on the player's own machine (the leading choice: Whisper, open source, on the GPU), so it's private, free, and works offline. It only ever produces text: the same commands, checked by the same laws | The fire made again, spoken rather than typed; recorded commands transcribed correctly in a test |
| 6 | Being near things | **Space within places**, an engine law: positions in metres inside each place, a reach (people's is 2.5 m), and walking with WASD. You have to walk up to the grass to cut it; commands that need a thing within reach say so ("the grass is too far away"). Walking off along a path takes you to the next place. Pressing a walking key during a commanded walk cuts it short and hands the islander to the keys. In free flight, WASD flies the camera, not the islander (the owner, 2026-09-30); to settle then: the owner's idea of the arrow keys for the camera and WASD for the islander, so both work at once | Scripts with positions: too far, walk closer, done; the fire made by walking up to each thing |

## Why this order

- **The camera and the console first**, because nothing else can be seen or tried without them, and together they already let the islander play every story the scripts prove.
- **The fire next**, as the owner asked: it needs no new laws, only drawing flames, so it's a quick first win in the client.
- **The backpack and the body** are small, and they're what make gathering, hunger, and wounds feel real.
- **Voice before nearness**, because typing is the owner's daily annoyance, and voice doesn't depend on anything else; and because nearness is the biggest step (a new law, and a design to agree first).
- **Nearness last**: it changes what "here" means for every law that uses it, so it deserves its own design conversation, its own proofs, and a clear head. Until then, the islander moves between places with `go`, typed or spoken, and WASD does nothing to them.

Then the companion's stories in the client (asking, trading, temperament) come almost free, and the road goes on: the camp, other players, towns.

## Stages 1 and 2, results (2026-09-30)

**Passed.** The client is now the islander's own view, with a console.

**The actor's view.** The camera sits above and behind the islander and follows them, smoothly while they walk, and at once when they jump (a script's `go`). Hold the right mouse button and drag to turn it around them; the angle stays however they walk. The wheel brings it closer (2 m) or further (800 m). F flies free (WASD, E/Q, Shift), and F again snaps back behind the islander. Pointing at anything drawn shows its name, in the islander's words ("you" for themselves). The first view looks inland from behind them, and `--yaw`, `--pitch`, and `--zoom` set the angle for screenshots.

**Only what they perceive.** A new law in the engine, `view::scene`, says what a person pictures: everything where they stand, and the fixed things they remember seeing at other places. The client draws only that. At the start, only the beach and whoever is on it are drawn. The forest appears when the islander arrives there, and afterwards stays as they remember it. The boars are drawn only while the islander is where the boars are. The land and sea are scenery everywhere.

**The console.** It sits at the bottom left, translucent, where games keep their chat:
- Enter opens the input line, and Enter again sends the command and gives the keys back to the game (changed after the owner's first try: with the line left open, `]` went into it instead of speeding the clock); Esc closes it without sending; up and down bring back earlier commands; the key left of 1 (`) cycles it through small, large, and hidden.
- Colours: what you typed in blue, replies in white, refusals in amber, news in gold, and the client's own messages in grey.
- It goes through the console's own session, so every reply is the one the scripts prove.
- Commands are the same as in the text console, but the clock doesn't wait for them. `go forest` starts the walk, the islander is drawn on their way, and "You go to the forest." arrives as news when they get there. The HUD shows "busy until 08:06" while they walk.
- News of being attacked, robbed, falling asleep, or dying comes the same way, as it happens.
- Space pauses the world, and [ and ] slow it down and speed it up. By default the clock runs at real speed.

**Proofs.**
- The engine test `a_person_pictures_only_what_s_where_they_stand_and_what_they_remember` (companion tests) covers the perception law: on the beach, nothing of the forest; after going to the forest and back, its fixed things are remembered where they stand, the boars aren't, and the beach isn't "remembered" while you're on it.
- The console test `in_real_time_actions_start_and_their_outcomes_are_told_when_they_come` covers the real-time session: a walk starts without moving the clock, instant commands still work while walking, and the arrival is told once, when it comes.
- `client/run.sh --script <file>` plays a script through the client's console, a line at a time, saves a screenshot from the islander's camera at each expectation, and exits with an error if the script fails. Two scripts were played this way, and both passed:
  - `companion-3-asking`: 16 commands, 8 screenshots;
  - `where-1-night-and-fire`, in the stranded world, at night by the fire: 36 commands, 8 screenshots.
- Every script still passes unchanged. The script runner now plays scripts a line at a time, and the test suite and the client both use it.
- `client/run.sh --type "go forest" --speed 16 --shot walk.png` types a command at the start. Screenshots show the islander walking inland with the camera following, then in the forest with the arrival news.

**Sabotage checks.** Each of these made a test fail:
- drawing every entity in the world;
- remembering creatures and loose things elsewhere;
- counting the current place as remembered (this first survived, and the test was strengthened);
- dropping the outcome news in real time;
- performing instead of starting in real time.

**Found along the way.** The sabotage loop ran under zsh, which doesn't split a variable's words, so cargo never ran the tests and every check looked like it "survived". Rerun under bash, all were caught.

**Simplified, for now:**
- Nothing is seen from one place to the next. From the beach you don't see the forest's trees until you go there, until places have real space (stage 6).
- Remembered things are drawn as they are now, not as they were when last seen: they disappear once gone, even if the islander hasn't seen them go.
- Loose things lie at a spot of their own around the place's middle, not where they were dropped.
- The pointer names things by their screen position, not by their shape. A thing's cluster of pieces all carry its name.
- Hovering and the mouse were not tried by hand (screenshots can't move a mouse). The owner's first try is the real test.
- WASD doesn't move the islander yet; they move only by commands such as `go forest`. In free flight WASD flies the camera, as the owner wants. Walking by keys, and taking over a commanded walk with them, come in stage 6.
- Commanding others (`as`, `become`) still works in the console, as a developer's tool.

## Stage 3, results (2026-09-30)

**Passed.** The islander makes a fire on the companion's island, and the client draws it burning.

**The challenge** is `data/scripts/first-steps-3-fire.txt`, played by players' rules:
- From the beach, walk to the forest for sticks, grass, and twigs, and to the hillside for five stones.
- Make a fire ring, and bring it back to the beach.
- Wait for sunset, then rub up a fire and feed it one size at a time.
- Ten minutes later, the wood is burning and the beach is "night, lit by fire".

No new laws: everything comes from the stranded challenge. The script runs with every other script on every test run, and passes in the client (35 commands, 6 screenshots).

**What the attempts found.** The laws decided, as they should:
- Lit in the morning, the fire burned out long before dark.
- A 2 kg lump of driftwood laid on a young fire of sticks smothered it: the cold log drew off the heat, and everything went out at 413 K.

So the script lights the fire after sunset, from sticks. Keeping a fire going all night (feeding it in sizes up to logs) is a later challenge.

**How it's drawn.** Anything burning where the islander stands, itself or inside something such as the fire ring, is drawn as a fire:
- **Flames** glow past their edges and flicker. They're taller for more burning mass: about 0.6 m when the tuft first catches, about 0.9 m once the sticks are burning.
- **A warm light** flickers and casts shadows: the shells on the sand throw long shadows away from the fire. Its brightness and reach grow with the flames.
- **Smoke** rises in thinning puffs and drifts downwind.

The camera gained bloom, which is what makes the flames glow.

**Proof.** Screenshots from the islander's camera, checked by eye:
- the tuft just caught (a small flame, a small circle of light);
- the fire burning at night (a bigger flame, the beach lit around it, the islander and the sleeping stranger at its edge).

These are visual only: no automatic test looks at pixels.

**Found along the way:**
- People and dropped things were scattered up to 54 m around the middle of a place, so the fire ring lay far from the islander and out of view. People now stand within 10 m of the middle, and dropped things lie within 5 m, until places have real space (stage 6).
- That change first didn't take effect: `cargo fmt` had rewrapped the line my edit targeted, so the edit silently missed it (a known pitfall). A debug print of where the client drew things showed it.

**Simplified:**
- Flames are cones, and smoke is see-through balls, until the assets conversation.
- Hot things that aren't burning, such as embers and ash at 1,000 K, don't glow.
- The fire's light is the client's own, sized from the burning mass. It isn't the engine's energy, and doesn't need to be: the engine already decides what's lit ("night, lit by fire") and what you can do by it.

## Stage 4, results (2026-09-30)

**Passed.** Two small windows at the top right, translucent like the console.

**The backpack (B)** lists what the islander carries, a line each, with masses. Alike things go together ("lump of wood x2: 400 g"), with what's inside any container, and the load at the bottom ("carrying 415 g of 40 kg"). It updates as things are gathered, dropped, eaten, or burned.

**The body (V)** shows the vitals, measured by the engine, never written:
- body fluid (thirst), and the level you die below;
- stored energy (food);
- stamina, under players' rules;
- time awake (tiredness), when the body sleeps;
- temperature;
- bleeding, when wounded;
- whether they're working hard;
- what they're wearing.

Click the body window for everything the engine measures about the islander (what they're made of, mass, how much they know, and so on), and again for just the vitals.

Both windows are the console's own `backpack` and `body [all]` commands, which the text console now has too, so every number is one `datasheet me` shows. `--open backpack,body` (or `body-all`) opens them at the start, for screenshots.

**Proofs:**
- The console test `the_body_shows_what_the_datasheet_measures_and_the_backpack_what_is_carried`:
  - every line of the body, compact or full, is a line of the islander's datasheet (apart from what they're wearing, which comes from what they carry);
  - the vitals start with full stamina;
  - the backpack groups two sticks and a tuft exactly as expected;
  - the body changes after gathering.
- The script `first-steps-4-backpack-and-body.txt`, played by players' rules:
  - the backpack before gathering (nothing), after (two lumps of wood and three tufts), and after dropping one tuft (two), then empty;
  - the body before a hard day's work (stamina 3 MJ of 3 MJ), and after hauling driftwood all day (stamina 0).
  - Played in the client with both windows open (91 commands, 13 screenshots). The screenshots show the windows following each step, and the beach strewn with hauled driftwood.

**Sabotage checks.** Each of these made the test fail:
- a body line with a word added to the measured value ("about 3 MJ");
- the backpack not counting alike things;
- stamina left out of the vitals.

**Found along the way.** The built-in font has no "µ", so "0 µJ" showed a box. The client now uses Windows' own Consolas when it's there.

**Simplified:**
- No bars or icons: lines of text, as asked ("labels").
- The windows update every frame from the engine.
- Clicking works only on the body window. The backpack opens and closes by its key.
- Hunger and thirst are shown as what the engine measures (stored energy and body fluid), not as words like "hungry". Under players' rules, vitality keeps both up.

## Stage 5, results (2026-09-30)

**Passed.** Hold T and speak; let go, and the words appear in the console and run as if typed.

**How it works.** Speech becomes text on the player's own machine: Whisper (whisper.cpp, open source), with its English base model (142 MB, downloaded once beside the program). It's private, free, and works offline.
- The client reads the microphone itself. On release, it hears the words in the background, in about 0.7 seconds on the CPU.
- It only ever makes text: the same commands, checked by the same laws.
- At start, the console says whether voice is ready, and which microphone it opened ("Microphone Array (Intel® Smart Sound Technology…)" on the owner's laptop).
- While T is held, the input line says "(listening…)", then "(hearing…)".

**Building it from WSL**, which the owner asked to try before running it as a separate program. It compiled, with one fix and one speed-up:
- **The fix.** whisper.cpp names its libraries without the "lib" prefix on every Windows build, which only Microsoft's compiler wants; Rust looks for `libggml.a`. `client/vendor.sh` makes a patched copy (one line: `if (WIN32)` becomes `if (MSVC)`) from cargo's own download, into `client/vendor/` (not in git), and `run.sh` runs it.
- **The speed-up.** Built for a baseline x86 processor, Whisper took 6 seconds a command. Built for processors with AVX2 (x86-64-v3, roughly 2015 onwards), it takes 0.65 seconds: ten times faster, on the CPU alone, without a GPU. `run.sh` sets this.

**Hearing well.**
- **Priming.** Whisper is given the console's command words and the names of what the islander can see, carry, and go to, in their own words. Priming fixed "Rubwood" (rub wood) and "dripped wood" (driftwood), and only primes words the islander could use.
- **Shaping.** The voice layer turns what Whisper writes into what the console reads: lower case, no full stops or commas, a first word run into the next split apart ("dropwood" becomes "drop wood"), and clock times as the console writes them ("1805", "18.05", "6:05 pm", "13:00 hours" all become "18:05" or "13:00"). Bare numbers are times only after "until": "wait 600" stays seconds.
- **The console** now reads units as people say them, typed or spoken: "wait 10 minutes", "2 hours", "30 seconds", "1 days".

**Proofs:**
- **Played by voice.** `client/voice-proof.sh <script>` has Windows' own speech voice say each of a script's commands into a recording. The client (`--hear-script`) then hears each one as the islander would hear it there and then, primed by what they can see at that moment, runs it as heard, and checks the script's expectations. All four scripts tried pass:

  | Script | Commands spoken | Heard differently |
  | --- | --- | --- |
  | The fire (`first-steps-3-fire`) | 35 | 1 ("wait 10 minutes", which works) |
  | Asking (`companion-3-asking`) | 16 | 2 (the same) |
  | Barter (`companion-4-barter`, with the spear recipe) | 23 | 0 |
  | Backpack and body (`first-steps-4-backpack-and-body`) | 91 | 0 |

  So the fire was made again, spoken rather than typed.
- **The client's own test** `spoken_words_become_commands` covers the shaping. Sabotage checks failed it each time: without the splitting, without clock times, and with bare numbers taken as times anywhere.
- **The script** `first-steps-5-times-as-said.txt` covers the console reading long units. With the change removed, the script fails.

**What attempts found along the way:**
- Without priming, Whisper misheard "rub wood" and "driftwood".
- With commands-only priming, it fixed one and broke the other. Priming with what's in sight fixed both.
- The first voice proof missed commands from shared recipes (`include`), and now follows them.

**Simplified, and open:**
- Push-to-talk with a real voice through the real microphone hasn't been tried by me: it can't be, from here. The microphone opens on the owner's laptop. The owner's first try is the real test.
- Windows' speech voice is clear and steady. Real voices, accents, and a noisy room will do worse, and the base model may need to become the small one (466 MB, slower) if so.
- CPU only. Whisper on the GPU (Vulkan) would need the Vulkan SDK's shader compiler in the build: possible later if speed matters.
- English only, for now.

## A debugging grid (2026-09-30, the owner's request)

On plain sand nothing shows movement, and there won't always be texture. G shows and hides a grid on the ground:
- fine dark lines every 5 m, and yellow lines every 50 m;
- fixed to the world and draped over the land, so a walker visibly crosses it;
- 150 m around the islander, or around the camera when flying.

`--open grid` shows it at the start. The grid showed up a flaw in the land's shape: within 1 m of a place's centre, the height jumped to the place's own height instead of blending into it, which made a small pit. Fixed: the blend is smooth everywhere.

## Stage 6, results (2026-09-30)

**Passed.** Places have room in them. The design is [ideas/nearness.md](../ideas/nearness.md), agreed with the owner first.

**In the engine:**
- **Spots.** Everything in a place has a spot in it. Arrivals stand at the middle, a step apart, and what you drop lies at your feet.
- **Patches.** On the companion's island, the stocks lie in patches of their own: the forest's grass 25 m north-west, its sticks 20 m east, its twigs south-east; the hillside's stones and flint; the beach's driftwood and shellfish; the stream's bog iron and clay. The sea, the trees, and the forest floor cover their whole place.
- **Reach.** A person reaches 2.5 m and a boar 1 m: the reach their data already gave them, now across the ground as well as up. Handling, gathering, lighting, striking, and giving need the thing within reach. Otherwise: "Out of reach: the fallen sticks, 11 m east." `look` says how far and which way for what's out of reach: "fallen sticks (50 kg, 11 m east)". Talking, pointing, naming, offering, and paying reach the whole place.
- **Walking within a place.** `go to the sticks` or `walk to 10 -10` (metres east and north of the middle) walks there, at the walker's pace and load. `stop` stops partway. The edge of a place stops you: "You can't go further that way: it's the edge of the forest."
- **Minds and creatures walk up to what they act on.** Hungry, the stranger walks 30 m to the rocks before gathering. Asked to hand over wood, they walk over to the islander first, and keep the request until they're within reach.

**In the client:**
- **Drawing.** Everything is drawn at its spot from the engine: stocks across their patches, walkers on their way between spots.
- **Keys.** WASD walks the islander, relative to the camera. A key pressed during a walk someone typed or spoke takes over from where the islander has got to. Walking into a place's edge towards a way out sets off along that path. The arrows turn the camera (and fly it, in free flight, so WASD keeps walking).
- **`/reach`** draws the reach as a circle on the ground.

**Proofs:**
- The script `first-steps-6-near-things.txt`: too far, with distance and direction; `go to`; already there; dropped at your feet, then out of reach after walking away; a spot in metres; the edge; a walk stopped partway, with the grass 31 m off before and 27 m after three seconds' walk. It also passes played in the client (22 commands, 16 screenshots, with the grid on).
- The engine test `minds_walk_up_to_what_they_act_on`: the stranger gathers from beside the rocks, never from where they stood.
- **Every older script passes.** The island's scripts gained a `go to` before each gather from a patch, which is what this stage asks. The shared spear recipe walks too: in worlds without patches, `go to` just says you're already there.
- **Sabotage checks**, each failing the scripts: reach ignored (the original and the faster rewrite); creatures not walking up; dropping elsewhere than at your feet; stopping without the partial walk.
- **Trials:**
  - the stranger alive after 30 days in 10 of 10;
  - 60 of 60 boars alive after 30 days;
  - the castaways as before.
- **Scale:** a village's 121 bodies for 30 days take 54 s (47,600 times faster than real time) against 49 s before, with memory flat at 39 MB. That's about 10% for measuring reach, after replacing the square roots with squares.

**Found along the way:**
- The first scale run was 30% slower: every action measured the distance to everything in the place, with a square root each time. Comparing squared distances brought it back to about 10%.
- A request to a mind was refused when what was asked was out of the listener's reach. A mind walks up first, so "too far" no longer counts against a request.
- A mind took up a request, then walked up to it, and forgot it. It now keeps the request until within reach.

**Simplified, and open:**
- Seeing and talking are still place-wide. A trade's goods change hands across the place.
- A walk between places still can't be taken over halfway: the keys wait until arrival.
- WASD walks in steps of 1.3 m, renewed while held, so there may be a slight hitch between steps at real speed. The owner's first try will tell.
- Nobody but the engine has pressed WASD yet: screenshots can't. The walking keys were checked through the same commands they send (`walk to`, `stop`, `go #n`).

## After the owner's first try (2026-09-30)

The owner got halfway through the fire, and found three things:

- **Too many steps.** The fire is very detailed and technical, and the number of steps is the problem. To discuss: skills, and how processes should go.
- **The window froze and crashed while being resized.** CPU went up, everything froze, and it crashed. Not reproduced here: 1,200 programmatic resizes, a simulated mouse drag of the window's corner, and shrinking it to 130×40 all kept drawing at 120 frames a second. What was caught, once, without any resizing: the graphics driver "lost the device" on Vulkan, after which the client panicked on several threads and hung before dying, which matches what the owner saw. Resizing rebuilds the window's drawing surface each time, which gives a flaky driver many more chances to fail. Changes:
  - The client now draws with **DirectX 12** on Windows (Vulkan with `WGPU_BACKEND=vulkan`). It passed every test.
  - Everything the client logs, and any panic, now goes to **`client.log`** beside the program. If it ever freezes or crashes again, the reason is kept, even when it was started by a double click.
- **The backpack merged sticks and twigs** into "lump of wood x5". The islander calls both a lump of wood (they're recognised by material, not size), and the backpack grouped by name alone. Now alike means the same name and about the same size (within a quarter): "lump of wood x2: 400 g, about 200 g each" and "lump of wood x3: 60 g, about 20 g each". The test covers it. Whether people should have words for sizes of wood (twig, stick, log) is a vocabulary question for the coming conversation.
