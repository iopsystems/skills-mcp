---
status: shipped
opened: 2026-08-31
updated: 2026-08-31
---

# Reconciliation applies; it does not land

## Goal

Stop `reconcile-vault` opening and merging a pull request on every clean run.
A clean run now applies the transitions, commits on the current branch, and
stops. A pull request is opened only when the user asks for one, or when a
finding needs somewhere durable to be reviewed.

## Scope

`skills/reconcile-vault/SKILL.md`. No change to the transition pre-flight, the
evidence rules, the review package, the token budget, or the prohibition on
`force_accept`.

## Evidence

The request named a draft pull request. The skill had none: it required an
ordinary pull request, created and merged without confirmation on the clean
path in both modes, and the only draft-PR rules in the repository are in
`plan-feature`. The request as worded landed on nothing, so the ambiguity was
put to the user rather than guessed at, and the answer was the pull-request
ceremony in this skill rather than the draft rule in the other one.

The ceremony was load-bearing in one respect and dead weight in the rest.
Interactive mode is invoked immediately after a phase skill commits an
artifact, so a branch already exists and something already owns landing it;
opening a second pull request for a frontmatter edit on that branch's
neighbors adds a review surface nobody asked for. Dream mode writes one
session-note.

What the pull request did carry was one of the four clean-run conditions:
"passes repository validation and required PR checks". Removing the pull
request removes those checks.

## Design and Implementation

The clean path becomes apply, validate, commit, push, stop. The disposition
section is renamed from "Default to merge" to "Default to apply", and states
the distinction the change turns on: **the default is applying, never
landing.** Opening a pull request, merging one, and asking a host to merge are
all things the skill now does only on request.

The lost checks are named where the gate is stated rather than left for a
reader to notice: repository validation is now the whole gate, a check that
only ever ran in CI no longer runs before the commit lands, and nothing else
moves — an uncertain transition is still not applied and the review package is
still produced.

Two consequences needed answers the old text did not have to give. Human
review previously happened in the pull request the skill had just opened;
review-required findings now surface in an existing pull request for the
branch when there is one, and in the session when there is not. And a clean
run that commits on the current branch commits to `main` when that is the
current branch, which the pull request used to prevent: the skill now says to
branch first and say so.

One pre-existing wart went with it — the review path introduced its output
twice, as "a concise review package" and then "a concise proposal list", for
one list.

## Outcome

Shipped. `cargo test --locked` and `cargo fmt --all -- --check` pass. The
skill has no corpus, so nothing tests the new disposition.

Unmeasured, and unmeasurable here: this skill runs against the `knowledge-iop`
vault, which is a different repository. Whether removing the pull request
loses a check that was catching something is a question that repository's CI
configuration answers, and it was not consulted.

## Derived Documents

None.

## Deferred or Reopen Items

- The checks that ran on the reconciliation pull request are unenumerated. If
  the vault's CI runs a validator that the skill's own "repository validation"
  step does not, this change silently dropped it, and the fix is to name that
  command in the skill rather than to restore the pull request.
- `plan-feature` still states "Use a feature branch and a draft PR" as a
  requirement at one site while offering it at another. That inconsistency was
  found while locating this change and deliberately left alone.
- `reconcile-vault` has no eval corpus. Every other skill changed this month
  gained cases with its rules; this one has nowhere to put them.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
