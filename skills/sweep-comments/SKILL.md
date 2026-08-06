---
name: sweep-comments
description: |
  Hold code comments and docstrings to a strict quality bar. Use whenever writing or editing comments — adding a comment to new code, being asked to "document this", "add comments", or "make this reviewer-ready" — and as a dedicated staleness sweep before opening or updating a pull request. Symptoms that this skill applies: comments that restate the adjacent code, the same design explanation sprinkled across several sites, docstring boilerplate on small internal helpers, derivation walkthroughs, war-story narration, or comments describing behavior a design pivot has since deleted.
---

# Sweep comments

Comment placement is a design question before it is a hygiene question.
Ask "where does the reader need this to be understood?" before "should
this comment exist?" — a sprinkled explanation is usually a symptom
that the design was never taught in one place, so every dependent site
grew a defensive footnote. Thoroughness is accuracy, not volume: an
accurate, lean file is stronger in review than one padded with
restatements.

## The reader

Write for the reader you actually have: a professional who is
competent at reading code and willing to learn the context on their
own. They can read a call signature, follow a loop, look up a library,
and derive what the language's own rules guarantee. They read the
module doc before the internals. A reader who did not read the code is
not a reader the codebase serves — their confusion is not grounds for
a comment. Assuming an incompetent reader is what produces restatement
comments; assuming this reader is what keeps them out.

## Explanation follows the architecture

Software design is hierarchical — system, subsystem, type, function —
and each level makes decisions the levels below inherit. Explanation
mirrors that contour: **every fact lives at the narrowest scope within
which it is invariant**, the level where its decision was made. The
crate doc teaches the system model; a module doc its subsystem's
protocol; a type doc the type's invariants; a method doc the call
contract; inline comments the point facts. Misplacement fails in both
directions: a fact repeated below its level is an echo, and a fact
hoisted above its level — a module doc narrating one method's edge
case — taxes every reader of the level and goes stale when that one
method changes. If no level exists where a fact is invariant, that is
a design smell, not a commenting problem.

Code materializes the leaves of that tree completely, which is why
comments have value at every altitude above the code and none at zero:
a comment at the code's own level restates what is already
materialized, while prose is the only medium that can state the
interior nodes — intent, models, protocols, why. Comments own the
interior of the design tree; code owns its leaves. The two media trade
in opposite directions: code must be functionally sound and so cannot
be vague — it is forced to total precision at the leaf level — while
prose is licensed to abstract and omit, which is exactly what lets it
serve one idea at different levels of detail. A comment competing with
the code on precision is a restatement; choose its altitude instead.

### The model home

Each subsystem designates one home — usually its module or crate doc —
that teaches the mental model: the execution order, the ownership
protocol, the invariants everything downstream leans on. State the
model there once and fully. Downstream sites then hold only three
kinds of comment:

- **Externals** — tier-3 facts (below) that no model statement can
  absorb, kept at their point of use.
- **Deviations** — places where the code does something the model
  would not predict. A reader carrying the model is *more* surprised
  there, so the local note earns its place.
- **Pointers** — one clause directing the reader to the model's home,
  at sites distant enough that the association is not obvious.

An **echo** — a restatement of the model at a downstream site — is a
delete, however true it is and however keeper-shaped its topic sounds.
One authoritative statement plus pointers; never two copies drifting
independently. An echo clause inside an otherwise-kept sentence is
still an echo: split the sentence, keep only the underivable clause,
and recompose it to stand alone ("cut whole sentences" protects
meaning, not echoes).

## The bar: three tiers by derivation cost

Classify every comment by what it would cost the reader to reconstruct
its fact from the code plus the language's own semantics:

1. **Locally derivable** — the fact is visible at the point of
   reading. Never comment it. This covers restatements of the next
   line, literal readings of a call, derivation walkthroughs of
   visible algebra — and topic is no defense: a `Relaxed` ordering
   justified by an exclusive lock held in the same function, or
   an atomic chosen because shared `Send` handles force a `Sync`
   cell, is generic language knowledge the reader owns.
