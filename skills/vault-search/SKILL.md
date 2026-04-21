---
name: vault-search
description: |
  Query the knowledge-iop vault as a habit before drafting anything. Before you write a new problem-brief, design-brief, inquiry, decision, discussion, scope, or arc, first check whether a relevant artifact already exists so you can supersede, relate to, or frame from it — rather than orphan your work. Use this whenever the user says things like "what do we know about X", "is there already a brief on Y", "show me the design for Z", "find related artifacts", "what's the status of...", or before any phase skill (frame-problem, propose-design, record-decision, ...). The underlying programmatic tools are vault_search, vault_edges, and vault_check_transition — this skill teaches when to reach for each.
---

# Vault search

You have three programmatic MCP tools available for reading the
knowledge-iop vault. They are fast, deterministic, and query a SQLite
index maintained by iop-skills. Use them instead of `grep` or `find`
whenever you can.

## The tools

### `vault_search` — find artifacts

```jsonc
// inputs (at least one filter required; empty filters return []):
{
  "type":    "design-brief",         // optional; see list below
  "status":  "draft",                 // optional
  "author":  "alex",                  // optional
  "topic":   "rate limiting",         // optional; FTS5 over title + body
  "limit":   50                       // optional; default 50, max 500
}
```

Artifact types: `scope`, `arc`, `problem-brief`, `design-brief`,
`decision`, `discussion`, `session-note`, `inquiry`, `exploration`,
`synthesis`, `claim`, `schema`.

Returns `{ count, matches: [{ id, type, status, author, created, path, title }] }`.

### `vault_edges` — walk the graph around an artifact

```jsonc
{
  "id":        "2026-04-17-vault-design",
  "kind":      "frames",               // optional; filter by edge kind
  "direction": "both"                  // outgoing | incoming | both (default both)
}
```

Edge kinds: `frames`, `supersedes`, `superseded_by`, `relates_to`,
`conflicts_with`, `depends_on`, `derived_from`, `arc`, `scopes`, `inquiry`.

Returns `{ id, count, edges: [{ from_id, to_id, kind, neighbor }] }`. Each
edge carries the neighbor artifact's metadata, so one call is usually
enough.

### `vault_check_transition` — "can this be accepted?"

```jsonc
{ "id": "2026-04-17-vault-design", "new_status": "accepted" }
```

Returns `{ allowed, blockers: [...], warnings: [...] }`. Use before
proposing a status change, not as a general search.

## When to use each

| You want to... | Tool | Example |
|---|---|---|
| Find artifacts about a topic | `vault_search` with `topic` | Before drafting any brief |
| List all of one type/status | `vault_search` with `type`/`status` | "Show me draft design-briefs" |
| Find what frames / is framed by X | `vault_edges` with `kind: frames` | Problem → design lookup |
| Find what supersedes X | `vault_edges` with `kind: superseded_by`, `direction: outgoing` | Pipeline walk |
| Find everything touching X | `vault_edges` with no `kind` | Impact analysis |
| Check if a status change is legal | `vault_check_transition` | Before accepting a decision |

## How to use this skill

When a phase skill (frame-problem, propose-design, record-decision, ...)
or the user's request tells you to "check the vault first", do this:

1. **Extract keywords** from the topic or artifact the user is working on
   (service names, problem nouns, technology names).
2. **Call `vault_search` with `topic`** first. If results are thin, try
   with `type` to widen.
3. **If you find a candidate match**, call `vault_edges` on its id to see
   what's already connected (does it have a design-brief? is it
   superseded? what arcs / scopes touch it?).
4. **Report back concisely**: matches + gaps. A phase skill will use the
   result to decide whether to draft anew, supersede an old artifact, or
   redirect the user to an existing one.

## Reporting format

When returning findings, use this structure so callers can parse it:

```
MATCHES (<n>)
- id: <artifact-id>
  type: <type>    status: <status>    created: <YYYY-MM-DD>
  path: <vault-relative path>
  title: <first heading>
  connected:
    - <kind> → <neighbor id> (<neighbor status>)

GAPS
- <what's missing that would matter here>

BLOCKERS (if vault_check_transition was called)
- <rule>: <message> — offenders: <ids>
```

Gaps are as important as matches. A phase skill deciding whether to
chain to another skill needs to know *what's missing*, not just what
exists.

## Do not

- Do not modify the vault from this skill. Writes are the job of phase
  skills (`frame-problem`, `propose-design`, ...).
- Do not call `vault_reindex` unless something is visibly stale — the
  index auto-refreshes on each query when files change.
- Do not grep / find in the vault directory unless one of these tools
  has failed. The index is the fast path.
