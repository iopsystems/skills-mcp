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
  rather than a guide. Beta — its ranking and publish rules are asserted rather
  than measured, so record friction and confirmation through
  `engineering-journal`. Symptoms that this skill applies: a description that
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

The output is the pull-request body. "The guide" and "the body" name that one
artifact throughout; nothing else here is called either.

Do not create a file for it. If the user asks for a checked-in document, write
that instead, but that is a different request.

## What this is, and what it is not yet

Treat this skill as **beta**. Its two central rules are asserted rather than
derived. The attention ranking below has never been measured against where
defects were actually found, and the four items of the publish test are a
judgment about what a reviewer cannot get from a diff, not a finding. No guide
this skill produced has yet been read by a reviewer who then said whether it
helped.

Use the rules wholesale anyway. A stated default that fails teaches more than a
hedge that cannot. When one fights the change in front of you, the override and
its reason are the most valuable thing the effort produces: record them through
`engineering-journal`, which names this skill in `beta_skills` and takes the
account under `## Skill Feedback` — what was asked, which instruction misfired,
and what you did instead.

Record the defaults that held, too. A channel collecting only complaints will
retire rules that were working.

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

Open with a TL;DR, above every heading. It is what the reviewer carries away if
they read nothing else. Write it last, once the rest exists.

It is a short essay, not a label, and it answers three questions in order:

1. **Why are we doing this?** The problem or the motion that makes the change
   necessary. Not what the change is — what made it needed.
2. **What is the key idea?** The one insight the change turns on. If the change
   has no idea in it, say what it does instead and move on.
3. **What is true once it lands?** The state the reviewer should expect
   afterwards, including what still does not work.

Length follows those answers and nothing else. A change that crosses subsystems
earns a paragraph. A small change collapses all three into one sentence, because
its why, its idea, and its effect are the same fact seen three ways. A change
that clears nothing on the publish test is one sentence and stops there.

A TL;DR that restates the title is wasted. So is one that opens a list the
reviewer must then read to use.

**No identifiers.** A type, field, function, flag, or file path in the TL;DR is
the strongest available signal that you described the change instead of the
reason for it. The reviewer meets `StreamSpec` before they know what a stream
is, and a name they cannot resolve costs them the sentence it appears in. Names
belong under Where to look first, after the mental model has given them
somewhere to land.

A worked example, answering the three questions above. The change splits one
registry field that was answering two questions at once:

> **Too specific.** Splits `StreamSpec.topic`, which was answering both "which
> ROS2 topic carries this" and "has the origin started enveloping", into two
> fields.

> **The reason.** Each sensor is moving from publishing a raw ROS2 topic to
> publishing through the Hub, which wraps the payload in an envelope. The
> payloads are byte-identical either way, so nothing downstream can tell which
> path a record took; using the presence of the envelope as the discriminator is
> what lets the sensors move one at a time.

Same change. The second tells a reviewer who has never opened this code why the
work exists and what the idea is. The first is a diff summary with the field
name pre-loaded.

The test: if a reader who has not seen the codebase cannot say what problem the
change solves, the TL;DR is describing the edit rather than the reason.

### What to look out for

The reviewer has read one paragraph. They cannot answer a question yet, and a
list of questions at the top of a page is a quiz before the lesson. So the ask
is not a request for answers — it is the set of things to carry while reading.

Number them. Then mark each one again, by number, at the place further down
where its evidence appears. The reviewer meets the item twice: once as a flag to
hold, once as the detail that lets them settle it.

```markdown
## What to look out for

Two calls and one risk, none blocking. Carry these while you read; each is
marked again where its evidence appears.

1. [ ] **Which machine publishes the operator's commands.** A claim about the
       vessel rather than about the code, and the recorded labels disagree.
       I cannot check this from here.
2. [ ] **Refusing to start on an unnameable machine.** Deliberate, but it turns
       a naming mistake into no recorder at all.
3. [ ] **Message-size headroom**, to accept or to send back for widening.
```

Then, in the section that carries the evidence:

