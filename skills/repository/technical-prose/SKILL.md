---
name: technical-prose
description: |
  State the word-level bar for technical prose: no hedged modality, no
  vocabulary that carries no fact, one name per thing. Use when asked to
  "de-slop this", "tighten this wording", "make this plainer", "cut the AI
  voice", or when another skill defers here for word choice. Symptoms that this
  skill applies: requirements softened to "should", possibility written as
  "may" or "could", intensifiers standing in for measurements ("robust",
  "blazingly fast", "seamlessly"), throat-clearing openers ("it is worth noting
  that"), a behavior described as "gracefully handled" rather than stated, the
  same thing called three names across one document, or an action buried in a
  nominalization ("perform compression of" for "compress"), a referent that
  resolves only for someone who was in the room ("this arc", "the fix"), or a
  metaphor standing where the fact should be. Also use for a cold read: "would
  this make sense to someone with no context?"
---

# Technical prose

This skill owns **what carries a fact to its reader** — at the scale of a word,
and at the scale of the material around it. It does not own sentence shape.

That boundary is not a division of labor; it follows from what varies between
readers. A word carries the same fact wherever it lands — "gracefully handles"
is a promise with no content in a doc comment, a README, and an error message
alike, so one rule serves all three. Sentence shape does not travel: a doc
comment sits beneath a declaration that already supplies its subject and its
article, while a README paragraph stands alone and must supply both. A rule that
mandated complete grammar everywhere would be right for one and wrong for the
other.

So the calling skill decides sentence length, articles, and whether fragments
are correct at its sites. This skill never overrides those, and a rewrite that
converts a fragment to a sentence — or the reverse — is out of scope here.

The same test admits a paragraph: one restating the paragraph above it carries
no fact anywhere, exactly as the empty word does not. So the ladder below is in
scope and fragment-versus-sentence is not — one asks whether material earns its
place, the other what shape the survivors take.

## The reader is not you

Name the reader before writing: who they are, and what they lack that you hold
now — the diff, the thread, the meeting, the last hour. Every rule below is
applied for that reader. **The default failure is writing for yourself a minute
ago.** A writer cannot un-know what they know, so the gap never shows from the
inside.

**The cold read.** Reread as the named reader. Every phrase that resolves only
because you were in the room gets its referent stated: "this arc" becomes the
review it names; "operator-chosen label" becomes what the label is and where it
lives. Grammar does not catch these: "this arc" is well formed and carries
nothing.

**Content before style.** Run the cold read before the word pass. Tightening
cannot find a missing noun: in the session that produced this rule, two
rewrites of one commit body each carried an unresolved referent through
untouched, and each was caught by a reader asking "what's that?"

**What ships with the text sets the bar.** A commit body ships with its diff. A
backlog entry, a journal line, or a message found by grep ships alone.

**A metaphor standing where the fact should be** is the same failure at the
word scale. "A wrong answer wearing a right one's clothes" costs every future
reader a decoding to recover "output that looks correct and is not".

## Modality

Modality is the load-bearing part of a technical sentence, because it is what
separates a requirement from an observation. It is also where hedging does its
damage: a requirement written as a suggestion is not a softer requirement, it is
a different claim, and a reader who acts on the difference is acting correctly.

| You wrote | Write instead |
|---|---|
| `should` (requirement) | `must` |
| `should` (recommendation) | Delete it, or state the fact: "X is faster because Y." |
| `might`, `could`, `may` (possibility) | `can` |
| `would` (hypothetical) | Restructure: "If X occurs, Y occurs." |

**The one exception: `may not` is a prohibition, and it survives intact.** The
ladder above maps possibility onto `can`, which is correct for "this may fail"
but destroys "callers may not hold the lock across an await". Rewriting that to
`cannot` claims the compiler prevents it, and to `can` inverts the rule. When
`may` marks permission rather than possibility, leave it alone.

Prohibitions and obligations keep `must`, `may not`, or `never` in every form,
including shortened ones. Dropping the modal to save a line converts a rule into
a description of the present, which licenses the very edit the sentence existed
to prevent.

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

1. **Nothing.** The named reader already holds it, or a sentence nearby
   already says it. Not the writer: this is the rung the curse of knowledge
   picks.
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
| "'Should' is softer and more collegial" | It is not softer, it is different. A requirement written as a suggestion gets treated as optional, and the reader who skips it followed your text correctly. |
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
| "I tightened it, so it is done." | Tightening finds needless words, not missing ones. A missing referent survives every style pass; only a cold read finds it. |

## Red flags

- A demonstrative with no antecedent in the text: "this arc", "that fix", "the
  review".
- A noun phrase that resolves only for someone who was in the room.
- A tightening pass that finished without a cold read.
- A metaphor or coined compound standing where the fact should be.
- A requirement whose only modal is `should`.
- A paragraph that would be equally true with its first sentence deleted.
- A metaphor restating the rule its own paragraph just stated.
- An opening that establishes an audience the rule already implies.
- A document that states a bar its own prose does not meet.
- `may` rewritten to `can` without checking whether it marked permission.
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

The reader rule follows Steven Pinker, *The Sense of Style*, on the curse of
knowledge as the primary cause of unclear expert writing, and Google's
technical writing guidance on knowing the audience.

This skill is unaffiliated with ASD and STEMG and is not an STE compliance tool.
ASD-STE100 is a registered trademark of ASD; the official standard is a free
download at asd-ste100.org.
