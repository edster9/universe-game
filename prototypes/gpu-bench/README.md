# gpu-bench

How many triangles, and how many draw calls, a second this machine draws: the same program run inside WSL and as a real Windows program, built from WSL. It settled how the game client is built and run. See "Proved" in [docs/ideas/the-client.md](../../docs/ideas/the-client.md).

- `gpu-bench`: one big draw of 0.5 to 64 million small triangles a frame, without waiting for the screen.
- `gpu-bench calls`: many small objects a frame (50 triangles each), each its own draw call, as a game draws.

Each writes a report next to itself (`gpu-bench-<os>[-calls].txt`). Results from 2026-09-30, on an RTX 4050 laptop GPU, are in `results/`.

## Building and running

Inside WSL (it reaches the GPU only through the DirectX 12 bridge, and only through OpenGL):

```sh
cargo build --release
env -u WAYLAND_DISPLAY WGPU_BACKEND=gl GALLIUM_DRIVER=d3d12 ./target/release/gpu-bench
```

As a Windows program, built in WSL with Zig as the linker (no Windows compilers needed), and run from the C: drive as a real Windows process:

```sh
export PATH=~/.cargo/bin:~/.local/zig/zig-x86_64-linux-0.16.0:$PATH:<cargo-zigbuild's bin>
cargo zigbuild --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/gpu-bench.exe /mnt/c/Users/edste/universe-game/gpu-bench/
cd /mnt/c/Users/edste/universe-game/gpu-bench && ./gpu-bench.exe
```

Set `GPU_BENCH_TRACE=1` to print the time of the first frames of each round.
