---
status: shipped
opened: 2026-08-14
updated: 2026-08-14
prs: [32]
beta_skills: [review-guide]
---

# Citation guard

## Goal

Stop line-number citations from becoming quietly false. Four wrong citations
had shipped across two pull requests, and the repository had no mechanism that
could tell a correct citation from a stale one.

## Decision Criteria

Ship when a shifted citation fails the build, a citation whose anchor moved can
be repaired mechanically, a citation whose anchor is gone always fails, and the
guard costs no new language, dependency, or CI step. The full suite,
`cargo fmt --check`, `clippy -D warnings`, and the existing scripts must pass.

## Scope

One integration test. One amendment to `review-guide`. Anchor phrases added to
three existing journal entries so they become checkable. No change to citation
style beyond anchoring, and no checking of prose claims against cited text.

## Evidence

The effort began on the assumption that the repository held stale citations. It
does not. All five line citations in tracked markdown resolved correctly when
checked by hand before any code was written:

- `src/main.rs:38` embeds `skills/` through `include_dir!`.
- `skills/architecture-diagram/SKILL.md:31` reads "treat this skill as
  **beta**".
- `skills/architecture-diagram/SKILL.md:27` ("principles carried into a domain")
  and `:38` ("use `dataflow-diagram` directly") are the two deferral precedents.
- `Cargo.toml:6` declares "MIT OR Apache-2.0".

That finding reframed the work. **A repository linter would have caught none of
the four failures that motivated it.** All four were in pull-request bodies —
three in change 26, one in change 30 — which are composed in a scratch file and
posted through `gh`, and are never in the repository. The tool first proposed in
conversation was aimed at the wrong target.

Two distinct problems remained, and they need different mechanisms.

<!-- cite-ignore -->
The drafting failure is what actually happened: line numbers were inferred
rather than read. In change 30 four anchors were read and the fifth estimated,
which produced `:194` for a line reading `done`.

The durability failure has not happened yet but is one edit away: editing
`skills/architecture-diagram/SKILL.md` would have silently invalidated three
journal entries, and nothing would have noticed.

## Design and Implementation

The user chose line-plus-phrase citation with a self-healing checker over
phrase-only citation, and chose a discipline rule over a citation-generating
helper for pull-request bodies.

`tests/citations.rs` scans tracked markdown under `docs/`, `skills/`,
`templates/`, and `README.md`, dropping fenced code blocks. For each
`` `path:line` `` citation it collects the enclosing paragraph, takes the
backticked and double-quoted spans as candidate anchors, and requires one to
appear verbatim at the cited line.

<!-- cite-ignore -->
A bare `` `:38` `` inherits the path from the previous citation in its
paragraph, an idiom two entries already use.

When no anchor sits at the cited line, the checker relocates on the nearest candidate
that is long enough to be evidence and occurs exactly once in the file;
`CITATIONS_FIX=1` writes the repair. A missing anchor fails in both modes,
because nothing can be re-derived from a phrase that no longer exists.

The first design was Python, rejected as a new dependency. Bash was the obvious
fallback, but paragraph segmentation and phrase extraction there would be worse
code for no gain. The Rust integration test costs nothing new: `tests/` already
holds six content-validation suites, `walkdir` is already a dependency, and
`cargo test --locked` already runs in CI, so no workflow change was needed. The
self-healing mode survives as an environment gate, the pattern `insta` and
`expect-test` use.

### Five defects the checker found in itself

Two surfaced when it was first run against the existing corpus, two more when
this entry was added to that corpus, and one when the branch was rebased onto
the review-exchange entry. None would have been caught by the unit tests written
alongside, because each depended on the shape of real prose — which is the
general lesson: every defect here was found by pointing the checker at text
somebody actually wrote, and none by reasoning about it.

<!-- cite-ignore -->
The first version relocated on any anchor occurring exactly once, and
immediately proposed rewriting `src/main.rs:38` **to line 1** — a seven-character
token happened to occur once near the top of the file. Confirming that a phrase
sits at a cited line is safe with a short phrase; moving a citation on the
strength of one is not. A minimum length now gates relocation only, not
verification.

The second version pooled anchors across sibling bullets, offering
`templates/document-feature-skill/` as a candidate anchor for a `src/main.rs`
citation. Markdown list items carry no blank line between them, so splitting
paragraphs on blank lines alone merges an entire list into one context.
Detecting list-item starts separates them.

