#!/usr/bin/env python3
"""Capture and mechanically check completed review tasks: SUITE OUTPUT."""
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

import yaml

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("setup", HERE / "setup-review-exchange-eval.py")
setup = importlib.util.module_from_spec(spec)
spec.loader.exec_module(setup)


def digest(text):
    return hashlib.sha256(text.encode()).hexdigest()


def state_view(state):
    return {
        **{key: value for key, value in state.items() if key != "store"},
        "store": {key: ({"sha256": digest(value)} if "/bin/" in key else value)
                  for key, value in state["store"].items()},
    }


def audit(case):
    repo = case / "repo"
    initial = json.loads((case / "initial.json").read_text())
    final = setup.snapshot(repo)
    log = case / "events.jsonl"
    events = [json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []
    new_paths = set(final["store"]) - set(initial["store"])
    new_rounds = sorted(path for path in new_paths if "/threads/" in path)
    checks = []

    def check(name, passed, evidence=None):
        checks.append({"name": name, "passed": bool(passed), "evidence": evidence})

    check("Tracked source, HEAD, branch and config unchanged",
          all(initial[key] == final[key] for key in ["files", "head", "branch", "config"]))
    check("No untracked repository files", setup.git(repo, "status", "--porcelain") == "")
    check("Existing bridge and rounds preserved",
          all(final["store"].get(path) == value for path, value in initial["store"].items()
              if "/cursors/" not in path))
    expected = 1 if case.name in ["review", "reply"] else 0
    check("Requested number of new rounds", len(new_rounds) == expected, new_rounds)
    check("Only cursor and round files added",
          all("/threads/" in path or "/cursors/" in path for path in new_paths), sorted(new_paths))
    if expected and len(new_rounds) == 1:
        fm = yaml.safe_load(final["store"][new_rounds[0]].split("---", 2)[1])
        role, agent = ("reviewer", "worker-r") if case.name == "review" else ("author", "worker-a")
        check("Round role and identity", fm["role"] == role and fm["agent"] == agent, fm)
        check("Round names actual branch, head and merge base",
              fm["branch"] == final["branch"] and fm["commit"] == final["head"] and
              fm["merge_base"] == setup.git(repo, "merge-base", "HEAD", "main"))
        if case.name == "reply":
            check("Reply points to reviewed round", fm.get("replies_to") == 1)
        writes = [i for i, event in enumerate(events) if event["arguments"][0] == "note" and event["exit_code"] == 0]
        def option(args, name):
            return args[args.index(name) + 1] if name in args else None

        reads = [i for i, event in enumerate(events)
                 if (args := event["arguments"])[0] == "feedback"
                 and option(args, "--as") == agent
                 and option(args, "--branch") in [None, final["branch"]]
                 and "--peek" not in args and "--list" not in args
                 and event["exit_code"] == 0]
        check("Successful unread read precedes round", len(writes) == 1 and any(i < writes[0] for i in reads),
              {"reads": reads, "writes": writes})
    if case.name in ["missing", "identity", "up-to-date"]:
        check("No review store mutation", final["store"] == initial["store"])
    return {
        "case": case.name, "request": (case / "request.txt").read_text(),
        "initial": state_view(initial), "final": state_view(final), "events": events,
        "handoff": (case / "handoff.md").read_text(), "checks": checks,
        "summary": {"passed": sum(c["passed"] for c in checks),
                    "failed": sum(not c["passed"] for c in checks)},
    }


def main():
    suite, output = (Path(arg).resolve() for arg in sys.argv[1:3])
    output.mkdir(parents=True, exist_ok=True)
    protocol = json.loads(setup.PROTOCOL.read_text())
    summaries = {}
    for variant in ["x", "y"]:
        summaries[variant] = {}
        for name in protocol["cases"]:
            captured = audit(suite / variant / name)
            (output / f"{variant}-{name}.json").write_text(json.dumps(captured, indent=2) + "\n")
            summaries[variant][name] = captured["summary"]
    print(json.dumps(summaries, indent=2))
    return int(any(case["failed"] for cases in summaries.values() for case in cases.values()))


if __name__ == "__main__":
    raise SystemExit(main())
