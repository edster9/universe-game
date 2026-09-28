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

## Browser and server

The server holds the truth. The browser shows what your character can perceive. A browser is never trusted, because anyone can modify their own.

| | Server: one authority per region | Browser |
| --- | --- | --- |
| The laws | Runs them on the whole region, every tick | Runs the same engine core on a partial copy, to predict the next moment |
| NPCs, towns, economy | All of it | Only what the player can perceive |
| Player programs | Runs them, and keeps them secret | Only the player's own, on the workbench, for testing |
| Conservation checks, saving | Yes | No |
| Input | Checks and applies it | Sends intents ("put ore in the furnace"), never results ("my gold is now 500") |

**The browser is sent only what the character can perceive.** This is the same rule as for programs: you know what your eyes and instruments measure. A player with a cheap scanner receives less than one with a good scanner. A hacked browser can't reveal what it was never sent.

## Staying in sync

**Within a region:**

1. Each browser sends commands tagged with a tick number.
2. The server applies all commands in a fixed order each tick.
3. It sends each player the changes they can perceive.
4. Browsers predict locally, and accept the server's correction when it differs.

At a 1-second tick in text, prediction hardly matters. It matters once there is real-time flight.

**Between regions.** Influence has a maximum speed, so region B doesn't need to know what happened in region A until a ship, a relay message, or a courier could have carried it. Regions don't run in lockstep. They exchange timestamped messages and otherwise run in parallel.

**Handoff happens exactly once.** A ship crossing between regions carries its state and its programs' memory. If a server crashes partway, the ship must be neither duplicated nor lost. That is conservation across servers, and it takes a two-step handoff.

**Crash recovery.** Every change already goes through the gate and is logged. The server saves snapshots periodically. After a crash, it reloads the latest snapshot and replays the log. Because the engine is deterministic, the replay produces the same world.

## Scaling

**The limit is players per region, not per universe.** A universe is a set of regions, each using one server core, so more regions means more capacity. The ceiling is the most crowded spot: when everyone piles into one market, each person's actions must reach everyone else, and the cost grows roughly with the square of the crowd.

What uses a region's capacity:

- **Simulation depth:** entities times laws, every tick. This is the expensive part of this design.
- **NPCs**, each with needs and decisions.
- **Player programs.** In-game power is real server CPU in the end. Programs that run while their owners are offline cost money with nobody playing.
- **Tick rate.** One tick a second in text is cheap. Real-time flight later needs roughly 20 to 30 ticks a second where it happens, which cuts capacity there.

**Overload is time dilation.** When a region can't finish a tick in time, its clock slows down, as EVE Online does. Everything still computes correctly, just more slowly. That is a law of the universe, not a crash.

**Target, to be measured:** a few hundred active players in one region and a few thousand in a universe. Slice 3 gives the first real numbers for simulation cost. When there are more players, open more universes.

## Adding content without client updates

The browser doesn't need to know what a gold detector is, any more than the engine does. A new item is a design (data) plus firmware (a compiled WebAssembly program). Neither is engine code.

1. The inventor writes and tests the program on the workbench.
2. They build the device in the world, which still takes parts, skill, and the chain.
3. They install the firmware. The server stores the program, compiles it in the background, and runs it from the next tick, with no restart.
4. Other players' browsers never receive the code. They receive the device as an ordinary entity (mass, shape, label) and what it visibly does: a light, a display reading "REJECT".

Browsers fetch entities when they first perceive them, and cache them after that.

**What does need an update:** a change to the laws or the display. Those are the team's releases, not player content. Because the game is a web page, an update is a reload. Each universe runs a pinned version of the laws, so new laws can roll out one universe at a time.

**Appearance will be generated.** When graphics arrive, nobody will have drawn an invented item. What it looks like has to come from its design: its parts, shapes, and materials. Only the look of each material needs downloading.

## Running on AWS

AWS is the cloud of choice. The plan grows in stages, and nothing is in the cloud until the slices need it.

