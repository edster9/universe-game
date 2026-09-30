//! The world engine: laws, the gate, nature, and data loading.
//!
//! The engine is pure. It has no files, clock, threads, network, or
//! randomness of its own: text and commands go in, a new world comes out.
//! That keeps it deterministic and lets the same code run in a browser, on a
//! server, and in tests. See docs/technology.md and docs/ideas/world-engine.md.

pub mod data;
pub mod datasheet;
pub mod gate;
pub mod instinct;
pub mod intent;
pub mod laws;
pub mod matter;
pub mod nature;
pub mod units;
pub mod view;
pub mod words;
pub mod world;
