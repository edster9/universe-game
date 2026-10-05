#!/usr/bin/env bash
# Runs one of these scripts in Windows' Blender, from WSL, without its window:
#   blender/blender.sh make_catalogue.py
#   blender/blender.sh make_skill_yard.py
# Elsewhere, run Blender directly: blender --background --factory-startup
# --python blender/<script>. BLENDER names another blender.exe.
set -euo pipefail
cd "$(dirname "$0")"
# The newest Blender installed on Windows, unless BLENDER names one.
B="${BLENDER:-$(ls -d "/mnt/c/Program Files/Blender Foundation/Blender "*/blender.exe 2>/dev/null | sort -V | tail -1)}"
[ -x "$B" ] || { echo "Can't find Blender: install it, or set BLENDER to its blender.exe."; exit 1; }
script="$1"
shift
"$B" --background --factory-startup --python "$(wslpath -w "$PWD/$script")" -- "$@" 2>&1 |
    grep -E "^(CATALOGUE|MISSING|WORLD|PREVIEW|LEVEL|RELINKED|Error|Traceback|  File|    )|Error:" || true
