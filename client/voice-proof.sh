#!/usr/bin/env bash
# Plays a script by voice: Windows' own speech voice says each of its
# commands into a recording, and the client hears each one as the islander
# would there and then, runs it as heard, and checks the script's
# expectations. `client/voice-proof.sh first-steps-3-fire.txt`. Build first
# with run.sh.
set -euo pipefail
cd "$(dirname "$0")"
SCRIPT=${1:?a script in data/scripts}
. ./where.sh
mkdir -p "$TARGET/voice"
# Each command once: not the header, expectations, repeats, or comments.
python3 - "../data/scripts/$SCRIPT" "$TARGET/voice/say.ps1" <<'PY'
import re, sys
import os
def expand(path):
    # Shared recipes ("include skills/x.txt") are relative to data/scripts.
    scripts = os.path.join(os.path.dirname(sys.argv[1]))
    for raw in open(path):
        line = raw.split("#")[0].strip()
        if line.startswith("include "):
            yield from expand(os.path.join(scripts, line[8:].strip()))
        else:
            yield line
lines = []
for line in expand(sys.argv[1]):
    if not line or line.split()[0] in ("world", "as", "luck", "rules", "expect", "repeat", "end", "include"):
        continue
    if line.startswith("try "):
        line = line[4:].strip()
    if line not in lines:
        lines.append(line)
names = ["-".join(w for w in re.split(r"[^A-Za-z0-9]", l) if w) for l in lines]
with open(sys.argv[2], "w") as out:
    out.write("Add-Type -AssemblyName System.Speech\n")
    out.write("$f = New-Object System.Speech.AudioFormat.SpeechAudioFormatInfo(16000, [System.Speech.AudioFormat.AudioBitsPerSample]::Sixteen, [System.Speech.AudioFormat.AudioChannel]::Mono)\n")
    for line, name in zip(lines, names):
        out.write("$s = New-Object System.Speech.Synthesis.SpeechSynthesizer\n")
        out.write(f"$s.SetOutputToWaveFile(\"{name}.wav\", $f)\n")
        out.write(f"$s.Speak(\"{line}\")\n$s.Dispose()\n")
PY
(cd "$TARGET/voice" && powershell.exe -NoProfile -ExecutionPolicy Bypass -File say.ps1)
cd "$TARGET"
./client.exe --hear-script "$SCRIPT"
