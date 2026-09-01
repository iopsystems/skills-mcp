---
status: shipped
opened: 2026-09-01
updated: 2026-09-01
---

# The heavy skills were dense, not padded

## Goal

Apply the prose ladder to the three skills the survey named after `review-guide`
— `sweep-comments`, `dataflow-diagram`, `architecture-diagram` — which together
hold 12,840 words and fifteen paragraphs over eighty.

## Scope

Those three, plus the two diagram templates that mirror them byte for byte, plus
three stale cross-references the ladder's own merge created. No rule was removed
from any skill.

## Evidence

The result is the finding. **119 words cut across six skills, about half a
percent.** `dataflow-diagram` gave up 79, `sweep-comments` 40,
`architecture-diagram` 10. `review-guide` grew by nine and
`format-layout-diagram` by one, because correcting a stale pointer cost more
words than the duplication cut saved.

Two passes were run. Reading every paragraph over eighty words found five
genuine cuts: a preview of a section the same paragraph had just pointed at, a
sentence stating its own inverse, a clause restating the rule above it, an
enumeration duplicating a set the same paragraph deferred elsewhere, and a
closing sentence restating an earlier clause. A mechanical near-duplicate scan
over every sentence pair in the four largest skills found three more — a diagram
trigger stated in two sections, a consequence stated twice twenty lines apart,
and a panel rule in both the runtime-chart list and the visual-language list.

Everything else that scan surfaced was a rule paired with its own red flag,
which is what a red-flag list is for.

The conclusion the numbers force: these files are long because they say a lot.
The verbosity complaint that started this was accurate about one newly added
section — 376 words to say "use plain words", cut sixty percent — and does not
generalize to the corpus. Fifteen paragraphs over eighty words became fourteen,
and the survivors each carry several distinct facts.

## Design and Implementation

The cuts are unremarkable individually. The stale references are worth naming.

`2026-09-01-prose-ladder.md` widened `technical-prose` from word choice to "what
carries a fact, at the scale of a word and of the material around it". Three
skills still described the old boundary: `sweep-comments` said it "rules on
words only", `review-guide` said it "owns word choice", and
`format-layout-diagram` said it "owns word choice in captions". Each was true
when written and false the moment the ladder merged.

Nothing caught them. The citation guard checks line numbers, not claims about
another document, so a skill can describe a sibling skill's scope indefinitely
after that scope changes. They were found by grepping for the old phrasing while
looking for something else.

## Outcome

Shipped. Full suite, clippy, formatter, and the three CI scripts pass. A
bolded-rule and list-item diff across all six skills shows one rewrite — the
panel bullet, now pointing at the runtime-chart rule that states it — and no
loss.

## Derived Documents

None.

## Deferred or Reopen Items

- The largest remaining duplication is deliberate and was left alone.
  `architecture-diagram` restates `dataflow-diagram`'s principles because each
  ships as an independently installable template, so a project that seeds one
  must receive them. Replacing the restatement with a pointer would break the
  single-template consumer.
- Nothing detects a stale claim about another skill's scope. The near-duplicate
  scan that found the within-file repeats would find cross-file ones too, and
  running it as a test is cheap; distinguishing a deliberate carry from a stale
  copy is not.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
