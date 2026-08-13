---
status: shipped
opened: 2026-08-12
updated: 2026-08-12
---

# Skill use in journal entries

## Goal

Give the engineering journal two new obligations: record friction and
confirmation for any skill that declares itself beta, and close every entry with
a roster of the skills invoked during the effort. Both exist to feed the
cross-repository survey planned as roadmap Stage 3, which today has no evidence
to harvest.

## Decision Criteria

Ship when both records are mechanically findable rather than prose-only, when
the rules make a fabricated record harder to write than an honest one, and when
the active skill and the inert template carry the same contract in their own
registers. `cargo fmt --check`, the full suite, a release build, and an MCP
list/call smoke test must pass.

## Scope

The journal skill and its template gain the two sections, the frontmatter key,
the reconciliation checks, and the profile fields that let a seeded instance opt
in. Out of scope: a structured `maturity:` field on skills, and the survey tool
that will eventually read these sections.

## Evidence

Three facts constrained the design:

- Beta is prose today, not a field. `skills/architecture-diagram/SKILL.md:30`
  instructs the reader to "treat this skill as **beta**", and
  `templates/architecture-diagram-skill/template.yaml:4` opens its purpose with
  "Beta —". No `maturity:` key exists anywhere in the repository.
- The consumer does not exist yet. `docs/roadmap.md` Stage 3 describes authorized
  surveys that compare instances and report recurring customizations, and the
  matching backlog items under "Later: Adoption and evolution" are unchecked.
  What lands here is the contract that survey will read, not an integration with
  it.
- The active skill and the template have diverged in register. The template
  routes every convention through `references/project-profile.md` and carries a
  trust-and-execution boundary the active skill omits, so the same contract had
  to be written twice rather than copied.

## Design and Implementation

Beta stays self-declared. The journal treats a skill as beta when the skill's own
instruction text or template manifest says so, or when the user or the project
profile says so, and never infers immaturity from one bad result. This keeps the
change inside the journal skill and leaves the designation with the skill that
knows its own maturity.

The harvest contract is a fixed `## Skill Feedback` heading with one `###`
subsection per beta skill, plus `beta_skills: [<name>]` in frontmatter. The
frontmatter key lets a survey select candidate entries without parsing prose;
the heading gives it a stable place to read once selected. Reconciliation now
checks that the two agree and that every skill named in the section also appears
in the roster.

`## Appendix: Skills Invoked` is the roster the feedback section annotates rather
than a second, parallel list. Beta members carry `(beta)` in both places, so the
join key is the skill name and the two features cannot drift into duplication.

The rules that matter most are the honesty rules, because the failure mode here
is silent. An agent closing an entry after a compaction or a handoff can produce
a plausible roster from nothing, and a survey cannot distinguish a reconstructed
roster from an observed one. The skill therefore requires that an incomplete
record be stated in one line instead of inferred, that updates append rather than
rewrite the roster down to the current session, that every friction name what was
asked and which instruction misfired and what was done instead, and that
reconciliation never backfill entries predating the convention. Feedback is
advisory in the same sense as the existing brief-input reporting: it does not
edit the beta skill, open an issue, or send anything upstream.

Confirmation is recorded alongside friction. A beta skill needs evidence that its
defaults survive contact; a channel that collects only complaints would bias the
promotion decision it exists to inform.

Artifacts:

- `skills/engineering-journal/SKILL.md` — a `Record Skill Use` section, the
  frontmatter key, two entry headings, an update obligation, and three
  reconciliation checks with the no-backfill prohibition
- `templates/engineering-journal-skill/SKILL.md` — the same contract in the
  project-contract register, gated on the profile's policy
- `templates/engineering-journal-skill/references/project-profile.md` — a
  skill-feedback policy field and a project-declared beta list
- `skills/engineering-journal/evals/trigger-evals.json`, ten cases to fifteen,
  adding friction, confirmation, the no-beta-skill case, incomplete history, and
  the no-backfill boundary
- `templates/engineering-journal-skill/evals/trigger-evals.json`, seven cases to
  nine in that corpus's activation and prohibition shape
- the eval count in `src/main.rs`, and refreshed digests in
  `templates/engineering-journal-skill/template.yaml`

The template manifest keeps `version: 0.1.0` and updates digests only, matching
the precedent set by `ca324cb` for the diagram templates.

## Outcome

Shipped on branch `yao/journal-skill-use`. `cargo fmt --check` passed, 136 tests
passed across the seven test binaries, `cargo build --release` succeeded, and a
raw JSON-RPC smoke test confirmed that `tools/call` on `engineering-journal`
returns an 8,749-character body containing `## Record Skill Use`, `beta_skills`,
and `## Appendix: Skills Invoked`.

This entry carries no `## Skill Feedback` section because no beta skill was
invoked, which is the intended behavior rather than an omission. The first real
beta-skill record will come from an effort that actually uses
`architecture-diagram`.

The eval corpora assert intended behavior. Neither corpus has been forward-tested
against independent agents, the same limitation recorded for `technical-prose`.

## Derived Documents

None. `docs/roadmap.md` Stage 3 and the "Later: Adoption and evolution" backlog
items already describe the survey this feeds, and neither changes shape because
the evidence source now exists.

## Deferred or Reopen Items

- The survey that reads these sections. Reopen when Stage 3 work starts; the
  frontmatter key and the fixed heading are the interface it should assume.
- A structured `maturity:` field on skills and template manifests. Deferred
  because self-declaration already works for the one beta skill in the
  repository. Reopen when a second beta skill appears, or when the survey needs
  to enumerate beta skills without reading every body.
- Independent evaluation of both trigger corpora.

## Appendix: Skills Invoked

- `superpowers:brainstorming` — context exploration, the three design questions,
  and the design-approval gate before any file was edited.

The `engineering-journal` skill was followed from its source file rather than
invoked as a tool, because this effort was editing that file.
