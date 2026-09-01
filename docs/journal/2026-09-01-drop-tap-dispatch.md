---
status: shipped
opened: 2026-09-01
updated: 2026-09-01
beta_skills: []
---

# The automation that never ran in anger

## Goal

Decide whether the release workflow should keep asking the Homebrew tap to bump
its formula, after that job failed on the first release that used it.

## Scope

`.github/workflows/release.yml`: the `notify-tap` job and the header comment
describing it. The tap's own daily cron, its `repository_dispatch` trigger, and
`TAP_DISPATCH_TOKEN` are untouched.

## Evidence

`notify-tap` failed on v0.5.1 with `HTTP 403: Resource not accessible by
personal access token` against `repos/iopsystems/homebrew-iop/dispatches`. The
token was present and authenticated, so this is a permissions answer rather than
the expiry the header comment tells a reader to check for; an expired token
returns 401. The same token dispatched successfully on 2026-08-19, so the access
was narrowed or lapsed between then and now.

The trigger history is the argument. Every tap bump that has ever landed —
v0.3.0 through v0.5.0 — came from a person running `gh workflow run` minutes
after the tag, except v0.4.0, which the daily cron picked up on its own. The
dispatch job landed after v0.5.0, was exercised once by hand, and v0.5.1 was its
first production use.

So the job automated a step that was already working, and its only real attempt
failed.

## Design and Implementation

Removed. The cron is the path, and the one-liner is in the header comment for a
release that wants the bump now.

The latency the job bought is smaller than it reads. The tap opens a pull
request; bottles are built only when a person applies the `pr-pull` label. Past
formula pull requests were labelled six to eight minutes after opening, because
whoever cut the release was still at the keyboard. A day of cron delay only
bites when nobody is watching, and nobody labels the pull request then either.

Against keeping it and letting it fail soft: the header comment argues a failure
here is worth marking red, which is right — a fast path that fails silently rots.
That argument defends the red X and does not defend the job. Weighed against a
cross-repository write credential to maintain, one command is the cheaper form.

## Outcome

Shipped. v0.5.1 reached the tap as pull request #128 through the one-liner.

## Derived Documents

None.

## Deferred or Reopen Items

- The tap still declares `repository_dispatch: [skills-mcp-released]`, and its
  comment there calls the cron a backstop for a dispatch that no longer arrives.
  The trigger is inert; the comment is wrong. Both live in another repository.

## Skill Feedback

None. No skill produced this.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
