"""A test of the translator: plain words in, the game's literal commands out.

The player describes a process in their own words. Claude, on Amazon
Bedrock, is shown only the character's scope (the console's `scope`: the
commands, the character's words, the ways they know to make things, what they
carry, and what they see) and turns the description into commands, reporting
whatever it can't map rather than inventing it. With --run, the commands are
played through the console's live channel, so the engine decides what
happens. See docs/ideas/ai-and-skills.md.

    uv run --with 'anthropic[bedrock]' prototypes/translator/translate.py \
        --setup "go forest" --run "find something small that burns ..."
"""

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

from pydantic import BaseModel
import anthropic

ROOT = Path(__file__).resolve().parents[2]
CONSOLE = ROOT / "target" / "release" / "console"

SYSTEM = """You translate a player's plain description of what they want to do into \
the literal commands of a text game's console, for their character.

You are given the character's SCOPE: the console's commands, the words the \
character knows, the ways they know to make things, what they carry, and what \
they see. That is everything you may use.

Rules:
- The process is the player's. Carry out the steps they describe, in their \
order, filling in only the literal detail each step needs: walking up to \
things ("go to <thing>") before gathering or taking them, how many pieces a \
way of making something needs, what to put where.
- Use only commands from COMMANDS, and name things only with words from WORDS \
YOU KNOW or names shown under WHAT YOU CARRY and WHAT YOU SEE. Map loose \
phrases to those ("something small that burns" may be dry grass in sight).
- Never invent things, tools, places, or abilities that aren't in scope. If \
part of the description can't be done with what's in scope (a thing the \
character has no word for, a tool they don't have and can't make, a step no \
command does), leave it out of the commands and report it under `unmapped`, \
with the phrase and a short reason. If something has an in-scope alternative \
the player offered ("dig a hole or build a ring"), use the one in scope and \
report the other.
- Don't add goals the player didn't describe. If a step is ambiguous, choose \
the simplest reading and say so under `assumptions`.
- One command per entry, exactly as typed at the console. You may use the \
console's shortcuts: "<command> x3" to repeat, and "wait <seconds>" or "wait \
<n> min" to let time pass."""


class Unmapped(BaseModel):
    phrase: str
    reason: str


class Plan(BaseModel):
    commands: list[str]
    unmapped: list[Unmapped]
    assumptions: list[str]


class Live:
    """The console's live channel: a command in, a line of JSON out."""

    def __init__(self, world: str, person: str):
        self.proc = subprocess.Popen(
            [str(CONSOLE), "--world", str(ROOT / "data" / world), "--as", person, "--live"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
        )
        self.read()

    def read(self) -> dict:
        return json.loads(self.proc.stdout.readline())

    def send(self, line: str) -> dict:
        self.proc.stdin.write(line + "\n")
        self.proc.stdin.flush()
        return self.read()


def translate(client, model: str, effort: str, scope: str, description: str, extra: str = ""):
    user = f"SCOPE\n{scope}\n\nTHE PLAYER SAYS\n{description}"
    if extra:
        user += f"\n\n{extra}"
    started = time.time()
    # Haiku 4.5 takes no effort and thinks only with a budget: plain there.
    extra_args = {} if "haiku" in model else {
        "thinking": {"type": "adaptive"},
        "output_config": {"effort": effort},
    }
    response = client.messages.parse(
        model=model,
        max_tokens=16000,
        system=SYSTEM,
        messages=[{"role": "user", "content": user}],
        output_format=Plan,
        **extra_args,
    )
    took = time.time() - started
    if response.stop_reason == "refusal":
        sys.exit(f"refused: {response.stop_details}")
    return response.parsed_output, response.usage, took


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("description")
    parser.add_argument("--world", default="companion.toml")
    parser.add_argument("--as", dest="person", default="survivor")
    parser.add_argument("--setup", default="", help="commands to play first, ';'-separated")
    parser.add_argument("--run", action="store_true", help="play the commands")
    # The most capable model enabled on the account tried (2026-10-01).
    parser.add_argument("--model", default="us.anthropic.claude-opus-4-6-v1")
    parser.add_argument("--effort", default="medium")
    parser.add_argument("--show-scope", action="store_true")
    args = parser.parse_args()

    live = Live(args.world, args.person)
    for line in filter(None, (s.strip() for s in args.setup.split(";"))):
        live.send(line)
    scope = live.send("scope")["reply"]
    if args.show_scope:
        print(scope, "\n")

    client = anthropic.AnthropicBedrock(aws_region=os.environ.get("AWS_REGION", "us-east-1"))
    plan, usage, took = translate(client, args.model, args.effort, scope, args.description)
    print(f"PLAYER: {args.description}\n")
    print("COMMANDS")
    for c in plan.commands:
        print(f"  {c}")
    if plan.unmapped:
        print("CAN'T BE DONE WITH WHAT'S IN SCOPE")
        for u in plan.unmapped:
            print(f"  {u.phrase}: {u.reason}")
    if plan.assumptions:
        print("ASSUMED")
        for a in plan.assumptions:
            print(f"  {a}")
    print(f"\n({took:.1f} s, {usage.input_tokens} tokens in, {usage.output_tokens} out)")

    if args.run:
        print("\nPLAYED")
        for c in plan.commands:
            state = live.send(c)
            print(f"> {c}\n{state['reply']}")
            if state.get("refused"):
                print("(refused: stopping here)")
                break
        print("\n> look\n" + live.send("look")["reply"])


if __name__ == "__main__":
    main()
