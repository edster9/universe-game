"""The translator's test bench: the same plain descriptions, played several
times on several models, each scored from what happens in the game.

    uv run --with anthropic --with boto3 prototypes/translator/bench.py --runs 2

Needs the keys in the project's .env (ANTHROPIC_API_KEY, GROK_API_KEY). See
docs/ideas/ai-and-skills.md.
"""

import argparse
import json
import re
import time
from concurrent.futures import ThreadPoolExecutor

import anthropic

import translate as t

FIRE = (
    "find something small that could burn first, find something we can rub together, "
    "dig a hole or build a rock circle, put things in there, and go ahead and rub things "
    "together and make a fire"
)


def burning(final: dict) -> bool:
    return "burning" in final["look"]


def wood_burning(final: dict) -> bool:
    # "stick of wood and ash (150 g, 1224 K, burning)": a stick or a log alight.
    return bool(re.search(r"(stick|log) of wood[^)]*burning", final["look"]))


CASES = [
    {
        "name": "the owner's fire",
        "setup": "go forest; go hillside; go forest",
        "says": FIRE,
        "after": "wait 20",
        "passes": burning,
    },
    {
        "name": "a cigarette lighter",
        "setup": "go forest",
        "says": "use a cigarette lighter to light some grass and twigs",
        "after": "",
        # Reported, and never put in a command.
        "passes": lambda f: any("lighter" in u.lower() for u in f["unmapped"])
        and not any("lighter" in c.lower() for c in f["commands"]),
    },
    {
        "name": "grab three sticks",
        "setup": "go forest",
        "says": "grab three sticks",
        "after": "",
        "passes": lambda f: "x3: 600 g" in f["backpack"],
    },
    {
        "name": "stones never seen",
        "setup": "go forest",
        "says": "build a ring of stones here and make a fire in it with some grass and two sticks",
        "after": "",
        # The islander has never seen stones: nothing may fetch them from a
        # place they don't know has any, and the stones must be reported.
        "passes": lambda f: not any("hillside" in c.lower() for c in f["commands"])
        and any("stone" in u.lower() for u in f["unmapped"]),
    },
    {
        "name": "a fire, fed",
        "setup": "go forest; go hillside; go forest",
        "says": "build a stone ring, light grass in it by rubbing two sticks, then once the "
        "grass is burning feed it twigs, and once the twigs catch put on sticks",
        "after": "wait 1 min",
        "passes": wood_burning,
    },
    {
        "name": "put it all down",
        "setup": "go forest; go to sticks; gather sticks x2; go to grass; gather grass",
        "says": "put everything I'm carrying down on the ground",
        "after": "",
        "passes": lambda f: f["backpack"].startswith("nothing"),
    },
]

MODELS = [
    ("anthropic", "claude-opus-5-5"),
    ("anthropic", "claude-sonnet-5-5"),
    ("anthropic", "claude-haiku-4-5"),
    ("xai", "grok-4.7"),
    ("xai", "grok-4.20-0309-non-reasoning"),
]


def client_for(provider: str):
    if provider == "anthropic":
        return anthropic.Anthropic()
    return provider


def run(provider: str, model: str, case: dict, effort: str, repairs: int) -> dict:
    client = client_for(provider)
    live = t.Live("companion.toml", "survivor")
    for line in filter(None, (s.strip() for s in case["setup"].split(";"))):
        live.send(line)
    commands, unmapped, seconds, tokens_in, tokens_out = [], [], 0.0, 0, 0
    error = None
    try:
        scope = live.send("scope")["reply"]
        plan, usage, took = t.translate(client, model, effort, scope, case["says"])
        seconds += took
        tokens_in, tokens_out = usage.input_tokens, usage.output_tokens
        unmapped += [f"{u.phrase}: {u.reason}" for u in plan.unmapped]
        queue, done, used = list(plan.commands), [], 0
        while queue:
            c = queue.pop(0)
            commands.append(c)
            state = live.send(c)
            if str(state.get("refused")).lower() != "true":
                done.append(c)
                continue
            if used >= repairs:
                break
            used += 1
            extra = (
                "ALREADY DONE\n" + "\n".join(done)
                + f"\n\nTHIS COMMAND WAS REFUSED\n{c}\n{state['reply']}"
                + "\n\nSTILL TO DO\n" + "\n".join(queue)
                + "\n\nGive the commands that carry on from here to finish the player's "
                "description, fixing what was refused. The scope above is as things stand now."
            )
            plan, usage, took = t.translate(
                client, model, effort, live.send("scope")["reply"], case["says"], extra
            )
            seconds += took
            tokens_in += usage.input_tokens
            tokens_out += usage.output_tokens
            unmapped += [f"{u.phrase}: {u.reason}" for u in plan.unmapped]
            queue = list(plan.commands)
    except Exception as e:  # a failed call or a reply that wasn't a plan
        error = str(e)[:2000]
    if case["after"]:
        live.send(case["after"])
    final = {
        "commands": commands,
        "unmapped": unmapped,
        "look": live.send("look")["reply"],
        "backpack": live.send("backpack")["reply"],
    }
    live.proc.kill()
    return {
        "model": model,
        "case": case["name"],
        "passed": error is None and bool(case["passes"](final)),
        "seconds": round(seconds, 1),
        "tokens_in": tokens_in,
        "tokens_out": tokens_out,
        "error": error,
        **final,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--runs", type=int, default=2)
    parser.add_argument("--effort", default="medium")
    parser.add_argument("--repairs", type=int, default=2)
    parser.add_argument("--out", default="bench-results.json")
    parser.add_argument("--models", default="", help="comma-separated model names to keep")
    args = parser.parse_args()
    models = [m for m in MODELS if not args.models or m[1] in args.models.split(",")]

    def one_model(pm):
        provider, model = pm
        out = []
        for case in CASES:
            for _ in range(args.runs):
                r = run(provider, model, case, args.effort, args.repairs)
                print(f"{model:32} {case['name']:22} {'PASS' if r['passed'] else 'fail'} "
                      f"{r['seconds']:6.1f} s {(r['error'] or '').splitlines()[0][:80] if r['error'] else ''}", flush=True)
                out.append(r)
        return out

    started = time.time()
    with ThreadPoolExecutor(len(models)) as pool:
        results = [r for rs in pool.map(one_model, models) for r in rs]
    with open(args.out, "w") as f:
        json.dump(results, f, indent=1)

    print(f"\n{'model':32} " + " ".join(f"{c['name'][:12]:>12}" for c in CASES) + "   total   avg s")
    for _, model in models:
        rs = [r for r in results if r["model"] == model]
        cells = []
        for case in CASES:
            cr = [r for r in rs if r["case"] == case["name"]]
            cells.append(f"{sum(r['passed'] for r in cr)}/{len(cr):<10}".rjust(12))
        total = sum(r["passed"] for r in rs)
        avg = sum(r["seconds"] for r in rs) / max(len(rs), 1)
        print(f"{model:32} " + " ".join(cells) + f"   {total}/{len(rs)}   {avg:5.1f}")
    print(f"\n({time.time() - started:.0f} s in all; details in {args.out})")


if __name__ == "__main__":
    main()
