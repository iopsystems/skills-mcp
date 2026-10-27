---
status: open
opened: 2026-10-22
updated: 2026-10-27
---

# Authors check review findings before applying them

## Goal and Scope

`review-exchange` told an author to give each finding a disposition but not to
check the finding first. A reviewer's finding is a report: its quote can sit at
the wrong line, its counts and behaviors can be wrong, and its proposed
replacement text adds claims nobody checked. This change adds those checks to
Answering, adds the rule to fix every instance of a kind when a finding is one
case of it, and makes the description agree with the installation rule.
Single-PR lifecycle. The installer is byte-identical.

## Changes

- Answering: before applying a finding or reporting it as fact, check at the
  round's `commit` that the quote is at the cited `file:line`, each factual
  statement holds, a proposed replacement is true, and a claimed failure
  reproduces. Dispute a finding only when a check shows its diagnosis is false;
  judge a misplaced quote where it actually is; when only the replacement is
  wrong, apply a corrected one and mark the finding fixed; when a check cannot
  be run, mark the finding deferred and ask the reviewer. When a finding is one
  case of a kind, search the files at the branch head, not only the diff, for
  the rest and fix them all before writing the author round.
- Red flags: applying a finding or its replacement without checking its claims.
- Description: it said the skill "installs itself", while the body says to ask
  before installing. It now says the skill offers to install the bridge and
  installs it only when the user agrees.
- Two trigger evaluation cases: a finding whose quote and replacement are
  wrong, and a finding that is one stale reference among several. The case
  count test moves from 14 to 16.

Words after the frontmatter and outside fenced code blocks (`wc -w`): 421
before, 535 at the first commit, and 605 after the second review's fixes.

## Evaluation

Base: `457b879511af815b94ef504cc8c4cc8f15af95bd`.

One synthetic Git repository per arm, built by the same script: a small Python
fetcher whose branch renames a configuration loader and raises the retry count
from 3 to 5, with the unchanged installer and one seeded reviewer round. The
round has three findings:

1. False: the backoff sleep was removed. It is present, at a different line
   than cited, and the branch changed only the retry constant in that file.
2. True: one stale reference to the old loader name in the README. Three more
   remain in the README and a docstring.
3. True count, false replacement: the proposed README line says backoff starts
   at 200 ms; it starts at 100 ms.

Outcomes were written before either arm ran: dispute finding 1 with evidence
and add no second sleep; fix all four stale references; list the extra fixes;
apply finding 3 without the 200 ms claim and say why; one author round with
`--replies-to 1`, one disposition each, fixed ones citing a commit. The
prediction was that any gap would show in the class fix or the replacement
check, and that a baseline passing both would make the result null.

Two fresh general-purpose agents on the default model each received the same
request ("Codex reviewed my branch. Apply its findings and reply to it.") and
their arm's skill: the base text, or the revised text. The revised arm ran an
earlier draft of the Answering paragraph with the same four checks and the
same class rule; the committed wording is shorter and adds the sentence on
correcting a failed replacement.

A third fresh agent graded both repositories under neutral labels against the
outcomes, reading rounds and Git history but not the skill files, and checked
the ground truth itself at the round's commit.

| Outcome | Base | Revised |
| --- | --- | --- |
| Finding 1 disputed with evidence | pass | pass |
| All four stale references fixed | pass | pass |
| Extra fixes listed | pass | pass |
| Replacement corrected to 100 ms | pass | pass |
| Correction explained | pass | pass |
| Round shape and dispositions | pass | pass |

Both arms scored 6/6. The base arm checked the false finding, found the other
stale references, and rejected the 200 ms figure without the new text. Both
arms also corrected "retries up to five times" to five attempts, which is
four retries. The critic found no new false claim in either round. Neither arm
nor the critic noticed the same miscount in the docstring on line 7 of the
fixture's `fetch.py`; the outcomes did not score it.

This is a null result: one task and one run per arm do not show that the new
text changes behavior for this model. The change is kept because it states
what an author must already do to give a correct disposition, and the
description contradiction was a defect regardless of the evaluation.

Evidence: the [setup script](../evals/setup-review-exchange-verify-eval.sh),
the [frozen outcomes and prompts](../evals/fixtures/review-exchange-verify-findings-v1.md),
and the [results](../evals/results/review-exchange-verify-findings-v1/): the
seeded round, each arm's commits, diff and reply round, and the critic's
report. The critic's report is transcribed from its final message; its full
transcript, and the arms' transcripts, were not kept.

## Deferred or Reopen Items

Reopen with a harder fixture if a weaker model or a longer review round is in
scope: a finding whose false claim cannot be seen from the cited file alone,
or more findings than an author would check by habit.

## Appendix: Skills Invoked

- `review-exchange` — the skill under change, run by both arms.
- `technical-prose` — wording of the added rules.
- `engineering-journal` — this entry.
