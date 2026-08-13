---
name: review-guide
description: |
  Draft the pull-request body as a guide for the human reviewing it: where to
  look first, what was tested and what was not, which calls were judgment
  rather than deduction, and what will only show up in production. Use when
  asked to "draft the PR body", "write the PR description", "make this
  reviewable", "write a review guide", or whenever opening or updating a pull
  request. It runs on every change; what it publishes is proportional to the
  change, and a change with nothing to direct attention to earns one sentence
  rather than a guide. Symptoms that this skill applies: a description that
  retells the diff, a reviewer who reads files in whatever order the diff
  listed them, "all tests pass" with no statement of what the tests do not
  cover, a change whose risky decisions are invisible because they look like
  ordinary code, or an author who resolved an ambiguous requirement silently
  and shipped the resolution as if it were the only reading.
---

# Review guide

The reviewer already has the diff. This skill does not summarize it. It decides
where a human's limited attention buys the most, and states plainly what the
author could not settle.

The output is the pull-request body. Do not create a file for it. If the user
asks for a checked-in document, write that instead, but that is a different
request.

## Always run; publish only when it earns it

Run on every change. The assessment costs little and its result is information
either way: a change with nothing to say is a fact about the change, not a
reason to have skipped looking.

What you publish is proportional to the change and to its complexity. A
one-file mechanical edit earns a sentence. A change that crosses subsystems,
alters an interface, or makes a performance claim earns every section. Length
is a consequence of what the change actually carries, never a target.

### The publish test

Publish a guide when it carries at least one item the reviewer could not get
from the diff itself:

- a reading order that differs from the order the diff presents
- a test gap
- a judgment call
- a production-only risk

If all four are empty, do not publish a guide. Write one sentence describing
the change, and state that you checked those four and found nothing. The empty
result is a claim, and stating it is what makes the claim checkable — silence
is indistinguishable from not having looked.

Never pad to reach the bar. A guide inflated to look thorough costs the
reviewer more than no guide, because it teaches them to skim the next one.

## The body

Always present:

1. **What changed, and the claim it makes.** One paragraph. The claim is what
   would be false if the change were wrong.
2. **Where to look first.** Ranked reading order, plus what is safe to skim.
3. **Testing.** Methodology, what ran, its actual output, and the gaps.
4. **Judgment calls and low certainty.**
5. **Production-only risks**, with a direct invitation to weigh in.

Present when earned:

6. **Mental model**, with a diagram, when the change crosses a boundary the
   reviewer cannot hold in their head.
7. **Not in scope**, when a reader would otherwise ask why something is missing.

Drop a section only when the change genuinely has nothing in it, never to reach
a size. Any section holding an item that cleared the publish test stays,
whatever the size of the change: dropping it would publish a guide that cannot
say why it exists. A small change often reduces to the first four sections
because the later ones are empty, not because it is small — and a change that
clears the bar on production risk alone keeps that section and drops the others.

A very small change reduces to a single sentence, but only when the publish test
found nothing at all.

The certainty section is the exception to emptiness: it never disappears
silently — it says "none, and here is why".

## Where to look first

Rank by the cost of a missed defect multiplied by the chance the reviewer
misses it. Never rank by file size, path order, or the order the diff happened
to list.

Highest attention first:

1. Code carrying a judgment call. The reviewer cannot see a decision that looks
   like ordinary code.
2. Code whose correctness depends on something outside the diff — an invariant
   held elsewhere, a caller not shown, an ordering another module guarantees.
3. Code no test covers.
4. Code the author is least sure of.

Then name what is safe to skim, and why: generated output, mechanical renames,
formatting, a change repeated identically across many files. Naming the skimmable
half is half the value. A reviewer who spends attention on a rename has none
left for the invariant.

Point at a specific place, not a file. `src/vault/transitions.rs:119` beats
"the transitions module". Cite the line where a wrong review would cost the
most.

## Testing

Discover what the repository actually has before writing this section. Do not
assume a taxonomy. Consider each surface and say which apply:

- unit and functional tests
- integration or contract tests
- smoke tests against a built artifact
- benchmarks or performance measurement
- property, fuzz, or generative tests
- evaluation corpora, where the artifact is an agent instruction rather than code
- manual or exploratory checks

For each surface that applies, state three things: whether it ran, what it
actually said, and what it does not cover. Quote the real command and the real
result. "All tests pass" is not a report; "136 tests pass across seven binaries;
none exercise the tap publish path" is.

