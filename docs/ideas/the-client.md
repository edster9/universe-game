# The client: web or native

Claude's report and take, 2026-09-30. **A native client is decided** (the owner, the same day: "definitely sold on a thick app"); the engine choice and the first window are still open. See "Building for Windows from WSL" below. The owner asked Claude to marry its own research ([research/webgpu-performance.md](../research/webgpu-performance.md)) with two reports from another AI (one on the web path, one on a thick client; summarised in "A thick client, not only the web" in [requirements.md](../requirements.md)), give a new report, and an honest answer on the approach. The other AI's load-bearing claims were checked against primary sources: [research/web-or-native-fact-check.md](../research/web-or-native-fact-check.md). This replaces the recommendation in [web-stack.md](web-stack.md).

## The short answer

**Build the game as a native, thick client: Windows first, then macOS, written in Rust, linking our engine directly.** Keep the web for what it's good at: the website, the launcher's download page, and tools for watching and running the world. The browser tab isn't the place for a GTA-scale world with rockets and space battles; it's a place with ceilings, and this game would hit every one.

Nothing we've built changes: the engine and the server are the same, and the protocol is still the contract. Only the client's home changes, and it becomes simpler: Rust from the laws to the screen, with no WebAssembly in between.

## What doesn't change

- **The engine is authoritative, deterministic, and pure**, and runs on the server. The client never decides what happens; it shows what the character perceives, predicts the player's own movements, and draws.
- **The protocol is the contract.** Console, tests, Claude's live channel, and any client speak it. A web client, a native client, or both can exist without touching the engine.
- **Our laws decide motion**, orbits included; a physics engine, if any, only gives the client feel ([web-stack.md](web-stack.md), "Performance first").

So "web or native" is a client decision, not an engine rewrite, and it's reversible at the cost of rewriting a client, not the world.

## Where all three reports agree

