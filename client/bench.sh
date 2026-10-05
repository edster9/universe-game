#!/usr/bin/env bash
# The rendering benchmarks (docs/research/rendering-benchmarks.md): makes
# any forest world that's missing (Blender, from WSL), converts it, installs
# the client, and runs the benchmark route in each forest at each quality,
# maximized. Results go to bench.csv beside the client, and are shown at the
# end.
#   client/bench.sh                     small, medium and large, every preset
#   client/bench.sh small medium        just those forests
#   QUALITIES="high" client/bench.sh    just those presets
#   EXTRA="/shadows off" client/bench.sh    another setting on top of each
set -euo pipefail
cd "$(dirname "$0")/.."
# Each forest: trees, and how far it reaches from the middle (m).
declare -A TREES=([small]=200 [medium]=2000 [large]=20000 [huge]=100000)
declare -A REACH=([small]=50 [medium]=150 [large]=500 [huge]=1100)
sizes=("$@")
[ ${#sizes[@]} -gt 0 ] || sizes=(small medium large)
QUALITIES=${QUALITIES:-low medium high ultra}
for size in "${sizes[@]}"; do
    world="forest-$size"
    if [ ! -f "assets/worlds/$world/$world.gltf" ] || [ ! -f "data/$world.toml" ]; then
        blender/blender.sh make_forest.py "$world" "${TREES[$size]}" "${REACH[$size]}"
        blender/convert.sh "$world"
    fi
done
client/run.sh --install | tail -1
. client/where.sh
cd "$TARGET"
from=$(wc -l < bench.csv 2>/dev/null || echo 0)
for size in "${sizes[@]}"; do
    for quality in $QUALITIES; do
        label="$quality${EXTRA:+ + $EXTRA}"
        echo "forest-$size, $label"
        timeout 600 ./client.exe --world "forest-$size" --bench "${REACH[$size]}" \
            --bench-label "$label" --maximized \
            --type "/quality $quality${EXTRA:+; $EXTRA}" > /dev/null 2>&1 ||
            echo "  ended with $? (see client.log)"
    done
done
echo
head -1 bench.csv | cut -d, -f2-
tail -n +$((from + 1)) bench.csv | grep -v '^when' | cut -d, -f2-
