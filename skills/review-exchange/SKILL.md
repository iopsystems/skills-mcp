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
  # Counts written rounds and in-flight claims alike. A claim that is not yet a
  # file must still reserve its number, or two writers racing under different
  # agent names would each believe the number is free — their filenames differ,
  # so no file-level exclusion would catch it.
  local dir=$1 max=0 f base num
  for f in "$dir"/[0-9][0-9][0-9]-*.md "$dir"/.claim-[0-9][0-9][0-9]; do
    [ -e "$f" ] || continue
    base=${f##*/}
    base=${base#.claim-}
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

get_cursor() {
  # $1 branch, $2 agent — prints 0 when no cursor exists
  local path v
  path=$(cursor_path "$1" "$2")
  if [ -r "$path" ]; then
    v=$(cat "$path" 2>/dev/null || printf '0')
    case "$v" in
      ''|*[!0-9]*) printf '0\n' ;;
      *)           printf '%s\n' "$v" ;;
    esac
  else
    printf '0\n'
  fi
}

round_number_of() {
  local base=${1##*/}
  printf '%s\n' "$((10#${base%%-*}))"
}

agent_of() {
  local base=${1##*/} rest
  rest=${base#*-}
  printf '%s\n' "${rest%-*}"
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

  local dir path claim n attempt=0
  dir="$(store_dir)/threads/$(slug "$branch")"
  mkdir -p "$dir"
  while : ; do
    n=$(next_round "$dir")
    claim=$(printf '%s/.claim-%03d' "$dir" "$n")
    # mkdir is atomic and fails if the name exists, so exactly one racing
    # writer wins a given round number.
    if mkdir "$claim" 2>/dev/null; then break; fi
    attempt=$((attempt + 1))
    [ "$attempt" -lt 10 ] || die 'could not claim a round number after 10 attempts'
  done
  path=$(printf '%s/%03d-%s-%s.md' "$dir" "$n" "$agent" "$short")

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
  rmdir "$claim" 2>/dev/null || true

  set_cursor "$branch" "$agent" "$n"
  printf '%s\n' "$path"
}

list_threads() {
  local agent=$1 dir name total unread from f n any=0
  for dir in "$(store_dir)"/threads/*/; do
    [ -d "$dir" ] || continue
    name=$(basename "$dir")
    total=0
    unread=0
    from=$(get_cursor "$name" "$agent")
    for f in "$dir"[0-9][0-9][0-9]-*.md; do
      [ -e "$f" ] || continue
      total=$((total + 1))
      n=$(round_number_of "$f")
      if [ "$n" -gt "$from" ]; then unread=$((unread + 1)); fi
    done
    [ "$total" -gt 0 ] || continue
    any=1
    printf '%s — %d rounds, %d unread\n' "$name" "$total" "$unread"
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

  local rounds=() f
  for f in "$dir"/[0-9][0-9][0-9]-*.md; do
    [ -e "$f" ] || continue
    rounds+=("$f")
  done
  if [ "${#rounds[@]}" -eq 0 ]; then
    printf 'no rounds yet on %s\n' "$branch"
    return 0
  fi

  local total=${#rounds[@]}
  local last=${rounds[$((total - 1))]}
  local from=0
  if [ "$all" -eq 0 ]; then
    from=$(get_cursor "$branch" "$agent")
  fi

  local show=() n
  for f in "${rounds[@]}"; do
    n=$(round_number_of "$f")
    if [ "$n" -gt "$from" ]; then show+=("$f"); fi
  done

  if [ "${#show[@]}" -eq 0 ]; then
    printf 'up to date on %s (%d rounds, last: %s round %s)\n' \
      "$branch" "$total" "$(agent_of "$last")" "$(round_number_of "$last")"
    return 0
  fi

  local noun=rounds
  if [ "${#show[@]}" -eq 1 ]; then noun=round; fi
  printf '%d new %s on %s since you last read:\n\n' "${#show[@]}" "$noun" "$branch"
  for f in "${show[@]}"; do
    printf -- '--- %s ---\n' "$(basename "$f" .md)"
    cat "$f"
    printf '\n'
  done

  if [ "$peek" -eq 0 ]; then
    set_cursor "$branch" "$agent" "$(round_number_of "$last")"
  fi
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
  legacy_count=0
  if [ -d "$store/reviews" ]; then
    legacy_count=$(find "$store/reviews" -name '*.md' -type f | wc -l | tr -d ' ')
  fi
  printf 'legacy one-way reviews left untouched under %s (%s file(s))\n' \
    "$store/reviews" "$legacy_count"
fi
```
<!-- INSTALLER-END -->

## Read before you write

Run `git review-feedback --as <your-name>` before writing anything. A round
written without reading the unread ones repeats findings that were already
answered, which is the failure the thread exists to prevent.

`--peek` shows the unread rounds without marking them read. Use it when you are
only checking whether there is work, not doing it.

## Reviewing

Scope the review at the merge base against the base branch, not at the branch
tip. You are reviewing what would merge.

Write one round with `--role reviewer`:

- **Findings**, in severity order. Each cites `file:line` and states the failure
  concretely — the input or state, and the wrong result. A finding a reader
  cannot act on is not a finding.
- **Open questions**, where the change is defensible either way and you need the
  author's intent.
- **Verification**: the commands you ran and what they returned. Name the ones
  that failed as plainly as the ones that passed.
- **Residual risk**: what you could not check.

An empty Findings section means you found nothing actionable. It does not mean
the change is defect-free, and it is not evidence of correctness. Say so.

## Answering

Write one round with `--role author` and `--replies-to <n>`.

Every finding in the round you are answering gets exactly one disposition:

- **fixed** — cite the commit that fixed it.
- **disputed** — cite the evidence that the finding is wrong or does not apply.
  A dispute without evidence is a refusal.
- **deferred** — state why, and what would reopen it.

Silence is not a disposition. A finding you skip will come back next round, and
the reviewer will not know whether you disagreed or missed it.

Then answer the open questions, and list anything else you changed that no
finding asked for.

## Convergence

A thread converges when a reviewer round carries no findings and no open
questions. Either agent may observe that; neither may declare it on the other's
behalf. If you are the author and you believe the thread is done, say so and
leave it open — the reviewer's round is what closes it.

## What this is not

The store lives in `.git/`. It is untracked, shared across linked worktrees, and
destroyed by a fresh clone. It is a working channel, not a record.

Anything that must outlive the branch goes somewhere durable: the pull-request
body through `review-guide`, or an entry through `engineering-journal`. Do not
commit the thread into the repository.

## Red flags

- Writing a round without reading the unread ones.
- Producing findings because the last round was empty and an empty round felt
  like a failure. It is not.
- Marking a finding fixed with no commit that fixes it.
- Disputing a finding by restating the original code.
- Reporting "no findings" as evidence the change is correct.
- Guessing an agent name because none was supplied.
