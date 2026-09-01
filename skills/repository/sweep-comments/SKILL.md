---
name: sweep-comments
description: |
  Hold code comments and docstrings to a strict quality bar. Use whenever writing or editing comments — adding a comment to new code, being asked to "document this", "add comments", or "make this reviewer-ready" — and as a dedicated staleness sweep before opening or updating a pull request. Symptoms that this skill applies: comments that restate the adjacent code, the same design explanation sprinkled across several sites, docstring boilerplate on small internal helpers, derivation walkthroughs, war-story narration, comments describing behavior a design pivot has since deleted, commented-out code kept "just in case", ticketless TODOs, banner comments, docstrings written only to silence a linter, or an invariant reduced to a pointer at the very site where a wrong edit would silently break it.
---

# Sweep comments

Comment placement is a design question before it is a hygiene question.
Ask "where does the reader need this to be understood?" before "should
this comment exist?" — a sprinkled explanation is usually a symptom
that the design was never taught in one place, so every dependent site
grew a defensive footnote. Thoroughness is accuracy, not volume: an
accurate, lean file is stronger in review than one padded with
restatements.

## The two readers

A comment is read by a human and by a coding agent, and it must serve
both. They differ in exactly one way that matters here, and the whole
of this skill's placement rule follows from it.

The **human** is a professional who is competent at reading code and
willing to learn the context on their own. They can read a call
signature, follow a loop, look up a library, and derive what the
language's own rules guarantee. They arrive through the file: module
doc first, then the internals. A reader who did not read the code is
not a reader the codebase serves — their confusion is not grounds for
a comment. Assuming an incompetent reader is what produces restatement
comments; assuming this one is what keeps them out.

The **coding agent** has that same competence and none of that reading
order. It arrives at a function by grep or symbol lookup, holding that
function and little else, and it edits confidently on that much. It
has usually not read the module doc, and a pointer costs it a
retrieval it may not spend.

The competence bar is shared, so **tier 1 dies for both**: a
restatement wastes a human's attention and burns the context an agent
needed for the code itself. Verbosity is not agent-friendly; it is the
tax both readers pay.

The reading order is not shared, and that is the entire difference. It
changes nothing about *what* is worth saying and one thing about
*where*: **a fact that constrains edits must be legible at the site
where a wrong edit would be made**, not only at the model home.

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
model there once and fully. Downstream sites then hold only four
kinds of comment:

- **Externals** — tier-3 facts (below) that no model statement can
  absorb, kept at their point of use.
- **Deviations** — places where the code does something the model
  would not predict. A reader carrying the model is *more* surprised
  there, so the local note earns its place.
- **Edit constraints** — the one clause of the model this site can
  silently break. Stated here *as well as* at the home, because the
  reader who breaks it is the one who never saw the home.
- **Pointers** — one clause directing the reader to the model's home,
  at sites distant enough that the association is not obvious.

An **echo** — a restatement of the model at a downstream site — is a
delete, however true it is and however keeper-shaped its topic sounds.
One authoritative statement plus pointers; never two copies drifting
independently. An echo clause inside an otherwise-kept sentence is
still an echo: split the sentence, keep only the underivable clause,
and recompose it to stand alone ("cut whole sentences" protects
meaning, not echoes).

### The retrieval test

An edit constraint is the one case where two copies are correct, so it
needs a test sharp enough that it cannot swallow the echo rule. Before
reducing any downstream comment to a pointer, ask:

**If a reader saw this site and nothing else, could they make a wrong
edit here that the model would have prevented — and would it compile
and pass?**

If yes, it is an edit constraint: keep one sentence here, stating what
must stay true, and leave the reasoning at the home. If no, it is an
echo and it goes.

Qualifying facts are narrow and recognizable:

- What sharing a handle actually shares — a clone that aliases rather
  than copies.
- An invariant another file's logic leans on: a sequence that is dense
  from 1, a buffer already sorted, a field never empty.
- An ordering the code cannot express: this store must follow that
  push; this lock must outlive that read.
- A teardown or lifetime obligation with no destructor to enforce it.

Not edit constraints: performance characteristics, anything the type
system or borrow checker already rejects, and restatements of the
model's *rationale* rather than its requirement. A wrong edit there
does not compile, or does not matter.

### The form the model takes

Placement decides where a fact goes; form decides what shape it arrives
in. The model home is the one place worth deciding form deliberately,
because it is the only place whose job is comprehension.

