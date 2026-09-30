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
