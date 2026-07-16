#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
BINARY="$ROOT_DIR/target/debug/skills-mcp"
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/skills-mcp-smoke.XXXXXX")
REQUESTS="$TMP_DIR/requests"
RESPONSES="$TMP_DIR/responses.jsonl"
ERRORS="$TMP_DIR/server.stderr"
SERVER_PID=""

cleanup() {
  exec 3>&- 2>/dev/null || true
  if [[ -n "$SERVER_PID" ]] && kill -0 "$SERVER_PID" 2>/dev/null; then
    kill "$SERVER_PID" 2>/dev/null || true
    wait "$SERVER_PID" 2>/dev/null || true
  fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

fail() {
  printf 'MCP smoke test failed: %s\n' "$1" >&2
  if [[ -s "$ERRORS" ]]; then
    printf '%s\n' '--- server stderr ---' >&2
    sed -n '1,120p' "$ERRORS" >&2
  fi
  exit 1
}

sha256_stream() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 | awk '{print $1}'
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum | awk '{print $1}'
  else
    fail "neither shasum nor sha256sum is available"
  fi
}

command -v jq >/dev/null 2>&1 || fail "jq is required"
[[ -x "$BINARY" ]] || fail "missing $BINARY; run cargo build --locked first"

mkfifo "$REQUESTS"
"$BINARY" <"$REQUESTS" >"$RESPONSES" 2>"$ERRORS" &
SERVER_PID=$!
exec 3>"$REQUESTS"

printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"mcp-smoke","version":"0.1.0"}}}' \
  '{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"skill_catalog","arguments":{}}}' \
  '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"skill_template_get","arguments":{"template_id":"document-feature-skill"}}}' >&3

for _ in {1..200}; do
  response_count=$(wc -l <"$RESPONSES" | tr -d ' ')
  [[ "$response_count" -ge 4 ]] && break
  kill -0 "$SERVER_PID" 2>/dev/null || fail "server exited before returning four responses"
  sleep 0.05
done
[[ $(wc -l <"$RESPONSES" | tr -d ' ') -ge 4 ]] || fail "timed out waiting for MCP responses"

exec 3>&-
for _ in {1..100}; do
  kill -0 "$SERVER_PID" 2>/dev/null || break
  sleep 0.05
done
kill -0 "$SERVER_PID" 2>/dev/null && fail "server did not exit after stdin closed"
wait "$SERVER_PID" || fail "server exited unsuccessfully"
SERVER_PID=""

jq -e -s 'map(.id) | index(1) != null and index(2) != null and index(3) != null and index(4) != null' \
  "$RESPONSES" >/dev/null || fail "missing an expected JSON-RPC response"

jq -e -s '
  (map(select(.id == 2))[0].result.tools | map(.name)) as $tools
  | ($tools | index("recommend-skills") != null)
    and ($tools | index("seed-skill-template") != null)
    and ($tools | index("skill_catalog") != null)
    and ($tools | index("skill_template_get") != null)
    and ($tools | index("document-feature-skill") == null)
    and ($tools | index("engineering-journal-skill") == null)
' "$RESPONSES" >/dev/null || fail "tools/list violated the active/programmatic/inert boundary"

jq -e -s '
  map(select(.id == 3))[0].result.content[0].text
  | fromjson
  | [.items[] | select(.kind == "active-skill") | .id] as $active
  | ($active | index("recommend-skills") != null)
    and ($active | index("seed-skill-template") != null)
' "$RESPONSES" >/dev/null || fail "skill_catalog omitted an active adoption skill"

jq -e -s '
  map(select(.id == 4))[0].result.content[0].text
  | fromjson
  | . as $bundle
  | ($bundle.manifest.id == "document-feature-skill")
    and ([.files[].path] | index("SKILL.md") != null)
    and (.aggregate_sha256 | test("^[0-9a-f]{64}$"))
' "$RESPONSES" >/dev/null || fail "skill_template_get returned an invalid bundle"

BUNDLE="$TMP_DIR/bundle.json"
jq -r -s 'map(select(.id == 4))[0].result.content[0].text' "$RESPONSES" >"$BUNDLE"

file_count=$(jq '.files | length' "$BUNDLE")
for ((index = 0; index < file_count; index++)); do
  expected=$(jq -r --argjson index "$index" '.files[$index].sha256' "$BUNDLE")
  actual=$(jq -jr --argjson index "$index" '.files[$index].content' "$BUNDLE" | sha256_stream)
  [[ "$actual" == "$expected" ]] || fail "retrieved file digest mismatch at index $index"
done

manifest_records=$(jq -c '[.manifest.files[] | {path, sha256}] | sort_by(.path)' "$BUNDLE")
returned_records=$(jq -c '[.files[] | {path, sha256}] | sort_by(.path)' "$BUNDLE")
[[ "$returned_records" == "$manifest_records" ]] || fail "manifest and returned file records differ"

expected_aggregate=$(jq -r '.aggregate_sha256' "$BUNDLE")
actual_aggregate=$(
  jq -jr '.files | sort_by(.path)[] | .path, "\u0000", .sha256, "\n"' "$BUNDLE" \
    | sha256_stream
)
[[ "$actual_aggregate" == "$expected_aggregate" ]] || fail "aggregate digest mismatch"

printf 'MCP smoke test passed (%s files, aggregate %s)\n' "$file_count" "$actual_aggregate"
