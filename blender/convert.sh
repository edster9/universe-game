#!/usr/bin/env bash
# Converts a world built in Blender for the game: exports each Blender file
# its manifest lists (Windows' Blender, from WSL, without its window), then
# runs the converter, which writes the world's data file.
#   blender/convert.sh skill-yard
# Elsewhere, run export_world.py in Blender for each file, then
# `cargo run -p console -- --convert blender/<world>.world.toml`.
set -euo pipefail
cd "$(dirname "$0")"
world="$1"
manifest="$world.world.toml"
B="${BLENDER:-/mnt/c/Program Files/Blender Foundation/Blender 5.0/blender.exe}"
exported=$(python3 -c "import tomllib; print(tomllib.load(open('$manifest','rb'))['exported'])")
for file in $(python3 -c "import tomllib; print(' '.join(tomllib.load(open('$manifest','rb'))['files']))"); do
    mkdir -p "$exported/$file"
    "$B" --background --factory-startup --python "$(wslpath -w "$PWD/export_world.py")" -- \
        "$(wslpath -w "$PWD/$file.blend")" "$(wslpath -w "$PWD/$exported/$file/$file.gltf")" 2>&1 |
        grep -E "^EXPORTED|Error" || true
done
cd ..
PATH=~/.cargo/bin:$PATH cargo run -q -p console -- --convert "blender/$manifest"
