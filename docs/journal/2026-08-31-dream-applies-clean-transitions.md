---
status: shipped
opened: 2026-08-31
updated: 2026-08-31
---

# The dream pass may land what it has already cleared

## Goal

Give the reconciler's dream pass a realization path. It wrote one session-note
per run and was forbidden from touching any artifact, so a transition it had
pre-flighted `allowed` with no blockers could only be described, never made.

Addresses `2026-08-31-dream-mode-pr-ceremony-problem` in the `knowledge-iop`
vault, on the branch `claude/elegant-ramanujan-prg2v9`.

## Scope

`skills/reconcile-vault/SKILL.md`, Mode B and the shared review gate. Mode A is
untouched by this change, which is the brief's constraint: the dream pass moves
to match interactive, not the reverse.

## Evidence

The brief supplies it. The dream routine has run about daily since 2026-07-14.
Three consecutive runs — 2026-08-11, 2026-08-12, 2026-08-30 — carried the same
four `vault-bootstrap`-closing transitions, each pre-flighted `allowed: true`
with no blockers or warnings, and none landed. The 2026-08-30 run added a fifth
edit that was mechanically a stale-flag lint: a design brief already carrying
`superseded_by`, whose successor was already `status: superseded`, with only
the predecessor's own flag lagging. It did not land either. Yesterday's run is
recorded as merged with all five still un-applied.

The mechanism is in the skill, not in the runs. Mode B's `Do not` list read
"Do not commit any artifact edits from this pass. The dream pass writes ONE
file: this session-note." So every run produced prose about edits it had
already earned the right to make, and each next reader — human, or the
interactive reconciler — had to rebuild the argument before applying one, or
defer and let the following run rebuild it again.

The second half of the brief is that the review layer sat over nothing. A pull
request whose only diff is a session-note asks the reviewer "is this napkin
correct?" about an artifact class `SCHEMA.md` defines as freely editable and
freely deletable, while the note's most useful content — the transitions — sat
inside it as prose no reviewer could apply from the review interface.

## Design and Implementation

Mode B applies the transitions that pass Mode A Step 5's clean-run test, reused
unchanged: `allowed` with no blockers or warnings, direct and sufficient
evidence, exactly one justified next state, repository validation passing. The
skill states the boundary in the same breath — this is permission to land what
the invariant machinery already cleared, not permission to reach past it, and a
transition Mode B may only describe is one Mode A could not have applied
either. No `force_accept` appears under another name, which is the brief's
first non-negotiable.

The note's `Proposed transitions` section becomes `Transitions` with two
headings, APPLIED and PROPOSED, because a reader acts on them differently: one
is a changelog and the other is a worklist. PROPOSED entries carry the exact
reason they did not clear. The brief named this shape as an open pick rather
than a requirement; two lists is the pick.

An empty APPLIED list on a run whose Part A found cleared transitions is
defined as a failure rather than a clean bill, on the same reasoning as the
interactive mode's rule that an empty retrieval-test list is a failed pass.

Part B is excluded explicitly. It is judgment written for a human, and none of
its recommendations are transitions this pass may enact.

Two operational rules follow from applying edits rather than describing them.
The note and the edits are one commit, because the note is the record of what
the edits were for and splitting them leaves a reader holding one half. And if
validation fails after applying, the edits are reverted, every transition moves
to PROPOSED with the failure as its reason, and the note commits alone: a dream
pass never leaves the vault failing validation.

## The pull request, reversed from the change stacked below

This entry's change sits on `2026-08-31-reconcile-vault-pr-opt-in`, which made
the pull request opt-in in both modes. For dream mode that is now wrong, and
this change puts it back.

The distinction is who is present. Interactive runs immediately after a phase
skill commits, on a branch someone already owns and will land, so a second pull
request reviews an edit its owner is already reviewing. Dream runs unattended
on a schedule, and once it lands artifact edits rather than a napkin, the pull
request is the only review event those edits ever get. The brief's own
constraint says the run "still stops after one PR", and its argument for the
chosen direction is that the pull request stops being ceremony precisely
because the diff now earns it.

So the two changes split on mode rather than agreeing: interactive opens a pull
request only on request, dream always opens one.

## Human review is a consumer, not a gate

The first version of this change kept the shared review gate: a run whose
report contained a blocker, a warning, or ambiguous evidence left its pull
request open for a human. The reviewer pointed out that gating on a reviewer is
the design weakness, not the fix for it.

They are right, and the evidence is the run this change exists to repair. A
dream that leaves its pull request open waits for a human who is not there, so
the next night's pass re-derives the same findings and opens a second pull
request beside it. Forty-four notes exist; the arc twelve of them recommend
closing is still open. The queue grew and the signal did not, and the
transitions that were already clear stayed unapplied exactly as long as the
unclear ones did — though nothing about a judgment call makes a mechanical edit
less correct.

So the merge is unconditional except on failed validation, which is a broken
vault rather than an undecided one. What needs judgment travels in the merged
note under a "Needs you" block above Part A: one line per item, what is being
asked, what it is waiting for, and an explicit "nothing this pass" when there is
nothing. That block is what the run's notification carries, which matches the
brief's own statement that notification is the routine's contract and the pull
request is a landing pad rather than the delivery mechanism.

## Outcome

Shipped. `cargo test --locked` and `cargo fmt --all -- --check` pass. The skill
has no corpus, so nothing tests the new disposition.

Unverified where it matters. Whether the five transitions the last three dreams
carried would now land is a claim about runs in another repository that have not
been re-run, and the brief's own reopen condition — a supposedly-clean
transition that breaks something the reflector did not reach — can only be
observed there.

## Derived Documents

None. The brief is a problem brief in the vault; no design brief was written,
because the brief already named the chosen direction and this is its
implementation. That shortcut is worth noticing rather than defending: the
paired-briefs convention exists so the design half can be disagreed with
separately.

## Deferred or Reopen Items

- The brief's reopen condition stands: a demonstrated cascade, where a clean
  transition applied by the dream pass breaks something `vault_reflect` did not
  reach — an orphaned citation, an accepted design brief with an obsolete
  problem-brief edge — makes propose-versus-apply a load-bearing safety
  boundary and this change wrong.
- The brief also names a tighter validator as a different fix than this one: if
  repository-wide validation caught the same class of error post-merge with
  acceptable latency, the ceremony could be dropped rather than earned.
- `reconcile-vault` still has no eval corpus, so neither this change nor the
  one below it has cases.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
