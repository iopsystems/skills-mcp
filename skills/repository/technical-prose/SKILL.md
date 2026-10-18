---
name: technical-prose
description: |
  Edit technical prose for reader relevance, clear wording, and claims
  supported by evidence. Use for "de-slop this", "tighten this wording",
  "make this plainer", "cut the AI voice", a cold read, or when another skill
  requests a prose pass. Symptoms include irrelevant detail, insider referents,
  repeated explanations, empty adjectives, inconsistent names, clause chains,
  softened requirements, or wording more certain than the evidence. Identify
  the reader from the request or artifact; leave artifact structure to its
  calling skill or conventions.
---

# Technical prose

This skill owns **what carries a fact to its reader** — at the scale of a word,
and at the scale of the material around it, including how many claims one
sentence carries. It does not own whether a unit is a fragment or a sentence.

That boundary is not a division of labor; it follows from what varies between
readers. A word carries the same fact wherever it lands — "gracefully handles"
is a promise with no content in a doc comment, a README, and an error message
alike, so one rule serves all three. So does a clause chain: three claims
joined by "and" cost the same reading in a doc comment and a README. Grammar
does not travel: a doc comment sits beneath a declaration that already
supplies its subject and its article, while a README paragraph stands alone
and must supply both. A rule that mandated complete grammar everywhere would
be right for one and wrong for the other.

So the calling skill decides articles and whether fragments are correct at its
sites. This skill never overrides those, and a rewrite that converts a fragment
to a sentence — or the reverse — is out of scope here.

The same test admits a paragraph. A true, nonrepeated fact still needs to help
this reader understand, decide, or act. This skill owns that relevance test;
the calling skill owns the artifact's purpose and required content.

## The reader is not you

**Naming the reader comes first.** Nothing below is applied until you can say
who will read this and what they lack that you hold now — the diff, the
thread, the meeting, the last hour. The reader is inherited when the user or a
calling skill names one, and inferred when the artifact does: a doc comment is
read by the file's next editor, a commit body by whoever has the log and the
diff open. **When neither settles it, ask.** A reader chosen on a guess is
the default failure with the guess in your place: writing for yourself a
minute ago. A writer cannot un-know what they know, so the gap never shows
from the inside.

**The cold read.** Reread as the named reader. Every phrase that resolves only
because you were in the room gets its referent stated: "this arc" becomes the
review it names; "operator-chosen label" becomes what the label is and where it
lives. Grammar does not catch these: "this arc" is well formed and carries
nothing.

**Content before style.** Run the cold read before the word pass. Resolve
missing referents, then ask what each passage helps the reader understand or
do. Cut facts that serve neither purpose, even when they are accurate and
unique. Do not remove content required by the user or the artifact's contract.

Keep the reason for a consequential choice before keeping the history of
making it. For example, "Count comments because the view measures source a
reader must navigate" explains a counting policy. "After two review rounds,
we settled on this policy" supplies no reason to accept it. Keep history when
the sequence itself explains a constraint, failure, or current state.

**What ships with the text sets the bar.** A commit body ships with its diff. A
backlog entry, a journal line, or a message found by grep ships alone.

**A metaphor standing where the fact should be** is the same failure at the
word scale. "A wrong answer wearing a right one's clothes" costs every future
reader a decoding to recover "output that looks correct and is not".

## Preserve obligation and uncertainty

Determine what a modal means before editing it. Use `must` for an established
requirement softened to `should`; do not promote a recommendation into an
obligation. A recommendation needs its reason or condition, not stronger wording.
Permission-sense `may` and `may not` retain their force: "may not hold the lock"
prohibits an action; "cannot hold the lock" claims it is impossible.

Use `can` for a known capability or failure mode. Keep `may`, `might`, or a
conditional when it expresses uncertainty the evidence has not resolved.
"The scanner might misclassify this syntax; it has not been tested" must not
become "The scanner misclassifies this syntax." A hypothesis is not an observed
failure. Remove a hedge only when removing it preserves the claim's strength.

Distinguish observation, attribution, and inference. "The author reports a
fix," "the regression tests pass," and "independent review confirmed the fix"
establish different facts. Do not turn any of them into "all issues resolved"
without evidence for that broader claim. State the relevant scope or remaining
gap once; do not add generic caveats to sound careful.

Prohibitions and obligations keep `must`, `may not`, or `never` in every form,
including shortened ones. Preserve negations, conditions, and uncertainty just
as carefully as numbers and identifiers.

## Words that carry no fact

The test is not formality; it is whether deleting the word changes what the
reader knows. If it does not, delete it rather than replacing it.

