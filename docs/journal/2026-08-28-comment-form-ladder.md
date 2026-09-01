---
status: shipped
opened: 2026-08-28
updated: 2026-08-31
beta_skills: [review-guide]
---

# The cheapest form that carries the fact

## Goal

Give `sweep-comments` a rule for where a comment sits. It decided whether a
comment survives and how tersely it is worded, and never decided its form, so
a fact binding one line could sit in a three-line block above it and clear
every rule in the skill.

## Scope

`skills/repository/sweep-comments/SKILL.md` and its corpus. No change to the tiers, the
retrieval test, the model home, the two-pass structure, or the one-context
requirement.

## Evidence

The skill answers two questions and not a third. "The bar: three tiers by
derivation cost" answers whether a fact is stated. "Prefer the fragment"
answers how many words it takes. Nothing answers where it goes, so a tier-3
fact about one line is equally correct as a trailing comment, a line above, a
paragraph on the enclosing function, or a section of the module doc.

The drift is upward and invisible one step at a time. A fact that would have
fit after the line becomes a line above it, then a sentence, then a paragraph
with a lead-in. Each step is defensible; the sequence is not, and no rule in
the skill fires at any point along it.

A second gap sat beside it: nothing said that a fact carried by a rename is
not a comment. A comment whose job is to say that `retry_after` is in
milliseconds is a bug report against the name, and the skill would have
compressed it rather than deleted it.

Pass 1 also had no report format. Pass 2 was required to produce "a written
list of the sites that passed the test and the one sentence each now carries",
with the rule that an empty list is a failed pass. Pass 1, which does the
deleting, reported nothing, so its results could not be checked without
re-reading the diff.

## Design and Implementation

The borrowed idea is the decision ladder from `ponytail`
(https://github.com/DietrichGebert/ponytail), a plugin that has agents stop at
the first applicable rung before writing code — does it need to exist, does it
already exist, is it in the standard library, and so on down to one line. The
structure transfers to comments because the question is the same shape: not
how short can this be, but which is the first form that carries it.

Six rungs, stopping at the first that carries the fact intact: no comment; a
name; trailing on the line it qualifies; one line above; a doc paragraph on
the item; the model home. Rungs three and four are named as where most
comments belong and where few of them sit.

Two guards keep the ladder from becoming a license, and both are borrowed as
well — the source's own non-negotiables are that minimalism never touches
validation, error handling, security, or accessibility.

The first is that **a rung is available only if the fact survives it intact**.
Modality, the subject of a claim, negations, and scope qualifiers do not
compress onto a trailing comment merely because the line has room; when they
will not fit, that rung is not available. This is the fragment section's rule
about wording, restated about placement, and it is stated in both places on
purpose: the reader moving a comment down a rung is not the one who just read
the wording rule.

The second is **be lazy about the form, never about the reading** — the
source's "lazy about the solution, never about reading". A rung is chosen
after the fact is understood, not instead of understanding it. A one-line
comment written to avoid reading the call sites is worse than the paragraph it
replaced, because it is now both wrong and cheap to skim.

Pass 1 gains the source's finding format, adapted: one line per finding giving
file, line, a disposition tag, what the comment said, and what stands there
now. The tags are the dispositions the skill already has — `drop`, `point`,
`shrink`, `inline`, `rename`, `keep` — and the report closes with the net
comment lines removed. An empty result is stated in the same form rather than
by silence, which is the rule pass 2 already carried.

Six red flags name the shapes. Corpus goes from twenty-eight cases to
thirty-two.

## Outcome

Shipped. `cargo test --locked`, `cargo fmt --all -- --check`, and clippy pass;
the citation guard stayed green.

Unmeasured. No sweep has run under the ladder, and the claim that rungs three
and four are where most comments belong is an assertion about a body of code
nobody has counted.

## Derived Documents

None.

## Deferred or Reopen Items

- The source carries an intensity dial — `lite`, `full`, `ultra` — that
  changes how aggressively the ladder is applied. Nothing here has one, and
  whether a comment sweep wants one is unexplored: the argument for is that a
  demo file and a protocol implementation deserve different aggression, and
  the skill already partitions files by bar, which may be the same idea
  arriving from the other direction.
- The source reports benchmark numbers for its ladder. Nothing in this
  repository measures a skill's effect on its output, so every rule here rests
  on judgment, and that gap is older and larger than this change.

## Skill Feedback

### review-guide (beta)

- **Friction** — the published guide asked the reviewer whether an exception to
  the echo test was justified. There was no exception. The duplication it
  described is the carve-out the swept skill already grants: an edit constraint
  is stated at its home and "in one sentence at the site that can break it"
  (`skills/repository/sweep-comments/SKILL.md:206`), and this same rule is already "stated
  in both places on purpose" (`:301`) for the reason the guide gave. The
  reviewer answered by pointing at the rules.

  The instruction that covers this tests what the author knows: settle what you
  can settle, and leave it out of the guide. Nothing tells the author to check
  the document being edited for an answer before asking about it, which is a
  different move from recalling one. The gap is narrow and this is one case:
  reopen on a second, with a form that names where to look rather than adding
  another thing to know.
- **Confirmation** — first use of the rule from
  `2026-08-25-review-guide-proportional.md` on a change that genuinely earns
  sections. The guide carries three headings because three answers needed
  paragraphs, and the reading order and production risk are a clause each. The
  rule did not fight a change that warranted length, which is the failure it
  could plausibly have had. Removing the manufactured question afterwards took
  the guide from 628 words to 562 without touching a heading.

## Appendix: Skills Invoked

- `review-guide` (beta) — this change's pull-request body.
- `engineering-journal` — this entry.

`sweep-comments` was the subject of the change rather than invoked.
