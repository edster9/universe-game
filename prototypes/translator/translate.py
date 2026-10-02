"""A test of the translator: plain words in, the game's literal commands out.

The player describes a process in their own words. Claude, on Amazon
Bedrock or Anthropic's API directly, is shown only the character's scope (the console's `scope`: the
commands, the character's words, the ways they know to make things, what they
carry, and what they see) and turns the description into commands, reporting
whatever it can't map rather than inventing it. With --run, the commands are
played through the console's live channel, so the engine decides what
happens. See docs/ideas/ai-and-skills.md.

    uv run --with 'anthropic[bedrock]' prototypes/translator/translate.py \
        --setup "go forest" --run "find something small that burns ..."

Anthropic's API directly needs ANTHROPIC_API_KEY set (or `ant auth login`);
Bedrock needs `--provider bedrock` and an AWS profile (AWS_PROFILE) whose
account has Claude enabled.
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
- The process is the player's: follow their steps, in their order. Your job \
is to make each step happen with the commands, so do everything that can be \
done in scope, filling in the literal detail each step needs:
  - choosing what to use: map loose phrases to things in scope ("something \
small that burns" can be dry grass; "something we can rub together" can be \
two sticks of wood);
  - getting it: walking up to things ("go to <thing>") before gathering or \
taking them, and fetching what a step needs from wherever the character can \
see it or remembers it ("go <place>", gather, and go back);
  - how many: as many pieces as a way of making something needs;
  - putting things where the step says.
- Large amounts in a place (a patch, a bed, a bank, "50 kg") are sources: \
"gather" from them, a piece at a time. "take" is for single things.
- Use only commands from COMMANDS, and name things only with words from WORDS \
YOU KNOW or names shown under WHAT YOU CARRY, WHAT YOU REMEMBER, and WHAT YOU \
SEE.
- Report under `unmapped` only what can't be done with anything in scope: a \
thing the character has no word for and can't see or remember, a tool they \
don't have and can't make, a step no command does. Give the phrase and a \
short reason. If the player offered alternatives ("dig a hole or build a \
ring"), use the one in scope and report the other.
- Don't add goals beyond the player's description. If a step is ambiguous, \
choose the simplest reading and say so under `assumptions`.
- One command per entry, exactly as typed at the console. You may use the \
console's shortcuts: "<command> x3" to repeat, "<command>; <command>" in a \
row, and "wait <seconds>" or "wait <n> min" to let time pass."""


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
    # The plan as JSON in the reply: not every endpoint takes structured
    # outputs, so the shape is asked for, then checked.
    schema = json.dumps(Plan.model_json_schema())
    response = client.messages.create(
        model=model,
        max_tokens=16000,
        system=SYSTEM + f"\n\nReply with only a JSON object matching this schema, nothing else:\n{schema}",
        messages=[{"role": "user", "content": user}],
        **extra_args,
    )
    took = time.time() - started
    if response.stop_reason == "refusal":
        sys.exit(f"refused: {response.stop_details}")
    text = "".join(b.text for b in response.content if b.type == "text").strip()
    # The object itself, even with words around it.
    text = text[text.find("{") : text.rfind("}") + 1]
    return Plan.model_validate_json(text), response.usage, took


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("description")
    parser.add_argument("--world", default="companion.toml")
    parser.add_argument("--as", dest="person", default="survivor")
    parser.add_argument("--setup", default="", help="commands to play first, ';'-separated")
    parser.add_argument("--run", action="store_true", help="play the commands")
    parser.add_argument("--provider", choices=["anthropic", "bedrock"], default="anthropic")
    parser.add_argument("--model", help="defaults to Claude Opus 5.5 for the provider")
    parser.add_argument("--effort", default="medium")
    parser.add_argument("--show-scope", action="store_true")
    parser.add_argument("--repairs", type=int, default=2, help="fixes allowed after refusals")
    args = parser.parse_args()

    live = Live(args.world, args.person)
    for line in filter(None, (s.strip() for s in args.setup.split(";"))):
        live.send(line)
    scope = live.send("scope")["reply"]
    if args.show_scope:
        print(scope, "\n")

    if args.provider == "bedrock":
        # Bedrock's InvokeModel endpoint, with global routing. (Its newer
        # Messages endpoint takes structured outputs, but on the account
        # tried it couldn't subscribe to the models; this one answers.)
        client = anthropic.AnthropicBedrock(aws_region=os.environ.get("AWS_REGION", "us-east-1"))
        model = args.model or "global.anthropic.claude-opus-5-5"
    else:
        client = anthropic.Anthropic()
        model = args.model or "claude-opus-5-5"
    plan, usage, took = translate(client, model, args.effort, scope, args.description)
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
    print(f"\n({model}, {took:.1f} s, {usage.input_tokens} tokens in, {usage.output_tokens} out)")

    if args.run:
        print("\nPLAYED")
        commands, repairs, done = list(plan.commands), 0, []
        while commands:
            c = commands.pop(0)
            state = live.send(c)
            print(f"> {c}\n{state['reply']}")
            if state.get("refused") != "true" and state.get("refused") is not True:
                done.append(c)
                continue
            if repairs >= args.repairs:
                print("(refused: stopping here)")
                break
            # Refused: the translator sees why, and what the character sees
            # now, and fixes the rest of the plan.
            repairs += 1
            scope = live.send("scope")["reply"]
            extra = (
                "ALREADY DONE\n" + "\n".join(done)
                + f"\n\nTHIS COMMAND WAS REFUSED\n{c}\n{state['reply']}"
                + "\n\nSTILL TO DO\n" + "\n".join(commands)
                + "\n\nGive the commands that carry on from here to finish the player's "
                "description, fixing what was refused. The scope above is as things stand now."
            )
            plan, usage, took = translate(client, model, args.effort, scope, args.description, extra)
            print(f"  (repaired in {took:.1f} s: {'; '.join(plan.commands)})")
            for u in plan.unmapped:
                print(f"  (can't: {u.phrase}: {u.reason})")
            commands = list(plan.commands)
        print("\n> look\n" + live.send("look")["reply"])


if __name__ == "__main__":
    main()