Prose is linear, which makes it good at causation, obligation, and
sequence — and bad at **topology** (what connects to what), **layering**
(what sits above what), and **simultaneity** (what happens at once).
Where the model is a shape, a paragraph makes every reader rebuild that
shape in their head, one at a time, forever. The trigger is not how
complex the design is; it is: **must the reader construct a picture to
follow this?** If so, supply the picture.

Pick the lightest form that carries the shape, because form is
maintenance:

- **An ordered list or table** — enumerable structure: what a module
  contains, what to read first, which error means what.
- **ASCII** — a small pipeline or sequence, roughly three to six nodes.
  It lives inline in the doc comment, needs no build step, and diffs as
  text.
- **A diagram** (D2 here) — real topology: many nodes, crossing edges,
  layers. It costs a separate file and a build step, so it must earn
  them.

Prefer forms whose **source is the artifact**. A rendered image is
readable by the human half of the readership and opaque to the other
half, while ASCII and D2 source are legible to both and reviewable in a
diff. A checked-in image blob with no source beside it is a diagram
only some of your readers have.

#### When the shape cannot live in the code

A doc comment can hold a list or ASCII; it cannot render D2. A model
whose best form is a diagram therefore lives on a docs page, with the
module doc holding a pointer.

That split is safe for comprehension and unsafe for constraint. A
reader who never opens the page loses the picture but breaks nothing —
whereas an edit constraint stranded on the far side of that pointer is
exactly the failure the retrieval test exists to catch. Shapes may live
one hop away; requirements may not.

#### Diagrams that do not survive

- One that shows what a single sentence already says — tier 1 in
  picture form, and dearer to keep true.
- One mirroring structure the code already materializes: a type
  hierarchy, a module tree, a call graph. The compiler and the file
  tree state those, and the drawing will drift from them.
- One kept because it was expensive to draw.

A diagram earns its place by showing what no single file materializes:
flow across modules, a protocol over time, a layering. Sweep diagrams
for staleness exactly as you sweep prose — a stale picture reads as
more authoritative than a stale paragraph, so it misleads harder.

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
   or nothing — unless the site passes the retrieval test above, which
   is the one carve-out: an edit constraint is stated at the home
   *and* in one sentence at the site that can break it.
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

### The cheapest form that carries the fact

The tiers decide whether a fact is stated at all. This decides what it
costs the reader once it is. Walk the rungs in order and **stop at the
first one that carries the fact intact**:

1. **No comment.** The tiers already ruled: locally derivable is
   deleted, and an echo of a distant model becomes a pointer or
   nothing.
2. **A name.** A fact a rename carries is not a comment.
   `retry_after_ms` needs no comment giving the unit, and a comment
   that exists to explain a name is a bug report against the name.
   This is the rung most often skipped, because renaming is work and
   a comment is not.
3. **Trailing, on the line it qualifies.** A fact binding one line
   rides on that line. It cannot drift away from what it describes, it
   costs no vertical space, and the reader meets it with the line
   under their eye rather than one beat before.
4. **One line above.** For a fact binding a block rather than a line,
   or a trailing form that would push the line past the column limit.
5. **A doc paragraph on the item.** For a contract a caller reads
   without reading the body: parameters, errors, panics.
6. **The model home.** Prose explaining a shape, stated once, pointed
   at from everywhere else.

Rungs 3 and 4 are where most comments belong and where few of them
sit. The drift is upward and it is invisible one step at a time: a
fact that would have fit after the line becomes a line above it, then
a sentence, then a paragraph with a lead-in, and no step buys the
reader anything.

**A rung is available only if the fact survives it intact.** Modality,
the subject of a claim, negations, and scope qualifiers do not
compress onto a trailing comment merely because the line has room.
When they will not fit, that rung is not available and you take the
next one. A shortening that changes what a comment says is not a
shorter comment — the same rule the fragment section states about
wording, here about placement.

**Be lazy about the form, never about the reading.** A rung is chosen
after the fact is understood, not instead of understanding it. A
one-line comment written to avoid reading the call sites is worse than
the paragraph it replaced: now it is both wrong and cheap to skim.

### Prefer the fragment

A comment is a label on code, not prose to be read aloud, and the
declaration beneath it already supplies the subject. Write the shortest
form that carries the fact:

- `/// Wrong `source_id`. Separate from `type_mismatch` so a
  misaddressed reader stays self-diagnosing.`
- not `/// Records whose `source_id` was not the expected one. Kept
  separate from `type_mismatch` so that the misaddressed-reader case
  stays self-diagnosing rather than being folded into a generic bucket.`

