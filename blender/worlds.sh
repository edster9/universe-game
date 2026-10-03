#!/usr/bin/env bash
# The source worlds (worlds/<world>/: Blender files and their manifest) live
# in S3, not git (the owner, 2026-10-02: temporary while we train the
# system). The bucket is public to read, so anyone can fetch them; sending
# needs the project's AWS profile. S3 keeps every version sent.
#   blender/worlds.sh list               the worlds in the bucket
#   blender/worlds.sh pull <world>       fetch a world into worlds/<world>
#   blender/worlds.sh push <world>       send what's changed in worlds/<world>
#   blender/worlds.sh versions <world>   every version of its files kept
set -euo pipefail
cd "$(dirname "$0")/.."
BUCKET="s3://universe-game-worlds"
PROFILE="${AWS_PROFILE:-positiveignition}"
# Blender's backups and the converter's leftovers stay at home.
EXCLUDE=(--exclude "*.blend1" --exclude "*.blend@")
what="${1:-}"
world="${2:-}"
need_world() { [ -n "$world" ] || { echo "Which world? blender/worlds.sh $what <world>"; exit 1; }; }
case "$what" in
    list)
        aws s3 ls "$BUCKET/" --no-sign-request
        ;;
    pull)
        need_world
        mkdir -p "worlds/$world"
        aws s3 sync "$BUCKET/$world" "worlds/$world" --no-sign-request --no-progress "${EXCLUDE[@]}"
        echo "Fetched worlds/$world."
        ;;
    push)
        need_world
        [ -d "worlds/$world" ] || { echo "There's no worlds/$world here."; exit 1; }
        aws s3 sync "worlds/$world" "$BUCKET/$world" --profile "$PROFILE" --no-progress "${EXCLUDE[@]}"
        echo "Sent worlds/$world to $BUCKET/$world."
        ;;
    versions)
        need_world
        aws s3api list-object-versions --bucket "${BUCKET#s3://}" --prefix "$world/" \
            --no-sign-request --query 'Versions[].[Key, LastModified, Size, IsLatest]' --output text
        ;;
    *)
        sed -n '2,9p' "$0" | sed 's/^# \{0,1\}//'
        exit 1
        ;;
esac