| Slop | Write instead |
|---|---|
| leverage, utilize | use |
| in order to | to |
| prior to, subsequent to | before, after |
| due to the fact that | because |
| in the event that | if |
| when it comes to | for |
| it is worth noting that, it is important to note | (delete — state the fact) |
| simply, just, easily, seamlessly, effortlessly | (delete) |
| robust, powerful, comprehensive, performant | (delete, or give the measurable property) |
| blazingly fast, state-of-the-art, battle-tested | (delete, or give the number) |
| enables you to, allows you to | you can |
| is designed to, aims to | (delete — say what it does) |
| gracefully handles | (say what it does: "retries three times, then fails") |
| under the hood | internally |
| out of the box | by default |
| plethora, myriad | many |
| streamline | make faster, make simpler |
| dive into, delve into | read, examine |
| functionality | function, feature |
| as needed, as appropriate | (state the condition) |
| and/or | pick one, or write "X, Y, or both" |
| e.g., i.e., etc. | for example, that is, (name the items) |

Two of these deserve their reason stated, because they look like content and are
not. **"Gracefully handles"** names a quality of the handling instead of the
handling, so a reader who needs to know what happens still does not; it is the
single most common way agent-written prose describes behavior without stating
it. **"Robust"** and its neighbors assert a property the reader cannot check —
if the property is real it has a measurement, and the measurement is the fact.

## One item, one name

Pick one term per thing and keep it for the whole document, and across a
document and the sites that point at it. Synonym rotation reads as variety to
the writer and as three distinct things to the reader, who has no way to know
that "the executor", "the runner", and "the scheduler" are one object.

This matters most at a boundary: when an explanation lives in one place and
other sites refer back to it, a renamed reference breaks the association it
existed to create.

Rotation is worth catching in both directions. One name for two things is the
worse failure of the two, and it hides better.

## One spelling: American

Write `color`, not `colour`; `behavior`, not `behaviour`; `-ize`, not `-ise`;
`gray`, not `grey`; `judgment` and `acknowledgment` without the middle `e`.

This is the same rule as one item, one name, applied to the word rather than
the thing. Mixed spelling inside one document reads as several authors who did
not talk to each other, and it costs the reader a half-second of wondering
whether the difference is meaningful. It never is.

Two carve-outs, both absolute:

- **The user's choice wins.** If the user or the project asks for British
  spelling, or for any other convention, use it and keep it consistent. This
  rule sets the default, not the policy.
- **Untouchables stay untouchable.** A `colour` inside an identifier, a flag, a
  configuration key, a quoted error, or a cited third-party title is a name.
  Renaming it breaks the artifact, exactly as the section below says.

## The rest

- **Actions are verbs, not nouns.** "Compress the file", not "perform
  compression of the file". A nominalized action needs a verb to carry it, and
  the verb it gets is always empty — perform, conduct, undertake.
- **Active voice**, except where the actor is genuinely unknown or irrelevant.
  Passive hides who acts, which is exactly the fact an instruction needs.
- **One claim per sentence.** A sentence chaining clauses with "and", "but",
  "so", or a dash hands the reader several claims to hold at once. Split it.
  Whether the pieces are sentences or fragments is the calling skill's call;
  that each carries one claim is this one's.
- **Parallel ideas in parallel form.** Items that do the same job get the
  same shape: "both need a label with whitespace, and both match the pattern"
  keeps its "both"; a list whose first item is a sentence and second a
  fragment has changed shape without changing meaning, and the reader looks
  for the meaning.
- **No phrasal verbs where a single verb exists**: "set up" is `configure` or
  `install`, "go down" is `decrease` or `stop`, "spin up" is `start`.
- **No Latin abbreviations.** "e.g." is "for example", "i.e." is "that is", and
  "etc." is a refusal to finish the list — name the items or write "and more".
- **State the condition before the command.** "If the network is slow, increase
  the timeout" beats the reverse: the reader who does not meet the condition
  stops reading one clause earlier, and never loads an instruction that does
  not apply to them.

## The cheapest form that carries the fact