Grammatical completeness is not the bar; unmistakable meaning in context
is. The gain is uneven by altitude and that is the point: field and
variant docs are labels and shorten the most, method contracts shorten
some, and a model home is prose doing real explanatory work — it
tightens but stays sentences.

**Two things a fragment may never drop.** Both are ways a shorter comment
says something *different* rather than something briefer:

- **Modality.** "Nothing may be moved out of it here" is a prohibition
  binding future edits; "nothing moved out here" merely describes the
  present and licenses the edit the comment existed to prevent. Every
  edit constraint keeps its `must`, `may not`, or `never`.
- **The subject of a claim.** "The operational *response* to any of them
  converges on one question" is not "all three converge on one question"
  — the first is about what an operator does, the second about the
  causes. Dropping a noun to save a line silently reassigns what the
  sentence is about.

Also survivable only as full sentences: negations, scope qualifiers
(`only`, `at most`, `never`), and conditionals whose antecedent carries
the constraint. When a fragment cannot keep these, write the sentence.

### The rest

Word choice is not comment-specific and is not stated here. The
`technical-prose` skill is the home for modality, for vocabulary that carries
no fact, and for one-name-per-thing; use it directly when the problem is
wording rather than placement. It rules on words only and defers sentence
shape back to the calling site, which is why the fragment rule above stands
against a standard that would otherwise demand full grammar. The modality
requirement above is stated in both places on purpose: it is an edit
constraint, and the reader shortening a comment here is the one who never
opened that skill.

The bullets below are what comment writing adds on top of that bar:

- One short constraint per comment. A kept comment carrying more than
  one must justify each as a distinct tier-2-at-home or tier-3 fact; connective tissue, illustrative examples, and rhetorical
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
  executor stores to it, between rounds, so the bound is immutable for
  the whole of one call") — the rationale is an echo of the model home
  and goes. The requirement passes the retrieval test and stays as one
  sentence: a store added anywhere else compiles and quietly moves a
  bound out from under a reader. Split the sentence; keep the half
  that constrains an edit.
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
  load-bearing code, not from code to its tests. Both readers depend
  on this, and an agent triaging a failure has little else.
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

The sweep runs as two passes with opposite polarity, and they are not
interleaved. One pass removes and the other restores; trying to do
both at once means judging every borderline comment under a frame that
wants it gone, and the frame wins.

**Setup, before reading a single comment:**

1. Partition the touched files by which bar applies: production, test,
   demo. Do this first and write it down. A file read under the wrong
   bar gets the wrong rule applied to every comment in it, and no
   later carve-out recovers it — the production bar deletes a test's
   claim bindings before it ever occurs to you that a different bar
   was owed.
2. Build the model inventory: list every design principle the touched
   code *relies on* — not just the ones it states, and not just the one
   you already have in mind — and assign each **a home and a form**.
   The echo test is only as good as this inventory: a principle with no
   assigned home leaves every copy of it looking like a local keeper.
   If a model is stated nowhere, that is the first fix — write it once,
   where the reader forms it. Recording the form is what stops every
   model defaulting to prose; see below for choosing one.

   **Inventory the subsystem, not the changed lines.** A diff does not
   contain its own model homes, and any change that adds a *consumer*
   of existing code will have most of its models homed elsewhere: a
   parent branch in a stacked PR, a dependency crate, a module doc the
   change never touched. For each model, write down where it is stated
   and whether that file is in this diff; anything homed outside it
   makes every statement of it inside the diff an echo until the
   retrieval test says otherwise. Skipping this reads as a diff full of
   local keepers, because from inside the diff that is exactly what
   they look like.

**Pass 1 — subtract.** Classify every comment and docstring into a
tier against the inventory. Tier 1 is deleted; tier 2 lives only at
the home, echoes become pointers or nothing; tier 3 is kept and
compressed to one sentence per fact. Then check each survivor against
the current design, not the design it was written for: a comment
referring to anything deleted or renamed is rewritten to the truth or
deleted.

**Report pass 1 as one line per finding**, so the sweep can be checked
without re-reading the diff:

```
<file>:<line>: <tag> <what it said>. <what stands there now>.
```

Tags are the dispositions: `drop` for a tier-1 deletion with no
replacement, `point` for an echo reduced to a pointer at its home,
`shrink` for a survivor compressed, `inline` for one moved onto the
line it qualifies, `rename` for one deleted because a name now carries
it, and `keep` for a tier-3 fact left alone. Close with the count:
`net: -N comment lines`.

