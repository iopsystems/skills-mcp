---
name: define-scope
description: |
  Define a new **scope** — a stable reference frame (service, domain, surface, cross-cutting concern) that the vault cares about for years rather than the duration of a narrative arc. Use this when the user says things like "we need a scope for X", "add a scope", "register the billing service", "document this as a domain we care about". Scopes are flat (no hierarchy), they don't have arcs' narrative lifecycle, and they retire rather than close. Don't create a scope lightly — scope proliferation is a smell. If it's a short-lived effort, use `open-arc` instead.
---

# Define scope

You are defining a **scope** in the knowledge-iop vault. Scopes are
the vault's stable reference frames — the named areas the vault
cares about for years. A service (`billing`), a domain
(`observability`), a surface (`public-api`), a cross-cutting concern
(`reliability`).

## Before defining — check you actually want a scope

1. Invoke `vault-search` with `type: scope` and keywords from the
   proposed name. Check for existing scopes that overlap.
2. Check the four gotchas below; if any applies, redirect:

| If the thing you're about to define is... | Do this instead |
|---|---|
| A short-lived effort (weeks–quarters) | `open-arc` |
| A sub-area of an existing scope | Extend the scope's body, don't create a child |
| A dependency you don't control | Add to `upstream_deps:` on an existing scope |
| A specific topic being discussed now | `discuss` and see what survives |

Scopes should accumulate slowly. If the vault gains a new scope
every week, something is miscategorized.

## Frontmatter shape

```yaml
---
id:              <short-slug>                # e.g. "billing", "public-api"
type:            scope
status:          active
established:     <YYYY-MM-DD>
retired:         null
owners:          [<person>, ...]
repos:           [<repo-ref>, ...]           # the grounding hook
services:        [<service-name>, ...]       # optional
upstream_deps:   [<dep>, ...]                # optional; external deps
related_scopes:  [<scope-id>, ...]           # flat — no hierarchy
tags:            [<tag>, ...]
author:          <person>
created:         <YYYY-MM-DD>
---
```

`id:` is a plain slug, not dated — scopes don't carry creation dates
in their id because they're meant to outlast any given moment.
Choose carefully; renaming is expensive.

`repos:` is the grounding hook — the scope exists in part because
real code lives in real repos. If you can't name repos, ask whether
this is truly a scope or just a conversation topic.

## Filename

Write to `scopes/<id>.md`.

## Body — the living description

A scope's body is a **living description** that gets amended (not
superseded) as the scope evolves. Cover:

1. **What this scope is.** One paragraph that a newcomer could use
   to decide whether a question or artifact belongs here.
2. **What it contains.** Systems, services, repos, docs — the
   concrete territory.
3. **What it doesn't.** Adjacent things that might confuse. Name
   them and link to their scopes.
4. **Interfaces.** How this scope relates to others — upstream
   dependencies, downstream consumers, handoffs.
5. **Ownership and health.** Who owns what. Where to find runbooks
   or oncall info (or note their absence).
6. **Norms.** Anything specific about how work happens in this
   scope that a visitor should know (review norms, deploy cadence,
   SLA expectations).

Scopes are flat. `auth` is not a sub-scope of `security`. Use
`related_scopes:` to note the adjacency, but don't encode a tree.

## Drafting workflow

1. Search; confirm no existing scope covers this.
2. Ask the user to name owners and repos. Push back on empty owner
   lists — an unowned scope is a smell.
3. Propose frontmatter + body as a diff.
4. Write on approval.
5. Commit: `git add scopes/<id>.md && git commit -m "Scope:
   <id>"`.

## Amending a scope

Scope bodies are living. When the scope grows, shrinks, or shifts:

1. Edit the file directly.
2. Commit with a clear message: `git commit -m "Scope <id>:
   <what changed>"`.

Do not `supersede` a scope unless its identity genuinely changed
(e.g. a rename from one slug to another).

## Retiring a scope

When a scope is no longer relevant:

1. Update frontmatter: `status: retired`, `retired: <today>`.
2. Add a closing paragraph at the bottom of the body: when, why,
   what took its place (if anything).
3. Commit.

Retired scopes stay in the vault. Grep still finds them. History
is honest.

## Do not

- Do not nest scopes. The flat list is the point.
- Do not create a scope you don't plan to maintain. Unowned
  scopes rot fastest.
- Do not supersede a scope to rename it casually. Think about
  whether the old id is load-bearing for references elsewhere
  first.
