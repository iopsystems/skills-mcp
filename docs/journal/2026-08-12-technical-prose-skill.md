---
status: shipped
opened: 2026-08-12
updated: 2026-08-12
---

# Technical prose skill

## Goal

Extract the reader-agnostic half of ASD-STE100 Simplified Technical English into
a thin active skill that other skills reference, so that word-level prose
discipline is stated once instead of copied into every skill that writes prose.
Wire `sweep-comments` to it as the first consumer.

## Decision Criteria

Ship when the served skill states the vocabulary/syntax boundary explicitly,
carries the modal ladder with the prohibition carve-out, and `sweep-comments`
references it without contradicting its own fragment rule. `cargo fmt --check`,
the full test suite, a release build, and an MCP list/call smoke test must pass.

## Scope

The skill owns vocabulary: modal discipline, slop substitutions, one-item-one-
name, verbs over nominalizations, phrasal verbs, Latin abbreviations, active
voice, and the untouchable set. It does not own syntax — sentence length,
articles, and fragments versus complete sentences belong to the calling skill.
It does not cover document structure, audience, or information architecture.

`document-feature` is deliberately not wired in this effort. See Deferred.

## Evidence

[AminBlg/SimpleEnglish](https://github.com/AminBlg/SimpleEnglish) (MIT) packages
ASD-STE100 Issue 9 as an agent skill: 53 rules in 9 sections, two modes, a modal
ladder, and a slop-substitution table the author marks as original work rather
than ASD material. Its README reports 72.9% fewer STE violations per 100 words
across six Claude models, and lower output token counts. That evaluation is the
author's own and was not independently reproduced here.

Three structural facts about this repository constrain the design:

- `src/main.rs:38` embeds `skills/` through `include_dir!` and serves each
  `SKILL.md` body as instruction text. `skill_template_get` is templates-only,
  and no active skill has a `references/` directory. An active skill is
  therefore exactly one file, and a shared rule set cannot be a servable
  sub-file — it must be its own skill with its own trigger.
- `skills/architecture-diagram/SKILL.md:27` ("principles carried into a domain")
  and `:38` ("use `dataflow-diagram` directly") establish the precedent for one
  skill deferring to another in prose, by name, with no import mechanism.
- `templates/document-feature-skill/` is an inert template, seeded and
  customized per project. A reference from it to an MCP-served skill dangles in
  any repository without `skills-mcp` installed.

## Design and Implementation

The seam is vocabulary against syntax, not this standard against this repository.
Word choice is reader-agnostic: "gracefully handles" is slop in a comment and in
a README alike. Sentence shape is reader-dependent, because a comment sits under
a declaration that supplies its subject and article while a README paragraph
stands alone.

That split is what makes the wrapper thin, and it dissolves the one head-on
conflict. STE Rule 4.2 is explicitly an anti-terseness rule — keep articles, keep
"that" — which contradicts the `sweep-comments` section "Prefer the fragment".
The contradiction is real but confined to syntax, so the wrapper simply declines
to rule on it and each consumer keeps its own sentence-shape rule.

One vocabulary rule needed a carve-out rather than adoption. STE's modal ladder
maps `may` to `can`, but `sweep-comments` requires every edit constraint to keep
its `must`, `may not`, or `never`. Rewriting a prohibition ("may not") to
impossibility ("cannot") or permission ("can") changes the fact, which is the
failure that skill's modality rule already exists to prevent. The carve-out is
universal rather than comment-specific — documentation says "you may not call
this twice" too — so it lives in the wrapper.

The wrapper becomes the model home for modal discipline. The existing
`sweep-comments` sentence stays rather than reducing to a pointer, because it
passes that skill's own retrieval test: a reader shortening a comment at that
site could drop the modal, and the result would still compile and pass. Home
plus one sentence at the edit site is the documented carve-out, so the design is
self-consistent under the doctrine it extends.

Trigger scope is narrow by choice: explicit wording requests and invocation from
another skill, not general prose authoring. A broad trigger would compete with
`document-feature` and with ordinary writing tasks.

Artifacts:

- `skills/technical-prose/SKILL.md`
- `skills/technical-prose/evals/trigger-evals.json`, eighteen cases covering
  modality, vocabulary, untouchables, and two boundary cases asserting the skill
  declines to rule on fragments and sentence length
- `technical_prose_evals_cover_key_scenarios` in `src/main.rs`, mirroring
  `sweep_comments_evals_cover_key_scenarios`
- a reference in `skills/sweep-comments/SKILL.md` under "Writing new comments"

Attribution: the slop table is adapted under MIT from AminBlg/SimpleEnglish; the
rules paraphrase ASD-STE100, unaffiliated with ASD or STEMG. The repository has
no `LICENSE` file today and `Cargo.toml:6` declares `MIT OR Apache-2.0`, so this
sets the third-party attribution precedent.

## Outcome

Shipped as an embedded MCP skill on branch `yao/technical-prose`. `cargo
fmt --check` passed, all 136 tests passed across the six test binaries,
`cargo build --release` succeeded, and a raw JSON-RPC smoke test confirmed that
`tools/list` exposes `technical-prose` and `tools/call` returns the 8,612-
character body.

The trigger corpus has not been forward-tested against independent agents; the
eighteen cases assert intended behavior rather than measured behavior.

## Derived Documents

None. The design was settled in conversation and recorded here directly rather
than in a separate specification file, because the repository has no `specs/`
convention.

## Deferred or Reopen Items

- Wiring `templates/document-feature-skill/` to the skill. Deferred until the
  wrapper proves itself on one consumer. The open question is how a seeded
  template reaches an MCP-served skill in a repository without `skills-mcp`: a
  soft reference that degrades to a pointer, a seeded `references/` copy that
  drifts, or shipping the wrapper as a template as well.
- Independent evaluation of the trigger corpus, which shares the delegated-
  evaluation constraint recorded in the engineering-journal entry.