2. **Derivable at a distance** — true, but reconstructing it means
   enumerating call sites or reading across files (a crate-private
   protocol, a cross-module invariant). Do not cache it at each site;
   state it once at the model home and reduce every echo to a pointer
   or nothing.
3. **Underivable in principle** — the fact comes from outside the
   code entirely. This is the never-delete floor, kept at point of
   use, one sentence each:
   - External-system behavior and vendor quirks (a filesystem cache
     pathology, a layout-engine rule, a protocol convention).
   - Measured values and the why behind magic numbers: weights,
     thresholds, limits, ports.
   - Schema documentation on data tables and wire formats.
   - Source-of-truth pointers ("the registry in X is authoritative").
   - A deliberate absence ("no retry here on purpose; the transport's
     watermark handles resumption").
   - Forward design commitments ("a parallel executor must preserve
     this invariant") — promises about code that does not exist yet.

## Writing new comments

- One short sentence per constraint. A kept comment longer than one
  sentence must justify every sentence as a distinct tier-2-at-home or
  tier-3 fact; connective tissue, illustrative examples, and rhetorical
  elaboration ("cheap is not free", storm-and-stall vignettes) do not
  survive.
- Compress, don't paraphrase: compression must not change meaning. A
  guarantee ("never steps backwards") must not become an obligation
  ("must not regress"); when in doubt, keep the original wording and
  cut whole sentences instead.
- Never mix a restatement with a real fact to justify the comment.
  Keep the real clause, delete the restatement half.
- A docstring longer than the function it documents is an action
  trigger, not a style note: cut it down, or justify it. The public-API
  allowance covers parameter, error, and panic contracts — not essays.
- When a comment enumerates parallel facts, format it as a bullet
  list, one fact per bullet; the one-sentence rule applies per bullet.
- Name specifics, not categories ("the vessel_command edge", not
  "the relevant edge").
- Do not pad a file with comments to appease an "under-documented"
  complaint, and never invent a rationale for a value whose reason you
  do not know — a wrong why-comment is worse than none.

## Examples that do not survive

Genericized from real sweeps; recognize the pattern, not the wording:

- `# Defaults to "primary" but honors SERVICE_ID from the environment`
  above the `os.environ.get` call — locally derivable (tier 1). What
  survived: which config layer wins when both are set (tier 3).
- A `Relaxed` justification reading "the lock's release/acquire edges
  already order these accesses" in a function that visibly holds the
  exclusive lock — tier 1, deleted; the reader owns the memory model.
- A type doc repeating the module doc's ownership protocol ("only the
  executor stores to it, between rounds") — an echo of the model home,
  deleted even though the fact itself is load-bearing.
- A four-line comment above a four-line classification function,
  paraphrasing its branches — deleted outright.
- A comment justifying a layout mechanism "because auxiliary edges do
  not constrain placement" after auxiliary edges were deleted — stale
  rationale from a design pivot, rewritten to the current truth.
- "After several failed attempts we discovered that the renderer
  ignores these edges during coordinate assignment" — the constraint
  survives in one sentence; the journey does not.

## Tests are labeled, not explained

A test is a leaf-level artifact pinning an interior-node claim, and
its comments serve failure diagnosis, not comprehension. Hold them to
a different bar:

- A test's doc comment states the property the test pins, in one
  sentence — and it may restate the model clause it enforces. Across
  the code/test boundary a restatement is a **claim binding**, not an
  echo: when the test fails, it names the promise that broke, which
  is the first thing a red CI run needs. The echo rule applies within
  load-bearing code, not from code to its tests.
- Scenario-contrivance comments are keepers: the why behind a
  deliberately odd fixture — a capacity below the append count, a
  scripted failure on the second send — is tier 3 at point of use.
  Without it the contrivance reads as arbitrary and gets "simplified"
  away along with the coverage it existed to create.
- What still dies in tests: narration of the mechanics ("stage one
  sample and open the round"), restating the test's own name, and
  derivation walkthroughs.
- Demo and tutorial files are designated teaching sites: pedagogy is
  their function, so hold them to "teaches each idea once, clearly"
  rather than to the production bar.

## The pre-PR sweep

Design pivots during a working session are the main source of lying
comments: a comment written for iteration 3 still sitting on the code
of iteration 12. Before opening or updating a PR:

**Do the sweep yourself, in one context. Do not partition it across
subagents.** A comment's value is holistic: whether it earns its place
depends on the model home, the neighboring comments, and the session's
pivots — context no per-file delegate has. Splitting the reading and
keeping the "judgment" is the same violation through a keyhole. If the
diff is large, sweep it in one pass anyway; reading the whole diff is
what the sweep *is*.

1. Build the model inventory before reading any comment: list every
   design principle the touched code relies on — not just the one you
   already have in mind — and assign each its home. A diff usually
   carries several, and the echo test is only as good as this
   inventory: a principle with no assigned home leaves every copy of
   it looking like a local keeper. If a model is stated nowhere, that
   is the first fix — write it once, where the reader forms it.
2. List every comment and docstring in the touched files and classify
   each into a tier against the inventory. Tier 1 is deleted; tier 2 lives only at the home,
   echoes become pointers or nothing; tier 3 is kept and compressed to
   one sentence per fact.
3. Check each survivor against the current design, not the design it
   was written for. A comment referring to anything deleted or renamed
   is rewritten to the truth or deleted.
4. Check the commit message and module docstring the same way — they
   go stale on the same pivots.
5. Verify the sweep was purely editorial: tests still pass, and any
   generated output (code generation, DOT/SVG, fixtures) is
   byte-identical before and after.

## Rationalizations

| Excuse | Reality |
|---|---|
| "It's a concurrency/memory-ordering comment — those are keepers" | The test is derivability, not topic. An ordering justified by a visible lock is tier 1; only the protocol fact with no other guard survives, and it lives at the model home. |
| "The reader might not have read the module doc" | They will — that is what the home is for. At most a pointer, never a copy. |
| "Each copy states a real constraint, so each copy is a keeper" | A fact stated in five places is one statement and four echoes. Distributed echoes are invisible comment-by-comment; only the model inventory catches them. |
| "The test doc restates the module doc, so it's an echo" | Across the code/test boundary a restatement is a claim binding: it names which promise the test enforces. Delete it and a red CI run stops saying what broke. |
| "This echo is convenient right where it's used" | Two copies of one model drift independently; the stale one becomes a lie with authority. Pointer or nothing. |
| "It's public API, so the length is fine" | Public API earns parameter, error, and panic contracts — not essays. |
| "Rewording it shorter is compression" | Compression preserves meaning exactly; a guarantee must not become an obligation. Cut sentences, don't mutate them. |
| "The derivation helps reviewers check the math" | Put derivations in the PR description. In code, one sentence states the invariant. |
| "The team lead wants thorough documentation" | Thorough means every comment is true, non-obvious, and taught in the right place — not that every line has one. |
| "I'll leave the old comment as historical context" | Git history is the historical context. |
| "No time to sweep before the PR" | A sweep of touched files takes minutes; a reviewer misled by a stale comment costs a review round. |
| "The diff is huge — I'll fan the reading out to subagents and keep the judgment" | Judgment built on delegated reading is delegated judgment. The sweep's value is one reader seeing the whole change. |
| "The reader might not know this API" | The reader is a competent professional who will look it up. Document your constraint, not their library. |

## Red flags — stop and re-check

- The same model stated in more than one place.
- A multi-subsystem diff swept against a single model home: the sweep
  anchored on the model you already knew.
- A comment whose fact a competent reader could derive from the code
  in front of them plus the language's own rules.
- A comment beginning with what the next line literally does.
- A docstring longer than the function it documents.
- "Previously", "used to", "we changed this to" in a comment.
- A comment you are keeping because deleting it feels like losing
  work — or because its *topic* sounds important.
- A sweep plan that contains the word "delegate", "fan out", or
  "per-file subagent".
