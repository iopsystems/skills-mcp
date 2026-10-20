---
name: sharable-artifact
description: |
  Build or revise a page a team reads on its own — a design preview, a status
  board, a reference on a format or a subsystem — as one self-contained HTML
  file published to a stable URL and republished in place. Use when asked to
  "write this up as a page", "make a design page", "publish a status board",
  "put the reference somewhere the team can read it", or when a write-up
  exists only in chat or in a file nobody will open. Beta — the conventions
  come from a handful of pages in one team; record friction through
  `engineering-journal`. Symptoms that this skill applies: a page whose
  sections renumber by hand, a figure cited as "below", a status row that says
  "fixed" when only the author says so, a number with no revision it was taken
  on, a figure whose labels overflow in the render, or a page that is white on
  white in dark mode.
---

# Sharable artifact

Build a page that can be read without its author: claims carry their evidence,
elements have stable citations, and revisions keep the page current.

## Scope and companion skills

These conventions held across a few design and reference pages for one team.
They have not been tried with other audiences; reading-time and word-count
defaults are that team's. Treat the skill as **beta**: adopt its shape, recording
overrides and their reasons through `engineering-journal` when a convention
conflicts with the page.

Run `technical-prose` before publishing, with the reader named. Use
`dataflow-diagram` for figures of running programs and `format-layout-diagram`
for byte layouts. Those skills govern figure claims; inline figures use the
page's color tokens.

## One file, one URL, one source

Keep one HTML file with inline CSS and SVG, requiring no build step. Republish
that source to the same stable URL with a short label per version. Do not
maintain a second copy in a repository, doc tool, or chat message; it becomes a
second source of truth that drifts on the first change.

Give the page a distinctive two-to-four-word name. Put the explainer in the
publish description, never in the title.

## Shape: a short paper

Use a header, parts by audience, numbered sections, numbered figures and tables,
and a footer of evidence and revision notes.

**Header.** Include an eyebrow with scope and assumptions (repository, packages,
plan), the name, a two-sentence problem statement, and a status sentence linking
the progress board when present.

**Reading guide.** After the introduction, give each part a navigation column:
linked title, reading time ("~12 min"), one sentence on its audience and purpose,
and numbered section links. Add a five-minute quick path through the summary,
key figures and table, and decision. Omit a total time and advisory sentence;
the reader decides from the summary.

**Parts, one per audience.** Order parts by how little time each reader has.
A design page defaults to:

- *Design*, for assessing the architecture: summary, options, each structure's
  high-level design, layering, the main consumer's use, public contract and
  guarantees, signals and health, and invariants.
- *Migration*, for planning the work: existing limitations, what the change
  enables, plan, implementation considerations, and decision summary.
- *Implementation details*, for writing or reviewing code.

The first two parts use figures, tables, and short prose, totaling under about
2,500 words. Where a mechanism is detailed in the third part, end the earlier
section with a pointer naming the destination's number and title. A reference
page with no work to plan drops the progress board and migration part; keep the
rest.

**Section numbering.** Use one section element per numbered section and draw
its number with a CSS counter. Part dividers are not sections and consume no
number.

**Footer, in order.** Evidence baseline (survey commits, branches, external
pull requests); one dated paragraph per revision explaining what changed and
why; authorship (who wrote what).

## Every element is citable

Give every section, figure, table, and listing a stable id. Show its anchor
beside the title as `[#id]` in small monospace, so citations survive renumbering.

- Use `fig-`, `table-`, and `listing-` prefixes; a part-specific prefix for
  deep-dive sections; plain words for other sections.
- Number figures, tables, and listings by hand in their captions, in order
  across the whole page. Figure captions go below; table captions above.
- Refer by linked number: "Figure 4", "Table 3", "Section 6". Never use "above",
  "below", or "the following" as references.
- Highlight a shared link's target when opened and keep it clear of the top edge.

## Every number names its baseline

A number without a revision cannot be re-derived.

- Link each pull request on its number. In status rows, follow it with the
  commit at which the row's claims were checked, or the merge date.
- Cite source as `path:line` in code font. State which commit the line numbers
  belong to in the containing paragraph or table. Durable links use
  commit-pinned blobs with full hashes, never moving branch links.
- Give survey facts and measurements their branch and commit.
- Use code font for identifiers, flags, channel names, and paths, including
  table cells and SVG text.

