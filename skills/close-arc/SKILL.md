---
name: close-arc
description: |
  Close an open **arc** — mark its narrative complete (`closed`) or explicitly set aside (`abandoned`), and audit that its closing conditions were actually met. Use this when the user says things like "close the X arc", "we're done with this arc", "abandon arc Y", "wrap up the arc". An arc cannot be `closed` while any of its inquiries are still `open` — this skill invokes `vault_check_transition` to enforce that and surfaces the blockers concretely. Arcs that trail off without explicit closure are the vault's biggest smell; spending two minutes on this skill is how that's avoided.
---

# Close arc

You are closing (or abandoning) an **arc** in the knowledge-iop
vault. Arcs that never get closed are the vault's biggest smell.
This skill exists to make closing frictionless enough that it
happens.

## Two flavors

- **`closed`** — the arc reached its closing conditions (as stated
  in the charter). The narrative resolved.
- **`abandoned`** — the arc is set aside without reaching its
  closing conditions. This is an explicit, logged act — not a
  euphemism. Abandoning happens when the situation changed, not
  when the effort stalled.

If you can't articulate which one applies, the answer is usually
"abandoned with a reason".

## Before closing — verify

1. Ask the user for the arc id.
2. Call `vault_edges` on the arc id to see what's connected:
   - Inquiries (`inquiry.arc = this`)
   - Briefs, decisions (`artifact.arc = this`)
3. Call `vault_check_transition` with the arc id and
   `new_status: closed`. Surface any blockers, in particular:
   - `arc_all_inquiries_closed`: an open inquiry under this arc
     must be resolved or abandoned first.
4. Re-read the arc's charter (the body of `arcs/<id>.md`). Verify:
   - The closing conditions are met (for `closed`).
   - The expected artifacts are mostly in place — draft briefs or
     unresolved inquiries may warrant finishing or explicit
     handoff before close.
   - If they're NOT met, the transition is `abandoned`, not
     `closed`.

## Updating the arc

Edit the arc file's frontmatter:

For `closed`:
```yaml
status: closed
closed: <YYYY-MM-DD>
```

For `abandoned`:
```yaml
status: abandoned
closed: <YYYY-MM-DD>
```

Append a **closing section** to the body:

```markdown
## Closed <YYYY-MM-DD>

<one paragraph on what got done, what didn't, and why this is the
right time to close>

**Key artifacts:** [<brief-id>, <decision-id>, ...]
**Spawned:** [<successor-arc-id | "none">]
**Unresolved:** <anything explicitly left on the floor, with reason>
```

For `abandoned`, append instead:

```markdown
## Abandoned <YYYY-MM-DD>

<one paragraph on why this arc is being set aside without resolution:
what changed, what became irrelevant, or what proved intractable>

**Picked up by:** <id | "nothing — the situation dissolved">
**Lessons:** <one or two; "we learned we should never again ...">
```

## Drafting workflow

1. Verify (see above). Abort if `vault_check_transition` returns
   blockers — fix those first.
2. Edit the arc file's frontmatter and append the closing section.
3. Propose the diff to the user.
4. Apply on approval.
5. Commit: `git add arcs/<id>.md && git commit -m "Close arc:
   <id>"` (or `"Abandon arc: <id>"`).

## If blockers come back

The most common blocker is an open inquiry. Options:

1. **Synthesize the inquiry** via the `synthesize` skill — if the
   barrier has released, this is the right move.
2. **Abandon the inquiry** explicitly — edit `inquiries/<id>/inquiry.md`
   to `status: abandoned`, add a reason, commit. Then retry the
   arc close.
3. **Keep the arc open** — if the inquiry genuinely isn't ready
   and can't be abandoned, the arc isn't actually ready to close.
   Tell the user.

Do not close the arc with a dangling open inquiry. The invariant
exists because resolution state is first-class, visible, and
non-contagious — pretending the inquiry is gone defeats the point.

## Spawning successors

If closing this arc implies a successor arc picks up the thread:

1. Close this arc first, noting the successor in the closing
   section.
2. Open the successor via `open-arc`, referencing the predecessor
   in the charter body.
3. Two separate commits — the handoff stays visible in history.

## Do not

- Do not close an arc with `vault_check_transition` blockers
  unresolved. There is no force-close.
- Do not conflate `closed` and `abandoned`. The vault trusts
  honest labels.
- Do not silently edit old briefs or decisions to "wrap up" an
  arc. Their immutability rules apply regardless of whether the
  arc is closing.
