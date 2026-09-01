---
name: review-guide
description: |
  Draft the pull-request body as a guide for the human reviewing it: where to
  look more closely, what was tested and what was not, which calls were
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

The output is the pull-request body; "the guide" and "the body" both mean it.
Do not create a file. A checked-in document is a different request.

## What this is, and what it is not yet

Treat this skill as **beta**. Both central rules are asserted: the attention
ranking has never been checked against where defects were found, and the publish
test is a judgment about what a diff cannot carry. No guide it produced has been
read back by a reviewer.

Use them wholesale anyway — a stated default that fails teaches more than a
hedge that cannot. When one fights the change in front of you, record the
override and its reason through `engineering-journal`.

Record the defaults that held, too. A channel collecting only complaints will
retire rules that were working.

## Always run; publish only when it earns it

Run on every change: a change with nothing to say is a fact about the change,
not a reason to have skipped looking.

What you publish is proportional to the change and to its complexity. A
one-file mechanical edit earns a sentence. A change that crosses subsystems,
alters an interface, or makes a performance claim earns every section. Length
is a consequence of what the change actually carries, never a target.

**When accessibility and brevity conflict, accessibility wins.** Establishing a
starting point the reviewer does not hold costs a paragraph and buys the section
it opens.

### The publish test

Publish a guide when it carries at least one item the reviewer could not get
from the diff itself:

- a reading order that differs from the order the diff presents
- a test gap
- a judgment call
- a production-only risk

If all four are empty, do not publish a guide. Write one sentence describing the
change and state that you checked those four and found nothing: silence is
indistinguishable from not having looked.

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

Close with a pointer to the ask, **as its own paragraph** — fused to the summary
it disappears into the sentence before it. A pointer, not the asks themselves,
because naming a decision costs the context that makes it one: "three decisions
want your opinion, none blocking; each is its own subsection under Decisions" is
the whole paragraph. Never leave it out — an absent ask and an empty one read
identically, so when nothing needs the reviewer it says so.

A small change collapses all three answers into one sentence, because its why,
its idea, and its effect are the same fact seen three ways.

A TL;DR that restates the title is wasted. So is one that opens a list the
reviewer must then read to use.

**No identifiers.** A type, field, function, flag, or file path in the TL;DR is
the strongest available signal that you described the change instead of the
reason for it. The reviewer meets `StreamSpec` before they know what a stream
is, and a name they cannot resolve costs them the sentence it appears in. Names
belong under Where to look more closely, after the mental model has given them
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

### One subsection per decision

A decision the reviewer can act on needs three things in front of it, in this
order, and the order is the rule:

1. **The context.** What the arrangement was, and what about it forces a choice.
   Written so someone who has not opened the code can follow it.
2. **The reference.** `path:line` for the place the choice is made, so the
   reviewer can go and look rather than take the description on trust.
3. **The question**, last, once both of those are standing. What was chosen,
   what was rejected, and what would change it.

Each gets its own subsection under `## Decisions`, with a heading that names it,
so a reviewer can answer one and leave the rest and a thread can point at it.

```markdown
## Decisions

### One repository-wide setting for a fact that differs by deployment

The registry is deployment-independent: one row per stream, the same on the boat
and in the simulator. But whether a sensor labels its own records is not the
same in both — it is true on the boat and false in the simulator, which
publishes a sensor's topics without running the sensor.

`packages/agrippa-config/src/streams.rs:290`

I chose one flag stating the vessel's truth, plus an explicit override for the
simulator, rather than teaching the registry about deployment profiles. The
override stops scaling at a third deployment that differs again.

**Is a single flag the right shape here, or should the registry carry the
deployment dimension?**
```

Rules:

- **Context before question, always.** A question the reviewer cannot yet parse
  is a delay, not an ask: they carry it until the context arrives, then come
  back. A subsection with a question and no context is that failure with extra
  structure.
- **Only what the reviewer can settle.** A decision you already know the answer
  to is a quiz. Settle it in the guide and leave it out.
- **Say what each needs** — a decision, a confirmation of fact, or an
  acknowledgment of a risk. Those cost different amounts.
- **Say which block the merge**, in the subsection, not only in the pointer.
- **One per subsection.** Two under one heading get answered as one, and usually
  only the first.

**Wanting an answer is not enough to earn a subsection**, which costs a context
paragraph, a reference, and a question whatever the item is worth. Two kinds go
in the recorded list instead, however curious you are: one whose context is
already visible in the diff, so the paragraph would narrate what the reviewer is
looking at; and one turning on a value rather than an approach — a width, a
name, a threshold — where one edit reverses the choice. A clause each, under a
sentence saying they are recorded rather than asked. That is the third tier:
what the change is for, what you are unsure of, what you are only recording.