<!-- cite-ignore -->
> **[1]** `src/registry/streams.rs:490` — the comment asserting that the four
> command streams originate on the cabin machine …

Rules that keep the list honest:

- **One item per thing to watch**, numbered, and marked again below. An item
  never marked again is a flag the reviewer carries to no purpose.
- **Only what the reviewer can settle.** If you can settle it, settle it in the
  guide and leave it out. An item you already know the answer to is a quiz.
- **Say what each will need by the end** — a decision, a confirmation of fact,
  or an acknowledgement of a risk. Those cost different amounts.
- **Say which block the merge**, if any, in the opening sentence.
- **Never empty.** When nothing needs the reviewer, write one line saying so:
  "Nothing here needs a decision — the checks that would produce one came back
  empty." An absent list and an empty one read identically, and only one of them
  means you looked.

### Then the mental model

The order of the opening is the point. The TL;DR says why this exists; the
look-out list says what to carry while reading. **The mental model comes next**,
before any section that names a type, because it is what the rest of the guide
is written in terms of. Content for it is below under The mental model.

Always present, in this order:

1. **TL;DR.** Why, the key idea, and what is true afterwards — as long as those
   three answers need, and no longer.
2. **What to look out for.** Numbered items to carry while reading, each
   marked again where its evidence appears. Never empty.
3. **The mental model.** The concepts the reviewer needs before any detail means
   anything, and where this change sits among them. A diagram when they have a
   shape.
4. **What changed, and the claim it makes.** One paragraph expanding the TL;DR
   rather than repeating it. The claim is what would be false if the change
   were wrong.
5. **Where to look first.** Ranked reading order, plus what is safe to skim.
6. **Testing.** Methodology, what ran, its actual output, and the gaps.
7. **Judgment calls and low certainty.**
8. **Production-only risks**, with a direct invitation to weigh in.

Present when earned:

9. **Not in scope**, when a reader would otherwise ask why something is missing.

Drop a section only when the change genuinely has nothing in it, never to reach
a size. Any section holding an item that cleared the publish test stays,
whatever the size of the change: dropping it would publish a guide that cannot
say why it exists. A small change often carries only the claim and the reading
order because the later sections are empty, not because it is small — and a
change that clears the bar on production risk alone keeps that section and
drops the others.

A very small change reduces to a single sentence, but only when the publish test
found nothing at all. That sentence is the TL;DR, and nothing follows it.

The certainty section is the exception to emptiness: it never disappears
silently — it says "none, and here is why".

## The mental model

This section sits directly after the look-out list. A
reviewer who meets a field name before they know what the thing holding it is
for has nowhere to put it. Type names, field names, and function names are
the last thing a guide reaches for, not the first.

Establish, in this order:

1. **What the system does at the level this change touches.** One or two
   sentences in the domain's terms, not the code's.
2. **The two or three concepts the change depends on.** Name each once and use
   that name everywhere after. A concept the guide leans on without naming is a
   concept the reviewer rebuilds from the diff, which is the work the guide
   exists to do for them.
3. **Where this change sits** among those concepts, and what it moves.

Only then can Where to look first name a type, because the name now has
somewhere to land.

State the starting point you assume, and name where to jump by its heading:
"This assumes you know the ring protocol; skip to Where to look first if you
do." That costs one line and releases the reviewer who already holds the model.

A cross-reference names a section exactly as its heading reads. "The reading
order" and "the certainty section" are descriptions, not addresses; a reviewer
sent to one has to guess which heading was meant.

Reach for a diagram when the concepts have a shape — a topology, a pipeline, a
set of hosts, a before and after. Prose carries a sequence; a picture carries a
shape, and re-deriving a shape from prose is the work being pushed back onto the
reviewer. See Diagrams below for which kind.

Two failure modes:

- **A subsystem tour.** This section gives the reviewer what this change needs,
  not what the subsystem is. If a paragraph would be equally true of a different
  pull request against the same files, cut it.