- **For this genre, the problem is streaming, level of detail, and memory**, not which engine draws.
- **Heavy simulation belongs in Rust**, not JavaScript.
- **A world of several scales** (street, planet, space) needs separate simulation "islands" and a floating origin for rendering. Our whole-number engine removes the precision problem from the *simulation* (a rocket at the far side of a system is as exact as one on the pad); the *renderer* still needs a floating origin, because graphics cards work in 32-bit floats.
- **Orbits are computed by purpose-built code**, not a general physics engine (Kerbal Space Program's patched conics).
- **Not Electron.** It carries the browser's own ceilings (the 4 GB JavaScript heap, confirmed; the same WebGPU) plus 150–250 MB of idle browser.

## Where they differ

My earlier recommendation and the other AI's web report both said: start in the browser with a TypeScript renderer and Rust in WebAssembly. That's right if the game has to live at a URL. The other AI's thick-client report says: if it doesn't, go native. **The owner has said it doesn't have to.** Once the URL isn't a requirement, the thick-client case is stronger, on the facts.

## The browser's ceilings, checked

| Ceiling | In a browser (checked) | Native |
| --- | --- | --- |
| Memory for the world | WebAssembly: 4 GB. 64-bit WebAssembly costs 10–100%+ in speed, is capped at 16 GB, and isn't in Safari. JavaScript heap: 4 GB. | The machine's RAM (16–64 GB) and all of the GPU's memory |
| Threads | Rust on the web runs single-threaded in practice; threads need special headers that restrict the page | Every core: physics, streaming, animation, and the renderer in parallel |
| GPU features | WebGPU's portable subset: 256 MiB buffers by default, more on request up to tiered limits; no ray tracing, no mesh shaders, no DLSS | Vulkan, DirectX 12, Metal through wgpu: bindless resources, larger buffers; ray tracing and DLSS natively (Bevy has both, ray tracing still experimental) |
| Loading a world | Downloaded over HTTP into storage the browser may evict (whole site at once; Safari after 7 days idle); no memory-mapping | Tens of GB on the player's disk, memory-mapped, streamed on worker threads |
| Reach | 82–89% of users get WebGPU (depending on how it's counted); Linux ~17% | Every Windows and Mac machine with a modern GPU |
| Sound | ~55–67 ms of delay by default, ~14–19 ms at best (one 2021 measurement) | The audio device directly, a few milliseconds |
| Networking | WebSockets or WebTransport (now in every major browser) | UDP or QUIC, any protocol we like; what large multiplayer games use |
| Updates | Refresh the page | Steam, or a small launcher that downloads only what changed |

The web's one true advantage is the first row nobody lists: **reach without installing**. The owner has said they'll give that up.

## What a native client buys this game, specifically

- **Rockets to orbit, cars, space battles:** real threads for physics near the player (a ship battle's projectiles, a city's traffic), with the engine's laws still deciding outcomes on the server.
- **A world bigger than a tab:** a city district, its interiors, and a planet's terrain streamed from disk, not squeezed under 4 GB.
- **Good visuals:** the full GPU, and native ray tracing and upscaling when we want them.
- **Our engine, linked directly.** The client can run a copy of the laws for prediction with no WebAssembly boundary and no copying: one language, one build, the same code the server runs. This is the "Rust for the client too" the owner hoped for, in its best form.
- **Fast networking:** QUIC or UDP, our own protocol, no browser in the way.

## What it costs

- **No "open a link and play".** A download and an install. For a game of this size, that's how nearly every large PC game works; the website and a small launcher do the rest.
- **Code signing:** Windows installers need a signing certificate to avoid SmartScreen warnings; macOS apps need Apple's notarisation (a developer account). Both are routine, and cost a little money each year.
- **Builds for each platform:** Windows first; macOS later (wgpu covers Metal, so it's mostly packaging and testing); Linux if wanted; no phones.
- **Distribution:** Steam (it handles updates and discovery, and takes a 30% share) or our own launcher with delta patches.
- **Claude's checks without a browser:** Playwright can't screenshot a native window, but the client can **render frames to image files** without a screen, in tests, which is sturdier than screenshots anyway.
- **This machine:** development runs in WSL on Windows. The Windows client can be built from here (cross-compiled) or on the Windows side; Linux builds can also open windows through WSL's own display. A detail to settle when building, not a blocker.

## Which native engine

Neither report weighed this fork fully:

| Option | For | Against |
| --- | --- | --- |
| **Bevy** (Rust) | Rust from laws to screen; links our engine directly; wgpu on Vulkan, DirectX 12, Metal; an entity system suited to thousands of things in a district; native ray tracing and DLSS arriving; open source | Before 1.0: breaking changes every release (confirmed), so versions must be pinned and upgraded on purpose; thin tools, no mature editor; open-world systems (streaming, traffic) are ours to build |
| **wgpu directly**, no engine | Total control | Writing a renderer is a multi-year project of its own |
| **Godot with Rust** (gdext) | A full editor; Jolt physics by default; open source | Rust is a guest in a C++ engine (usable, "occasional breaking changes"); visuals good, not top-tier |
| **Unreal** | The best visuals and open-world tools available (world streaming, vehicles, Nanite, Lumen) | C++, heavy, a royalty on revenue; our engine would be a library bolted into someone else's world; the most machinery between our laws and the screen |

**My lean: Bevy**, for three reasons that are specific to us. Our worlds come from data files and laws, not hand-placed levels, so an editor matters less than for most games. Sharing the engine's code directly with the client is a real advantage for prediction and for keeping "one set of laws". And Rust's compiler catches most of what a Bevy upgrade breaks, so the churn is a cost we pay on purpose, a release at a time. If the goal later becomes photoreal visuals with artists building content, Unreal is the honest alternative, and the protocol keeps that door open.

**Settle it by measuring, as the other AI suggests**, but at our scale: our own island, drawn natively in Bevy from the engine's views, before the city-scale bake-off matters.

## What changes now

**The companion challenge's last stage becomes a native window instead of a web page.** A small Rust app that opens the companion's island: the time, where you are, who's here and what they're doing, what you carry, the places you know, a map, and a command line, reading the same protocol. Panels in egui (which carries straight into Bevy later), and a mode that renders frames to images so Claude can check them. It's the real client's first step: a window, input, the protocol, and panels.

**The web keeps two jobs:** the website (and later the launcher's download page), and tools for watching and running the world on AWS, where a browser dashboard is the natural fit. Neither needs 3D.

**Networking** moves to QUIC (quinn, active and widely used, though still before 1.0) when fast play arrives; the text protocol stays for tools.

## Honest caveats

- **The web isn't slower at drawing.** Our own research still stands: for the GPU's work, the language makes no difference. The case for native is the ceilings (memory, threads, storage, GPU features, audio, networking), not raw speed.
- **Native costs reach.** Some players will never install anything. The owner has accepted that; it's the one thing the web did better.
- **Bevy's churn is real**, and so is building open-world systems ourselves. That's the price of Rust all the way.
- **None of this is urgent.** The next year of the climb is text, a companion, a camp, a hamlet. The decision matters now only because it decides what the first window is.

## Building for Windows from WSL

Claude's take, 2026-09-30, on the owner's experience ("Sold on a thick client, and building for Windows from WSL2" in [requirements.md](../requirements.md)): a WSL2 window reaches the GPU through a translation layer, and a real Windows build drew about ten times faster.

**What matters is where the client *runs*, not where it's built.** The ten-times loss came from running inside WSL's display layer. A program built anywhere, but run as a genuine Windows process, gets the RTX 4050 at full speed.

**Recommendation: build in WSL, run on Windows.**

- **Everything stays in WSL:** the engine, the server, and all the tests are Linux, as the AWS servers will be. One copy of the source, on WSL's fast disk; one set of scripts.
- **The client is cross-compiled for Windows** (`cargo build --target x86_64-pc-windows-gnu`, the same MinGW flavour of Windows program MSYS2 makes). Unlike C++ with its libraries, cross-compiling is a first-class part of Rust, and our client's dependencies are almost all Rust.
- **A script copies the program and its data to a folder on the C: drive and starts it** through WSL's interop, which launches Windows programs directly: a real Windows process on the real GPU. (Checked: interop is on, the C: drive is reachable, and this is Windows 11 with an RTX 4050.)
- **Claude's checks still work:** the client's mode that renders frames to image files runs on Windows too, and writes them to the C: drive, where WSL reads them.
- **What it needs:** a Windows linker in WSL, one command the owner runs once, since it needs their password: `sudo apt install mingw-w64`.
- **Why not build in MSYS2:** it's the right tool for C++, and it's installed (`C:\msys64`, with MinGW's compilers), but building there means either reading the source across the WSL boundary (slow) or moving the repository to the C: drive (which slows everything WSL does), and two environments to keep in step. **MSYS2 stays the fallback**: if some library misbehaves when cross-compiled, the same Windows target builds there, with no code changes.
- **Later:** the Microsoft flavour of Windows program (MSVC) may be needed for some vendor libraries (NVIDIA's DLSS SDK, for example); it can also be cross-compiled from WSL (cargo-xwin) when that day comes. macOS builds need a Mac or a cloud Mac (GitHub Actions has them), for Apple's signing anyway.

**First, prove it:** a tiny Rust program that opens a window and draws a heavy scene through wgpu, built in WSL, run on Windows, and the same program run inside WSL, with frame rates side by side. If the route works and shows the expected gap, it's settled; if cross-compiling fights us, we move to MSYS2 that afternoon.

## Proved, 2026-09-30: building for Windows from WSL

The owner asked to be convinced first: a benchmark run inside WSL, and the same built for Windows. `prototypes/gpu-bench` draws as many triangles as it can, and, in a second test, as many separate small objects (draw calls) as it can, the way a game's frame is made. Same source, same RTX 4050 laptop GPU, 1280×720, no waiting for the screen.

| Test | Inside WSL (OpenGL over the DirectX 12 bridge) | A real Windows program (Vulkan) | Windows gain |
| --- | --- | --- | --- |
| 0.5 M triangles a frame | 220 frames a second | 1,393 | 6.3× |
| 4 M triangles a frame | 337 | 540 | 1.6× |
| 64 M triangles a frame | 46 | 52 | 1.1× |
| Best triangles a second | 2.9 billion | 3.3 billion | 1.1× |
| 10,000 small objects a frame | never drew a frame | 1,332 frames a second | |
| Best draw calls a second | never finished | 13.3 million | |

**What it shows:**

- **The route works, with nothing installed on Windows:** the client is built in WSL (Rust's Windows target, with Zig as the linker, installed in the user's own folder: no password, no MSYS2), copied to the C: drive, and started from WSL, where it runs as a genuine Windows process on the GPU.
- **WSL's cost is per frame and per call, not the GPU's raw power.** When the GPU is just busy with a mountain of triangles, WSL is nearly as fast. But WSL adds about 4 ms to every frame (more than half the time a frame has at 144 a second), and the test that draws the way games draw, thousands of separate objects, never completed inside WSL at all, while the Windows build drew 10,000 objects a frame over 1,300 times a second. That's the owner's "tenfold as the game scaled", and then some.
- **By default, this WSL doesn't use the GPU at all:** its OpenGL and Vulkan fall back to drawing on the processor (llvmpipe), and reach the GPU only when asked for the DirectX 12 bridge (`GALLIUM_DRIVER=d3d12`). A likely reason the owner's earlier project was so slow inside WSL.
- **WSL's display is fragile under load:** stalled runs left frozen windows, cleared by ending `msrdc.exe` (the program that draws every WSL window on Windows) in Task Manager.

**Settled:** the client is always run as a Windows program, built from WSL. MSYS2 isn't needed; it stays a fallback.

## Built, 2026-09-30: the first scene

The owner: go ahead with Bevy and a basic scene render, built as a native Windows program only from now on ("The first scene, and assets for later" in [requirements.md](../requirements.md)).

`client/` is the game's native client (Bevy 0.19, its own newer Rust, outside the engine's workspace so the proofs never compile it). It runs the engine itself, the same code the tests use, on the companion's island, with the clock running (60 times real speed by default), and draws what the world holds:

- **The land is shaped from the engine's own data:** every place's position and height, and the paths between them. Land reaches around each place and along each path over ground, further for higher places (a mountain's base is wider than a hill's), heights blend between places, and the coast slopes into the sea. Sand, grass, rock, and snow by height. The result is a real island: the snow-capped mountain, the long slope down to the home corner (beach, stream, forest, hillside), and the islet out to sea. Nothing is invented but the shape between places.
- **Things are drawn from a style file** (`client/style.toml`, the client's own data): a colour and a simple form for each material and kind of creature (rocks, logs, trees, bushes, grass, pools, heaps, figures), until the game has real assets ([assets.md](assets.md)). Each fixed source is a cluster of pieces, more for more mass.
- **People and creatures move:** each is drawn at their place, or, while walking, between where they set off and where they're going, over the time the walk takes. The engine gained `laws::journey`, which tells any client where a walker is heading and when they set off and arrive; the client never re-derives the laws.
- **Day and night** follow the world's clock: the sun's arc between sunrise and sunset, and the sky's colour.
- **Controls:** WASD to fly, E/Q up and down, Shift faster, hold the right mouse button to look; Space pauses the world, [ and ] slow it and speed it up.
- **Checking without watching:** `--shot file.png --after 5 --from x,y,z --look x,y,z` saves a frame and exits. `client/run.sh` builds it for Windows from WSL, copies it to `C:\Users\edste\universe-game\client`, and starts it.

**Simplified, and next:** an overview of the whole world, not yet one character's view (the engine sends only what a character perceives, and the player's view will follow that); no labels; figures are plain capsules; the program is 108 MB until it's stripped. The rendering conversation and the assets conversation come next.

## Questions for the owner

1. ~~A native client~~ (decided).
2. **Rust and Bevy** as the lean, settled by drawing our own island natively first; Unreal as the alternative if photoreal art ever drives the game: agree?
3. **The first window**, the companion challenge's last stage, as a small native app (egui panels, frames rendered to images for checking) instead of a web page: go ahead?
