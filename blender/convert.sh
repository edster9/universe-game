#!/usr/bin/env bash
# Converts a world built in Blender for the game: exports each Blender file
# its manifest (worlds/<world>/<world>.world.toml) lists, with Windows'
# Blender from WSL, without its window, then runs the converter, which
# writes the world's data file.
#   blender/convert.sh skill-yard
# Elsewhere, run export_world.py in Blender for each file, then
# `cargo run -p console -- --convert worlds/<world>/<world>.world.toml`.
set -euo pipefail
cd "$(dirname "$0")/.."
world="$1"
folder="worlds/$world"
manifest="$folder/$world.world.toml"
[ -f "$manifest" ] || { echo "There's no $manifest: fetch the world first (blender/worlds.sh pull $world)."; exit 1; }
B="${BLENDER:-/mnt/c/Program Files/Blender Foundation/Blender 5.0/blender.exe}"
read_manifest() { python3 -c "import tomllib, sys; m = tomllib.load(open('$manifest', 'rb')); print($1)"; }
exported="$folder/$(read_manifest "m['exported']")"
for file in $(read_manifest "' '.join(m['files'])"); do
    mkdir -p "$exported/$file"
    "$B" --background --factory-startup --python "$(wslpath -w "$PWD/blender/export_world.py")" -- \
        "$(wslpath -w "$PWD/$folder/$file.blend")" "$(wslpath -w "$PWD/$exported/$file/$file.gltf")" 2>&1 |
        grep -E "^EXPORTED|Error" || true
done
PATH=~/.cargo/bin:$PATH cargo run -q -p console -- --convert "$manifest"
