#!/usr/bin/env bash
# Runs one of these scripts in Windows' Blender, from WSL, without its window:
#   blender/blender.sh make_catalogue.py
#   blender/blender.sh make_skill_yard.py
# Elsewhere, run Blender directly: blender --background --factory-startup
# --python blender/<script>. BLENDER names another blender.exe.
set -euo pipefail
cd "$(dirname "$0")"
B="${BLENDER:-/mnt/c/Program Files/Blender Foundation/Blender 5.0/blender.exe}"
script="$1"
shift
"$B" --background --factory-startup --python "$(wslpath -w "$PWD/$script")" -- "$@" 2>&1 |
    grep -E "^(CATALOGUE|MISSING|WORLD|PREVIEW|RELINKED|Error|Traceback|  File|    )|Error:" || true
