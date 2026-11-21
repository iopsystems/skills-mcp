---
status: open
opened: 2026-10-21
updated: 2026-10-21
prs: []
---

# Repository skills as a Claude Code plugin

## Goal

Publish the skills that need no server as a Claude Code plugin, installed the
same way as `brayniac/custom-skills`: this repository is both the plugin's
source and its marketplace. The MCP server stays the way to get the vault and
catalog skills.

## Decision Criteria

The plugin must contain only skills that work without the server. `skills/`
must keep its current layout, because `include_dir!` embeds it in the server
and tests name paths under it. An install from GitHub must load every
plugin skill with its sibling files.

## Evidence

Nine vault skills call `vault_search`, `vault_edges`, `vault_check_transition`,
`vault_reflect` or `vault_reindex`; `define-scope`, `open-arc`, `open-inquiry`
and `plan-feature` hand off to vault skills that do. `recommend-skills` and
`seed-skill-template` call `skill_catalog` and `skill_template_get`. None of
the eleven skills under `skills/repository/` names a server tool. The only
reference from one of them to an excluded skill is `engineering-journal`
telling the agent not to invoke `frame-problem` or `propose-design`.
`architecture-diagram` and `dataflow-diagram` mention their templates, and
`engineering-journal` mentions template manifests; only the server serves them,
and each skill works without them.

Measured with Claude Code 2.1.306, in a scratch config directory:

- `claude plugin validate` does not follow symlinks; it reports the eleven
  entries as unread and says a session does follow them.
- A marketplace added from a git URL and installed copied `plugins/iop/` into
  `plugins/cache/skills-mcp/iop/<commit>/` with every symlink replaced by the
  directory it pointed to (0 symlinks in the cache, 11 skill directories).
- A session with `--plugin-dir plugins/iop` listed all eleven as `iop:<name>`.
  In a one-skill test repository, a symlinked skill's
  `${CLAUDE_SKILL_DIR}/references/<file>` resolved and was read.
- `check-skills` from `custom-skills` passed all eleven: 7,033 description
  characters.

## Design and Implementation

`plugins/iop/skills/<name>` is a relative symlink to
`../../../skills/repository/<name>`. `.claude-plugin/marketplace.json` at the
root names the marketplace `skills-mcp` and lists `iop` with source
`./plugins/iop`. `plugin.json` sets no `version`, so an install's version is
the commit and every merge is an update.

`tests/plugin_layout.rs` requires one link per repository skill, no other
entries, each a symlink with that exact target, no `version` in `plugin.json` or
the marketplace entry, and no repository skill file that names a server tool or
an excluded skill. Adding a
link to `skills/vault/discuss` failed two tests. Replacing the `catchup` link
with a copy failed one. An adversarial review found that a marketplace-entry
`version` passed the first version of the test and pinned installs to it; the
test now checks both places.

A checkout with `core.symlinks=false` (the Git for Windows default) turns the
links into text files, and `claude plugin install` then reports success with no
skills. The first review tested a symlink-free alternative: a marketplace entry with
`"source": "./"`, `"strict": false` and an explicit `skills` list loaded all
eleven from such a checkout, at the cost of caching the whole repository (4.9M
against 352K). The symlink layout was kept, and the README tells Windows users
to set `core.symlinks=true`. A second review found that setting it after the
marketplace was added does not help: `plugin update` and a reinstall still
showed no skills, and only removing the marketplace, adding it again and
reinstalling the plugin did. The same
review found that the first version of the server-tool test read only
`SKILL.md`, missed hand-offs to excluded skills, and skipped dot-prefixed
directories, which Claude Code and the server both load. The test now reads
every file and ignores only `.DS_Store`. A third review found that the recovery steps omitted the reinstall
(removing a marketplace uninstalls its plugins), that a tool registered through
a constant or with a comment before its name dropped out of the parse silently,
and that the "Do not invoke" exemption covered every skill on its line. A fourth
review found that parsing the source also missed `Tool::new_with_raw` and the
`#[tool]` macro, that `mcp__<server>__vault_search` passed, and that
`` `/frame-problem <topic>` `` and `` `skills-mcp:frame-problem` `` passed. The
test now spawns the built binary, sends `tools/list`, and treats every listed
tool that is not a skill as server-only; it matches a tool after `__`, and
reads the skill name out of each backticked span. The exemption covers only the
name it precedes. A fifth review found that `` `mcp__skills-mcp__frame-problem` ``
passed and that the spawn had no timeout; the test now takes the part of a span's
first word after its last `__`, so `` `mcp__<server>__name` `` names `name`, and
gives up on the server after 30 seconds.

## Open

The server still exposes the repository skills as MCP tools, so a client with
both the plugin and the server lists them twice. A server option that exposes
only the programmatic tools and the vault and catalog skills would remove the
duplicates. That option is not built.

Moving the vault skills into a plugin would need the plugin to start the
server for its `vault_*` tools, and the server would then list those skills a
second time. The catalog skills have the same problem with `skill_catalog` and
`skill_template_get`, and `recommend-skills` describes the catalog as the
server's tool list, so it would also need rewording for a plugin install.

## Appendix: Skills Invoked

- `cs:open-pr` and `cs:review` — commit, adversarial review, and PR.
