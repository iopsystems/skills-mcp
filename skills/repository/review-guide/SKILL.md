---
name: review-guide
description: |
  Draft or revise a pull-request body for its reviewer: the problem and resulting
  behavior, consequential choices, where to look closely, and validation with
  its limits. Use when asked to "draft the PR body", "write the PR description",
  "make this reviewable", or when opening or updating a pull request. Assess
  every change; publish only what helps evaluate it. Symptoms include a body
  that retells the diff, buries a review decision, lists checks without their
  limits, or implies that reported fixes have been independently verified.
  Beta — reader outcomes remain unmeasured; record concrete feedback through
  `engineering-journal`.
---

# Review guide

Write for a competent engineer reviewing this change without the author's
conversation history. The reviewer has the diff. The body supplies the reason
for the change, the consequences that need judgment, and evidence the diff
cannot provide.

The default output is the pull-request body. Do not create a checked-in guide
unless requested. Follow repository conventions for PR descriptions, including
required templates or retained commit messages; otherwise let the review needs
determine the structure.

## Assess broadly; publish selectively

Consider each of these before drafting:

- What problem does the change solve, and what behavior results?
- Which choices or assumptions affect whether it is acceptable?
- Where would a missed defect matter most, and what is safe to skim?
- What ran, what did it establish, and what material gaps remain?
- What depends on deployment conditions unavailable in local checks?

These are assessment questions, not required sections. Publish only findings
that help the reviewer understand, check, or decide on this change.

A mechanical change can need one sentence plus relevant validation. Crossing
subsystems or making a performance claim calls for closer assessment; neither
automatically earns every section. A two-line change with an untested failure
path still needs that gap stated. Omit empty categories without announcing that
the checks found nothing. Do not invent asks, risks, or a reading order to show
that the assessment happened.

Preserve consequential rationale before summaries, incidental details, or
process history. Do not impose a fixed word count.

## Open with the concrete change

Lead with the problem and resulting behavior. Include the central mechanism
when it helps connect them. A trigger and before/after example often do this in
one or two sentences; a complex change can need more context.

For example:

> A timed-out upload can be retried indefinitely. This change stops retries
> when the configured budget is exhausted and returns the last upload error.

Do not replace concrete behavior with claims such as "improves reliability,"
or invent a before-state or motivation to complete the opening.

Identifiers are welcome when they make the change more precise. Explain an
unfamiliar concept at first use, including concepts that predate the diff.
For example, "Each stream's configuration (`StreamSpec`) now records its origin
host" names both the concept and the type. A list of renamed fields without
their purpose still fails the reader.

If a decision or risk needs attention, make it findable near the opening.
State a simple ask directly when the context is already clear. If it needs
substantial explanation, point to the section that supplies it. Do not require
a separate routing paragraph, a count of asks, or a statement that no decision
is needed. A material limitation belongs in the opening when omitting it would
make the promised result misleading; explain it fully once where it is useful.

If the body retains concatenated commit messages, check whether later commits
supersede earlier claims. When they do, add a current-state synopsis before the
history so the reviewer need not reconstruct the final behavior. Keep any
required verbatim messages intact and identify them as historical. Do not
rewrite Git history merely to improve the PR body.

## Supply only the missing context

Explain the concepts and invariants needed to evaluate this change. Give each
concept one name and keep it consistent. A clause can introduce a familiar
mechanism; interacting ownership rules or a topology can need a paragraph or a
diagram. Add a separate mental-model section only when that organization helps.

Explain a before/problem/change sequence once where it clarifies a consequence.
If the opening supplies the model, continue to the review evidence.

Cut generic subsystem tours. Keep invariants needed to review this change,
even when they also matter to other changes.

In a stack, link the base guide's shared model and explain this change's delta.
Do not reproduce the same subsystem explanation or diagram in every PR. When
pointing to another section, use its actual heading or a direct link.

## Distinguish rationale from open decisions

Explain consequential settled choices without asking for reconfirmation. Omit
incidental reversible values; a timeout or retention threshold still deserves
attention when its effect matters.

For a real open decision, give enough context to understand the alternatives,
link the evidence, and state what needs deciding. Include the proposed choice,
its tradeoff, and what evidence would change it. If the context is already in
the opening, a short question can suffice. Use separate bullets or subsections
when several decisions need independent answers.

For example:

```markdown
### Deployment-specific source metadata

The same stream registry runs on the vessel and in the simulator, but only
vessel sensors label their own records. The proposed registry flag describes
the vessel, with a simulator override.

`packages/agrippa-config/src/streams.rs:290`

The override keeps this change small but needs another special case for a
third deployment with different behavior. Should the registry represent
that deployment difference directly before this merges?
```

Order open decisions by their effect on accepting the change. The central
design choice usually comes first; a blocking correctness risk can outrank it.
Do not demote an issue simply because it is peripheral to the feature's purpose.
State which decisions block the merge and distinguish decisions from requests
to confirm a fact or accept a risk. Do not infer approval from a lack of response.

Ground uncertainty in evidence: an ambiguous requirement, an uncovered path,
an unverified assumption, a missing measurement, or conflicting contracts.
A concern without a rejected alternative can still be a valid test gap or
risk; do not force it into a design-decision format. Omit vague worry and
self-reported confidence ratings. Preserve uncertainty the evidence leaves open.

## Direct review attention

Rank by the consequence of a missed defect and the chance it will be overlooked,
not file size, alphabetical order, or the order of edits. Consider:

- Code carrying a consequential judgment call.
- Correctness that depends on a caller, invariant, or ordering outside the diff.
- Behavior no test exercises.
- Assumptions the author has not verified.

