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

Edit wording, sentence complexity, and relevance to the reader. Follow the
calling skill's rules for articles, fragments, structure, and required content.
Do not convert fragments to sentences or vice versa. Keep material that helps
the reader understand, decide, or act.

## The reader is not you

**Naming the reader comes first.** Identify who will read this and what context
they lack. Use the reader named by the user or calling skill, or established by
the artifact: a doc comment serves the next editor; a commit body accompanies
the log and diff. **When neither settles it, ask.**

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

## One item, one name

Use one term per thing throughout the document and its referring sites. Calling
one component "the executor", "the runner", and "the scheduler" suggests three
components. Also check the reverse: one name must not conceal two things.

## One spelling: American

Write `color`, not `colour`; `behavior`, not `behaviour`; `-ize`, not `-ise`;
`gray`, not `grey`; `judgment` and `acknowledgment` without the middle `e`.

Two exceptions:

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

Choose a rung after understanding the material. Brevity does not excuse an
unsupported claim.

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