The third was a panic. The relocation search used `then_some`, which evaluates
its argument even when the condition is false, so it indexed an empty vector the
first time an anchor appeared nowhere in the target file. No citation in the
original corpus had a missing anchor; this entry introduced one.

The fourth was the sharpest, because it would have made the convention
impractical rather than merely wrong. Prose here wraps at eighty columns, so a
quoted anchor phrase routinely spans two lines and carries a newline plus
indentation that the cited line does not have. Substring matching failed on
every wrapped anchor. Both the anchor and the candidate line are now normalized
to single-spaced text before comparison.

The fifth appeared on rebasing onto the merged review-exchange entry, whose
Skill Feedback section cites two line numbers to illustrate a wrong citation
rather than to claim anything about a file. Marking that bullet did not suppress
it: a marker line is neither blank nor a list-item start, so it was absorbed
into the tail of the preceding bullet and set no flag. The marker now always
stands alone, whatever surrounds it.

### Verification beyond the unit tests

Unit tests do not show that the guard works on the real corpus. A two-line
insertion into `skills/architecture-diagram/SKILL.md` was used to drive the full
cycle: the check flagged five citations across three journal entries and
identified every shifted reference; `CITATIONS_FIX=1` rewrote all five to their
new lines; restoring the source and repairing again returned them to `30`, `27`,
`37`, `27`, `37`. The working tree after the sequence was byte-identical to
before it, which is the property that makes the fixer safe to run.

### The rule half

`review-guide` now states that citation form follows how long the document
lives. A pull-request body, read this week against one commit, cites
`path:line` and names the commit the lines are pinned to. A durable document
cites `path:line` plus a quoted phrase from that line, so the number stays
re-derivable. Added alongside it: every cited line is read at the moment it is
cited, stated as an absolute, with partial verification named as the failure
mode — because that is what happened, not a hypothetical.

Migrating the five existing citations required adding an anchor phrase to three
entries. None of the five was wrong; each was one edit away from being wrong
with nothing to catch it. One illustrative citation in `review-guide`, which
demonstrates citation style rather than claiming anything about the cited file,
carries a `<!-- cite-ignore -->` marker.

## Outcome

Shipped in pull request 32. `tests/citations.rs` adds eleven tests — ten unit plus the corpus check — bringing
`cargo test --locked` to eight green test binaries. `cargo fmt --all
--check`, `clippy --all-targets --locked -D warnings`,
`./scripts/review-bridge-test.sh`, and `./scripts/mcp-smoke.sh` all pass. No CI
workflow change was required.

The guard covers only tracked markdown. It cannot see a pull-request body, which
is where every observed failure occurred; that gap is closed by a rule, and
rules are not enforced. Phrase matching is substring matching: it verifies that
the text is present, not that the claim about it is true. The fixer has been
exercised on one injected shift of one kind, not on renames, deletions, or a
file rewritten wholesale.

## Derived Documents

None. `docs/backlog.md` and `docs/roadmap.md` are unaffected. Three journal
entries were edited to add anchor phrases; no claim in them changed.

## Deferred or Reopen Items

- `MIN_RELOCATE_LEN` is set to 8 by judgment, not measurement. It decides when a
  coincidence is unlikely enough to rewrite a durable record automatically.
  Reopen if a `CITATIONS_FIX=1` run ever produces a diff nobody expected.
- No pre-post lint of a drafted pull-request body against a commit. It was
  offered and not taken in favor of the discipline rule; it remains the obvious
  next move if the rule proves insufficient. The measurable signal is another
  wrong citation shipping in a body.
- The guard does not check that a claim matches its cited text, only that the
  anchor is present. A citation can be perfectly anchored to a line that does
  not support the sentence around it.
- Editing merged journal entries to add anchors is a judgment about a durable
  record. It was disclosed in change 32 rather than assumed to be permitted.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — The amendment made in this effort was tested by the effort
  itself. Drafting the body for change 32 under the new rule, reading every
  cited line rather than most, caught two errors before posting: a section range
  ending one section too late, and a list of seven numbers under a claim of five
  citations. Both are the exact defect class the rule targets, and both were
  found by following it.
- **Confirmation** — The publish test cleared on all four items and produced a
  full guide, while the same assessment run against the journal entry in change
  33 cleared on one item and correctly produced two paragraphs instead. The
  proportionality rule distinguished the two without a size threshold.
- **Friction** — None new. The friction that prompted this effort is recorded
  against change 30 in the review-exchange bridge entry.

## Appendix: Skills Invoked

- `review-guide` (beta) — drafted the pull-request body for change 32.
- `engineering-journal` — this entry.
