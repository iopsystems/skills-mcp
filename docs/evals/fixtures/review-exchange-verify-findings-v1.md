# Frozen outcomes (written before either arm ran)

Task: as author `claude`, answer codex's round 1 on feature/settings.
Ground truth at the round's commit:
- F1 is false: fetch.py:12 still has the sleep; the branch's diff to fetch.py
  changes only MAX_RETRIES. Quote is not at :9.
- F2 is true, and is one instance of a class: stale `load_config` remains at
  README.md:11, :12, :14 and app.py:6 (docstring).
- F3's count is true (5), but its replacement text is false: backoff starts
  at 100 ms (0.1 s), not 200 ms.

Outcomes (pass/fail each):
O1  F1 disputed with evidence (sleep present at the round's commit, or the
    diff shows fetch.py changed only MAX_RETRIES). No second sleep added.
O2  F2 fixed, and every other stale `load_config` reference (README.md:11,
    :14, app.py:6) also fixed in the same or a cited commit.
O3  The extra fixes beyond F2's single line are listed in the round.
O4  F3 fixed without the false "200 ms" claim (100 ms, or no backoff figure).
O5  The round states that F3's replacement text was corrected, or why.
O6  One author round with --replies-to 1; each finding exactly one
    disposition; fixed ones cite a commit.
Prediction: baseline passes O6 and probably O1 (models read code); the gap,
if any, shows in O2/O3 (class) and O4/O5 (replacement checked). Refutation:
baseline passes O2 and O4 as well, i.e. a null result.

The sections below were added when the evidence was committed; the outcomes
above are unchanged.

## Arm prompt (identical for both arms; the skill file differed)

You are the author agent, named `claude`, of branch feature/settings in the
git repository <arm-repo>. The other agent, `codex`, has posted a review
round. Follow the skill at <arm-repo>/.skill/SKILL.md (read it first; the
bridge is already installed).

Request from the user: "Codex reviewed my branch. Apply its findings and reply
to it."

Work only inside that repository (cd into it for every command). Commit any
changes on feature/settings. Do not touch anything outside it. When done,
reply with a short summary of what you did for each finding and the path of
the round you wrote.

## Critic prompt (summary)

Grade repositories P and Q against O1-O6 (not the Prediction paragraph),
read-only, without reading either `.skill/` directory; verify the ground truth
at the round-1 commit; PASS or FAIL per outcome with one line of evidence;
report any defect outside O1-O6. P ran the base text, Q the revised text; the
critic was not told which.
