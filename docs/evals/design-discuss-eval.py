#!/usr/bin/env python3
"""Local fixtures: setup OUTPUT BASELINES; tool CASE NAME JSON; capture SUITE OUTPUT."""
import hashlib
import json
from pathlib import Path
import random
import re
import subprocess
import sys

import yaml

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
PROTOCOL = HERE / "fixtures/design-discuss-concision-v1.json"
SKILLS = ["propose-design", "discuss", "frame-problem", "vault-search"]
PROBLEM = "2026-10-01-export-retries-problem"
DESIGN = "2026-10-02-export-resume"
INQUIRY = "2026-10-03-proxy-range"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(repo, *args):
    return subprocess.check_output(["git", *args], cwd=repo, text=True).strip()


def save(path, data):
    path.write_text(json.dumps(data, indent=2) + "\n")


def artifact(repo, folder, data, body):
    path = repo / folder / (data["id"] + ".md")
    path.parent.mkdir(exist_ok=True)
    path.write_text("---\n" + yaml.safe_dump(data, sort_keys=False) + "---\n\n" + body + "\n")


def seed(repo, kind):
    if kind in ["accepted", "obsolete"]:
        status = "obsolete" if kind == "obsolete" else "accepted"
        body = "# Export retries\n\nCustomers restart hour-long exports after dropped connections. Resume must preserve account isolation, existing downloads and retention limits."
        if kind == "obsolete":
            body += "\n\nSet aside because export storage was retired; customers now download from a different service. The old ownership and retention assumptions no longer apply."
        artifact(repo, "briefs", {"id": PROBLEM, "type": "problem-brief", "status": status,
                                 "author": "Arun", "created": "2026-10-01"}, body)
    if kind == "accepted":
        artifact(repo, "briefs", {"id": DESIGN, "type": "design-brief", "status": "accepted",
                                 "author": "Arun", "created": "2026-10-02", "frames": PROBLEM},
                 "# Export resume\n\nUse signed client offsets for resumption. Validate account ownership and retain existing downloads. Revisit if revocation cannot be enforced.")
        artifact(repo, "inquiries", {"id": INQUIRY, "type": "inquiry", "status": "open",
                                    "author": "Arun", "created": "2026-10-03", "relates_to": [PROBLEM]},
                 "# Proxy range support\n\nDetermine whether supported proxies preserve range requests. No prototype result is available.")
    if kind == "reader":
        artifact(repo, "briefs", {"id": "2026-10-01-reader-repeat-problem", "type": "problem-brief",
                                 "status": "accepted", "author": "Imani", "created": "2026-10-01"},
                 "# Reader repeat views\n\nAnalysts wait for historical usage charts to reload when switching tabs during account reviews. Repeat views should be faster without exposing another account's data. Acceptable staleness, request volume and budget are not yet known.")


def snapshot(repo):
    return {
        "head": git(repo, "rev-parse", "HEAD"), "branch": git(repo, "branch", "--show-current"),
        "config": (repo / ".git/config").read_text(),
        "files": {name: (repo / name).read_text() for name in git(repo, "ls-files").splitlines()},
        "status": git(repo, "status", "--porcelain"),
    }


