---
name: open-arc
description: |
  Open a new **arc** — a narrative unit of change in the knowledge-iop vault that spans weeks to quarters. Arcs have a beginning, middle, and end. Use this when the user says things like "let's start an arc for X", "open the Y arc", "we're kicking off a project on Z", "this deserves its own arc". Arcs group briefs, inquiries, and decisions under a shared charter — the situation, the tension, the direction of resolution. If the effort is permanent (not going to close), use `define-scope` instead. If it's a one-off, maybe it just needs a brief.
---

# Open arc

You are opening a new **arc** in the knowledge-iop vault. An arc is
a narrative unit of change — it opens, progresses through briefs
and inquiries, and closes (or is abandoned). Spans weeks to
quarters. Operates within one or more scopes.

## Before opening — check the alternatives

1. Invoke `vault-search` with `type: arc`, `status: open` to see
   what's already in flight. An arc should be a distinguishable
   narrative, not overlap another open arc.
2. Check the alternatives:

| If the thing you're about to open is... | Do this instead |
|---|---|
| A permanent reference frame | `define-scope` |
| A single brief with no follow-on | Just `frame-problem` + `propose-design` |
| A sub-effort of an existing open arc | Extend that arc; don't spawn |
| A decision that's already obvious | `record-decision` |

Arcs without multiple expected artifacts are overhead. If the
answer to "what will this arc contain?" is one brief, skip the arc.

## Frontmatter shape

```yaml
---
id:         <short-slug>                 # e.g. "vault-bootstrap", "billing-refactor"
type:       arc
status:     open
opened:     <YYYY-MM-DD>
closed:     null
leads:      [<person>, ...]
scopes:     [<scope-id>, ...]            # one or more; required
tags:       [<tag>, ...]
author:     <person>
created:    <YYYY-MM-DD>
---
```

`id:` is a plain slug. Unlike briefs, arcs aren't dated in their id
— the `opened:` field handles timing.

`scopes:` is required. An arc that doesn't touch any scope is
either improperly scoped or needs a new scope opened first.

## Filename

Write to `arcs/<id>.md`.

## Body — the charter

The body of an arc is a **charter** — amendable over the arc's life,
but not superseded. Cover:

1. **The situation.** What's going on that warrants opening this
   arc.
2. **The tension.** What's not working, or what opportunity is
   newly available. The arc exists because of a gap between current
   and intended state.
3. **Direction of resolution.** Where you expect this to go. Not a
   promise — a trajectory. If you're wrong, amend the charter.
4. **Expected artifacts.** Roughly what will land here — briefs,
   inquiries, decisions. Gives the arc shape without committing.
5. **Closing conditions.** What has to be true to close this arc
   as `closed` (vs `abandoned`). Be concrete. "Reasonable progress"
   isn't a closing condition; "X service is live and Y decision is
   recorded" is.
6. **Scopes touched.** One paragraph each if the arc crosses
   several scopes, explaining how.

Arcs aren't roadmaps. Don't include a Gantt chart or commit dates;
the vault isn't a project tracker.

## Drafting workflow

1. Search for overlapping open arcs. Confirm the alternatives don't
   apply.
2. Ask the user for leads, scopes, and closing conditions. Push
   back on "we'll know when we get there" — vague closes are how
   arcs drift into permanence.
3. If any named scope doesn't exist in the vault, chain to
   `define-scope` first.
4. Propose frontmatter + body as a diff.
5. Write on approval.
6. Commit: `git add arcs/<id>.md && git commit -m "Open arc:
   <id>"`.

## Amending a charter

An arc's charter is amendable as the narrative unfolds:

1. Edit the file. No supersession.
2. Commit: `git commit -m "Arc <id>: <what changed>"`.

If amendments start pushing the arc away from its original
trajectory and closing conditions, consider whether the arc should
be closed and a successor opened — that's visible in history, which
is the point.

## Sub-arcs are not a thing

Per SCHEMA.md: "Sub-arcs are not a thing. If an arc grows too large,
close it and spawn successors with explicit handoff."

Closure with handoff: write a decision or session note explaining
what closes, what opens, and why.

## Do not

- Do not open an arc without closing conditions. Unbounded arcs
  become permanent, which defeats the point.
- Do not open an arc in scopes that don't exist yet. Open the
  scopes first.
- Do not open an arc as a container for a single brief. The brief
  alone is enough.
