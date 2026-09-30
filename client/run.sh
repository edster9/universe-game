#!/usr/bin/env bash
# Builds the client as a Windows program, from WSL, copies it to the C: drive,
# and starts it there: a real Windows process on the GPU. Any arguments go to
# the client, e.g. `client/run.sh --shot shot.png --after 5`.
set -euo pipefail
cd "$(dirname "$0")"
ZIGBUILD_BIN=$(dirname "$(ls ~/.version-fox/temp/*/rust/cargo/bin/cargo-zigbuild 2>/dev/null | head -1)" 2>/dev/null || true)
export PATH=~/.cargo/bin:~/.local/zig/zig-x86_64-linux-0.16.0:$PATH:${ZIGBUILD_BIN:-}:~/.cargo/bin
cargo zigbuild --release --target x86_64-pc-windows-gnu
TARGET=/mnt/c/Users/edste/universe-game/client
mkdir -p "$TARGET"
cp target/x86_64-pc-windows-gnu/release/client.exe "$TARGET/"
cd "$TARGET"
exec ./client.exe "$@"
