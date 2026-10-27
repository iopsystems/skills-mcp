#!/usr/bin/env python3
"""Build a catch-up fixture: OUTPUT [quiet].

OUTPUT/work is the user's checkout on feature/export, last worked in five days
ago. Since then a colleague pushed to main (not yet fetched), an agent session
committed under the user's identity 25 minutes ago, and CI on the branch's PR
is red. OUTPUT/bin/gh is a stub that answers `gh pr view` and `gh pr list`
and logs every call to OUTPUT/gh-calls.log.

With `quiet`, no agent session ran: the newest reflog entry is five days old.
Instead the user pulled one colleague commit from eight days ago at the start
of that session, so a window started at an older reflog gap includes a commit
the user has already seen.
"""
from datetime import datetime, timedelta, timezone
import json
import os
import shlex
from pathlib import Path
import subprocess
import sys

USER = ("Sam Lee", "sam@example.com")
PEER = ("Ada Park", "ada@example.com")
TRAILER = "\n\nCo-Authored-By: Claude <noreply@anthropic.com>\n"
NOW = datetime.now(timezone.utc).replace(microsecond=0)


def git(cwd, *args, who=USER, ago=None):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    env.update(GIT_AUTHOR_NAME=who[0], GIT_AUTHOR_EMAIL=who[1],
               GIT_COMMITTER_NAME=who[0], GIT_COMMITTER_EMAIL=who[1])
    if ago is not None:
        stamp = (NOW - ago).isoformat()
        env.update(GIT_AUTHOR_DATE=stamp, GIT_COMMITTER_DATE=stamp)
    return subprocess.run(["git", *args], cwd=cwd, env=env, check=True,
                          capture_output=True, text=True).stdout


def commit(repo, files, message, **kw):
    for name, text in files.items():
        path = repo / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
        git(repo, "add", name, **kw)
    git(repo, "commit", "-q", "-m", message, **kw)