- **Restating the diff in prose.** The mental model is what holds before the
  change, plus where the change lands. It is not the change.

Omit it only when the change touches one concept the reviewer certainly holds: a
typo, a version bump, a rename inside one file. The burden is on omitting.
A guide that opens on field names with no picture behind them is the failure
this section exists to prevent, and it is the one readers report.

### Before, problem, change

Every explanation in the guide takes the same shape, from one paragraph to a
whole section:

1. **Where things stood before.** The arrangement that worked until now.
2. **The new problem or condition that motivates the change.** What arrived, or
   what stopped being true.
3. **The change.** Now it lands as the response to something, rather than as a
   fact the reviewer has to accept on its own.

> When only one machine recorded, nothing needed to distinguish the topic a
> record arrived on from the sensor that produced it. As streams migrate to
> arriving over the link instead, the topic alone can no longer tell the two
> paths apart. So a second label is needed, and this change adds it.

Three beats, three sentences. Each appears once: a sentence that makes the
previous sentence's point from a new angle reads as emphasis and costs the
reader exactly what new information would have cost them. The most common way
to inflate this is to state the "before" twice, once plainly and once as the
problem.

**Gloss what you did not introduce.** Below the TL;DR, identifiers are allowed;
undefined ones are not. An identifier the change **adds** introduces itself —
the sentence that adds it says what it is. An identifier that **already
existed** has no such sentence, which is exactly why it is the one left
undefined, and the guide then reads as though the reviewer should have known it.
The "before" beat is where it gets its one clause, in the domain's terms: not
its type, its job.

<!-- cite-ignore -->
> **Undefined.** `StreamSpec.topic` was answering two questions at once.

> **Glossed, in the before beat.** Every stream has a registry entry saying
> where it comes from and how it is carried, and one field on that entry named
> the message-bus topic it arrives on.

Same work. The second can be read by someone who has never opened the file.

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

Point at a specific place, not a file. Cite the line where a wrong review would
cost the most.

<!-- cite-ignore -->
A line number beats a module name — `src/vault/transitions.rs:119` beats "the
transitions module" — but a line number is a fact with a short shelf life, so
its form follows how long the document lives.

**In the pull-request body**, which is read this week against one commit, cite
`path:line` and state near the top which commit the lines are pinned to. Commits
landing on the branch after you draft will shift them; naming the commit is what
keeps the citation true.

**In a durable document** — a journal entry, a skill — a bare line number rots
silently the next time someone edits the cited file. Quote a phrase from the
cited line in the same paragraph, so the number stays re-derivable from the
text. `tests/citations.rs` enforces this and can repair a number that moved; it
cannot repair a citation that never carried a phrase.

Read every line you cite, at the moment you cite it. Not most of them. A line
number you did not open in this session is not a citation you may write — it is
a guess that looks exactly like a fact, and the reviewer cannot tell which one
they are holding. Partial verification is the failure mode here: checking four
anchors and estimating the fifth produces a body that is wrong in a way its own
care makes harder to doubt.

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

This is the section the reviewer cannot reconstruct alone, and the one faked
most often in both directions.

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

A diagram belongs in the mental model, where it does the work prose is bad at:
carrying a shape. Reach for one when the concepts have a topology, a pipeline, a
set of hosts, or a before and after.

Which kind:

- **Structure — what exists and what contains what.** Units, layers, processes,
  hosts. Defer to `architecture-diagram`, which owns the build-time structure
  chart and the runtime thread and request charts.
- **Movement — what flows where.** Pipelines, stream topologies, service graphs,
  a record's path across processes. Defer to `dataflow-diagram`.

Where it lives follows how long it is worth:

- **Checked in** when one already exists, or when the picture outlives this
  review. Link it; do not paste it. Both skills above produce a source file and
  a rendered image, which is what makes it maintainable.
- **Inline mermaid** when the picture is scaffolding for this review only. It
  renders in the pull-request body with no committed asset. Keep it small
  enough to read without scrolling — a diagram a reviewer must pan is prose
  with extra steps.

