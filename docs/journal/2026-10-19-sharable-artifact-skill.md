---
status: open
opened: 2026-10-19
updated: 2026-10-19
---

# Sharable artifact: a page a team reads alone

## Goal and Scope

Draft an active skill for pages published as one self-contained HTML
artifact: design previews, status boards, reference pages. The conventions
come from a format summary written after the Vessel Data Movement design page
(`~/workspace/notes/artifact-page-format.md`, 2026-10-20) and two memories it
references: the standing prose pass on every artifact, and the rendered
label-fit check. No tool surface changes.

## Evidence

The format summary records what held across one team's design page and was
revised each time a reader said what they could not find: numbered sections
by CSS counter with anchors shown beside titles, figure and table numbers by
hand across the page, references by number and never "below", commit-pinned
citations with checked-at commits beside pull-request links, inline SVG on the
page's tokens with a rendered label-fit check, a progress board whose state
vocabulary keeps the author's claim distinct from the reviewer's confirmation,
a footer of evidence baseline and dated revisions, and a two-block dark theme
with every color on a token.

The memories add the reader-named prose pass as a standing step, the part
outline for a design page (design, migration, implementation details, the
first two under about 2,500 words), the reading guide without a total time or
an advisory sentence, the first-part table rule, and the comparison before
adopting another agent's prose pass.

A second page, the JFR reference page, used the same shape without a progress
board or a migration part, which is the reference-page variant the skill
names.

### Baseline

One scenario, a status-page day with an author-claimed fix, a figure edit, and
an unpinned measurement, was answered by a fresh agent with no skill and by a
fresh agent with the draft. The outcomes are recorded under Validation.

## Design

`skills/repository/sharable-artifact/SKILL.md` states the shape as a recipe:
one file republished to one URL; a short-paper structure with parts by
audience; every element citable by a shown anchor; every number with its
baseline; figures inlined on the page's tokens with measured label fit;
first-part tables kept to property columns; a progress board with a fixed
state vocabulary and a four-touch status edit; both theme blocks; a
before-publishing sequence; a ten-item checklist; red flags.

The skill defers words to `technical-prose`, figure claims to the diagram
skills (whose embedding rule it depends on), and agent review to
`review-exchange`. It is marked beta: the reading-time, word-count, and
three-part defaults are one team's.

Sixteen scenarios cover triggering, a chat summary that stays in chat, each
convention's failure, the reference-page variant, and the beta override.

## Decision Criteria

An agent updating a page must keep the author's claim distinct from the
reviewer's confirmation, pin every number and citation to a revision, measure
label fit in a renderer after a figure edit, make the four status touches
together, and run the prose pass before publishing.

## Validation and Outcome

Baseline, no skill: the agent kept the author's claim distinct from the
reviewer's sign-off, qualified the measurement with a date, branch, and
"single run", added a footer changelog line, and said it would open the page in
both themes. It gave the measurement no commit, put no checked-at commit beside
the pull-request link, had no as-of stamp, judged the edited figure by eye
rather than measuring label fit, said nothing about token colors or the
figure's accessible label, and ran no prose pass.

Re-run, with the skill: the agent named the state as awaiting re-review from
the fixed vocabulary, made the four touches together, re-read every other row
against live state, drew the new box on the page's tokens with the figure's
own markers, updated the accessible label and caption, ran the label-fit
check with fonts confirmed loaded, pinned the measurement to branch, commit,
and date and added that to the evidence baseline, ran the prose pass with the
reader named, checked both theme states, and republished with a label. One
thing the control did that the skill does not say: re-read the live page
source before republishing so teammates' edits are not clobbered.

Local CI on the branch: fmt, clippy, the test suite including the README
inventory check at 30 entries, the diagram freshness check, the MCP smoke
test, the bridge test, and the release build.

The scenario corpus specifies expected behavior; one fresh-agent run per arm
is a wording check, not a behavioral evaluation.

## Deferred Work

- Try the shape on a page for a non-engineering audience before treating the
  three-part default as more than one team's.
- The label-fit harness is described, not shipped; a reusable script would
  belong beside the skill if a second repository needs it.
- Measure whether the reading-time estimates in the guide are read.

## Appendix: Skills Invoked

- `superpowers:writing-skills` — baseline before relying on the draft, recipe
  form, conditionals on observable predicates.
- `technical-prose` — the skill text.
- `engineering-journal` — this record.
