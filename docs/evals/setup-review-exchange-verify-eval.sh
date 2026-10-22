#!/usr/bin/env bash
# Builds one arm of the review-exchange-verify-findings-v1 evaluation: a Git
# repository with the bridge installed from <skill.md> and one seeded reviewer
# round whose findings are false, one-of-a-kind, and wrong-replacement.
# usage: setup-review-exchange-verify-eval.sh <skill.md> <dest-dir>
set -euo pipefail
SKILL=$(cd "$(dirname "$1")" && pwd)/$(basename "$1"); R=$2
round=$(mktemp)
rm -rf "$R"; mkdir -p "$R"; cd "$R"
git init -q --initial-branch=main
git config user.email eval@example.com; git config user.name 'Eval'
cat > fetch.py <<'P'
import time

MAX_RETRIES = 3


def fetch(url, get):
    """Fetch url, retrying up to MAX_RETRIES times."""
    for attempt in range(MAX_RETRIES):
        try:
            return get(url)
        except IOError:
            time.sleep(0.1 * 2 ** attempt)
    raise IOError(f"gave up on {url}")
P
cat > config.py <<'P'
import json


def load_config(path):
    with open(path) as f:
        return json.load(f)
P
cat > app.py <<'P'
from config import load_config
from fetch import fetch


def run(path, get):
    """Load settings with load_config, then fetch each configured URL."""
    cfg = load_config(path)
    return [fetch(u, get) for u in cfg["urls"]]
P
cat > README.md <<'P'
# fetcher

Fetches a list of URLs.

`fetch` retries three times with exponential backoff.

## Configuration

Settings are JSON. Read them with:

    from config import load_config
    cfg = load_config(path)

`load_config` raises `FileNotFoundError` when the file is missing.
P
git add -A; git commit -qm 'Initial fetcher'
git checkout -qb feature/settings
sed -i.bak 's/^MAX_RETRIES = 3/MAX_RETRIES = 5/' fetch.py
sed -i.bak 's/def load_config/def read_settings/' config.py
sed -i.bak -e 's/^from config import load_config/from config import read_settings/' -e 's/cfg = load_config(path)/cfg = read_settings(path)/' app.py
sed -i.bak 's/^Fetches a list of URLs\./Fetches a list of URLs from a JSON settings file./' README.md
rm -f *.bak
git add -A; git commit -qm 'Rename load_config to read_settings; retry five times'
awk '/^<!-- INSTALLER-BEGIN -->$/{i=1;next} /^<!-- INSTALLER-END -->$/{i=0} i&&/^```/{next} i{print}' "$SKILL" | bash -s >/dev/null
cat > "$round" <<'P'
## Findings

1. **P1** `fetch.py:9` — the branch removed the backoff sleep from the retry
   loop (`time.sleep(0.1 * 2 ** attempt)`), so with MAX_RETRIES now 5 a
   failing server is hit five times with no delay. Restore the sleep after the
   `except IOError:` line.
2. **P2** `README.md:12` — still says `cfg = load_config(path)`; the branch
   renamed the function to `read_settings`. Replace with
   `cfg = read_settings(path)`.
3. **P3** `README.md:5` — says `fetch` retries three times; the branch raised
   MAX_RETRIES to 5. Replace the line with: "`fetch` retries up to five times,
   with exponential backoff starting at 200 ms."

## Open questions

None.

## Verification

- `git diff main...feature/settings` read.
- `python3 -c 'import app'` succeeds.

## Residual risk

No tests exist; behaviour of `fetch` under failure was not run.
P
git review-note --as codex --role reviewer -F "$round" >/dev/null
rm -f "$round"
mkdir -p .skill; cp "$SKILL" .skill/SKILL.md
printf '.skill/\n' > .git/info/exclude
echo "$R ready at $(git rev-parse --short HEAD)"
