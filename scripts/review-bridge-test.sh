#!/usr/bin/env bash
set -euo pipefail

# An exported REVIEW_AGENT would mask the "missing --as" assertions below.
unset REVIEW_AGENT

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
SKILL="$ROOT_DIR/skills/repository/review-exchange/SKILL.md"
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/review-bridge-test.XXXXXX")

cleanup() { rm -rf "$TMP_DIR"; }
trap cleanup EXIT

fail() { printf 'review-bridge test failed: %s\n' "$1" >&2; exit 1; }
pass() { printf '  ok  %s\n' "$1"; }

# The installer lives inside SKILL.md between marker comments. Extracting it
# here is deliberate: the tested bytes are the shipped bytes.
extract_installer() {
  awk '
    /^<!-- INSTALLER-BEGIN -->$/ { inblock = 1; next }
    /^<!-- INSTALLER-END -->$/   { inblock = 0 }
    inblock && /^```/            { next }
    inblock                      { print }
  ' "$SKILL"
}

make_repo() {
  local dir="$TMP_DIR/$1"
  mkdir -p "$dir"
  git -C "$dir" init --quiet --initial-branch=main
  git -C "$dir" config user.email test@example.com
  git -C "$dir" config user.name 'Test User'
  printf 'seed\n' > "$dir/README.md"
  git -C "$dir" add README.md
  git -C "$dir" commit --quiet -m 'seed'
  printf '%s' "$dir"
}

install_into() {
  ( cd "$1" && bash -s ) < "$TMP_DIR/installer.sh"
}

extract_installer > "$TMP_DIR/installer.sh"
[[ -s "$TMP_DIR/installer.sh" ]] || fail 'installer block is empty or markers are missing'
pass 'installer extracted from SKILL.md'

REPO=$(make_repo fresh)
install_into "$REPO"

[[ -x "$REPO/.git/review-feedback/bin/review-bridge" ]] \
  || fail 'installer did not write an executable bridge script'
pass 'bridge script installed'

[[ "$(cat "$REPO/.git/review-feedback/VERSION")" == "2" ]] \
  || fail 'VERSION is not 2'
pass 'VERSION written'

git -C "$REPO" config --local --get alias.review-note >/dev/null \
  || fail 'alias.review-note not configured'
git -C "$REPO" config --local --get alias.review-feedback >/dev/null \
  || fail 'alias.review-feedback not configured'
pass 'aliases configured'

out=$(cd "$REPO" && git review-feedback --as codex 2>&1)
[[ "$out" == *"no rounds yet on main"* ]] \
  || fail "empty-state message wrong: $out"
pass 'empty state reported'

out=$(cd "$REPO" && git review-feedback 2>&1) && fail 'missing --as should exit non-zero' || true
[[ "$out" == *"--as"* ]] || fail "missing --as error should name the flag: $out"
pass 'missing --as errors clearly'

status=$(git -C "$REPO" status --short)
[[ -z "$status" ]] || fail "installer dirtied the working tree: $status"
pass 'working tree clean'

# Re-running must be a no-op that still verifies.
install_into "$REPO" >/dev/null
pass 'installer is idempotent'

# --- write path ---------------------------------------------------------

cd "$REPO"
git checkout --quiet -b yao/feature-one
printf 'change\n' > file.txt
git add file.txt
git commit --quiet -m 'a change'
HEAD_SHA=$(git rev-parse HEAD)
SHORT=$(git rev-parse --short HEAD)
MERGE_BASE=$(git merge-base HEAD main)

path=$(git review-note --as codex --role reviewer -m 'P1 unchecked index in parse()')
[[ "$path" == *"/threads/yao-feature-one/001-codex-$SHORT.md" ]] \
  || fail "first round path wrong: $path"
pass 'first round numbered 001 with agent and short sha'

grep -q "^round:       1$"               "$path" || fail 'round field wrong'
grep -q "^agent:       codex$"           "$path" || fail 'agent field wrong'
grep -q "^role:        reviewer$"        "$path" || fail 'role field wrong'
grep -q "^branch:      yao/feature-one$" "$path" || fail 'branch field wrong'
grep -q "^base:        main$"            "$path" || fail 'base field wrong'
grep -q "^commit:      $HEAD_SHA$"       "$path" || fail 'commit field wrong'
grep -q "^merge_base:  $MERGE_BASE$"     "$path" || fail 'merge_base field wrong'
grep -q "^written:     20"               "$path" || fail 'written field wrong'
grep -q 'P1 unchecked index'             "$path" || fail 'body missing'
pass 'header derived from git, body preserved'

grep -q "^replies_to:" "$path" && fail 'first round must not carry replies_to' || true
pass 'replies_to omitted on first round'

path2=$(git review-note --as claude --role author --replies-to 1 -m 'fixed in abc1234')
[[ "$path2" == *"/002-claude-$SHORT.md" ]] || fail "second round path wrong: $path2"
grep -q "^replies_to:  1$" "$path2" || fail 'replies_to not recorded'
pass 'second round increments and records replies_to'

out=$(git review-note --as codex 2>&1) && fail 'missing --role should exit non-zero' || true
[[ "$out" == *"--role"* ]] || fail "missing --role error should name the flag: $out"
pass 'missing --role errors clearly'

out=$(git review-note --as codex --role bogus -m x 2>&1) && fail 'bad role should exit non-zero' || true
[[ "$out" == *"bogus"* ]] || fail "bad role error should name the value: $out"
pass 'invalid --role errors clearly'

out=$(git review-note --as codex --role reviewer --base no-such-branch -m x 2>&1) \
  && fail 'unresolvable base should exit non-zero' || true
[[ "$out" == *"no-such-branch"* ]] || fail "base error should name the ref: $out"
pass 'unresolvable base errors clearly'

out=$(printf '   \n' | git review-note --as codex --role reviewer 2>&1) \
  && fail 'empty body should exit non-zero' || true
[[ "$out" == *"empty body"* ]] || fail "empty body error wrong: $out"
pass 'empty body rejected'

printf 'from a file\n' > "$TMP_DIR/body.md"
path3=$(git review-note --as codex --role reviewer -F "$TMP_DIR/body.md")
grep -q 'from a file' "$path3" || fail '-F body not used'
pass '-F reads the body from a file'

status=$(git status --short)
[[ -z "$status" ]] || fail "writing a round dirtied the tree: $status"
pass 'working tree still clean'
cd "$ROOT_DIR"

# --- read path ----------------------------------------------------------

cd "$REPO"

# claude wrote round 2, so its cursor is at 2; round 3 is unread for it.
out=$(git review-feedback --as claude --peek)
[[ "$out" == *"1 new round"* ]]        || fail "peek should report 1 new round: $out"
[[ "$out" == *"003-codex-"* ]]         || fail "peek should show round 3: $out"
[[ "$out" != *"001-codex-"* ]]         || fail "peek must not show read rounds: $out"
pass 'unread rounds reported'

out=$(git review-feedback --as claude --peek)
[[ "$out" == *"1 new round"* ]] || fail "--peek must not advance the cursor: $out"
pass '--peek does not advance'

out=$(git review-feedback --as claude)
[[ "$out" == *"1 new round"* ]] || fail "read should report the unread round: $out"
out=$(git review-feedback --as claude)
[[ "$out" == *"up to date on yao/feature-one"* ]] || fail "up-to-date message wrong: $out"
[[ "$out" == *"3 rounds"* ]]                      || fail "up-to-date should state the total: $out"
[[ "$out" == *"round 3"* ]]                       || fail "up-to-date should name the last round: $out"
pass 'reading advances the cursor and reports up to date'

out=$(git review-feedback --as newcomer --peek)
[[ "$out" == *"3 new rounds"* ]] || fail "an agent with no cursor should see everything: $out"
pass 'missing cursor means everything is unread'

out=$(git review-feedback --as claude --all)
[[ "$out" == *"001-codex-"* ]] || fail "--all should print read rounds too: $out"
[[ "$out" == *"003-codex-"* ]] || fail "--all should print the last round: $out"
pass '--all ignores the cursor'

out=$(git review-feedback --as codex --list)
[[ "$out" == *"yao-feature-one"* ]] || fail "--list should name the thread: $out"
[[ "$out" == *"3 rounds"* ]]        || fail "--list should count rounds: $out"
pass '--list summarises threads'

out=$(git review-feedback --as codex --branch no-thread-here)
[[ "$out" == *"no rounds yet on no-thread-here"* ]] || fail "unknown branch message wrong: $out"
pass 'unknown branch reports no rounds'
cd "$ROOT_DIR"

# --- concurrency --------------------------------------------------------

cd "$REPO"
git review-note --as a --role reviewer -m 'one' >/dev/null &
git review-note --as b --role reviewer -m 'two' >/dev/null &
git review-note --as c --role reviewer -m 'three' >/dev/null &
wait

dir="$REPO/.git/review-feedback/threads/yao-feature-one"
round_names() { for f in "$dir"/[0-9][0-9][0-9]-*.md; do [[ -e "$f" ]] && basename "$f"; done; }
count=$(round_names | wc -l | tr -d ' ')
[[ "$count" == "6" ]] || fail "expected 6 rounds after concurrent writes, got $count"
uniq_nums=$(round_names | cut -d- -f1 | sort -u | wc -l | tr -d ' ')
[[ "$uniq_nums" == "6" ]] || fail "round numbers collided: $uniq_nums distinct of 6"
[[ -z "$(ls -A "$dir"/.claim-* 2>/dev/null || true)" ]] || fail 'a claim directory was left behind'
pass 'concurrent writers get distinct round numbers'
cd "$ROOT_DIR"

# --- upgrade of a legacy repository -------------------------------------

LEGACY=$(make_repo legacy)
mkdir -p "$LEGACY/.git/review-feedback/reviews/yao-old-branch"
printf '# old review\n' > "$LEGACY/.git/review-feedback/reviews/yao-old-branch/2026-08-13T0134-0700.md"
printf '# latest\n'     > "$LEGACY/.git/review-feedback/latest.md"
printf '# readme\n'     > "$LEGACY/.git/review-feedback/README.md"
git -C "$LEGACY" config --local alias.review-feedback '!echo old-alias'

out=$(install_into "$LEGACY")
[[ "$out" == *"legacy one-way reviews left untouched"* ]] \
  || fail "upgrade should report the archive: $out"
[[ "$out" == *"1 file"* ]] || fail "upgrade should count the legacy files: $out"
[[ -f "$LEGACY/.git/review-feedback/latest.md" ]] || fail 'upgrade deleted latest.md'
[[ -f "$LEGACY/.git/review-feedback/reviews/yao-old-branch/2026-08-13T0134-0700.md" ]] \
  || fail 'upgrade deleted a legacy review'
[[ "$(cat "$LEGACY/.git/review-feedback/README.md")" == "# readme" ]] \
  || fail 'upgrade overwrote the legacy README'
pass 'upgrade preserves legacy artifacts'

out=$(cd "$LEGACY" && git review-feedback --as codex --list)
[[ "$out" == *"no threads yet"* ]] || fail "upgraded alias not replaced: $out"
pass 'upgrade replaces the old alias'

[[ "$(cat "$LEGACY/.git/review-feedback/VERSION")" == "2" ]] || fail 'upgrade did not write VERSION'
pass 'upgrade writes VERSION'

printf 'all review-bridge tests passed\n'
