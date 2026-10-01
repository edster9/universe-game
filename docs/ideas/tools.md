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

1. **Always allowed** (utility): tools that change only how you see and use the game, never the world: the grid, the console's size, the windows, help, listing settings. Allowed on any server. The free-flying camera belongs here because the client draws only what the actor pictures (`view::scene`), so flying shows no more than the actor knows.
2. **Server's choice** (multiplayer): tools that change the world or yourself without changing anyone's time: god mode, feeding yourself, extra strength, acting as others, editing the world map. Each is allowed or not by the server's administrator, as Quake servers allowed god mode or didn't. Editing the world map has no cone-of-influence problem, so a server may allow it.
3. **Single player only**: tools that change the shared clock: speed, pause, and snapping back. In multiplayer they'd change time for everyone, so no server setting allows them. A server's own clock rate is the server's to set, not a tool.

In single player, every layer is allowed: nobody else is affected.

Edits still go through the gate, as a named source, like sunlight: god mode's healing and feeding yourself come from "the administrator" or "the designer", never from nowhere. Conservation holds, and the world records what was given and by whom.

Inspecting tools (datasheets, the gate's log, minds and memories) show what the actor couldn't know, so in multiplayer they're the server's choice, like god mode.

## Next steps (proposed)

1. The console's testing tools (`totals`, `datasheet`, `log`, `as`, `become`) also answer to a slash (`/datasheet me`), keeping the old names so scripts don't change.
2. Settings are remembered between sessions, in a file beside the program.
3. Tools can be spoken: "slash speed sixty-four".
4. Editing tools, with the designer as a named source, when the world designer's turn comes.