A stacked series shares one shape. Draw it once, and in each guide show the same
picture with this change's piece marked, rather than a different diagram per
pull request: the reviewer learns one model and reuses it across the stack.

When a change makes an existing diagram wrong, say so. A stale diagram left
unmentioned costs more than no diagram.

## Boundaries

`engineering-journal` owns the durable record: effort-scoped, written for
someone reading in a year. This guide is change-scoped and written for one
reviewer this week. They can overlap, and the guide may restate a decision
rather than making the reviewer click through. The guide never becomes the
durable record — when an entry exists, link it.

`technical-prose` owns word choice, and every guide is held to it. Three of its
rules carry most of the weight here:

- **Modality.** A judgment call written with `should` is not a softer claim, it
  is a different one, and the reviewer who treats it as optional has read you
  correctly. A risk that `might` happen is one that `can` happen. Permission-
  sense `may not` survives intact.
- **Words that carry no fact.** "Gracefully handles" in a testing row names a
  quality of the handling instead of the handling, which is the failure the
  Testing section exists to prevent. "Robust" asserts what the reviewer cannot
  check; if the property is real it has a measurement, and the measurement is
  the row.
- **One name per thing.** A guide that calls one component three names across
  its sections destroys the association each section was building.

One interaction belongs to this skill alone. A guide quotes evidence: real
command output in Testing, a requirement sentence in the certainty section, a
flag or a path under Where to look first. Those are untouchable under that skill's
own rule, and a word-level pass that edits a pasted result, a quoted error, or
an identifier has broken the evidence it was cleaning.

`sweep-comments` owns the comments inside the diff; a comment defect belongs in
the review, not in the guide's prose.

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
| "I will add the diagram if someone asks." | Nobody asks. They review the wrong thing instead. |
| "Anyone reviewing this already knows the subsystem." | Then one line saying so releases them. Assuming it strands everyone else. |
| "The details are the review; context is padding." | A detail with nothing to attach to is not reviewed, only read. |
| "Every PR deserves a full guide." | A guide with nothing in it teaches reviewers to skim the next one. |
| "The title already says it." | Then the TL;DR says what it asks of the reviewer, which a title cannot. |
| "The identifier is the change." | The reviewer cannot resolve a name in sentence one. Name the motion, not the symbol. |
| "Saying why it exists is context, not summary." | A summary of an edit nobody can place is not a summary. |
| "The change was trivial, so I skipped the check." | Run it anyway and say the four were empty. Silence looks like not looking. |

## Red flags

- The body opens with a heading instead of a TL;DR.
- The ask is folded into the summary instead of standing as its own list.
- The look-out list is absent rather than saying nothing needs a decision,
  or contains an item the author could have settled.
- A numbered item is never marked again where its evidence appears.
- A cross-reference names a section by description rather than by its heading,
  so the reviewer has to guess which one was meant.
- An identifier the change did not introduce is used with no gloss, as though
  the reviewer should already hold it.
- A change is stated as a fact rather than as the response to a problem the
  paragraph just named.
- A sentence restates the previous sentence's point from another angle, most
  often by stating the "before" twice — once plainly, once as the problem.
- The TL;DR restates the title, or says what changed without saying what it
  asks of the reviewer.
- The TL;DR answers none of why, what the idea is, and what is true
  afterwards — or runs longer than those three answers need.
- The TL;DR names a type, field, function, flag, or file path.
- A reader who has not seen the codebase cannot say, from the TL;DR, what
  problem the change solves.
- The first concrete detail — a type, a field, a function — arrives before
  the reader has been given anything to attach it to.
- The mental model is a subsystem tour that would fit any pull request
  against the same files.
- Concepts with a shape are described in prose where a diagram was owed.
- The body restates the diff, file by file.
- Reading order matches path order.
- No section names anything as safe to skim.
- Testing reports a percentage instead of a gap.
- A command is described as run without its actual output.
- A line number arrived at by inference rather than by opening the line.
- A durable document cites a line with no phrase quoted from it.
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
