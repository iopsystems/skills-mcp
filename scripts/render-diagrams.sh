#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
DOT_SOURCE="$ROOT_DIR/docs/skill-feedback-loop.dot"
SVG_TARGET="$ROOT_DIR/docs/skill-feedback-loop.svg"
MODE=render

if [ "${1-}" = "--check" ]; then
  MODE=check
elif [ "$#" -ne 0 ]; then
  printf 'usage: %s [--check]\n' "$0" >&2
  exit 2
fi

command -v dot >/dev/null 2>&1 || {
  printf 'render-diagrams: Graphviz dot is required\n' >&2
  exit 1
}

sha256_file() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    printf 'render-diagrams: shasum or sha256sum is required\n' >&2
    exit 1
  fi
}

TMP_SVG=$(mktemp "$ROOT_DIR/docs/.skill-feedback-loop.svg.XXXXXX")
TMP_MARKED=$(mktemp "$ROOT_DIR/docs/.skill-feedback-loop.marked.svg.XXXXXX")
cleanup() {
  rm -f "$TMP_SVG" "$TMP_MARKED"
}
trap cleanup EXIT HUP INT TERM

DIGEST=$(sha256_file "$DOT_SOURCE")
dot -Tsvg "$DOT_SOURCE" -o "$TMP_SVG"
awk -v marker="<!-- source-sha256: $DIGEST -->" '
  { print }
  NR == 1 { print marker }
' "$TMP_SVG" >"$TMP_MARKED"

if [ "$MODE" = check ]; then
  [ -f "$SVG_TARGET" ] || {
    printf 'render-diagrams: missing %s\n' "$SVG_TARGET" >&2
    exit 1
  }
  grep -Fx "<!-- source-sha256: $DIGEST -->" "$SVG_TARGET" >/dev/null || {
    printf 'render-diagrams: source digest marker is stale\n' >&2
    exit 1
  }
  printf 'render-diagrams: SVG is current (%s)\n' "$DIGEST"
else
  chmod 0644 "$TMP_MARKED"
  mv "$TMP_MARKED" "$SVG_TARGET"
  printf 'render-diagrams: rendered %s (%s)\n' "$SVG_TARGET" "$DIGEST"
fi
