---
status: shipped
opened: 2026-10-16
updated: 2026-10-16
---

# The claim was never exclusive

## Goal

Close a race in the review-exchange bridge that let two concurrent writers
take the same round number.

## Scope

The installer block in `skills/repository/review-exchange/SKILL.md` and the
concurrency check in `scripts/review-bridge-test.sh`. No change to the
thread layout, cursors, or any command's interface.

## Evidence

`scripts/review-bridge-test.sh` failed locally on `main` with "round numbers
collided: 5 distinct of 6", then passed, then failed again: about one run in
three. GitHub CI had passed the same commit. A copy of the test with twelve
concurrent writers collided in four runs of five.

A first reading found a window in the design. The 2026-08-13 entry records
it: `mkdir .claim-NNN` is the claim, `next_round` counts claims as well as
files, and the claim is released after the round file is written. A writer
that computed its number before the claim was taken can win the `mkdir`
after the claim is released, and nothing looked at the files afterward. A
recheck closed that window and the twelve-writer test still failed three
runs in ten.

A trace of the installed bridge, timestamped from bash's own clock so the
trace would not hide the race, showed the thing the design assumed was
impossible:

```
1792202783.933820 b  won      11
1792202783.933828 k  won      11
1792202783.933832 g  lost     11
```

Two writers won `mkdir` of one path eight microseconds apart with no release
between them. Reproduced outside the bridge: two processes racing `mkdir` on
one name both succeeded in 170 of 2,000 rounds on tmpfs and 381 of 2,000 on
ext4. Two processes racing Python's `os.mkdir` never did. The binary is
`mkdir (uutils coreutils) 0.8.0`, Ubuntu 26.04.1's default, and `strace`
shows it `statx` the name before creating it; an existing name is reported
from the stat, so two racers that both see nothing both create. GNU mkdir
calls `mkdir(2)` and reports its `EEXIST`. The bridge's comment said "mkdir
is atomic and fails if the name exists", which was true of the mkdir it was
written against.

## Design and Implementation

The claim is now a file taken with bash's own noclobber, which is `O_EXCL`
in the shell and involves no coreutils: 0 double wins in 2,000 racing
rounds on the same machine. The recheck stays, since the design window is
real on any mkdir. The attempt cap moves from ten to one hundred, because
twelve contending writers can lose more than ten times. The test runs
twelve writers and expects fifteen distinct rounds: ten consecutive runs
pass.

## Outcome

Shipped. Full CI job green locally, and the bridge test ten times over.

## Derived Documents

None.

## Deferred or Reopen Items

- Every other `mkdir` used as a lock on a machine with uutils coreutils has
  the same hole. Nothing else in this repository uses one; projects that
  installed this bridge before the fix do, until they re-run the installer.
- The 2026-08-13 entry says the bounded retry "is a backstop, not the
  mechanism". With a claim that is released, it is part of the mechanism.

## Skill Feedback

### review-exchange

- **Friction** — the concurrency check passed on CI and failed one run in
  three locally for a defect that twelve writers expose almost every time.
  A test whose pass rate depends on the machine is a test of the machine.
- **Friction** — the bridge's exclusivity rested on a property of a
  coreutils binary, and the comment stating the property named the tool
  and not the syscall. When the tool was swapped underneath it, the comment
  stayed true of GNU and false of what ran.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