### Then the mental model

**The mental model comes next**, before any section that names a type; its
content is below under The mental model.

Below are the answers a guide owes its reviewer, in order — not a list of
headings to fill. **A heading is earned by its content.** An answer needing a
clause is a clause in the opening paragraph; one needing a paragraph gets its
heading. A change crossing subsystems grows all seven; a one-file addition
answers most in its first sentence and heads the two with something in them.

The two failures look identical from outside and are opposites. Dropping an
answer publishes a guide that cannot say why it exists. Heading a one-clause
answer pads it to look thorough, and that lands hardest on the reviewer of a
small change, who reads seven sections to find the two that mattered.

Answer every one. Give a heading only where it is earned:

1. **TL;DR.** Why, the key idea, and what is true afterwards — as long as those
   three answers need, and no longer.
2. **The mental model.** The concepts the reviewer needs before any detail means
   anything, and where this change sits among them. A diagram when they have a
   shape.
3. **What changed, and the claim it makes.** One paragraph expanding the TL;DR
   rather than repeating it. The claim is what would be false if the change
   were wrong.
4. **Decisions.** One subsection per decision that wants the reviewer, each
   laying out its context before it asks its question. Then the calls that need
   no answer, as a short list.
5. **Where to look more closely.** Ranked reading order, plus what is safe to
   skim.
6. **Testing.** Methodology, what ran, its actual output, and the gaps.
7. **Production-only risks**, with a direct invitation to weigh in.

Present when earned:

8. **Not in scope**, when a reader would otherwise ask why something is missing.

Drop an answer only when the change genuinely has nothing in it, never to reach
a size: any answer holding an item that cleared the publish test stays. Losing
its *heading* is not dropping it — a one-sentence testing answer is one sentence
in the opening paragraph, and a heading over it is furniture.

A change that cleared nothing at all reduces to the TL;DR, and nothing follows
it. Decisions is the exception to emptiness: it never disappears silently — it
says "none, and here is why".

## The mental model

A reviewer who meets a field name before they know what the thing holding it is
for has nowhere to put it. Type, field, and function names are the last thing a
guide reaches for, not the first.

Establish, in this order:

1. **What the system does at the level this change touches.** One or two
   sentences in the domain's terms, not the code's.
2. **The two or three concepts the change depends on.** Name each once and use
   that name everywhere after. A concept the guide leans on without naming is a
   concept the reviewer rebuilds from the diff, which is the work the guide
   exists to do for them.
3. **Where this change sits** among those concepts, and what it moves.

State the starting point you assume, and name where to jump by its heading:
"This assumes you know the ring protocol; skip to Where to look more closely
if you do." That costs one line and releases the reviewer who already holds the model.

A cross-reference names a section exactly as its heading reads. "The reading
order" and "the certainty section" are descriptions, not addresses; a reviewer
sent to one has to guess which heading was meant.

Reach for a diagram when the concepts have a shape — a topology, a pipeline, a
set of hosts, a before and after. Prose carries a sequence; re-deriving a shape
from one is work pushed back onto the reviewer. See Diagrams for which kind.

Two failure modes:

- **A subsystem tour.** This section gives the reviewer what this change needs,
  not what the subsystem is. If a paragraph would be equally true of a different
  pull request against the same files, cut it.
- **Restating the diff in prose.** The mental model is what holds before the
  change, plus where the change lands. It is not the change.

Omit it only when the change touches one concept the reviewer certainly holds: a
typo, a version bump, a rename inside one file. The burden is on omitting.

**In a stack, the model is written once.** Retyped per pull request it produces
near-identical sections that disagree after the first edit, and makes a reviewer
working up the stack reread a page they hold. Write it in the base guide, link
it from each guide above by pull-request number, and state only this change's
delta: what it adds, or what it makes false.

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

Three beats, three sentences, each appearing once. The common inflation is
stating the "before" twice: plainly, and then again as the problem.

**Gloss what you did not introduce.** Below the TL;DR, identifiers are allowed;
undefined ones are not. One the change **adds** introduces itself, in the
sentence that adds it. One that **already existed** has no such sentence, which
is why it is the one left undefined, and the guide then reads as though the
reviewer should have known it. The "before" beat gives it one clause, in the
domain's terms: not its type, its job.

<!-- cite-ignore -->
> **Undefined.** `StreamSpec.topic` was answering two questions at once.

> **Glossed, in the before beat.** Every stream has a registry entry saying
> where it comes from and how it is carried, and one field on that entry named
> the message-bus topic it arrives on.

Same work. The second can be read by someone who has never opened the file.

## Rewriting a guide