A diff with nothing to cut says so in the same form — "no tier 1 or 2
comments in the touched files" is a claim, and a claim can be wrong,
which is what makes it worth more than silence.

**Pass 2 — restore.** Walk the edit sites — every place a future
change could land, not every place a comment currently sits — and at
each one ask the retrieval test: could a reader who saw this site and
nothing else make a wrong edit here that still compiles and passes?

This pass may only keep or add. It is not a review of pass 1 and it
does not re-litigate a deletion on taste; it asks one question and
acts on the answer.

Its output is a written list of the sites that passed the test and the
one sentence each now carries. **An empty list is a failed pass, not a
clean bill** — a diff that touches load-bearing code has edit
constraints in it, and finding none means the test was not actually
run. Report the list with the sweep.

**Then:**

3. Check the commit message, the module docstring, and any diagram the
   touched code is drawn in — all three go stale on the same pivots,
   and the diagram is the one nobody thinks to open.
4. Verify the sweep was purely editorial: tests still pass, and any
   generated output (code generation, DOT/SVG, fixtures) is
   byte-identical before and after.

## Rationalizations

### Comments that should not be written at all

Some comments fail at write time, regardless of wording — the content
has a home, and it is not the code:

- **Change-narration** ("fixed the off-by-one", "added guard") — the
  commit message and PR description exist for this; in code it becomes
  a "previously" red flag the moment it lands.
- **Commented-out code** kept "in case we need it back" — git history
  is the fallback: reverting a commit beats resurrecting a block that
  has rotted against the surrounding code.
- **Idle TODOs** — a TODO with no ticket, owner, or design commitment
  is a musing that ships as a stale comment with authority. File the
  ticket or drop the thought. (A forward design commitment — tier 3 —
  is different: it binds future code to an invariant.)
- **Banner comments** (`// ---- helpers ----`) — writing-session
  scaffolding that carries zero facts. A file that needs banners wants
  to be split into modules instead.
- **Linter-appeasement docstrings** on trivial internals — suppress
  the check at the site and fix the config; a machine's
  "under-documented" complaint licenses tier-1 prose no more than a
  human's does.
- **Insurance comments** written to pre-empt or placate a reviewer —
  answer in the review thread and strengthen the model home; a comment
  written to end a conversation serves the wrong reader.
- **Guessed rationales** — never write a why you do not actually know.

