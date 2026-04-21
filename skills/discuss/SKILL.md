---
name: discuss
description: |
  Capture a conversation or working session as a **discussion** (multiple attendees) or **session note** (solo author, often with an agent) in the knowledge-iop vault, AND surface what existing artifacts the conversation implicates. Discussions are the vault's napkin — freely editable and deletable — but their real value is triaging what they imply. Use this whenever the user says things like "let's write this down", "record the meeting", "capture this session", "note the decision points", "what did we discuss", or when a conversation has clearly concluded a loop that should be logged. This skill also surfaces follow-up actions: does this discussion invalidate an existing brief? does it warrant a new inquiry? does it frame a problem that isn't in the vault yet?
---

# Discuss

You are capturing a conversation in the knowledge-iop vault. There
are two flavors — pick based on the context:

- **Discussion** — multiple attendees, agenda-ish topics, what was
  said, what was decided.
- **Session note** — solo author, often with an agent, reflecting on
  a working session. No attendees.

Both are freely editable and freely deletable. Treat them like
napkins — but triage their implications before wrapping up.

## Frontmatter shape

### Discussion
```yaml
---
id:          <YYYY-MM-DD>-<short-slug>
type:        discussion
status:      <omit — discussions have no status>
author:      <who wrote it up>
created:     <YYYY-MM-DD>
attendees:   [<person>, <person>, ...]
arc:         <arc-id | omit>
scopes:      [<scope-id>, ...]
relates_to:  [<id>, ...]
---
```

### Session note
```yaml
---
id:          <YYYY-MM-DD>-<short-slug>-session
type:        session-note
author:      <person>
created:     <YYYY-MM-DD>
arc:         <arc-id | omit>
scopes:      [<scope-id>, ...]
relates_to:  [<id>, ...]
---
```

## Filename

Write to `discussions/<id>.md`.

## Body

A loose structure serves best — rigid templates make notes fake.
Suggest (but don't enforce):

1. **Context** — one paragraph on what prompted the conversation.
2. **What was said** — summary, not transcript. Attribute where it
   matters ("Alex argued that ...", "Sam was skeptical about ...").
3. **Open questions** — anything the conversation raised but didn't
   resolve. These are the seeds of next steps.
4. **Follow-ups** — concrete actions and who owns them. Tag candidate
   artifacts to spawn (problem brief, inquiry, decision).

## Triage — the real value of this skill

After the notes are drafted, scan them for **artifact implications**
and surface each one to the user. For every item, invoke `vault-search`
and propose a next action:

| Pattern in the note | Proposed next action |
|---|---|
| A problem got stated that isn't in the vault | `frame-problem` |
| A design was sketched for an existing problem | `propose-design` |
| A decision was reached | `record-decision` |
| A disagreement needs bounded exploration | `open-inquiry` |
| An existing brief's framing no longer fits | Supersede via `frame-problem` or `propose-design` |
| An arc is ready to close | Invoke `vault_check_transition` for `status: closed` |

Present the list, let the user pick which to act on. Don't chain
automatically — the user decides what's worth the follow-up.

## Drafting workflow

1. Ask who the attendees were (for a discussion) or confirm solo
   authorship (for a session note).
2. Draft the note from the conversation history in this session, or
   ask the user to summarize if you don't have it.
3. Propose frontmatter + body as a diff. Show the filename.
4. Write on approval.
5. Commit with `git add discussions/<id>.md && git commit -m "Note:
   <one-line>"`.
6. **Triage** — scan for implications, run `vault-search` on each
   candidate topic, present the follow-up menu to the user.

## When the discussion invalidates existing state

The conversation may have concluded that an artifact in the vault is
obsolete, that an inquiry should be abandoned, or that an arc should
close. **Do not apply those transitions directly from this skill.**
Instead:

1. Note the implication in the discussion body ("this conversation
   made the vault-design brief obsolete").
2. Propose the transition to the user as a follow-up.
3. On their go-ahead, invoke `vault_check_transition` with the new
   status, present blockers, and guide them through the appropriate
   phase skill or a manual commit. The reconciler (phase 4) will
   eventually propose these automatically — for now, do it by hand.

## Do not

- Do not assign `status:` to a discussion or session note. They
  don't have one.
- Do not edit an artifact's status from this skill. Discussions are
  notes; transitions go through phase skills or direct edits with
  `vault_check_transition`.
- Do not invent attendees or claim decisions that weren't actually
  reached. Napkin, not fiction.