Describe the structure of the tests — what they are organized around — so a
reviewer can tell whether a gap is deliberate. Do not report a coverage
percentage; it answers a question nobody asked and hides the gap that matters.

Never state that a command ran when it did not. If a check was skipped, say
which and why. A skipped benchmark named is useful; a skipped benchmark implied
to have passed is a defect in the guide.

## Judgment calls and low certainty

This is the section the reviewer cannot reconstruct alone, and the one most
easily faked in both directions.

Every item cites concrete evidence:

- a requirement sentence that admits two readings, quoted
- a path with no test
- an assumption made and never verified
- a measurement not taken
- an interface whose contract the caller and callee state differently

Every item carries the call that was made, the alternative that was rejected,
and what evidence would change it. An item without those three is a worry, not
a finding; drop it.

Two failure modes, both defects:

- **Manufactured uncertainty.** Hedging to look careful. If nothing was
  genuinely unsettled, say so and say why the change was deducible.
- **Suppressed uncertainty.** Presenting one reading of a vague requirement as
  the only reading. If a requirement was ambiguous and a choice was made, the
  reviewer owns that choice as much as the author.

Do not rate confidence. A number invites the reviewer to trust a self-report
that carries no information. The evidence carries the signal.

## Production-only risks

State what cannot manifest before deployment, and what would reveal it:

- scale — volumes, cardinality, or concurrency not reachable in a test
- real data shape, including values the fixtures never contain
- timing, ordering, and partial failure across processes or hosts
- configuration and environment differences from the test environment
- migration and rollback behavior against existing state
- third-party or upstream behavior under conditions not reproducible locally

For each, name what would surface it — a metric, a log line, an alert, a canary
— so the reviewer can judge whether the change is observable after it ships.

Then invite a response. The reviewer is being asked either to weigh in or to
become comfortable with the risk before the next stage. Say which risks are
which. A risk nobody accepted is a risk nobody owns.

## Diagrams

A diagram earns its place only when the change crosses a boundary the reviewer
cannot hold in their head. Most changes do not need one.

- Use a checked-in diagram when one already exists, or when the picture is
  worth keeping past this review. Defer to `architecture-diagram` for
  build-time structure and runtime charts, and to `dataflow-diagram` for
  pipelines and topologies. Link it; do not paste it.
- Use inline mermaid when the picture is scaffolding for this review only. It
  renders in the pull-request body without a committed asset.

When a change makes an existing diagram wrong, say so. A stale diagram left
unmentioned costs more than no diagram.

## Boundaries

`engineering-journal` owns the durable record: effort-scoped, written for
someone reading in a year. This guide is change-scoped and written for one
reviewer this week. They may overlap, and the guide may restate a decision
rather than making the reviewer click through. The guide never becomes the
durable record — when an entry exists, link it.

`technical-prose` owns word choice in the guide. `sweep-comments` owns the
comments inside the diff; a comment defect belongs in the review, not in the
guide's prose.

## Rationalizations

| Excuse | Reality |
|---|---|
| "The diff is self-explanatory." | Then the ranked reading order costs two lines. Write it. |
| "I listed every file I touched." | A file list is the diff again. Rank, or say nothing. |
| "All tests pass." | Which tests, what did they cover, what did they not. |
| "I did not want to sound unsure." | The reviewer is the person who can settle it. Hiding it wastes them. |
| "Adding caveats shows rigor." | A caveat with no evidence is noise that buries the real one. |
| "The requirement was clear to me." | If it admitted two readings, you chose. Say which. |
| "Production risk is the ops team's problem." | Review is the last moment it is cheap to change. |
| "I will add the diagram if someone asks." | Nobody asks. They just review the wrong thing. |
| "Every PR deserves a full guide." | A guide with nothing in it teaches reviewers to skim the next one. |
| "The change was trivial, so I skipped the check." | Run it anyway and say the four were empty. Silence looks like not looking. |

## Red flags

- The body restates the diff, file by file.
- Reading order matches path order.
- No section names anything as safe to skim.
- Testing reports a percentage instead of a gap.
- A command is described as run without its actual output.
- Uncertainty items have no cited evidence.
- Every uncertainty item is hedged, or none exists on a change that clearly had
  a fork in it.
- Confidence ratings appear.
- Production risks are listed with nothing that would reveal them.
- A diagram is pasted where a link belongs, or a stale one goes unmentioned.
- The guide is written to a file when the request was a pull-request body.
- A guide is published carrying none of the four items in the publish test.
- A section is padded so the guide looks substantial.
- A change was skipped entirely rather than checked and reported as empty.
