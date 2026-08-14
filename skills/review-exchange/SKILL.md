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

next_round() {
  local dir=$1 max=0 f base num
  for f in "$dir"/[0-9][0-9][0-9]-*.md; do
    [ -e "$f" ] || continue
    base=${f##*/}
    num=${base%%-*}
    num=$((10#$num))
    if [ "$num" -gt "$max" ]; then max=$num; fi
  done
  printf '%s\n' "$((max + 1))"
}

set_cursor() {
  # $1 branch, $2 agent, $3 round number
  local path
  path=$(cursor_path "$1" "$2")
  mkdir -p "$(dirname "$path")"
  printf '%s\n' "$3" > "$path"
}

cmd_note() {
  local agent='' role='' branch='' base='' replies_to='' body_file='' message='' have_message=0
  while [ $# -gt 0 ]; do
    case "$1" in
      --as)         agent=${2:-}; shift 2 ;;
      --role)       role=${2:-}; shift 2 ;;
      --branch)     branch=${2:-}; shift 2 ;;
      --base)       base=${2:-}; shift 2 ;;
      --replies-to) replies_to=${2:-}; shift 2 ;;
      -F)           body_file=${2:-}; shift 2 ;;
      -m)           message=${2:-}; have_message=1; shift 2 ;;
      -h|--help)    usage ;;
      *) die "unknown option: $1" ;;
    esac
  done

  [ -n "$agent" ] || agent=${REVIEW_AGENT:-}
  [ -n "$agent" ] || die 'missing --as <name> (or set REVIEW_AGENT)'
  case "$role" in
    reviewer|author) ;;
    '') die 'missing --role reviewer|author' ;;
    *)  die "invalid --role: $role (expected reviewer or author)" ;;
  esac

  [ -n "$branch" ] || branch=$(current_branch)
  if [ -z "$base" ]; then
    base=$(git config review.base 2>/dev/null || true)
    [ -n "$base" ] || base=main
  fi

  local commit merge_base short
  commit=$(git rev-parse --verify --quiet "$branch^{commit}") \
    || die "cannot resolve branch: $branch"
  merge_base=$(git merge-base "$commit" "$base" 2>/dev/null) \
    || die "cannot find the merge base of $branch and $base"
  short=$(git rev-parse --short "$commit")

  local body
  if [ -n "$body_file" ]; then
    [ -f "$body_file" ] || die "no such file: $body_file"
    body=$(cat "$body_file")
  elif [ "$have_message" -eq 1 ]; then
    body=$message
  else
    body=$(cat)
  fi
  [ -n "${body//[[:space:]]/}" ] || die 'empty body'

  local dir path n attempt=0
  dir="$(store_dir)/threads/$(slug "$branch")"
  mkdir -p "$dir"
  while : ; do
    n=$(next_round "$dir")
    path=$(printf '%s/%03d-%s-%s.md' "$dir" "$n" "$agent" "$short")
    if (set -C; : > "$path") 2>/dev/null; then break; fi
    attempt=$((attempt + 1))
    [ "$attempt" -lt 10 ] || die 'could not claim a round number after 10 attempts'
  done

  {
    printf -- '---\n'
    printf 'round:       %d\n' "$n"
    printf 'agent:       %s\n' "$agent"
    printf 'role:        %s\n' "$role"
    printf 'branch:      %s\n' "$branch"
    printf 'base:        %s\n' "$base"
    printf 'commit:      %s\n' "$commit"
    printf 'merge_base:  %s\n' "$merge_base"
    if [ -n "$replies_to" ]; then
      printf 'replies_to:  %s\n' "$replies_to"
    fi
    printf 'written:     %s\n' "$(date +%Y-%m-%dT%H:%M:%S%z)"
    printf -- '---\n\n'
    printf '%s\n' "$body"
  } > "$path"

  set_cursor "$branch" "$agent" "$n"
  printf '%s\n' "$path"
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
  note)     shift; cmd_note "$@" ;;
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
