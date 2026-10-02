#!/usr/bin/env bash
# Builds the client as a Windows program, from WSL, copies it to the C: drive,
# and starts it there: a real Windows process on the GPU. Any arguments go to
# the client, e.g. `client/run.sh --shot shot.png --after 5`, or
# `client/run.sh --script companion-3-asking.txt` to play a script.
set -euo pipefail
cd "$(dirname "$0")"
ZIGBUILD_BIN=$(dirname "$(ls ~/.version-fox/temp/*/rust/cargo/bin/cargo-zigbuild 2>/dev/null | head -1)" 2>/dev/null || true)
export PATH=~/.cargo/bin:~/.local/zig/zig-x86_64-linux-0.16.0:$PATH:${ZIGBUILD_BIN:-}:~/.cargo/bin
# Whisper, for voice: a patched copy (see vendor.sh), bindings made with
# libclang, and built for CPUs with AVX2 (2015 on), which makes it ten times
# faster.
./vendor.sh
export LIBCLANG_PATH=${LIBCLANG_PATH:-/usr/lib/llvm-18/lib}
export CFLAGS_x86_64_pc_windows_gnu="-march=x86-64-v3" CXXFLAGS_x86_64_pc_windows_gnu="-march=x86-64-v3"
cargo zigbuild --release --target x86_64-pc-windows-gnu
TARGET=/mnt/c/Users/edste/universe-game/client
mkdir -p "$TARGET"
# A running game can't be overwritten, but it can be renamed: set it aside.
if ! cp target/x86_64-pc-windows-gnu/release/client.exe "$TARGET/" 2>/dev/null; then
    rm -f "$TARGET/client-old.exe" 2>/dev/null || true
    mv "$TARGET/client.exe" "$TARGET/client-old.exe"
    cp target/x86_64-pc-windows-gnu/release/client.exe "$TARGET/"
fi
# Explorer shows when a file was created, which overwriting keeps (Windows
# even keeps it for a file deleted and made again under the same name), so
# set it to now: the date shown is the build's.
powershell.exe -NoProfile -Command \
    "(Get-Item 'C:\\Users\\edste\\universe-game\\client\\client.exe').CreationTime = Get-Date" \
    >/dev/null 2>&1 || true
# Whisper's English model, beside the program, once.
mkdir -p "$TARGET/models"
[ -f "$TARGET/models/ggml-base.en.bin" ] || curl -sSL -o "$TARGET/models/ggml-base.en.bin" \
    https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin
# DirectX 12's fast shader compiler, beside the program, once: without it,
# the old one takes seconds before anything is drawn.
DXC=v1.9.2609/dxc_2026_09_29.zip
if [ ! -f "$TARGET/dxcompiler.dll" ]; then
    tmp=$(mktemp -d)
    curl -sSL -o "$tmp/dxc.zip" "https://github.com/microsoft/DirectXShaderCompiler/releases/download/$DXC"
    unzip -q -o "$tmp/dxc.zip" -d "$tmp" 2>/dev/null || true
    cp "$(find "$tmp" -ipath '*x64*dxcompiler.dll' | head -1)" "$(find "$tmp" -ipath '*x64*dxil.dll' | head -1)" "$TARGET/"
    rm -rf "$tmp"
fi
# Models and other assets, beside the program, where the renderer looks for
# them: only what's changed is copied.
mkdir -p "$TARGET/assets"
cp -ru ../assets/. "$TARGET/assets/"
# The worlds and scripts, beside the program.
rm -rf "$TARGET/data"
cp -r ../data "$TARGET/data"
cd "$TARGET"
exec ./client.exe "$@"