| Excuse | Reality |
|---|---|
| "It's a concurrency/memory-ordering comment — those are keepers" | The test is derivability, not topic. An ordering justified by a visible lock is tier 1; only the protocol fact with no other guard survives, and it lives at the model home. |
| "The reader might not have read the module doc" | The human will — that is what the home is for. The agent will not, which buys exactly one sentence at a site that passes the retrieval test, and nothing anywhere else. |
| "Agents don't follow pointers, so every pointer should go back to being a copy" | Only where a wrong edit would compile and pass. Everywhere else the agent that skips the pointer also skips a fact it had no use for. The carve-out is edit constraints, not comfort. |
| "A pointer can't drift, so it beats a copy everywhere" | It also can't be read by someone who never follows it. At a site that can silently break the fact, an unread pointer is a missing fact, and drift is the cheaper failure. |
| "Serving coding agents means keeping more comments" | It means one sentence at edit sites and the same deletions everywhere else. Tier 1 costs an agent more than a human — it burns the context window the code needed. |
| "This diff states the model, so the diff is its home" | Being the only copy you can see is not being the home. Look in the dependency and the parent branch first — a stacked PR's models are usually homed one branch down, and a new consumer's are homed in what it consumes. |
| "Agents don't read pictures, so a diagram is wasted effort" | The model home's job is comprehension and half the readership is human. Keep the source text-based (ASCII, D2) and the cost to the other half is zero. |
| "A picture is always clearer than a paragraph" | Not when the paragraph already carries it. A diagram that restates one sentence is a restatement that also has to be kept true. |
| "The diagram is a bit out of date but still roughly right" | A diagram reads as more authoritative than prose, so a stale one misleads harder. Redraw it or delete it. |
| "Each copy states a real constraint, so each copy is a keeper" | A fact stated in five places is one statement and four echoes. Distributed echoes are invisible comment-by-comment; only the model inventory catches them. |
| "The test doc restates the module doc, so it's an echo" | Across the code/test boundary a restatement is a claim binding: it names which promise the test enforces. Delete it and a red CI run stops saying what broke. |
| "This echo is convenient right where it's used" | Two copies of one model drift independently; the stale one becomes a lie with authority. Pointer or nothing. |
| "It's public API, so the length is fine" | Public API earns parameter, error, and panic contracts — not essays. |
| "Rewording it shorter is compression" | Compression preserves meaning exactly; a guarantee must not become an obligation. Cut sentences, don't mutate them. |
| "Shorter is always better, so drop the verb" | Only while the fact survives. Modality and the subject of a claim are content, not grammar — a fragment that loses either says something else. |
| "The fragment is obvious from context" | Obvious that the fact is *true*, perhaps. Check that it is still obvious the fact *binds*: a description reads as reversible where a prohibition does not. |
| "It's an edit constraint, so it has to be a full sentence" | Only the modal clause does. `Ring must outlive the reader.` is a fragment and a prohibition at once. |
| "The derivation helps reviewers check the math" | Put derivations in the PR description. In code, one sentence states the invariant. |
| "The team lead wants thorough documentation" | Thorough means every comment is true, non-obvious, and taught in the right place — not that every line has one. |
| "I'll leave the old comment as historical context" | Git history is the historical context. |
| "No time to sweep before the PR" | A sweep of touched files takes minutes; a reviewer misled by a stale comment costs a review round. |
| "The diff is huge — I'll fan the reading out to subagents and keep the judgment" | Judgment built on delegated reading is delegated judgment. The sweep's value is one reader seeing the whole change. |
| "The reader might not know this API" | The reader is a competent professional who will look it up. Document your constraint, not their library. |
| "One more true comment can't hurt — it buys reviewer goodwill" | Every comment is a liability that must stay true through every future edit. Goodwill belongs in the review thread, not pinned to the code. |
| "A TODO is free — I'll leave it in case we want this later" | A TODO with no ticket or commitment is a stale-comment seed with no owner. It costs nothing today and lies within a quarter. |
| "Suppressing the linter is cheating; writing the docstring is compliance" | The manufactured docstring is the cheat — it fakes documentation to green a check. A visible suppression records the deliberate absence honestly. |
| "I'll keep the old code commented out until the new path is proven in prod" | If prod breaks you revert the commit; you don't uncomment a block that stopped compiling against its neighbors weeks ago. |

## Red flags — stop and re-check

- The same model stated in more than one place.
- A multi-subsystem diff swept against a single model home: the sweep
  anchored on the model you already knew.
- A model inventory whose homes are all inside the diff, on a change
  that adds a consumer of existing code — that is a sweep that never
  looked up from the diff.
- A comment whose fact a competent reader could derive from the code
  in front of them plus the language's own rules.
- A comment beginning with what the next line literally does.
- A docstring longer than the function it documents.
- "Previously", "used to", "we changed this to" in a comment.
- A comment you are keeping because deleting it feels like losing
  work — or because its *topic* sounds important.
- A sweep plan that contains the word "delegate", "fan out", or
  "per-file subagent".
- Commented-out code, a ticketless TODO, or a section banner in a
  diff you are about to ship.
- A comment you are writing mainly so a reviewer or linter stops
  asking — not because the reader at that site needs the fact.
- A tier-2 fact reduced to a pointer without asking whether an edit at
  that site could break it and still compile.
- A sweep whose diff is deletions only: the retrieval test keeps
  things, and a sweep that kept nothing probably never ran it.
- Pass 2 folded into pass 1 "to save a read", or run as a re-read of
  pass 1's deletions rather than a walk of the edit sites.
- A test or demo file swept before the bars were partitioned.
- A model inventory whose every entry has the form "prose": the form
  column was filled in after the fact, not chosen.
- A paragraph at a model home that names three or more components and
  the connections between them — that is a shape being spelled out.
- An edit constraint reachable only by following a link to a diagram.
- An "edit constraint" you are keeping whose violation the compiler
  would catch — that is the carve-out being used as an excuse.
- An edit constraint with no `must`, `may not`, or `never` in it.
- A shortened comment whose grammatical subject differs from the one it
  replaced.
- A shortened comment that dropped `only`, `at most`, or a negation.
- A model home rewritten into fragments: explanation is the one place
  prose is doing the work.
- A comment above a line whose fact binds that line alone.
- A comment whose job is to explain the name directly beneath it.
- A block comment that survived because it was already a block, rather
  than because a shorter rung would have lost something.
- A trailing comment that dropped a `must`, a `never`, or an `only` to
  fit the column limit.
- A pass-1 report that is prose rather than one line per finding, so a
  reader cannot tell which comments were touched.