def main():
    out = Path(sys.argv[1]).resolve()
    quiet = sys.argv[2:] == ["quiet"]
    out.mkdir(parents=True, exist_ok=False)
    origin, work, peer = out / "origin.git", out / "work", out / "peer"
    git(out, "init", "-q", "--bare", "-b", "main", str(origin))

    d = timedelta(days=1)
    h = timedelta(hours=1)
    git(out, "clone", "-q", str(origin), "work", ago=12 * d)
    git(work, "config", "user.name", USER[0])
    git(work, "config", "user.email", USER[1])
    commit(work, {"CLAUDE.md": "Run `cargo test` before pushing.\n",
                  "src/config.rs": "pub struct Config { pub timeout: u64 }\n",
                  "src/export.rs": "pub fn export() {}\n",
                  "README.md": "# ledger\n"}, "Initial layout", ago=12 * d)
    git(work, "push", "-q", "origin", "main", ago=12 * d)
    git(work, "remote", "set-head", "origin", "main")
    git(out, "clone", "-q", str(origin), "peer", who=PEER, ago=12 * d)

    if quiet:
        commit(peer, {"docs/ledger.md": "# Ledger format\n"},
               "Document the ledger format", who=PEER, ago=8 * d)
        git(peer, "push", "-q", "origin", "main", who=PEER, ago=8 * d)
        git(work, "pull", "-q", "--ff-only", ago=5 * d + 4 * h)

    # The user's last session: five days ago.
    git(work, "switch", "-q", "-c", "feature/export", ago=5 * d + 3 * h)
    commit(work, {"src/export.rs": "pub fn export() { /* csv */ }\n"},
           "Write CSV rows from the ledger", ago=5 * d + 2 * h)
    commit(work, {"src/config.rs": "pub struct Config { pub timeout: u64, pub delimiter: char }\n"},
           "Add a delimiter setting", ago=5 * d + h)
    git(work, "push", "-q", "-u", "origin", "feature/export", ago=5 * d + h)
    git(work, "worktree", "add", "-q", "-b", "hotfix/timeout",
        str(out / "work-hotfix"), "main", ago=5 * d)

    # A colleague's work on main, not fetched into work.
    git(peer, "pull", "-q", "--ff-only", who=PEER, ago=4 * d)
    landed = [
        (PEER, {"src/config.rs": "pub struct Settings { pub timeout_ms: u64 }\n"},
         "Rename Config to Settings and timeout to timeout_ms", 4 * d),
        (PEER, {"CLAUDE.md": "Run `./scripts/check.sh` before pushing; it runs clippy and the tests.\n",
                "scripts/check.sh": "#!/bin/sh\ncargo clippy -- -D warnings && cargo test\n"},
         "Require scripts/check.sh before pushing", 3 * d + 5 * h),
        (PEER, {"src/import.rs": "pub fn import() {}\n"}, "Add JSON import", 3 * d),
        (PEER, {"src/import.rs": "pub fn import() { /* json */ }\n"}, "Parse nested accounts in import", 2 * d + 6 * h),
        (USER, {"src/retry.rs": "pub fn retry() {}\n"}, "Retry ledger writes on lock timeout" + TRAILER, 2 * d),
        (PEER, {"docs/import.md": "# Import\n"}, "Document JSON import", 2 * d - 3 * h),
        (PEER, {"tests/import.rs": "#[test] fn t() {}\n"}, "Test nested import", d + 4 * h),
        (PEER, {"README.md": "# ledger\n\nImport and export ledgers.\n"}, "Update README", d),
        (PEER, {".github/workflows/ci.yml": "on: [push]\n"}, "Run CI on every push", 20 * h),
    ]
    for who, files, message, ago in landed:
        commit(peer, files, message, who=who, ago=ago)
    git(peer, "push", "-q", "origin", "main", who=PEER, ago=20 * h)

    # An agent session under the user's identity, 25 minutes ago. No fetch.
    minutes = timedelta(minutes=1)
    stash_ago = 5 * d - 5 * minutes
    if not quiet:
        commit(work, {"src/export.rs": "pub fn export() { /* csv, quoted */ }\n"},
               "Quote fields containing the delimiter" + TRAILER, ago=30 * minutes)
        commit(work, {"tests/export.rs": "#[test] fn quoted() {}\n"},
               "Test quoted export fields" + TRAILER, ago=25 * minutes)
        git(work, "push", "-q", "origin", "feature/export", ago=25 * minutes)
        stash_ago = 20 * minutes
    (work / "src/export.rs").write_text("pub fn export() { /* csv, quoted, bom */ }\n")
    (work / "notes.txt").write_text("try a BOM for Excel\n")
    git(work, "stash", "push", "-q", "-u", "-m", "wip: header row", "--", "notes.txt", ago=stash_ago)

    head = git(work, "rev-parse", "HEAD").strip()
    checks = [
        {"name": "lint", "status": "COMPLETED", "conclusion": "SUCCESS"},
        {"name": "test", "status": "COMPLETED", "conclusion": "FAILURE"},
    ]
    view = {"number": 14, "state": "OPEN", "headRefOid": head,
            "mergeStateStatus": "BLOCKED", "statusCheckRollup": checks}
    prs = [
        {"number": 14, "title": "CSV export", "headRefName": "feature/export",
         "isDraft": False, "reviewDecision": "REVIEW_REQUIRED", "statusCheckRollup": checks},
        {"number": 12, "title": "Lock timeout retry", "headRefName": "hotfix/timeout",
         "isDraft": True, "reviewDecision": "", "statusCheckRollup": [
             {"name": "test", "status": "IN_PROGRESS", "conclusion": ""}]},
    ]
    (out / "gh-view.json").write_text(json.dumps(view))
    (out / "gh-list.json").write_text(json.dumps(prs))
    stub = out / "bin/gh"
    stub.parent.mkdir()
    stub.write_text(f"""#!/bin/sh
echo "gh $*" >> {shlex.quote(str(out / "gh-calls.log"))}
case "$1 $2" in
  "pr view") cat {shlex.quote(str(out / "gh-view.json"))} ;;
  "pr list") cat {shlex.quote(str(out / "gh-list.json"))} ;;
  "pr checks") printf 'lint\\tpass\\ntest\\tfail\\n'; exit 1 ;;
  *) echo "stub gh: unsupported: $*" >&2; exit 1 ;;
esac
""")
    stub.chmod(0o755)
    print(json.dumps({"now": NOW.isoformat(), "head": head}))


if __name__ == "__main__":
    main()
