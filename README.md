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

## Getting set up

### What's in git, and what isn't

Everything that's text is in git: the engine, the console, the client, the worlds' data, the scripts and proofs, the docs, the catalogue of what models mean, and a README for each asset pack. Three things aren't, and each has one place to come from:

| Not in git | Why | Where it comes from |
| --- | --- | --- |
| **Asset packs** (models and textures) | Size, and their makers' licences | Download each pack its README names, into `assets/third-party/` ([how](assets/third-party/README.md)) |
| **Source worlds** (Blender files) | Binary, and changing often while the system is trained | The S3 bucket `universe-game-worlds`, public to read: `blender/worlds.sh pull <world>` ([how](worlds/README.md)) |
| **What the scripts make** | Made from the above, or fetched by the scripts | Made again by running them: the Blender catalogue, worlds' exports, the voice model, DirectX's shader compiler, saves |

Without the packs and the worlds, everything still works: the proofs need neither, and the game client draws simple shapes of its own instead of models. (One more thing is never in git, for one experiment only: the AI translator's test harness, `prototypes/translator`, needs your own API key, in a `.env` file at the top of the project, as its `translate.py` explains.)

### 1. The engine and its proofs (Linux, macOS, or WSL)

You need Rust, installed with [rustup](https://rustup.rs/); the toolchain version is pinned in `rust-toolchain.toml`, and rustup installs it on first use.

```sh
# Every proof: law checks, conservation property tests, and every scripted story (about 15 s)
cargo test --workspace

# Play one scripted story and print its transcript
cargo run -p console -- --script data/scripts/stranded-8-crossing.txt

# Explore a world yourself at the console
cargo run -p console -- --world data/living.toml --as survivor

# Drive a person from a program: commands in, one line of JSON per reply out
cargo run -p console -- --world data/strangers-words.toml --as islander --live
```

At the console, type `help` for commands, and `datasheet <thing>` to see everything the engine measures about something. The long trials (`cargo test --workspace -- --ignored --nocapture`) take minutes and are run only now and then.

### 2. The game client (Windows, built in WSL)

The client is a native Windows program, built from Linux inside WSL2 and run on Windows' graphics card. In WSL you need, besides Rust:
- [Zig](https://ziglang.org/download/) 0.16, unpacked into `~/.local/zig/` (or on your `PATH`), and `cargo install cargo-zigbuild`;
- `cmake`, `python3`, `curl`, and `unzip` (`sudo apt install cmake python3 curl unzip`).

```sh
client/run.sh                    # build, install in C:\Users\<you>\universe-game\client, and play
client/run.sh --install          # build and install, without starting it
client/run.sh --world stranded   # another world
```

The first run downloads Whisper's English model (about 150 MB, for voice) and Microsoft's shader compiler (`dxcompiler.dll`, without which DirectX takes seconds to start). Set `UNIVERSE_HOME` to install somewhere else. Once installed, `client.exe` can be started from Windows too. The default world is the skill yard; see `client/src/main.rs` for every option.

### 3. The models

Download the packs listed in [assets/third-party](assets/third-party/README.md) and put their files where each pack's README says (for now, one free pack). The client picks them up the next time `client/run.sh` installs it.

### 4. Worlds built in Blender

You need [Blender](https://www.blender.org/download/) 5.0 on Windows (`blender/blender.sh` runs it from WSL; set `BLENDER` if it's installed somewhere else) and, in WSL, the [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html), which fetches worlds without an AWS account.

```sh
blender/blender.sh make_catalogue.py   # the models' library, from the packs (once, and when the catalogue changes)
blender/worlds.sh pull skill-yard      # fetch the world
blender/convert.sh skill-yard          # export it and convert it for the game
client/run.sh                          # and play it
```

To change a world, open `worlds/<world>/<world>.blend` in Blender, save, convert again, and send it back with `blender/worlds.sh push <world>`, which needs write access to the bucket (the project's AWS profile); in a fork, set `WORLDS_BUCKET` to a bucket of your own. See [blender/README.md](blender/README.md).

### 5. Committing

Commit as usual. The ignore rules keep the packs, the worlds, and everything the scripts make out of git, so `git status` never shows them. Before committing, run `cargo fmt --all`, `cargo clippy --workspace --all-targets`, and the proofs; for the client, the same in `client/` (it's built on its own, outside the workspace).

## How it's laid out

| Path | What's there |
| --- | --- |
| `crates/engine` | The world engine: pure, deterministic, whole numbers only (no floating point), with no files, clock, or randomness of its own |
| `crates/console` | The text console, the script runner, and the live JSON channel |
| `data/` | Worlds and the things in them: materials, shapes, designs, kinds of creature, peoples' words |
| `data/scripts/` | Scripted stories that prove each stage, with shared recipes in `skills/` |
| `client/` | The game's native client (Rust and Bevy): the world drawn live from the engine, built in WSL for Windows by `client/run.sh` |
| `blender/` | Tools for worlds built in Blender: the catalogue of what models mean, and scripts to make the models' library, convert worlds, and fetch and send them |
| `worlds/` | Source worlds, fetched from S3; only its README is in git |
| `assets/third-party/` | Where downloaded asset packs go. **The packs aren't in git** (size and licences); each pack's README says where to get it and which files the game uses. Without them the client draws its own simple shapes |
| `docs/` | The design record: requirements in the owner's words, proposals and decisions, the [register of every law](docs/laws.md), and research. Start at [docs/README.md](docs/README.md) |

## How it's being made

The project is steered by conversation. Its owner sets the direction and the rules in plain language, and the code, tests, and design notes are written with [Claude Code](https://claude.com/claude-code); commits carry a co-author line. Nothing is taken on trust: every stage has scripted proofs, laws are deliberately broken to check the tests catch it, and the docs record what each attempt found, including what went wrong. `CLAUDE.md` holds the standing rules and working practices.

**Next:** a village, with other people who follow the realistic rules, trade, and money, and a first simple browser view.

## License

[MIT](LICENSE). Free to use, study, and adapt. The MIT licence covers this repository's own files; third-party asset packs keep their makers' licences and aren't included (see [assets/third-party](assets/third-party/README.md)).
