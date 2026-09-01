---
status: shipped
opened: 2026-08-17
updated: 2026-08-17
beta_skills: [review-guide]
---

# Review guide under the technical-prose bar

## Goal

Hold the guides `review-guide` produces to the word-level bar `technical-prose`
states, and clear the violations in the skill's own text.

## Scope

`skills/repository/review-guide/SKILL.md` and its corpus. No change to the publish test,
the attention ranking, the certainty rules, the TL;DR, or the routing block.

## Evidence

`review-guide` already named `technical-prose` as the owner of word choice, but
in one clause inside the Boundaries section, next to the `sweep-comments`
pointer. A pointer that names an owner is not a requirement, and nothing said
which of that skill's rules matter for a pull-request body or what happens when
they meet evidence a guide quotes.

Scanning the skill's own prose against the same bar found three violations: a
possibility written as `might` where `can` is correct, and the deleted-word
entries `easily` and `just`.

## Design and Implementation

Three rules do most of the work in a pull-request body, and the reference now
names them rather than delegating wholesale. Modality, because a judgment call
written with `should` is a different claim and the reviewer who treats it as
optional has read the author correctly. Vocabulary that carries no fact, because
"gracefully handles" in a testing row names a quality of the handling instead of
the handling, which is the failure the Testing section exists to prevent. One
name per thing, because a guide that renames a component between sections
destroys the association each section was building.

One interaction belongs to this pairing alone and is now stated. A guide quotes
evidence — real command output in Testing, a requirement sentence in the
certainty section, a flag or a path in the reading order — and those are
untouchable under `technical-prose`'s own rule. A word-level pass that edits a
pasted result or a quoted error has broken the evidence it was cleaning. The two
skills agree on this, but neither said so where an agent applying both would
read it.

The naming rule caught the skill itself. It calls its output both "the guide"
and "the body". A full rename was rejected as churn against a live document; the
skill instead declares the two terms as one artifact where the output is
defined. That satisfies what the rule protects — a reader knowing the two are
one thing — without a diff across every section.

Corpus goes from twenty-six cases to twenty-eight: one requiring modality to
survive in a judgment call, one requiring quoted evidence to stay byte-identical
while the prose around it is cleaned.

## Outcome

Shipped. `cargo fmt --check` passed, the suite passed with no failures, and the
citation guard stayed green.

Whether the bar changes what guides read like is unmeasured. The three
violations found in the skill's own text are the only evidence so far that the
rules bind anything, and the skill was written by the same author who wrote the
bar.

## Derived Documents

None.

## Deferred or Reopen Items

- The guide/body naming compromise. Reopen if a reader reports the two terms
  reading as two artifacts, at which point the rename stops being churn.
- Nothing checks a produced guide against the prose bar. The rules are stated in
  one skill and applied by an agent reading both, with no mechanism, which is
  the same shape as the citation rule before `tests/citations.rs` existed.

## Skill Feedback

### review-guide (beta)

- **Friction** — asked to apply `technical-prose` to this skill. Its reference
  to that skill was one clause in a boundaries list, which is enough to assign
  ownership and not enough to bind anything. A skill that defers by naming
  another skill leaves the caller to guess which rules apply and what happens
  when two skills disagree.
- **Confirmation** — running the bar over the skill's own prose found three real
  violations in text written by the author of the bar, which is the cheapest
  available evidence that the rules catch something rather than describing what
  a careful writer already does.

## Appendix: Skills Invoked

- `technical-prose` — the word-level bar, invoked as a tool and applied both to
  this skill's prose and to the requirement it now places on produced guides.
- `review-guide` (beta) — used to draft this change's own pull-request body,
  followed from its source file rather than invoked as a tool, because this
  effort was editing that file.