def setup(output, baselines):
    output.mkdir(parents=True, exist_ok=False)
    protocol = json.loads(PROTOCOL.read_text())
    roles = ["baseline", "candidate"]
    random.SystemRandom().shuffle(roles)
    manifest = {"protocol_sha256": sha(PROTOCOL), "harness_sha256": sha(Path(__file__)), "variants": {}}
    for label, role in zip(["r", "s"], roles):
        root = output / label
        (root / "skills").mkdir(parents=True)
        hashes = {}
        for name in SKILLS:
            source = (baselines / f"{name}.md" if role == "baseline" and name in SKILLS[:2]
                      else REPO / "skills/vault" / name / "SKILL.md")
            target = root / "skills" / f"{name}.md"
            target.write_bytes(source.read_bytes())
            hashes[name] = sha(target)
        for name, case in protocol["cases"].items():
            folder = root / name
            repo = folder / "vault"
            repo.mkdir(parents=True)
            git(repo, "init", "-q", "-b", "work")
            git(repo, "config", "user.name", "Fixture owner")
            git(repo, "config", "user.email", "fixture@example.invalid")
            (repo / "README.md").write_text("# Disposable knowledge vault\n")
            seed(repo, case["seed"])
            git(repo, "add", ".")
            git(repo, "commit", "-q", "-m", "Seed local vault")
            save(folder / "initial.json", snapshot(repo))
            request = f"""Use {case['skill']} at {root / 'skills' / (case['skill'] + '.md')}.
Local dependency instructions are in {root / 'skills'}.

User request:
{case['request']}

Vault: {repo}
Save your user-facing response to {folder / 'handoff.md'}.
Save proposed diffs or temporary notes beside the vault, not inside it.

This is a local execution trial. Perform authorized file writes and Git commits
in this disposable vault. There is no remote. No network, live vault or GitHub
access is allowed. Git config identifies the fixture owner, not the note author.
For vault reads/checks, use this fixture stand-in instead of live MCP calls:
python {Path(__file__).resolve()} tool {folder} TOOL 'JSON'
TOOL is vault_search, vault_edges or vault_check_transition. Arguments follow
the schemas in the supplied vault-search skill. Search uses simple token overlap;
edges cover frames, supersedes, depends_on and relates_to. Transition checks
only enforce the fixture's unresolved frame/dependency blockers. These are
simulated tool results, not production-validator evidence. Read returned files
for their full content. The tool writes its own event log outside the vault.

Read only this case, assigned skill/dependencies and tool results. Do not read
the harness source, protocol, initial.json, other cases, variants or history
outside this vault. Do not modify instructions, harness, initial state or logs.
If user input is needed, save the question in handoff.md and stop the case.
Do not wait for a simulated answer or infer approval from the test environment.
"""
            (folder / "request.txt").write_text(request)
        manifest["variants"][label] = {"role": role, "skills": hashes,
            "requests": {name: sha(root / name / "request.txt") for name in protocol["cases"]}}
    save(output / "manifest.json", manifest)
    print(output)


def records(repo):
    result = []
    for path in sorted(repo.rglob("*.md")):
        text = path.read_text()
        if not text.startswith("---\n"):
            continue
        data = yaml.safe_load(text.split("---", 2)[1])
        if not isinstance(data, dict) or "id" not in data:
            continue
        if "created" in data:
            data["created"] = str(data["created"])
        result.append({**data, "path": str(path.relative_to(repo)),
                       "title": text.split("---", 2)[2].strip().splitlines()[0].lstrip("# "),
                       "_text": text})
    return result