Rewriting is not a text edit. Every sentence in a guide is a claim about code,
and prose carries no trace of whether it was ever checked — a fabricated
sentence and a verified one look identical, and the fabricated one often reads
better, because nothing constrained it.

**Re-read the source behind a section before rewriting that section.** Not the
section. The source. Editing prose from prose is how a wrong claim survives
every revision: each draft inherits the last one's errors and adds fluency.

The structure can be right — a before beat, a problem, a change, in that order —
with the before beat filled by an arrangement that never existed. A reader who
does not know the code cannot tell, and a reviewer who does stops trusting the
rest.

When a reader says a section does not make sense, that is evidence the claim is
wrong, not only that the wording is. Go back to the code before reaching for
better sentences. The temptation runs the other way, because rewording is fast
and re-reading is not.

## Decisions

This is the section the reviewer cannot reconstruct alone, and the one faked
most often in both directions. Its shape — context, reference, question, one
subsection each — is under One subsection per decision above.

**Two things earn a reviewer's attention, and they are not equal.** The first is
what the change is *for* — the choice the whole change rests on, the one that
would make the work wrong if it were wrong. The second is everything the author
happens to be unsure about. They often overlap, and when they do the item
appears once, at the top.

So the section is ordered by centrality — not by severity, and not by the order
the questions occurred to you. It opens with the decision the change is for,
then a line demoting the rest — "the rest is lower stakes: things I am less sure
of rather than what this change is for" — then the remaining subsections.

A leftover uncertainty presented beside the change's central question reads as
equally weighted, and the reviewer answers whichever is easiest.

When the point of the change is not itself in question, say so in one line and
go straight to the rest. A manufactured question about the central choice is
worse than admitting it was deducible.

Every subsection cites concrete evidence:

- a requirement sentence that admits two readings, quoted
- a path with no test
- an assumption made and never verified
- a measurement not taken
- an interface whose contract the caller and callee state differently

Every one carries the call that was made, the alternative that was rejected, and
what evidence would change it. Without those three it is a worry, not a finding;
drop it.

Two failure modes, both defects:

- **Manufactured uncertainty.** Hedging to look careful. If nothing was
  genuinely unsettled, say so and say why the change was deducible.
- **Suppressed uncertainty.** Presenting one reading of a vague requirement as
  the only reading. If a requirement was ambiguous and a choice was made, the
  reviewer owns that choice as much as the author.

Do not rate confidence. A number invites the reviewer to trust a self-report
that carries no information. The evidence carries the signal.

## Where to look more closely

This follows Decisions, so the reviewer arrives knowing what is asked of them
and reads for those answers rather than evenly — and an item here can point at a
decision by its heading instead of restating it.

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

The heading says *more closely*, not *first*: three sections in, what this ranks
is attention rather than order.

Then name what is safe to skim, and why: generated output, mechanical renames,
formatting, a change repeated identically across many files. A reviewer who
spends attention on a rename has none left for the invariant.

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

Read every line you cite, at the moment you cite it. Not most of them: checking
four anchors and estimating the fifth produces a body wrong in a way its own
care makes harder to doubt. A line number you did not open in this session is a
guess that looks exactly like a fact, and the reviewer cannot tell which one
they hold.

## Testing

Discover what the repository actually has before writing this section, rather
than assuming a taxonomy. Consider each surface and say which apply:

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

Then invite a response, and say which risks want a judgment and which want
acceptance. A risk nobody accepted is a risk nobody owns.

## Diagrams

A diagram belongs in the mental model, where it does the work prose is bad at:
carrying a shape. The mental model section says when to reach for one; this says
which kind.

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

A stacked series shares one shape. Draw it once and mark this change's piece in
each guide, rather than a different diagram per pull request.

When a change makes an existing diagram wrong, say so. A stale diagram left
unmentioned costs more than no diagram.

## Explain it, then name it

**Say what a thing does before you name it.** "A pull request runs automatic
checks before a change lands, and those were one of the four conditions" beats
"required PR checks were part of the clean-run gate" — including for the reader
who knows both terms, who then no longer unpacks a phrase to recover the fact.

Hardest on vocabulary this change invented. A phrase coined in the design
discussion is jargon everyone outside it lacks, and the one an author will not
notice using.

**Precision is untouched.** Numbers, identifiers, paths, and quoted errors stay
exact. The register changes; the claims do not.

**Length is not the objection.** Plain wording runs longer, and proportionality
counts answers rather than words: three plain sentences are still one answer.
Padding is material with nothing in it.

The test: **could a competent engineer outside this subsystem act on it?**

## Boundaries

