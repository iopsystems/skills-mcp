---
status: shipped
opened: 2026-08-25
updated: 2026-08-25
prs: [47, 48]
beta_skills: [review-guide]
---

# A README the library fits in

## Goal

Make the README say what the skill library holds. It named five of the
twenty-four active skills and two of the four templates, so a reader could not
learn the inventory from the document that exists to introduce it.

## Scope

`README.md`, and the `architecture-diagram-skill` manifest purpose that the
overview surfaced as stale. No change to any skill body, the catalog schema, or
the tests.

## Evidence

The "Current adoption surfaces" table carried three rows. Active skills were
`recommend-skills`, `seed-skill-template`, `engineering-journal`, "plus inquiry
and vault workflows"; their shared purpose was "Invocable shared workflows".
Inert templates were `document-feature-skill` and `engineering-journal-skill` —
the two diagram templates had been added without reaching the table.

Two facts a reader needs were absent. Thirteen of the twenty-four active skills
read and write the `knowledge-iop` vault and are inert without it; eleven do
not. And three of the four templates have an active twin, so seeding one is a
choice rather than the only way to use that workflow.

## Design and Implementation

The table is replaced by a section grouped first on where a skill's output
lands — the repository, the vault, or this catalog — and then on what it
produces. The first axis is the one a reader acts on: it decides which entries
apply to them at all. Templates are the fourth group because a template is
adopted rather than invoked.

A plain-text minimap sits above that section, naming every entry with no
description. The two answer different questions: the map answers *what is here*,
which needs twenty-eight names in view at once, and the section answers *what is
this one for*, which needs a line of purpose each. The map is plain text rather
than a rendered diagram: the audience charter requires a textual equivalent
beside every diagram, and for a list of names that equivalent is the artifact,
so an SVG would be a picture of text carrying a second file and a freshness
check.

Coverage is checked by script rather than by eye — every skill and template
appears exactly once, and no line passes 75 columns.

## Outcome

Shipped. The overview landed in #47, merged as `5ff5884`; the minimap and this
entry landed in #48, merged as `41023d1`.

The charter gates visual-hierarchy and navigation changes on maintainer review,
with the evidence recorded here. The maintainer merged both pull requests, which
is that approval.

The guide for #48 went through three drafts before it was proportionate to a
32-line addition: 733 words, then 293, then 216. Only the last was published
before the merge.

## Derived Documents

None.

## Deferred or Reopen Items

- Nothing checks the minimap against `skills/` and `templates/`. A skill added
  under `skills/` reaches `skill_catalog` and nothing else, and the map goes
  quietly wrong — the failure mode three skills in this repository exist to
  prevent. The natural home is a test beside the citation guard, which already
  scans `README.md` for a different kind of decay.
- The prose overview has the same exposure and no script behind it at all.

## Skill Feedback

### review-guide (beta)

- **Friction** — asked for the pull-request body of a one-file, additive,
  32-line README change, the skill produced 733 words. Two instructions pull
  against each other. "What you publish is proportional to the change" and "a
  one-file mechanical edit earns a sentence" set the target; the body section
  then lists seven always-present headings and permits dropping one only when
  "the change genuinely has nothing in it". Each heading draws at least a short
  paragraph, so the floor sits well above what a small change warrants. The
  mental model cost ninety-four words explaining a split the reviewer had merged
  two commits earlier — and the skill's own one-line escape, stating the assumed
  starting point, was available and unused. Rewritten to 293 words by dropping
  the mental model and folding the reading order into testing.
- **Friction** — "one subsection per decision that wants the reviewer, each
  laying out its context before it asks its question" has a fixed cost per item:
  context, reference, question. An item worth one clause — whether the map's
  group labels should be abbreviated to hold 75 columns — became five lines and
  an italic question, and Decisions reached 323 words, forty-four percent of the
  body. Demoted to the calls-recorded-rather-than-asked list on the rewrite. The
  skill orders decisions by centrality but does not say that an item too small to
  carry a subsection belongs in that list rather than getting one.
- **Confirmation** — the publish test was right to say publish. Two items
  cleared it, a judgment call that cuts against the repository's own diagram
  convention and a test gap, and both survived the rewrite unchanged. The defect
  was the length of the guide, not the decision to write one.
- **Confirmation** — writing the reading-order section is what surfaced the
  finding that the map is hand-maintained and unchecked, which is now the
  deferred item above. The section earned its place by producing something the
  reviewer could not have read off the diff.

## Appendix: Skills Invoked

- `review-guide` (beta) — the pull-request bodies for #47 and #48.
- `engineering-journal` — this entry.

The roster covers the README effort only. The ringline ingest that shared #47
has its own entry.