## Figures are inline SVG with the page's tokens

Use hand- or generator-authored SVG with a viewBox inside a scrolling wrapper,
so narrow screens scroll the figure rather than the page. Give it an image
role and an accessible label describing the relationships, including arrows.
The caption states the same claim more briefly, as a conclusion. Name each
panel with a small uppercase monospace label at its top.

Use page color tokens consistently across figures and nearby prose. Inline
figures follow the theme; figures exported as images bake their ink. Define
arrowhead markers per figure in the current color or a token. Put a legend
under a figure only when its encoding repeats across figures, with swatches
using the tokens they name.

**After every figure edit, measure label fit in a real renderer.** Confirm the
page's fonts are loaded first; a fallback face has different measurements.
Check each text element against its containing shape, free labels against
unrelated boxes, text against text, and everything against the viewBox. SVG
text does not wrap. Character-count estimates have missed overflows found by
rendered checks. Fix overflow, crossing, and out-of-bounds findings before
publishing.

## Tables, cards, and listings

- Put table captions first. Use small uppercase column headers with
  letter-spacing. Layer-colored cells use figure tokens; yes/no matrices use
  dots rather than words. Wide tables scroll within their own wrappers.
- First-part tables keep property columns: what the thing is, what the reader
  sees, and the owning layer. Target three or four columns. Move derivation,
  carriage, construction, and verification columns to a twin table in the
  implementation part, repeating row labels. Link that twin from the first
  table's caption.
- Use three summary cards, each with a role eyebrow (Problem, Proposal,
  Enforcement), heading, and one paragraph. Its top-border color identifies
  the role in the page's palette.
- Treat a listing as a figure containing a preformatted block. Apply three
  spans by hand: comments, introduced names, and hidden or elided material.
  Do not use a syntax highlighter; highlighting conveys the argument.

## The progress board

For pages tracking work, put a board in the header with columns for goal,
what it delivers, pull request with checked-at commit, and state.

Start each state cell with a bold word from the board's stated vocabulary:
*merged*, *approved*, *awaiting re-review*, *ready*, *draft*, *not started*.
Keep author-addressed findings (*awaiting re-review*) distinct from reviewer
confirmation (*approved*). Color the state word by row class. Stamp the board
with the date and main-branch commit it was checked against.

For each status edit:

1. Update the row's pull-request and state cells.
2. Set its row class.
3. Update the board's stamp.
4. Append a dated footer paragraph explaining the change.

Make all four changes together. Re-read every row against live pull-request
state before publishing.

## Theme and type

- Define the complete light palette on the root element. Redefine tokens for
  dark mode under both the system-preference query and the explicit dark
  attribute. Guard the system query so explicit light wins; explicit dark
  must also override system light. The unstamped state follows the system.
  Set the body's background from a token. Never define a color only inside a
  theme block.
- Use one semantic hue per kind of thing: public contract, hidden
  implementation, problem, signals and health, or not built yet. Give each
  hue a fill twin for boxes. Use it consistently in figures, cells, and cards.
  Bias neutrals slightly toward the accent rather than pure gray.
- Use fonts from a host allowed by the publishing platform, each with a
  system fallback: condensed display face, text around 17px at 1.5 line
  height, and monospace for code, anchors, and eyebrows.
- Use one column at a readable maximum width, sections in grids with a fixed
  gap, and phone side gutters. Collapse cards and guide columns to one column
  on narrow screens. Prevent horizontal page scrolling.

## Before publishing

Check the page against the structure, citation, figure, and theme requirements
above. Then:

1. Run `technical-prose` with the reader named: an engineer on the team who has
   not read the plan. The page ships alone.
2. After any figure edit, complete the rendered fit check and repair findings.
3. Re-read progress rows against live pull-request state.
4. Verify both theme states render with the root light palette, both dark
   blocks, and an explicit body background.
5. Re-read the live published version and merge onto it to preserve teammates'
   intervening edits.
6. Publish to the same URL with a version label.

Use `review-exchange` for another agent's review, supplying a local Markdown
render so the review addresses text rather than markup. Before adopting another
agent's prose pass, compare the versions: preformatted blocks and table rows
must be byte-identical, decision-critical terms retained, per-section word
counts checked, the resulting figures checked for fit, and attribution retained.
