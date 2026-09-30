# Universe game

**A world engine that knows laws, not things.** It simulates heat, hardness, energy, wounds, hunger, and memory, and conserves mass, energy, and money through a single gate. It has no idea what a sword, a raft, or a boar is: those live in data files, and inventions get their names from the people who make them.

> **This is an educational experiment.** It's an exploration of how far a small set of physical laws can go toward a believable world, built in the open, one tested stage at a time. It isn't a game you can play yet, and nothing here is stable.

## The idea

The long-term goal is a small, networked space game. Rather than hardcoding a space-age world, the engine is being *trained* by climbing the technological ladder the way people did: from a castaway on a desert island with nothing, through fire, tools, metal, boats, and villages, toward electricity, industry, flight, and eventually a spaceship put into orbit. Every rung is built under the same laws, so everything that exists later has a real route back to raw materials.

A few rules hold throughout:

- **Laws, not things.** The engine knows how heat flows and what an edge does. It never names a material, shape, item, or creature; a test scans the engine's source to make sure.
- **Nothing from nowhere.** Mass, energy, and credits are conserved. Energy enters only through named sources, such as sunlight.
- **No oracles.** People perceive and name only what they know. A stranger who has never seen flint sees "glassy grey stone".
- **Dying is an outcome, not a failure.** A scenario fails only if there's no logical way forward.
- **AI proposes, the engine decides.** Anything that interprets what a player says only produces commands; the laws decide what happens.

## What works so far

Everything is text, driven by commands like a classic text adventure. The engine has passed a series of training challenges, each written down with what it found and what's simplified:

| Challenge | What happens |
| --- | --- |
| [Stranded](docs/challenges/stranded.md) | Survive on a desert island: water, fire by friction, a spear and fishing, casting an iron axe, felling trees, rope, a raft, and the crossing to the next island |
| [Where am I?](docs/challenges/where-am-i.md) | Day and night, sleep, exploring unknown paths, carrying water in fired pots, and climbing a 1,800 m mountain to see an archipelago, and a village's smoke |
| [Living with the island](docs/challenges/living-with-the-island.md) | Shelter, wild boars with an instinct of their own, wounds and bleeding, the hunt, butchering and cooking, leather shoes that wear out on stony paths, and a raised cache boars can't reach |
| [A stranger's words](docs/challenges/strangers-words.md) | Names live in minds, not in the world: two people from different peoples see and name the same things differently, invent things, and teach each other words |

Along the way: memory and maps that can be wrong, players who live on "vitality" instead of food and sleep, and a clock that runs for everyone, with actions that take time and can be cut short.

A taste, from the [stranger's words](data/scripts/words-1-two-peoples.txt) proof. The islander has made a spear; the stranger's people have never seen one:

```text
> take the spear
You don't see the spear here.

> take the pole
You take the barb of glassy grey stone joined to long straight pole of wood.

(the islander: tell the stranger that the spear is a spear)

> take the spear
You take the spear of glassy grey stone and wood.
```

## Running it

You need Rust, installed with [rustup](https://rustup.rs/); the toolchain version is pinned in `rust-toolchain.toml`.

```sh
# Every proof: law checks, conservation property tests, and every scripted story (about 15 s)
cargo test --workspace

# The long trials: survivors on many seeds, with real chance (a few minutes)
cargo test --workspace -- --ignored --nocapture

# Play one scripted story and print its transcript
cargo run -p console -- --script data/scripts/stranded-8-crossing.txt

# Explore a world yourself at the console
cargo run -p console -- --world data/living.toml --as survivor

# Drive a person from a program: commands in, one line of JSON per reply out
cargo run -p console -- --world data/strangers-words.toml --as islander --live
```

At the console, type `help` for commands, and `datasheet <thing>` to see everything the engine measures about something.

## How it's laid out

| Path | What's there |
| --- | --- |
| `crates/engine` | The world engine: pure, deterministic, whole numbers only (no floating point), with no files, clock, or randomness of its own |
| `crates/console` | The text console, the script runner, and the live JSON channel |
| `data/` | Worlds and the things in them: materials, shapes, designs, kinds of creature, peoples' words |
| `data/scripts/` | Scripted stories that prove each stage, with shared recipes in `skills/` |
| `docs/` | The design record: requirements in the owner's words, proposals and decisions, the [register of every law](docs/laws.md), and research. Start at [docs/README.md](docs/README.md) |

## How it's being made

The project is steered by conversation. Its owner sets the direction and the rules in plain language, and the code, tests, and design notes are written with [Claude Code](https://claude.com/claude-code); commits carry a co-author line. Nothing is taken on trust: every stage has scripted proofs, laws are deliberately broken to check the tests catch it, and the docs record what each attempt found, including what went wrong. `CLAUDE.md` holds the standing rules and working practices.

**Next:** a village, with other people who follow the realistic rules, trade, and money, and a first simple browser view.

## License

[MIT](LICENSE). Free to use, study, and adapt.
