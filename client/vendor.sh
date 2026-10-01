#!/usr/bin/env bash
# Makes the patched copy of whisper-rs-sys the client builds with (Whisper,
# for voice), in client/vendor/, from cargo's own download. One change:
# whisper.cpp drops the "lib" prefix from its libraries on every Windows
# build, but only Microsoft's compiler wants that; building for windows-gnu
# from WSL, Rust looks for libggml.a. See docs/challenges/first-steps.md,
# stage 5.
set -euo pipefail
cd "$(dirname "$0")"
VERSION=0.15.0
DEST=vendor/whisper-rs-sys
[ -f "$DEST/.patched" ] && exit 0
SRC=$(ls -d ~/.version-fox/temp/*/rust/cargo/registry/src/index.crates.io-*/whisper-rs-sys-$VERSION ~/.cargo/registry/src/index.crates.io-*/whisper-rs-sys-$VERSION 2>/dev/null | head -1 || true)
if [ -z "$SRC" ]; then
    # Not downloaded yet: fetch it with a throwaway project.
    TMP=$(mktemp -d)
    (cd "$TMP" && cargo init -q --name fetch && cargo add -q whisper-rs-sys@=$VERSION && cargo fetch -q)
    rm -rf "$TMP"
    SRC=$(ls -d ~/.version-fox/temp/*/rust/cargo/registry/src/index.crates.io-*/whisper-rs-sys-$VERSION ~/.cargo/registry/src/index.crates.io-*/whisper-rs-sys-$VERSION 2>/dev/null | head -1)
fi
rm -rf "$DEST"
mkdir -p vendor
cp -r "$SRC" "$DEST"
python3 - "$DEST/whisper.cpp/ggml/CMakeLists.txt" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read()
old = "# remove the lib prefix on win32 mingw\nif (WIN32)\n"
assert old in s, "whisper.cpp changed: check the patch"
s = s.replace(old, "# remove the lib prefix on win32 mingw\n# (universe-game: only for MSVC)\nif (MSVC)\n")
open(p, "w").write(s)
PY
touch "$DEST/.patched"
