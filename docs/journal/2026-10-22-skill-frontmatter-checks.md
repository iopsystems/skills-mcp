---
status: open
opened: 2026-10-22
updated: 2026-10-22
prs: []
---

# Skill frontmatter that Claude Code loads correctly

## Goal

Fail the test suite on a skill that Claude Code would load wrongly without
reporting an error. The checks match the ones `check-skills` in
`brayniac/custom-skills` makes, and they cover every active skill under
`skills/` and every plugin skill under `plugins/iop/skills/`.

## Evidence

Measured with Claude Code 2.1.308 in scratch directories.
`claude plugin validate --strict` passed a skills directory whose
`foo/SKILL.md` declared `name: bar` and an unknown key `allowd-tools`. It does
report frontmatter that fails to parse: an unterminated `"` gave exit status 1
and "At runtime this skill loads with empty metadata". Claude Code re-quotes a
value containing `: ` before giving up, so a colon in a plain description
loads correctly. In a `claude -p --plugin-dir` session, a plugin skill whose
description began `Use when: ` was listed with that text intact.

The server's `load_skills` (`src/main.rs`) already fails on frontmatter
`serde_yaml` cannot parse. An unterminated quote in `catchup` failed 10
server unit tests and `repository_skills_need_nothing_only_the_server_provides`
before this change. Its `Frontmatter` lacks `deny_unknown_fields`, and
nothing compared `name` with the directory name. The plugin lists a skill by
its directory, while the server serves it by `name`.

Claude Code 2.1.308 sizes the skill listing at
`(contextWindow ?? 200000) * 4 * 0.01` characters, which is 8,000 for a
200,000-token window and 40,000 for 1,000,000. `skillListingBudgetFraction`
and `SLASH_COMMAND_TOOL_CHAR_BUDGET` override it. Each entry counts as the
qualified name (`iop:<name>`), four characters, and the description, with a
newline between entries. Past the budget, Claude Code drops the descriptions
of the least-used skills.

## Design and Implementation

`tests/plugin_layout.rs` gains `check_skill` and three tests:

- `skill_frontmatter_loads_in_claude_code` covers the 26 skills under
  `skills/` and the 11 plugin links. The parse step duplicates `load_skills`.
  The test adds four checks:
  - the frontmatter holds no key but `name` and `description`;
  - `name` matches `[a-z0-9_-]{1,64}`;
  - `name` equals the directory name;
  - the trimmed description is 1 to 1,536 characters, where Claude Code
    truncates one listed description.
- `skill_check_rejects_what_claude_code_loads_wrongly` passes inline broken
  skills to `check_skill`. Removing `deny_unknown_fields` made it fail with
  `accepted "...allowed-tools: Bash..."`.
- `plugin_listing_fits_the_budget_of_a_200k_window` prints the plugin's
  listing size (7,306 characters: 7,033 of descriptions, plus `name.len() + 9`
  for each skill) and fails above 8,000. Past that bound, the plugin alone
  overflows the listing on a 200,000-token model. The bound does not cover
  the other skills on the host.

The server keeps its lenient parse of unknown keys. The tests reject such a
skill before a release, and a stricter server would only move the same
failure to start-up.

With `cargo test --no-fail-fast`, these changes to a skill failed only the new
tests:

| Change | New test failure | Existing tests failing |
| --- | --- | --- |
| `catchup`: added `allowd-tools: Bash` | `unknown field 'allowd-tools'` | none |
| `catchup`: 1,200 characters added to the description | `description is 1731 characters` | none |
| `catchup`: 700 characters added to the description | `listing totals 8006 characters, past 8000` | none |
| `synthesize`: `name: synthesise` | `differs from its directory` | none |
| `synthesize`: `name: Synthesize` | `must match [a-z0-9_-]{1,64}` | none |

Renaming `catchup` also fails existing tests, because they look the skill up
by name.

## Open

`brayniac/custom-skills` also refuses a PR merge that is not pinned to an
adversarially reviewed head, using a PreToolUse hook. This change does not add
that hook.

## Appendix: Skills Invoked

None.