| Stage | What runs | Where |
| --- | --- | --- |
| Slices 0 to 5 | The console program | Locally. No cloud. |
| Slice 6 | The workbench and compiler as a static web page | S3 plus CloudFront |
| Slice 8 | One universe server, two players | One EC2 instance |
| Slice 9 | Several regions with handoffs | Separate processes on that one instance, then several instances |
| Many universes | Many region servers | ECS on EC2, with a directory of universes and regions |

### The universe server

A region's server is a long-running process that holds its world in memory, runs a steady tick loop, and keeps a live connection to every player in it. That shape decides the service:

- **Not Lambda** for the simulation. Lambda functions are short-lived and hold no state between calls.
- **Not GameLift for now.** It is built mainly for match-based games with sessions that start and end, and adds cost and complexity a persistent universe doesn't need early. Revisit it if matchmaking ever matters.
- **Not SpacetimeDB.** It was listed as a candidate earlier. It would own the tick and state model that our engine already defines, so it's dropped.
- **EC2 first, then ECS on EC2.** One Graviton (ARM) instance runs the Rust server during slices 8 and 9. Rust compiles for ARM easily, and Graviton instances cost less for the same work. Later, each region group runs as an ECS task on a cluster of instances.

Inside the server, one thread runs the simulation, deterministic and alone. Networking runs on other threads (Rust's tokio), and feeds commands into a queue the simulation reads each tick.

### Other services

- **Browser connections: WebSockets over TLS.** Early on, the server handles TLS itself (or through Caddy on the same instance), which avoids a load balancer's monthly cost. An Application Load Balancer comes later. Real-time flight may later want WebTransport, which browsers are adopting for faster, UDP-style traffic.
- **Snapshots and logs:** written to the instance's disk as they happen, and shipped to S3 regularly.
- **Player programs:** stored in S3 under a hash of their contents. Uploaded programs are checked and compiled in isolation. Lambda suits this job: short, stateless, and each run in its own micro-VM.
- **Accounts:** Amazon Cognito.
- **Directory** (universes, regions, which server hosts which region, player accounts' metadata): DynamoDB. The world itself is never in a database; it lives in memory, snapshots, and logs.
- **Monitoring:** CloudWatch. The key number is tick time. When it approaches the tick length, a region is about to dilate.
- **Infrastructure as code:** AWS CDK, so every environment is reproducible.

### Rough cost

Approximate, and to be checked against current pricing:

- **Slices 0 to 5:** nothing.
- **Slice 6:** a static site on S3 and CloudFront, pennies to a few dollars a month.
- **Slices 8 and 9:** one small Graviton instance for testing, roughly $10 to $60 a month depending on size.

## Code layout

One Rust workspace, with separate parts:

- **engine:** the pure core: laws, the gate, data loading. (Rust reserves the name `core`.)
- **console:** the text program for slices 0 to 5.
- **compiler:** the player language, from slice 6.
- **web:** the browser client and workbench.
- **server:** the universe server, from slice 8.
- **protocol:** the messages browsers and servers exchange.

## Local development

- **Rust comes from rustup**, Rust's own installer. `rust-toolchain.toml` pins the version (1.90.0) and the lint and format tools, and rustup installs them automatically for anyone who builds the project.
- **Not vfox for Rust.** The Rust that vfox installed on the development machine has the compiler but not the standard library, so it can't build anything. If vfox's Rust comes first in the shell's path, rustup's tools in `~/.cargo/bin` have to come before it.
- **No Docker for the development loop.** Building on the host is faster, especially under WSL, and the pinned toolchain already makes builds repeatable. Docker arrives with the server at slice 8, because ECS runs containers anyway.
- **Commands:** `cargo test --workspace` runs every test. `cargo run -p console` starts the console. `cargo clippy --workspace --all-targets` runs the lints.

## Deferred

- **Graphics**, at slice 10. Bevy (Rust, runs on the web) is the natural candidate.
- **Real-time transport** (WebTransport) for flight.
- **Multiple AWS environments** (test and live) and their deployment pipeline.
