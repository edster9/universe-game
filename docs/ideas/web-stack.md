# The browser stack

Claude's take, 2026-09-30. **Not decided, and superseded:** once the owner said the game needn't live at a URL, the recommendation moved to a native client ([the-client.md](the-client.md)). This file keeps the web analysis. The owner asked for a quick discussion of the browser-side stack, with pros and cons, before the first browser page ("The browser stack" in [requirements.md](../requirements.md)). Not 3D yet; that's the rendering conversation ([rendering.md](rendering.md)).

## Already decided

[technology.md](../technology.md) settled the big lines: players open **a URL**; **the server holds the truth** and sends each browser only what its character perceives; browsers talk to it over **WebSockets**; and the engine, being pure Rust, **compiles to WebAssembly** as well as to native code. What's open is how the browser side is built.

## The layers

A browser client has four layers, and they can be chosen separately:

| Layer | What it does | Choice now? |
| --- | --- | --- |
| 1. The protocol | Messages between server and browser: commands in, views out | Yes: it's the one thing every client shares |
| 2. The engine in the browser | A copy of the laws running locally, in WebAssembly | Later |
| 3. The interface | Panels, inventory, map, command box, skill buttons, chat | Yes, simply |
| 4. The renderer | Drawing the world, first as a map, then in 3D | At the rendering conversation |

### 1. The protocol: the contract

We already have it: `console --live` answers each command with one line of JSON holding what the person perceives. The browser page reads the same thing over a WebSocket. **This is the part to get right**: every client (console, tests, Claude, the page, a future 3D client) speaks it, so any client can be thrown away and rebuilt without touching the engine.

