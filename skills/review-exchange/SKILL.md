---
name: review-exchange
description: |
  Exchange threaded review notes with another local agent through a repo-local
  git bridge. Use when asked to "review this branch", "get a second opinion",
  "check the review feedback", "reply to the review", "what did Codex say", or
  whenever two independent agents are reviewing each other's work on the same
  repository. Installs itself into a repository that does not have the bridge
  yet, and upgrades one whose format has drifted. Symptoms that this skill
  applies: a review that arrives as chat text and is lost on the next
  compaction, a finding you disagree with and have nowhere to dispute, a
  reviewer repeating a finding that was already answered, or a repository where
  the other agent's notes have no agreed home.
---

# Review exchange

Two independent local agents trade review notes on a branch. Each note is a
round in a thread; the thread lives in this repository's git directory, so it is
untracked, branch-independent, and shared by every linked worktree.

## Install or upgrade the bridge

Run `git review-feedback --as <your-name> --list` first. If the alias is
missing, or `VERSION` is absent or below 2, offer to run this installer. Ask
before running it; do not install unprompted.

<!-- INSTALLER-BEGIN -->
```sh
#!/usr/bin/env bash
set -euo pipefail

common=$(git rev-parse --git-common-dir) || {
  printf 'not inside a git repository\n' >&2; exit 2; }
store="$(cd "$common" && pwd -P)/review-feedback"

mkdir -p "$store/bin" "$store/threads" "$store/cursors"
printf '2\n' > "$store/VERSION"

cat > "$store/bin/review-bridge" <<'REVIEW_BRIDGE_EOF'
#!/usr/bin/env bash
# review-bridge — threaded review notes between two local agents.
# Written by the review-exchange skill. Do not edit in place; re-run the
# installer to upgrade.
set -euo pipefail

FORMAT_VERSION=2

die() { printf 'review-bridge: %s\n' "$1" >&2; exit 2; }

store_dir() {
  local common abs
  common=$(git rev-parse --git-common-dir 2>/dev/null) \
    || die 'not inside a git repository'
  abs=$(cd "$common" && pwd -P) || die 'cannot resolve the git common directory'
  printf '%s/review-feedback\n' "$abs"
}

slug() { printf '%s\n' "$1" | tr '/' '-'; }

cursor_path() {
  # $1 branch, $2 agent
  printf '%s/cursors/%s/%s\n' "$(store_dir)" "$(slug "$1")" "$2"
}

current_branch() {
  local b
  b=$(git rev-parse --abbrev-ref HEAD 2>/dev/null) || die 'cannot resolve HEAD'
  [ "$b" != HEAD ] || die 'detached HEAD: pass --branch <name>'
  printf '%s\n' "$b"
}

usage() {
  cat >&2 <<'USAGE'
usage:
  git review-note     --as <name> --role reviewer|author
                      [--branch <b>] [--base <b>] [--replies-to <n>]
                      [-F <file> | -m <text>]
  git review-feedback --as <name> [--branch <b>] [--all] [--peek] [--list]
USAGE
  exit 2
}

list_threads() {
  local dir any=0
  for dir in "$(store_dir)"/threads/*/; do
    [ -d "$dir" ] || continue
    any=1
    basename "$dir"
  done
  [ "$any" -eq 1 ] || printf 'no threads yet\n'
}

cmd_feedback() {
  local agent='' branch='' all=0 peek=0 list=0
  while [ $# -gt 0 ]; do
    case "$1" in
      --as)     agent=${2:-}; shift 2 ;;
      --branch) branch=${2:-}; shift 2 ;;
      --all)    all=1; shift ;;
      --peek)   peek=1; shift ;;
      --list)   list=1; shift ;;
      -h|--help) usage ;;
      *) die "unknown option: $1" ;;
    esac
  done
  [ -n "$agent" ] || agent=${REVIEW_AGENT:-}
  [ -n "$agent" ] || die 'missing --as <name> (or set REVIEW_AGENT)'

  if [ "$list" -eq 1 ]; then
    list_threads "$agent"
    return 0
  fi

  [ -n "$branch" ] || branch=$(current_branch)
  local dir
  dir="$(store_dir)/threads/$(slug "$branch")"
  if [ ! -d "$dir" ]; then
    printf 'no rounds yet on %s\n' "$branch"
    return 0
  fi
  printf 'no rounds yet on %s\n' "$branch"
}

case "${1:-}" in
  note)     shift; die 'not implemented yet' ;;
  feedback) shift; cmd_feedback "$@" ;;
  version)  printf '%s\n' "$FORMAT_VERSION"; exit 0 ;;
  *)        usage ;;
esac
REVIEW_BRIDGE_EOF

chmod +x "$store/bin/review-bridge"

git config --local alias.review-note \
  '!f() { d=$(cd "$(git rev-parse --git-common-dir)" && pwd -P); test -x "$d/review-feedback/bin/review-bridge" || { echo "review bridge missing; re-run the review-exchange installer" >&2; exit 2; }; exec "$d/review-feedback/bin/review-bridge" note "$@"; }; f'
git config --local alias.review-feedback \
  '!f() { d=$(cd "$(git rev-parse --git-common-dir)" && pwd -P); test -x "$d/review-feedback/bin/review-bridge" || { echo "review bridge missing; re-run the review-exchange installer" >&2; exit 2; }; exec "$d/review-feedback/bin/review-bridge" feedback "$@"; }; f'

printf 'review bridge installed at %s (format %s)\n' "$store" "$(cat "$store/VERSION")"
if [ -d "$store/reviews" ] || [ -f "$store/latest.md" ]; then
  printf 'legacy one-way reviews left untouched under %s\n' "$store"
fi
```
<!-- INSTALLER-END -->
