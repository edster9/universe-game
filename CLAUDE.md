# Universe game

A world engine for a small, networked space game. Design docs are in `docs/`; start at `docs/README.md`. The owner steers by conversation and reads very little code, so tests are the review: each slice's pass/fail test in `docs/slices.md` is its acceptance test. Record the result there when a slice is built.

## Commands

- `cargo test --workspace`: every test, including the conservation property tests and the challenge trials (about 45 seconds; tests build optimised).
- `cargo run -p console`: the text console (`-- --world <file> --as <person-id>` to change world or player).
- `cargo clippy --workspace --all-targets` and `cargo fmt --all` before committing.
- Rust comes from rustup, pinned in `rust-toolchain.toml`. vfox's Rust on this machine lacks the standard library; put `~/.cargo/bin` first in `PATH`.

## Engine rules

- `crates/engine` is pure: no files, clock, threads, network, or randomness. I/O belongs in `console` (and later `server`).
- No floating point in the simulation. Whole numbers in real units (`units.rs`): milligrams, and so on.
- Deterministic: `BTreeMap`/`BTreeSet`, never iteration over a `HashMap`.
- Laws propose `Change`s; only the gate (`World::apply`) changes the world. It checks that mass, energy, and credits are conserved and the invariants hold, and logs every accepted set of changes. Nature (`nature.rs`) proposes changes through the gate too.
- Energy is stored as heat per piece of matter, in whole microjoules; temperature is derived from it.
- Laws in code, things in data: engine code never names a particular material, shape, or item, and `tests/slice1.rs` scans the source to enforce it. Things go in `data/*.toml`.
- No oracles: names resolve only among what the actor can perceive, and programs will only know what their sensors measure.
- Every action has an actor.
- Challenges (`docs/challenges/`) are played by a survivor in the tests, through commands. Dying is an outcome; only running out of options fails. Record each stage's result in the challenge doc, and each law in `docs/laws.md`.
- Datasheets (`datasheet.rs`) are measured, never written in data. An assembly is measured once from its parts' datasheets and never looks inside them.
