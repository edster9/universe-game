# Tools: debugging, designing, and settings

Proposed 2026-09-30, from the owner's words in [requirements.md](../requirements.md) ("Debugging as a lasting feature"). Debugging tools aren't scaffolding to be thrown away: they become the single player's options and the world designer's tools. This is a proposal: the first piece is built, and the rest waits for the owner's agreement.

## Two vocabularies, kept apart

- **Commands** are the actor acting in the world: `go forest`, `gather sticks`, `ask the stranger to…`. They're in the actor's words, spoken or typed, and the laws decide what happens. Only what the actor perceives can be named (no oracles).
- **Tools** start with a slash: `/set grid on`, `/speed 64`. They're the person at the keyboard, outside the world, changing how they see it, how fast its time runs, and later what's in it. The slash means a tool can never be mistaken for something the actor does, and players know it from Minecraft and many online games.

## Settings: one list

Every setting has a name, a value (on or off, a number, or a choice), a key if it has one, and a line of help. The keys and the terminal change the same settings, so they can never disagree.

- `/settings` lists them all, with their values and keys.
- `/set <name> <value>` changes one; `/<name> <value>` is the same, shorter; `/<name>` alone flips an on/off setting.
- `/help` lists the tools.

The first settings (built):

| Setting | Key | What it does |
| --- | --- | --- |
| `speed` | `[` `]` | Game seconds a second (1 is real time) |
| `pause` | Space | The world stops |
| `snap` | | Back to real speed when what you're doing is done (on by default) |
| `grid` | G | A grid on the ground |
| `fly` | F | The camera flies free |
| `backpack` | B | The backpack window |
| `body` | V | The body window |

## Who may use what (later)

Tools come in kinds, and a world or a server says who may use each kind:

- **Viewing**: grid, flying, speed and pause in single player, overlays. Change nothing in the world. Anyone, offline.
- **Inspecting**: datasheets, the gate's log, totals, minds and memories. Developers and designers, since they show what the actor couldn't know.
- **Acting as others**: `as`, `become`, commanding NPCs. Developers and designers.
- **Editing**: placing and removing things, changing a place, setting time and weather. Designers. Edits still go through the gate, as a named source, like sunlight: "nothing from nowhere" holds, and a designed world records what its designer put in.

## Next steps (proposed)

1. The console's testing tools (`totals`, `datasheet`, `log`, `as`, `become`) also answer to a slash (`/datasheet me`), keeping the old names so scripts don't change.
2. Settings are remembered between sessions, in a file beside the program.
3. Tools can be spoken: "slash speed sixty-four".
4. Editing tools, with the designer as a named source, when the world designer's turn comes.
