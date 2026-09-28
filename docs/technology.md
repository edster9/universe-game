# Technology

Decided 2026-09-28 unless marked as a proposal. The constraints behind these choices are in [ideas/code-and-laws.md](ideas/code-and-laws.md).

## How this project is built

Most of the code is written by AI in conversation. The project owner sets direction and requirements, and reads very little code. So:

- The compiler and the tests do the reviewing. Strong types, property tests, and each slice's pass/fail test in [slices.md](slices.md) are how we know the code is right.
- These docs are the record of intent. When code and docs disagree, one of them gets fixed on purpose.

## Decisions

- **Engine language: Rust.** It gives tight control of determinism, it's fast, and its compiler catches many mistakes before the code runs. It also compiles to WebAssembly and has the best tooling for hosting sandboxed player code.
- **Delivery: a URL.** Players open a web page and play. Nothing to download. The engine compiles to WebAssembly for the browser, and to native code for servers.
- **The core is pure.** The engine core is a library with no file access, clock, threads, network, or randomness of its own. Commands go in, a new world state comes out. That keeps it deterministic, and it lets the same core run in a browser, on a server, and in tests.
- **Whole numbers in real units.** For example milligrams, millijoules, millikelvin, and milliseconds, stored as integers. Floating-point sums drift, so a check that "total mass never changes" would fail on rounding alone. Integers make conservation exact and give identical results on every machine.
- **Data in TOML.** Materials, processes, and designs are TOML files, readable and editable by people.
- **No game engine or framework yet.** Slices 0 to 5 are a plain Rust program with a text console.
- **Testing.** Property tests (thousands of generated random commands) for conservation, and scripted console sessions for each slice's test.

## Player code

### Compiled, but not into the engine

Player programs must run fast, so they can't be slowly interpreted. But they also must not be compiled *into* the engine as native Rust code:

- Native code can't be sandboxed. A program could read or change anything in memory, including other players' secrets and the conservation totals.
- It can't be metered, so in-game power couldn't limit it.
- Every new program would need a new build of the engine, and one bad program could crash a whole universe.

### WebAssembly instead

A player's program is compiled ahead of time to WebAssembly. On the server, the runtime (wasmtime) compiles that into native machine code before it runs, so it runs at close to native speed, not interpreted. What makes it safe:

- **Sandboxed by construction.** A module can only call the functions the engine hands it.
- **Metered.** The runtime counts "fuel" per instruction, which becomes in-game power.
- **Portable.** The same module runs on the server and in the browser.

### The programming model: one tick at a time

Each tick, the engine calls the program's `tick` function. The program reads its sensors, sets its actuators, and returns. Its memory persists between ticks.

- If it runs out of fuel partway through, the tick's actions are discarded: the chip browned out.
- Because programs never pause mid-call, saving a program or moving it to another region's server is just copying its memory.
- Screeps has used this model for a decade.

### The interface

A program sees exactly this:

- it exports `tick`;
- it imports `read(port)`, `write(port, value)`, `send(port, message)`, and `receive(port)`.

There is nothing else: no clock, no randomness, no files. Time is a sensor. Randomness is a device you install, a noise sensor that is a part like any other. Everything a program knows comes through a port (see [ideas/code-and-laws.md](ideas/code-and-laws.md)).

### Chips

A chip's datasheet sets:

- **fuel per tick**, its speed;
- **memory**, the pages of WebAssembly memory it allows;
- **ports**, how many devices it can be wired to;
- **features.** WebAssembly is every chip's instruction set, but a cheap or old chip may lack some features, such as floating point. A program that uses a missing feature is refused when it's loaded.

### Languages (proposal)

The engine only sees WebAssembly, so any language that compiles to it works. The question is which language the game offers by default.

- **The default: a small language of our own.** Simple to read, in the spirit of Lua or Python. Typed enough that mistakes show up before the program runs. Units in the types, so adding kilograms to joules is an error. The device interface built in. Its compiler is written in Rust, so it runs in the browser: write, compile, and test without leaving the page. In the fiction, the language and its compiler can be released knowledge like any other design.
- **Bring your own.** Rust (with a small library we publish), Zig, C, AssemblyScript, TinyGo, or anything else that compiles to WebAssembly. Players upload the compiled module. It gets the same sandbox and the same fuel.

Why not Rust as the default: compiling Rust in the browser isn't practical, and it's a steep language for someone who wants to write an autopilot.

Why not Lua, JavaScript, or Python: they would run as interpreters inside the sandbox, which is slower. And hackmud's experience is that JavaScript is hard to sandbox.

### The workbench

Because the engine runs in the browser, a player can test a program against a local copy of a device before installing it for real: feed the validator a fake coin and watch what it does. The real run is always on the region's server.

## Deferred

- **Networking**, at slice 8. SpacetimeDB or our own server.
- **Graphics**, at slice 10. Bevy (Rust, runs on the web) is the natural candidate.
- **Hosting and cost**, once there is something to host.
