---
status: shipped
opened: 2026-09-01
updated: 2026-09-01
---

# Skills grouped by what they act on

## Goal

Put the twenty-four active skills into subdirectories. `skills/` was a flat
list, so the only way to see what a skill was for was to open it or to read the
README's overview, which the directory did not reflect.

## Scope

The skill tree, `load_skills`, the tests and fixtures that named a skill path,
and two README lines. No skill body changed, and no MCP tool name changed —
tool names come from frontmatter, not from paths, so nothing a client sees
moves.

## Evidence

The README already carries a taxonomy, merged in #47 and held by a test since
#50: skills group by where their output lands — the repository, the
`knowledge-iop` vault, or this catalog. Thirteen of the twenty-four need the
vault and are inert without it; nine need only the repository; two act on this
inventory. That split is the first thing a reader needs, and the filesystem
contradicted it by presenting all twenty-four as peers.

The finer sub-groups in the README — diagrams, prose that ships with a change,
paired briefs, parallel inquiry — were considered and not used. Eleven
directories for twenty-four skills puts several at one or two members, and a
second taxonomy that disagrees with the documented one is worse than either
alone. Coarse now, refined when there is a reason.

## Design and Implementation

Three clusters: `repository/` (9), `vault/` (13), `catalog/` (2). A skill is
`skills/<cluster>/<name>/SKILL.md`.

`catalog/` holds two skills, which is small enough to question. It stays
because its membership rule is sharp — the skill's subject is this repository's
own inventory — and because folding it into `repository/` would put "recommend
skills for my project" beside "diagram this codebase", erasing the distinction
a reader most needs.

`load_skills` walked `SKILLS.dirs()`, one level, and `find_skill_md` looked for
a `SKILL.md` directly inside each. A directory without one was `continue`d.
Nesting a skill under that loader would not have failed: it would have vanished
from the tool list while the server started normally and every test passed.

The walk is now recursive, and the skip is gone. A directory holding a
`SKILL.md` is a skill and is not descended into; one holding only directories
is a cluster; one that is neither aborts the load, naming the path. Verified by
planting an empty directory: the server refuses to start with
"repository/orphan-dir holds no SKILL.md and no skill directories: a skill
placed here would be served by nothing."

## Outcome

Shipped. `cargo test --locked`, `cargo fmt --all -- --check`, and clippy pass.
A live `tools/list` returns 31 tools — 24 skills, `skill_catalog`,
`skill_template_get`, and five `vault_*` — with the deepest-nested skills
present.

The move surfaced how much of the repository names skill paths. Seven test-only
`include_str!` paths in `src/main.rs`, eight constants across four test files,
one evaluation fixture, the digest of that fixture recorded in a 2026-07-13
journal entry, and seventy-three citations across the journal. The citation
guard caught the last group rather than a human, which is what it exists for,
and the fixture digest chain caught the fixture edit one step later.

## Derived Documents

None. The README's overview and minimap name skills, not paths, so both stayed
correct; only the two lines that spell out where a skill file lives changed.

## Deferred or Reopen Items

- Nothing enforces that a skill sits in the right cluster. The loader accepts
  any depth, so a vault skill filed under `repository/` loads fine and is wrong
  only to a reader. The README inventory test checks membership of the set, not
  of a group.
- The finer sub-groups remain available if a cluster grows unwieldy. `vault/`
  at thirteen is the one to watch.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
