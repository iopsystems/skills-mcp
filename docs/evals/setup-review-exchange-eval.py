#!/usr/bin/env python3
"""Create isolated Git review tasks: SUITE SKILL_X SKILL_Y."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import textwrap

HERE = Path(__file__).resolve().parent
PROTOCOL = HERE / "fixtures/review-exchange-concision-v1.json"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(repo, *args):
    env = dict(os.environ)
    env.pop("REVIEW_AGENT", None)
    return subprocess.check_output(args, cwd=repo, env=env, text=True, stderr=subprocess.PIPE).strip()


def git(repo, *args):
    return run(repo, "git", *args)


def write(repo, name, text):
    path = repo / name
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def commit(repo, message):
    git(repo, "add", ".")
    git(repo, "commit", "-m", message)
    return git(repo, "rev-parse", "HEAD")


def install(repo, installer):
    subprocess.run(["bash"], input=installer, cwd=repo, text=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, check=True)


def note(repo, agent, role, body, reply=None):
    git(repo, "review-feedback", "--as", agent)
    args = ["review-note", "--as", agent, "--role", role, "-m", body]
    if reply:
        args += ["--replies-to", str(reply)]
    git(repo, *args)


def wrap_bridge(repo, case):
    bridge = repo / ".git/review-feedback/bin/review-bridge"
    core = bridge.with_name("review-bridge-core")
    bridge.rename(core)
    bridge.write_text(textwrap.dedent(f"""\
        #!/usr/bin/env python3
        import datetime,json,subprocess,sys
        from pathlib import Path
        started=datetime.datetime.now(datetime.timezone.utc).isoformat()
        result=subprocess.run([{str(core)!r},*sys.argv[1:]],text=True,capture_output=True)
        event={{'started':started,'arguments':sys.argv[1:],'exit_code':result.returncode,
               'stdout':result.stdout,'stderr':result.stderr}}
        with Path({str(case / 'events.jsonl')!r}).open('a') as output:
            output.write(json.dumps(event)+'\\n')
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise SystemExit(result.returncode)
    """))
    bridge.chmod(0o755)


def snapshot(repo):
    return {
        "head": git(repo, "rev-parse", "HEAD"),
        "branch": git(repo, "branch", "--show-current"),
        "files": {p: (repo / p).read_text() for p in git(repo, "ls-files").splitlines()},
        "config": (repo / ".git/config").read_text(),
        "store": {str(p.relative_to(repo / ".git")): p.read_text()
                  for p in (repo / ".git/review-feedback").rglob("*") if p.is_file()},
    }


def setup_case(case, name, installer):
    repo = case / "repo"
    repo.mkdir(parents=True)
    git(repo, "init", "-b", "main")
    git(repo, "config", "user.name", "Fixture owner")
    git(repo, "config", "user.email", "fixture@example.invalid")
    write(repo, "README.md", "# Reservation service\n\nPublc API example.\n")
    if name == "review":
        write(repo, "reservation.py", "def reserve(stock, quantity):\n    if quantity > stock:\n        raise ValueError('insufficient stock')\n    return stock - quantity\n")
        write(repo, "test_reservation.py", "import unittest\nfrom reservation import reserve\n\nclass Tests(unittest.TestCase):\n    def test_partial(self):\n        self.assertEqual(reserve(5, 2), 3)\n    def test_shortage(self):\n        with self.assertRaises(ValueError):\n            reserve(5, 6)\n")
    elif name == "reply":
        write(repo, "report.py", "def normalize_label(label):\n    return label\n\ndef cached_keys(cache):\n    return tuple(cache)\n\ndef stream_report(rows):\n    return '\\n'.join(rows)\n")
    commit(repo, "Baseline fixture")
    git(repo, "switch", "-c", "topic/reserve" if name == "review" else "topic/report")
    if name != "missing":
        install(repo, installer)
    if name == "review":
        path = repo / "reservation.py"
        path.write_text(path.read_text().replace("quantity > stock", "quantity >= stock"))
        commit(repo, "Adjust reservation capacity check")
        note(repo, "peer", "reviewer",
             "## Findings\n\nF1: reservation.py permits a negative quantity and increases stock. Reject nonpositive requests.\n\n## Open questions\n\nNone.\n\n## Verification\n\nThis is a seeded fixture review; no test execution is claimed.\n")
        path.write_text(path.read_text().replace("    if quantity >= stock:", "    if quantity <= 0:\n        raise ValueError('quantity must be positive')\n    if quantity >= stock:"))
        fix = commit(repo, "Reject nonpositive reservation quantities")
        note(repo, "author", "author", f"F1 — fixed in {fix}; nonpositive quantities now raise ValueError. No other findings or questions were supplied.", 1)
        git(repo, "switch", "main")
        write(repo, "main_only.py", "def ratio(value):\n    return value / 0\n")
        commit(repo, "Unrelated main-only example")
        git(repo, "switch", "topic/reserve")
    elif name == "reply":
        note(repo, "peer", "reviewer",
             "## Findings\n\nF1: report.py:2 preserves unwanted surrounding whitespace in labels.\n\nF2: report.py:5 exposes a live cache-key view; later cache insertions change the returned collection.\n\nF3: report.py:8 buffers the whole report; a million-row workload can exceed the caller's memory budget.\n\n## Open questions\n\nDoes the trim fix change the public function signature?\n\n## Verification\n\nThese are seeded fixture findings; no test run is claimed.\n")
        path = repo / "report.py"
        path.write_text(path.read_text().replace("return label\n", "return label.strip()\n"))
        write(repo, "README.md", "# Reservation service\n\nPublic API example.\n")
        commit(repo, "Trim labels and correct README spelling")
    elif name == "up-to-date":
        note(repo, "peer", "reviewer", "## Findings\n\nNone actionable; this does not establish correctness.\n\n## Open questions\n\nNone.\n\n## Verification\n\nNo commands run in this seeded fixture review.\n")
        git(repo, "review-feedback", "--as", "worker-r")
    if name != "missing":
        wrap_bridge(repo, case)
    (case / "initial.json").write_text(json.dumps(snapshot(repo), indent=2) + "\n")
    return repo


def main():
    suite = Path(sys.argv[1]).resolve()
    suite.mkdir(parents=True, exist_ok=False)
    protocol = json.loads(PROTOCOL.read_text())
    manifests = {}
    installers = []
    for variant, source in zip(["x", "y"], sys.argv[2:4]):
        source = Path(source).resolve()
        directory = suite / variant
        directory.mkdir()
        skill = directory / "SKILL.md"
        skill.write_bytes(source.read_bytes())
        installer = skill.read_text().split("<!-- INSTALLER-BEGIN -->\n```sh\n")[1].split("\n```\n<!-- INSTALLER-END -->")[0]
        installers.append(installer)
        for name, settings in protocol["cases"].items():
            case = directory / name
            repo = setup_case(case, name, installer)
            request = f"""Use the review-exchange skill at {skill}.
User request: {settings['request']}

Repository: {repo}
This is an execution trial in a disposable local repository. Perform the task
with real Git commands and inspect actual results. There is no remote.
REVIEW_AGENT is unset; keep it unset unless this request supplies a name.
The Git user.name belongs to the fixture owner.

Only the requested local review actions are authorized. Do not edit source,
tests, installer scripts, logging wrappers, initial.json or event logs.
Do not inspect other cases, variants, setup source, grading criteria or earlier
evaluations. Do not contact external services. For permission or identity that
the task does not supply, write the question in your handoff and stop that case;
do not wait for a simulated reply. Save temporary probes/body files beside the
repo and the final user-facing response to {case / 'handoff.md'}.
"""
            (case / "request.txt").write_text(request)
        manifests[variant] = {"skill_sha256": sha(skill)}
    assert installers[0] == installers[1], "The comparison must preserve installer bytes"
    (suite / "manifest.json").write_text(json.dumps({
        "protocol_sha256": sha(PROTOCOL),
        "setup_sha256": sha(Path(__file__)),
        "variants": manifests,
        "installer_sha256": hashlib.sha256(installers[0].encode()).hexdigest(),
    }, indent=2) + "\n")
    print(suite)


if __name__ == "__main__":
    main()
