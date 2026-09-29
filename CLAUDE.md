# Universe game

A world engine for a small, networked space game: laws in code, things in data, nothing created from nothing. Design docs are in `docs/`; start at `docs/README.md`. This file holds the standing rules and how we work, so a new session can pick up where the last left off.

## Standing rules

These hold for every design decision unless the owner changes them.

1. **Don't overcomplicate.** Simulate the spirit of the real world, not every detail. Prefer fewer, broader laws, and remove a law before adding a finer one. Put fussiness in data files as numbers. Stop refining once a challenge plays right: the right steps succeed, the wrong ones fail for the right reason, and nothing depends on timing a player couldn't reasonably know. (Set 2026-09-28 after the fire stage; see "Enough physics" in `docs/ideas/world-engine.md`.)
2. **Laws, not things.** The engine knows how heat, hardness, energy, and money behave. It never names a particular material, shape, item, or design. Things go in `data/*.toml`. `tests/slice1.rs` scans the engine source to enforce it: one-word names are forbidden as words, longer names as phrases.
3. **Nothing from nowhere.** Mass, energy, and credits are conserved through one gate. Energy enters only through named sources (so far, sunlight, which the gate tracks).
4. **No oracles.** Names resolve only among what the actor can perceive. Programs will only know what their sensors measure.
5. **Dying is an outcome, not a failure.** A challenge fails only if the player runs out of logical options.
6. **AI proposes, the engine decides.** Anything that interprets what a player says (our own interpreter, or an AI if one helps) only produces commands; the laws decide what happens.

## How we work

- **The owner steers by conversation and reads very little code** ("vibe coding"). Explain changes in terms of behaviour and design, not code. The compiler and the tests are the review.
- **Docs are the record of intent.** Record the owner's requirements in their own words in `docs/requirements.md`. Put proposals in `docs/ideas/`, sourced surveys in `docs/research/`, and when a proposal becomes a decision, date it. Update `docs/laws.md` whenever a law is added, changed, or removed.
- **Build in stages, each with a test.** Slices are in `docs/slices.md`; challenges are in `docs/challenges/`. Record each stage's result, what the attempt found, and what's simplified, in its doc.
- **Check tests have teeth.** After a stage passes, sabotage its key law and confirm a test fails, then restore it.
- **Report honestly**, including failures, bugs found, and anything left simplified.
- **Hosting is AWS.** Recommend AWS services first; the staged plan is in `docs/technology.md`.
- **Commits** end with a `Co-Authored-By` line for Claude.

## Testing

- **Proofs, every run** (`cargo test --workspace`, about 15 seconds): law checks, conservation property tests, the no-names scan, and every script in `data/scripts/`.
- **Scripts** are plain command files with `repeat … end`, optional `try` commands, and expectations (`alive`, `dead of`, `said`, `not said`, `time after/before`, `conserved`). See `crates/console/src/script.rs`. Use fixed luck (`luck average|good|bad`), not a seed: a seed's luck shifts whenever the engine's history changes.
- **Trials, on demand** (`cargo test --workspace -- --ignored --nocapture`): survivors play many seeds with real chance and report success rates. Run them when a law changes and before recording a stage's result.
- `cargo run -p console -- --script data/scripts/<file>.txt` plays one script and prints its transcript. `cargo run -p console -- --world <file> --as <person-id>` opens the console.
- Run `cargo fmt --all` and `cargo clippy --workspace --all-targets` before committing.

## Toolchain

- Rust comes from rustup, pinned in `rust-toolchain.toml`. vfox's Rust on this machine lacks the standard library; put `~/.cargo/bin` first in `PATH`.
- Tests build optimised (`[profile.test]` in `Cargo.toml`) so multi-day simulations run in seconds.

## Engine rules

- `crates/engine` is pure: no files, clock, threads, network, or randomness of its own. I/O belongs in `console` (and later `server`).
- No floating point in the simulation. Whole numbers in real units (`units.rs`).
- Deterministic: `BTreeMap`/`BTreeSet`, never iteration over a `HashMap`. Chance comes from `World::roll`, drawn from the seed or fixed luck.
- Laws propose `Change`s; only the gate (`World::apply`) changes the world. It checks conservation and invariants and logs every accepted set of changes. Nature (`nature.rs`) proposes changes through the gate too, in steps of one second when things are hot or busy and up to a minute when calm.
- Energy is stored as heat per piece of matter, in whole microjoules; temperature is derived from it.
- Datasheets (`datasheet.rs`) are measured, never written in data. An assembly is measured once from its parts' datasheets and never looks inside them.
- Every action has an actor. Things in hand are found before things around.

## Where things stand

- Slices 0 to 2 are built. See `docs/slices.md`.
- The stranded challenge (`docs/challenges/stranded.md`) is in progress, ahead of slice 3: stages 1 to 4 have passed. Next is stage 5, timber: felling trees with the axe, where work takes time set by the tool's edge and the effort.
- Directions recorded for later: skills and a spoken interface (`docs/ideas/skills-and-interface.md`), recognising what things are (`docs/ideas/recognition.md`), money (`docs/ideas/money.md`), many universes (`docs/ideas/universes.md`).
