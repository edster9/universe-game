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
| 6 | Being near things | **Space within places**, an engine law: positions in metres inside each place, a reach (people's is 2.5 m), and walking with WASD. You have to walk up to the grass to cut it; commands that need a thing within reach say so ("the grass is too far away"). Walking off along a path takes you to the next place. WASD keeps moving the islander while the camera flies free | Scripts with positions: too far, walk closer, done; the fire made by walking up to each thing |

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
- Enter types and sends; Esc stops typing; up and down bring back earlier commands; the key left of 1 (`) cycles it through small, large, and hidden.
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
- While flying free, WASD moves the camera, not the islander. The islander doesn't walk by keys until stage 6.
- Commanding others (`as`, `become`) still works in the console, as a developer's tool.
