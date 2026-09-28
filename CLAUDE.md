# Universe game

A world engine for a small, networked space game. Design docs are in `docs/`; start at `docs/README.md`. The owner steers by conversation and reads very little code, so tests are the review: each slice's pass/fail test in `docs/slices.md` is its acceptance test. Record the result there when a slice is built.

## Commands

- `cargo test --workspace`: every test, including the conservation property tests.
- `cargo run -p console`: the text console (`-- --world <file> --as <person-id>` to change world or player).
- `cargo clippy --workspace --all-targets` and `cargo fmt --all` before committing.
- Rust comes from rustup, pinned in `rust-toolchain.toml`. vfox's Rust on this machine lacks the standard library; put `~/.cargo/bin` first in `PATH`.

## Engine rules

- `crates/engine` is pure: no files, clock, threads, network, or randomness. I/O belongs in `console` (and later `server`).
- No floating point in the simulation. Whole numbers in real units (`units.rs`): milligrams, and so on.
- Deterministic: `BTreeMap`/`BTreeSet`, never iteration over a `HashMap`.
- Laws propose `Change`s; only the gate (`World::apply`) changes the world. It checks conservation and invariants and logs every accepted action.
- Laws in code, things in data: engine code never names a particular material or item. Things go in `data/*.toml`.
- No oracles: names resolve only among what the actor can perceive, and programs will only know what their sensors measure.
- Every action has an actor.
