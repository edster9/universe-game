# Tools: debugging, designing, and settings

Proposed 2026-09-30, and the layers decided the same day, from the owner's words in [requirements.md](../requirements.md) ("Debugging as a lasting feature"). Debugging tools aren't scaffolding to be thrown away: they become the single player's options and the world designer's tools. This is a proposal: the first piece is built, and the rest waits for the owner's agreement.

## Two vocabularies, kept apart

- **Commands** are the actor acting in the world: `go forest`, `gather sticks`, `ask the stranger to…`. They're in the actor's words, spoken or typed, and the laws decide what happens. Only what the actor perceives can be named (no oracles).
- **Tools** start with a slash: `/set grid on`, `/speed 64`. They're the person at the keyboard, outside the world, changing how they see it, how fast its time runs, and later what's in it. The slash means a tool can never be mistaken for something the actor does, and players know it from Minecraft and many online games.

## Settings: one list

Every setting has a name, a value (on or off, a number, or a choice), a key if it has one, and a line of help. The keys and the terminal change the same settings, so they can never disagree.

- `/settings` lists them all, with their values and keys.
- `/set <name> <value>` changes one; `/<name> <value>` is the same, shorter; `/<name>` alone flips an on/off setting.
- `/help` lists the tools.

The first settings (built):

| Setting | Key | Layer | What it does |
| --- | --- | --- | --- |
| `speed` | `[` `]` | single player | Game seconds a second (1 is real time) |
| `pause` | Space | single player | The world stops |
| `snap` | | single player | Back to real speed when what you're doing is done (on by default) |
| `grid` | G | always | A grid on the ground |
| `fly` | F | always | The camera flies free |
| `backpack` | B | always | The backpack window |
| `body` | V | always | The body window |

## Who may use what: three layers (decided 2026-09-30)

The owner's design ("Debugging as a lasting feature" in [requirements.md](../requirements.md)). Every tool declares its layer, and the layer, with the server's settings, says who may use it:

1. **Always allowed** (utility): tools that change only how you see and use the game, never the world: the grid, the console's size, the windows, help, listing settings. Allowed on any server. The free-flying camera is here for now, because the client draws only what the actor pictures (`view::scene`), so flying shows no more than the actor knows. **Open:** the owner doubts it belongs here, since in real multiplayer it could still help someone cheat; to discuss when multiplayer comes.
2. **Server's choice** (multiplayer): tools that change the world or yourself without changing anyone's time: god mode, feeding yourself, extra strength, acting as others, editing the world map. Each is allowed or not by the server's administrator, as Quake servers allowed god mode or didn't. Editing the world map has no cone-of-influence problem, so a server may allow it.
3. **Single player only**: tools that change the shared clock: speed, pause, and snapping back. In multiplayer they'd change time for everyone, so no server setting allows them. A server's own clock rate is the server's to set, not a tool.

In single player, every layer is allowed: nobody else is affected.

Edits still go through the gate, as a named source, like sunlight: god mode's healing and feeding yourself come from "the administrator" or "the designer", never from nowhere. Conservation holds, and the world records what was given and by whom.