Borrowed from [ponytail](https://github.com/DietrichGebert/ponytail), whose
ladder has an agent stop at the first rung that solves the problem rather than
writing the fullest thing it can. Walk it before writing, and again when
cutting. **Stop at the first rung that carries the fact intact.**

1. **Nothing.** The fact does not help this reader's task, the reader already
   holds it, or another sentence supplies it. Judge against what the reader
   has available, not what the writer knows.
2. **A word** added to a sentence that already exists.
3. **A clause.**
4. **A sentence.**
5. **A paragraph.**
6. **A section with a heading.**

The drift is upward and invisible one step at a time: a clause becomes a
sentence, then a paragraph with a lead-in, then a section, and no single step
looks wrong.

**Be lazy about the prose, never about the thinking.** A rung is chosen after
the material is understood, not instead of understanding it. A short paragraph
written to avoid working out what is true is worse than the long one it
replaced.

**Cut assertions before examples.** When a paragraph and the example under it
say the same thing, the example survives. It shows; the paragraph asserts, and
an assertion is what the reader was going to take on trust anyway.

**Never cut:** modality, the subject of a claim, negations, scope qualifiers,
quoted evidence, and everything under Untouchables. A rung that loses one of
these is not available, and the shortening that drops one is not a shorter
text — it is a different claim.

**A document that states a bar is subject to it.** An instruction file arguing
for succinctness in three hundred words has refuted itself, and its author is
the reader least likely to notice.

## Untouchables

Never rewrite these, even when they break every rule above:

- Code, inline code, identifiers, CLI commands, flags, and file paths. The `run`
  in `sqlpipe run --config` is a name, not a verb.
- Quoted error messages and log lines. They are matched against reality.
- Product names, API endpoints, and configuration keys.
- Anything inside a code fence.

A de-slop pass that edits an identifier has broken the artifact it was cleaning.
When a rule and a name conflict, the name wins silently.

## Rationalizations

| Excuse | Reality |
|---|---|
| "'Should' is softer and more collegial" | An established requirement needs must. A recommendation stays a recommendation; explain its reason. |
| "'May not' is banned modality, so it becomes 'cannot'" | `cannot` claims impossibility. If the thing were impossible you would not be writing a rule about it. Permission-sense `may` survives. |
| "'Robust' tells the reader it is well built" | It tells them you believe so. If there is a property behind the word it has a number, and the number is what they needed. |
| "Varying the word avoids repetition" | Repetition is not a defect in technical prose; it is how the reader knows it is the same thing. Save variety for writing that has a plot. |
| "The intensifier conveys confidence" | Confidence is conveyed by a fact that survives checking. An unfalsifiable adjective reads as marketing and gets discounted wholesale. |
| "This skill says complete sentences, so my fragments are wrong" | This skill rules on words, not sentence shape. Fragments are the calling skill's call, and in some places they are correct. |
| "Passive voice is more objective" | It is more anonymous. "The value is cached" leaves the reader asking by what, which is usually the fact they came for. |
| "'Etc.' implies the reader knows the rest" | It implies the writer stopped. If the list is obvious, finishing it costs a line; if it is not, "etc." dropped real content. |
| "The paragraph gives it room to breathe." | Name the fact the room carries. If there is none, the rung below fits. |
| "The rule needs its rationale spelled out." | Once. Not once as a metaphor, once as a restatement, and once as a summary. |
| "The reviewer knows what 'this arc' means." | Today. The text was written for whoever reads the log, the backlog, or the grep hit in six months, and they were not in the room. |
| "Asking who the reader is would be overhead; I'll assume an engineer." | An assumed reader is the writer's guess, and the writer is the one person who cannot check it. When the artifact does not settle it and nobody said, one question costs less than the rewrite. |
| "I tightened it, so it is done." | Tightening finds needless words, not missing ones. A missing referent survives every style pass; only a cold read finds it. |
| "Every sentence is true and unique." | A fact still needs to help this reader understand, decide, or act. |
| "The tests passed, so the issue is resolved." | Report what the tests establish. Preserve attribution and any material gap in review or coverage. |

## Red flags

- A reader chosen without the user, a calling skill, or the artifact naming
  one.
- A demonstrative with no antecedent in the text: "this arc", "that fix", "the
  review".
- A noun phrase that resolves only for someone who was in the room.
- A tightening pass that finished without a cold read.
- True details that do not serve the reader's task.
- Process history replacing the reason for a consequential choice.
- A rewrite that upgrades a hypothesis, author report, or passing test into a
  broader claim of correctness.
- A metaphor or coined compound standing where the fact should be.
- A sentence joining three or more clauses with "and", "but", "so", or a
  dash.
- An established requirement whose only modal is `should`, or a recommendation
  rewritten as an obligation.
- A paragraph that would be equally true with its first sentence deleted.
- A metaphor restating the rule its own paragraph just stated.
- An opening that establishes an audience the rule already implies.
- A document that states a bar its own prose does not meet.
- `may` rewritten to `can` without checking whether it marked permission or
  unresolved uncertainty.
- An adjective asserting a quality with no measurement anywhere near it.
- A sentence that survives deletion of its first six words with no loss.
- The same object under two names in one document, or under a different name at
  a site that points back to its explanation.
- "Perform", "conduct", or "undertake" followed by a noun that is really a verb.
- A rewrite whose diff touches an identifier, a flag, or a quoted error string.
- A condition trailing its command in an instruction.
- Two spellings of one word in a document, or a spelling changed inside an
  identifier or a quoted string.

## Attribution

The vocabulary substitutions are adapted under the MIT license from
[AminBlg/SimpleEnglish](https://github.com/AminBlg/SimpleEnglish), whose slop
table is that project's original work rather than ASD material. The remaining
rules paraphrase ASD-STE100 Simplified Technical English, restricted here to its
reader-agnostic vocabulary rules.

Five rules restate Strunk and White, *The Elements of Style*: omit needless
words, use the active voice, use definite and concrete language, keep parallel
ideas in parallel form, and avoid a succession of loose sentences.

The reader rule follows Steven Pinker, *The Sense of Style*, on the curse of
knowledge as the primary cause of unclear expert writing, and Google's
technical writing guidance on knowing the audience.

This skill is unaffiliated with ASD and STEMG and is not an STE compliance tool.
ASD-STE100 is a registered trademark of ASD; the official standard is a free
download at asd-ste100.org.