- **JSON** now: readable, easy to debug, and fast enough for text and panels.
- **A compact binary form** (MessagePack, or Rust's own) later, if 3D and many players need it. Swapping the encoding doesn't change what's said.

### 2. The engine in the browser (WebAssembly)

| | Pros | Cons |
| --- | --- | --- |
| **Thin client** (the browser only shows what the server says) | Simplest; nothing to cheat with; one copy of the laws | Every action waits for the server; no play offline |
| **The engine in the browser too** | Instant response (the browser predicts, the server confirms), a workbench for testing inventions and programs, offline single player, free hosting of single-player worlds | Bigger download; two copies must agree exactly, which our determinism makes possible but has to be tested |

**For now: a thin client.** The engine in the browser comes with 3D movement (for prediction) and the in-game computer (the workbench). Our pure, deterministic engine is built for it, so it's a later step, not a redesign.

### 3. The interface: panels and buttons

| Option | Pros | Cons |
| --- | --- | --- |
| **Plain TypeScript** (no framework) | Smallest and simplest; nothing to learn or upgrade; fine for a handful of panels | Gets tangled as the interface grows: many panels updating at once |
| **React** | The biggest ecosystem; the most examples and help; works with three.js through react-three-fiber | Heavier; more machinery than a few panels need |
| **Svelte or Solid** | Light and fast; little code for live-updating panels | Smaller ecosystems than React |
| **A Rust interface** (Leptos, Dioxus, Yew) | One language with the engine; the protocol's types shared exactly | Slower to change; bigger downloads; fewer examples; weaker ties to 3D libraries |
| **egui** (Rust, drawn in a canvas) | Superb for debug tools and inspectors | Looks like a tool, not a game; poor for text-heavy or accessible pages |

TypeScript itself is worth having in every case: it catches mistakes before the page runs, as Rust's compiler does for the engine.

### 4. The renderer, for later

The fork that matters, for the rendering conversation:

| Option | Pros | Cons |
| --- | --- | --- |
| **three.js** | The most widely used web 3D library; stable; huge ecosystem | A library, not a game engine: physics, animation, and tools are ours to add |
| **Babylon.js** | A full game engine in TypeScript: physics, animation, an inspector | Heavier than three.js; smaller community |
| **Bevy** (Rust) | One language from engine to screen; could run the engine in the same program | Before its 1.0, with breaking changes every few months; large downloads; the web is a secondary target |
| **Godot or Unity, exported to the web** | Full editors, artists' tools | Heavy downloads; the web export is a second-class target; our engine would sit beside theirs, not inside |

Whatever the renderer, **the interface can stay HTML laid over the canvas**, which is how most browser games do it: menus, inventory, and chat as page elements over the drawn world. So choosing the interface now doesn't close any renderer door.

**Drawing technology:** WebGL2 works in every browser; WebGPU, faster and more modern, is arriving across browsers. The libraries above handle both.

## Serving and hosting

- **Locally:** a small Rust web server (axum) serves the page and bridges its WebSocket to the same session the console uses. One program, started with one command.
- **On AWS, later:** the page is static files (S3 behind CloudFront); the universe server runs on EC2 and holds the WebSockets, as [technology.md](../technology.md) plans.
- **Tests:** Playwright opens the page, types commands, and takes screenshots, so Claude can check the page and show the owner.

## Recommendation

- **Now (the first page):** TypeScript with no framework, built with Vite, served by a small Rust server that bridges to the same live channel. A thin client. Plain panels: the time, where you are, who's here and what they're doing, what you carry, the places you know, a command box. Playwright screenshots.
- **When the interface grows** (the camp, the hamlet): adopt a framework then, most likely React for its ecosystem and its path to three.js, or Svelte if we want it light. Plain TypeScript makes that switch cheap.
- **The renderer** at the rendering conversation, with the fork above: a TypeScript renderer (three.js or Babylon.js) beside our Rust engine, or Rust all the way (Bevy). Claude's lean today is TypeScript and three.js, for stability and help available, but it's that conversation's to decide.
- **The engine in the browser** when 3D movement needs prediction, or the in-game computer needs its workbench.

## Performance first: Claude's take, 2026-09-30

The owner's words are in "Performance first" in [requirements.md](../requirements.md): performance in 3D matters most; also a good, reliable physics engine (rockets to orbit, cars, space battles), sound, and fast, robust networking; updates by refreshing the page. The evidence is in [research/webgpu-performance.md](../research/webgpu-performance.md).

**What decides speed in a browser 3D game** is how many separate calls each frame makes to the graphics card, and how much work the CPU does per object. It's not the language. Every call to WebGPU goes through the browser's JavaScript interface, even from Rust; the GPU's work costs the same either way; and the cure, in any language, is to draw many things with few calls. Where WebAssembly clearly wins is heavy simulation, which is our engine's job, not the renderer's.

**Recommendation: a TypeScript renderer, with our Rust engine in the browser as WebAssembly** (option B), passing **one block of numbers a frame** between them: every position at once, read straight from the engine's memory, never one call per object.

- **Why not all Rust (Bevy) now:** no speed advantage for drawing; Bevy's WebGPU build for the web is still experimental, it changes with every release, and Rust on the web runs on a single thread in practice. The JavaScript renderers (three.js, Babylon.js) are mature, with WebGPU renderers still catching up. Because the protocol is the contract, an all-Rust client stays possible later.
- **Which renderer** (three.js or Babylon.js): chosen at the rendering conversation **by measuring our own scenes**, not by reputation. Both have WebGPU renderers that still lag their WebGL ones on scenes with many objects.

**Physics: our laws decide, a physics engine only makes it feel right.**

- **Orbits belong in our engine's laws.** Every space game found computes orbits with its own mathematics (Kerbal Space Program's patched conics), not a physics engine. Our whole numbers have the same precision everywhere, so a rocket at the far side of a solar system is as exact as one on the pad: the precision bugs that plagued Kerbal's rockets can't happen in the simulation.
- **Our engine already has what physics engines work hardest for:** the same result on every machine, bit for bit. Jolt, Rapier, and Box3D now promise that too, but only under strict conditions.
- **Collisions, cars, ships:** start with our own simple contact and motion laws in whole numbers (don't overcomplicate). A physics engine (Rapier or Jolt, both actively developed and fast in the browser) may run **in the browser only**, for feel, prediction, and debris that doesn't matter: the way Rocket League's clients predict and its server corrects. If one ever has to decide outcomes, it runs on the server and hands its results to the gate as whole numbers, like any law.

**Sound:** the browser's own audio system, through the renderer's library. It costs the same from any language; spatial sound (HRTF) only for a few nearby sounds.

**Networking:** WebSockets now; **WebTransport** for fast 3D (unreliable datagrams for positions that supersede themselves, reliable streams for the rest), which now works in every major browser. **Bandwidth matters more than the transport:** quantised whole numbers, sending only changes, and sending only what each character perceives, which our design already requires. Shipped games send 30 to 128 updates a second.

**One engine change fast play will need:** our clock counts whole seconds, and fast games step 30 to 120 times a second. Near players, vehicles and fights will need finer steps; the cone of influence decides where ([cone-of-influence.md](cone-of-influence.md)).

**Updates by refresh:** every release gets new file names, so browsers cache each version and fetch only what changed; the compiled engine is cached after the first load.

**For the first page, nothing changes:** TypeScript, no framework, a small Rust server bridging the live channel, JSON over WebSockets. It's already option B's shape.

## Questions for the owner

1. **Option B** (TypeScript renderer, our Rust engine in the browser as WebAssembly, one block of numbers a frame), with the renderer chosen later by measuring our scenes: agree?
2. **Our laws decide motion**, orbits included; a physics engine only in the browser for feel, prediction, and debris: agree?
3. **WebSockets now, WebTransport for fast 3D:** agree?
4. **The first page as proposed** (TypeScript, no framework, a thin client, the protocol as the contract): go ahead?
