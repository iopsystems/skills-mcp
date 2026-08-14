#!/usr/bin/env bash
set -euo pipefail

# An exported REVIEW_AGENT would mask the "missing --as" assertions below.
unset REVIEW_AGENT

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
SKILL="$ROOT_DIR/skills/review-exchange/SKILL.md"
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

printf 'all review-bridge tests passed\n'