def tool(case, name, args):
    items = records(case / "vault")
    public = lambda item: {key: value for key, value in item.items() if key != "_text"}
    if name == "vault_search":
        matches = [item for item in items if all(item.get(key) == args[key]
                   for key in ["type", "status", "author"] if key in args)]
        if "topic" in args:
            terms = set(re.findall(r"\w+", args["topic"].lower()))
            matches = [item for item in matches if terms & set(re.findall(r"\w+", item["_text"].lower()))]
        matches = matches[:args.get("limit", 50)]
        result = {"count": len(matches), "matches": [public(item) for item in matches]}
    elif name == "vault_edges":
        edges = []
        by_id = {item["id"]: item for item in items}
        for item in items:
            for kind in ["frames", "supersedes", "depends_on", "relates_to"]:
                if args.get("kind", kind) != kind:
                    continue
                values = item.get(kind, [])
                for target in values if isinstance(values, list) else [values]:
                    direction = args.get("direction", "both")
                    outgoing = item["id"] == args["id"] and direction in ["outgoing", "both"]
                    incoming = target == args["id"] and direction in ["incoming", "both"]
                    if outgoing or incoming:
                        neighbor = by_id.get(target if outgoing else item["id"])
                        edges.append({"from_id": item["id"], "to_id": target, "kind": kind,
                                      "neighbor": public(neighbor) if neighbor else None})
        result = {"id": args["id"], "count": len(edges), "edges": edges}
    elif name == "vault_check_transition":
        by_id = {item["id"]: item for item in items}
        item = by_id.get(args["id"])
        blockers = []
        if item is None:
            blockers.append("Artifact does not exist")
        elif args["new_status"] in ["proposed", "accepted"]:
            frame = by_id.get(item.get("frames"))
            if item.get("frames") and (not frame or frame.get("status") != "accepted"):
                blockers.append("Framed problem is not accepted")
            for dependency in item.get("depends_on", []):
                if by_id.get(dependency, {}).get("status") not in ["closed", "accepted", "resolved"]:
                    blockers.append("Unresolved dependency: " + dependency)
        result = {"allowed": not blockers, "blockers": blockers, "warnings": []}
    else:
        raise ValueError("Unsupported fixture tool: " + name)
    with (case / "events.jsonl").open("a") as log:
        log.write(json.dumps({"tool": name, "arguments": args, "result": result,
                             "head": git(case / "vault", "rev-parse", "HEAD")}) + "\n")
    print(json.dumps(result, indent=2))


def capture(suite, output):
    output.mkdir(parents=True, exist_ok=True)
    protocol = json.loads(PROTOCOL.read_text())
    summaries = {}
    for label in ["r", "s"]:
        summaries[label] = {}
        for name in protocol["cases"]:
            case = suite / label / name
            repo = case / "vault"
            initial = json.loads((case / "initial.json").read_text())
            final = snapshot(repo)
            added = set(final["files"]) - set(initial["files"])
            commits = git(repo, "rev-list", initial["head"] + "..HEAD").splitlines()
            history = {commit: git(repo, "diff-tree", "--no-commit-id", "--name-status", "-r", commit)
                       for commit in commits}
            writes = name in ["design-supplied", "discuss-supplied"]
            prefix = "briefs/" if name == "design-supplied" else "discussions/"
            checks = {
                "existing_artifacts_preserved": all(final["files"].get(k) == v for k, v in initial["files"].items()),
                "config_and_branch_preserved": all(initial[k] == final[k] for k in ["config", "branch"]),
                "clean_worktree": final["status"] == "",
                "expected_new_file_count": len(added) == (1 if writes else 0),
                "new_files_in_authorized_folder": all(path.startswith(prefix) for path in added) if writes else not added,
                "expected_commit_presence": bool(commits) == writes,
                "history_only_adds_authorized_files": all(line.startswith("A\t") and line[2:] in added
                    for changes in history.values() for line in changes.splitlines()),
            }
            log = case / "events.jsonl"
            result = {"case": name, "request": (case / "request.txt").read_text(),
                      "initial": initial, "final": final, "new_files": sorted(added),
                      "commits": history, "events": [json.loads(line) for line in log.read_text().splitlines()] if log.exists() else [],
                      "handoff": (case / "handoff.md").read_text(), "checks": checks,
                      "summary": {"passed": sum(checks.values()), "failed": sum(not v for v in checks.values())}}
            save(output / f"{label}-{name}.json", result)
            summaries[label][name] = result["summary"]
    print(json.dumps(summaries, indent=2))
    return int(any(c["failed"] for cases in summaries.values() for c in cases.values()))


if __name__ == "__main__":
    command = sys.argv[1]
    if command == "setup":
        setup(Path(sys.argv[2]).resolve(), Path(sys.argv[3]).resolve())
    elif command == "tool":
        tool(Path(sys.argv[2]).resolve(), sys.argv[3], json.loads(sys.argv[4]))
    elif command == "capture":
        raise SystemExit(capture(Path(sys.argv[2]).resolve(), Path(sys.argv[3]).resolve()))
    else:
        raise SystemExit("Expected setup, tool or capture")