Point to the specific place and explain what to check there. Name mechanical
renames, generated output, or repeated edits as safe to skim when that saves
attention. Omit the reading list when it adds nothing to the diff.

Place this guidance where the reviewer can use it. It can lead after a short
opening, follow necessary context, or accompany an open decision. A reference
to an explanation is enough; do not repeat the explanation in the reading list.

**In the pull-request body**, cite the relevant `path:line` or a source link
and state which commit the locations refer to. Read every cited line before
publishing. Use a commit-pinned link when possible so later edits do not change
what the citation shows.

**In a durable document**, a bare line number decays. Quote a phrase from the
cited line in the same paragraph so the number stays re-derivable from the
text. A check that finds the quoted phrase in the cited file can repair a
number that moved; it cannot repair a citation whose anchor no longer exists.

## Report validation at its actual strength

Inspect the relevant test surfaces: unit and functional tests, integration or
contract checks, built-artifact smoke tests, performance measurements, property
or fuzz tests, agent evaluations, and manual checks. This is a discovery aid,
not a list to reproduce in the body.

For material checks, report the command or method, the observed result, and
what it establishes. Name gaps that affect the claims of this change. For
example, "136 tests pass across seven binaries; none exercise the tap publish
path" is more useful than "all tests pass." Include the revision or environment
when it limits which changes the result supports.

Group routine successful checks. Link detailed logs or an existing journal
instead of copying the validation history. Keep the result and any material
limitation in the body so the reviewer need not click through to discover it.
A coverage percentage alone does not establish that the changed path works.
If a relevant check was skipped, say which and why; keep its claim unverified.
Never state that a command ran when it did not, or edit quoted output to improve
its wording.

Separate these claims:

- **Author report:** a fix has been implemented according to its author.
- **Observed check:** a named test or inspection passed at a stated revision.
- **Independent review:** another reviewer examined the current change and
  confirmed the relevant finding was addressed.

One does not imply the next, and none establishes correctness outside its
scope. "All issues resolved" or "only behavioral risk remains" requires evidence
for that breadth. Do not replace unresolved implementation questions with a
clean conclusion because the regression tests pass. Scenario files describing
expected agent behavior are not evidence that agents executed those scenarios.

For example, if the evidence consists of an author report and a passing
regression fixture:

> The author reports a fix for the scanner's attribute handling. Its regression
> fixture passes; no independent review of the fix is recorded.

That states the current review status without retelling each review round.

## Explain material deployment risks

Include deployment-dependent risks that affect acceptance: scale beyond local
tests, real data shapes, concurrency or partial failure, configuration,
migration and rollback, or upstream behavior. Do not add generic risks to fill
a section or imply every validation gap can only be checked in production.

For each material risk, identify what would reveal it: a relevant metric, log,
canary, or deployment check. If observability or mitigation is missing, state
that gap rather than inventing a signal. Ask for judgment or risk acceptance
when needed; omit an empty risk section.

## Use diagrams when they carry the explanation

Use `architecture-diagram` for structure and containment, or `dataflow-diagram`
for movement between stages or systems. A diagram earns its place when it makes
the relationships easier to check. Do not add one just because the change
crosses a boundary.

Link an existing checked-in diagram when it is current; flag it when this
change makes it stale. Use a small inline Mermaid diagram for an explanation
that serves only this review. A durable diagram belongs with its maintained
source and rendering. In a stack, share one diagram and identify each PR's part.

## Rewrite from evidence, then cold-read

**Re-read the source behind a section before rewriting that section.** Check
its claims against code, tests, requirements, and current review status.
Editing only the previous prose can preserve a false claim while making it
sound more certain. If the reader finds a section confusing, check whether
its premise is wrong before trying smoother wording.

When the scope changes, reassess the title, opening, review priorities, and
validation against the final diff. Rebuild the explanation around the final
change rather than appending an account of each new commit.

When revising an existing body, verify dependency and draft-blocker claims
against current PR status or supplied project records. A merged dependency no
longer blocks on merging, but its integration or validation may remain
unfinished. Remove obsolete blockers without implying readiness. If current
status is unavailable, identify the claim as unverified rather than repeating
it as current or assuming it is resolved.

After drafting, read as the reviewer and check:

- Can they identify the problem and resulting behavior from the opening?
- Does the opening agree with the detailed behavior and source? Check claims
  such as "once," "always," "never," and "for its lifetime" against retry,
  replacement, and failure paths. Put material qualifications in the summary
  rather than making a later paragraph retract it.
- Can they find the consequential rationale, open decisions, and places
  needing close inspection without reading the work history?
- Can they distinguish what was checked from what remains unverified?
- Does each passage help them evaluate this change?

Judge a rewrite by what its text contributes, not its length. Preserve reasons,
conditions, material gaps, precise numbers, identifiers, commands, and quoted
errors.

## Boundaries and feedback

`technical-prose` owns relevance, wording, and preservation of claim strength
across artifacts. This skill owns what a PR reviewer needs and how to direct
review attention. `sweep-comments` owns the comments inside the diff.

`engineering-journal` owns the durable effort history, including experiments,
abandoned approaches, and detailed validation. Link it when useful, while
retaining the rationale and limitations needed to judge this PR. The body is
not a chronological account of producing the change.

Treat this skill as **beta**. The attention-ranking criteria and selective
publication rules need behavioral evaluation; passing repository checks does
not establish reviewer comprehension. Record specific feedback and resulting
changes through `engineering-journal`, including cases where context was cut
too far. Do not treat a longer or shorter output alone as evidence of improvement.
