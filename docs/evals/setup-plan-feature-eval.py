#!/usr/bin/env python3
"""Prepare neutral planning packets: OUTPUT BASELINE_SKILL CANDIDATE_SKILL."""
import hashlib
import json
from pathlib import Path
import random
import sys

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
PROTOCOL = HERE / "fixtures/plan-feature-concision-v1.json"
DEPENDENCIES = ["frame-problem", "propose-design", "discuss", "vault-search"]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    output, baseline, candidate = (Path(arg).resolve() for arg in sys.argv[1:4])
    output.mkdir(parents=True, exist_ok=False)
    protocol = json.loads(PROTOCOL.read_text())
    sources = [("baseline", baseline), ("candidate", candidate)]
    random.SystemRandom().shuffle(sources)
    manifest = {
        "protocol_sha256": sha(PROTOCOL), "setup_sha256": sha(Path(__file__)),
        "variants": {},
    }
    for label, (role, source) in zip(["p", "q"], sources):
        root = output / label
        root.mkdir()
        skill = root / "SKILL.md"
        skill.write_bytes(source.read_bytes())
        dependencies = root / "dependencies"
        dependencies.mkdir()
        hashes = {}
        for name in DEPENDENCIES:
            target = dependencies / f"{name}.md"
            target.write_bytes((REPO / "skills/vault" / name / "SKILL.md").read_bytes())
            hashes[name] = sha(target)
        for name, case in protocol["cases"].items():
            text = f"""Use plan-feature at {skill}.
Dependency skills are local files in {dependencies}; read them when invoked.
User request:
{case['request']}

Supplied context:
{case['context']}

Evaluation environment:
This is a planning simulation. Use the supplied local vault snapshot as search
evidence and disclose that live search was not run. Read only your assigned
skill, dependencies and request files. Do not inspect other cases before they
are assigned, variants, protocol, repository history or earlier evaluations.
No live vault, GitHub, network, commits, pushes or project mutations are allowed.
For authorized external actions, show the concrete proposed artifact and action
instead of executing it. This restriction is not a request to ask the user to
reconfirm authorization. Save the user-facing response as {root / (name + '.md')}.
If you need user input, save your question there and stop that case.
"""
            (root / f"{name}-request.txt").write_text(text)
        manifest["variants"][label] = {
            "role": role, "skill_sha256": sha(skill), "dependencies": hashes,
            "requests": {name: sha(root / f"{name}-request.txt")
                         for name in protocol["cases"]},
        }
    (output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(output)


if __name__ == "__main__":
    main()