Inspecting tools (datasheets, the gate's log, minds and memories) show what the actor couldn't know, so in multiplayer they're the server's choice, like god mode.

## Shortcuts for development (agreed 2026-10-01)

The owner: "all these shortcuts will be needed". See [challenges/skill-grounds.md](../challenges/skill-grounds.md).

| Tool | Layer | What it does |
| --- | --- | --- |
| `/save <name>` | single player | Writes the whole world to a file: the clock, every piece of matter, every mind, and who's played. **Built.** |
| `/load [name]` | single player | Picks a saved world up exactly where it was; with no name, "last", the save made when a game ends. **Built.** |
| `/saves` | single player | The saves there are, newest first. **Built.** |
| `/make <thing> [in <container>]` | server's choice | Puts something from the world's data a step and a half from you (the owner, 2026-10-02: not on your feet), from the designer as a named source: an amount of a material (`/make 2 kg wood`), shaped (`/make 300 g wood as shaft`), a design (`/make fire ring`), or a kit (`/make fire`: a fire laid in a ring and already burning). **Built.** |
| `/light <thing>` | server's choice | The designer's flame: heats something within reach past the point where it catches. **Built.** |
| **Build mode** (M, or `/build`) | server's choice | The owner's (2026-10-02): rearrange the world by hand. Hover over a thing for its bounding box and axes (red east–west, green up–down, blue north–south); hold the left button to drag it along the ground (it follows the ground's rise and fall), the middle button to lift or lower it; let go and it stays. The islander still walks; clicks don't open menus. Each move goes through the engine as the designer, so `/save` keeps it. Pointing finds what the pointer's ray meets first, by the shapes as drawn (a tree by its crown, a rock anywhere on it), or, for things too thin to hit, the nearest foot on screen; a thing is moved across the level it was taken hold of at, so it follows the pointer however it was taken. Showcase models move too, on screen only. It moves sources as well as things (a source whole), as the designer, outside the laws: what may move in play is the laws' and a flag's business, not this tool's (the owner, 2026-10-02). **Built.** Not yet: turning things (the engine keeps no turn for things), and anything alive or carried |
| **Showcase** (`/showcase`) | always | Models from an asset pack set out on the ground near the middle, listed in `client/style.toml`, to judge a pack before choosing models for things (2026-10-02, the owner: see them before buying the Pro pack). Not in the world: pointing names them and build mode moves them, but `/save` doesn't keep where. On by default while packs are on trial. **Built** |
| **Drawing** (`/quality low\|medium\|high\|ultra`, and one by one `/shadows off\|low\|medium\|high`, `/smoothing off\|fxaa\|smaa\|msaa`, `/bloom`, `/view <metres>`, `/fog`, `/vsync`, `/fps-cap <n\|off>`) | always | How the scene is drawn, for speed against looks, as games usually offer: a preset sets them all, changing one makes the quality "custom", and `/quality` alone says what each preset sets. They're the machine's, not the world's, so they're kept in `graphics.toml` beside the program (not when measuring, taking pictures, or playing scripts). High by default. Detail (simpler distant models), texture quality, and render scale join them once the benchmarks build them (`docs/research/rendering-benchmarks.md`). **Built** 2026-10-04 |
| `/place <thing> <east> <north> [<up>]` | server's choice | The same as text, in metres from the middle of the place. **Built.** |

**How it's built** (2026-10-01):
- **Saves** are files in a `saves` folder beside the data folder, one per name, about 75 KB for the companion's island. The client and the console save as "last" when a game ends (not the live channel or scripts, and not a client run that only takes a picture), and both start from a save with `--load <name>`. A save from an older version of the game that no longer fits is refused with the reason; there's no upgrading of old saves yet.
- **The designer** is a named source like sunlight and vitality: the gate counts the matter and energy the designer gives (heat, and the energy held in what's made), and conserves everything else. Two changes come from it: a new piece of matter, and heat. A design is made part by part, each piece the size it's gathered in the world, and assembled and measured like any other. A design with a shaped part is refused with what to do instead, since the part's material is the maker's choice. Kits are data (`[[kit]]`: a holder, what's inside, and whether it's lit), so the engine still names nothing.
- **Height** (2026-10-02): the engine keeps how far above the ground the designer holds something (`World::raised`, set by a designer's change), saved with the world and drawn by the client. No law uses it: nothing falls yet.
- **A layout made in build mode lives in a save.** To make it the world's own starting layout, Claude reads the save's spots back into the data file; a tool for that can come when it's needed.
- **Not yet:** making creatures (`/make <kind>`, a boar, a horse once there are horses), and words for sizes (`/make log`) until the skill grounds have them.

## Next steps (proposed)

1. The console's testing tools (`totals`, `datasheet`, `log`, `as`, `become`) also answer to a slash (`/datasheet me`), keeping the old names so scripts don't change.
2. Settings are remembered between sessions, in a file beside the program.
3. Tools can be spoken: "slash speed sixty-four".
4. Editing tools, with the designer as a named source, when the world designer's turn comes.