`engineering-journal` owns the durable record: effort-scoped, written for
someone reading in a year. This guide is change-scoped and written for one
reviewer this week. They can overlap, and the guide may restate a decision
rather than making the reviewer click through. The guide never becomes the
durable record — when an entry exists, link it.

`technical-prose` owns what carries a fact — a word, and the material around it
— and every guide is held to it. Three of its rules carry most of the weight
here:

- **Modality.** A judgment call written with `should` reads as optional, and the
  reviewer who treats it so has read you correctly. Permission-sense `may not`
  survives intact.
- **Words that carry no fact.** "Gracefully handles" in a testing row names a
  quality of the handling instead of the handling — the failure Testing exists
  to prevent. "Robust" asserts what the reviewer cannot check; if the property
  is real it has a measurement, and the measurement is the row.
- **One name per thing.** Three names for one component across three sections
  destroys the association each section was building.

One interaction belongs to this skill alone. A guide quotes evidence — real
command output in Testing, a requirement sentence under Decisions, a flag or a
path under Where to look more closely — and a word-level pass that edits a
pasted result, a quoted error, or an identifier has broken the evidence it was
cleaning.

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
| "Each guide in the stack should stand alone." | Then each one restates the model, and the copies disagree by the third edit. Link the base and state the delta. |
| "The look-out list makes the asks impossible to miss." | It states them where the reviewer cannot follow them yet. A decision needs its context in front of it, which is why each gets a subsection. |
| "Listing the asks up front is friendlier than making them scroll." | Not when the list is unreadable there. The summary points; the subsection asks. |
| "All the open questions matter, so they go in one flat list." | Then the reviewer answers the easiest one. Lead with what the change is for; the rest is a by-the-way. |
| "Leading with the central choice buries my real worry." | If the worry is what the change is for, it *is* the central choice. If it is not, it is a by-the-way, and saying so is what makes the first one findable. |
| "Establishing the before-state makes it long." | Length is not the cost. A reader who cannot follow it is. |
| "I am only rewording this section." | The words are claims about code. Re-read the code, or you are polishing a guess. |
| "'Writes nothing' is obviously about the disk." | It writes to buffers and publishes to the bus. Name the scope or the claim is false. |
| "The reader found it confusing, so it needs clearer prose." | It may need a truer claim. Check the source before the sentences. |
| "The title already says it." | Then the TL;DR says what it asks of the reviewer, which a title cannot. |
| "The identifier is the change." | The reviewer cannot resolve a name in sentence one. Name the motion, not the symbol. |
| "Saying why it exists is context, not summary." | A summary of an edit nobody can place is not a summary. |
| "The change was trivial, so I skipped the check." | Run it anyway and say the four were empty. Silence looks like not looking. |
| "Anyone reviewing this knows the term." | The reader who knows still pays to unpack it. The one who does not is stuck. |
| "That is just the standard term." | Standard among the people in the design discussion is not standard. |

## Red flags

- The body opens with a heading instead of a TL;DR.
- The ask is fused onto the last clause of the summary instead of standing as
  its own paragraph.
- The ask is missing rather than saying nothing needs a decision, or contains
  an item the author could have settled.
- A decision is stated before the context that makes it a decision, so the
  reviewer meets the question before they can parse it.
- The summary names the decisions instead of pointing at them, which costs the
  context they need and gives the reviewer the question twice.
- Two decisions share one subsection, so they get answered as one — usually
  only the first.
- The decisions are ordered by when they occurred to the author, or by severity,
  rather than by which one the change is for.
- A Decisions section longer than the rest of the body combined.
- A subsection asking about a value — a width, a name, a label — that one edit
  would reverse.
- A heading over a single sentence, or more headings than items that cleared the
  publish test.
- A section that would read the same on every change of its kind: "no test
  covers README content" is a category, not a testing answer.
- A leftover uncertainty sits above the central question, or beside it with
  nothing marking the two as different weights.
- A decision has a question and no reference, so the reviewer has to take the
  description of the code on trust.
- A stacked guide restates the shared mental model instead of linking the guide
  that holds it and stating this change's delta.
- A cross-reference names a section by description rather than by its heading,
  so the reviewer has to guess which one was meant.
- An identifier the change did not introduce is used with no gloss, as though
  the reviewer should already hold it.
- A term of art in the summary or the ask that the guide never unpacks.
- A phrase coined during this change's own design, used as established
  vocabulary.
- Precision traded for readability: a number rounded or a quoted error
  paraphrased so a sentence flows.
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
- A section was rewritten without re-reading the code it describes.
- A before beat describes an arrangement that never existed, in the right shape.
- An absolute — writes nothing, never blocks, always succeeds — with no scope
  named, in a system where the verb has more than one meaning.
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
