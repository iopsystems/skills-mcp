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

The vault skills call `vault_search`, `vault_edges`, `vault_check_transition`,
`vault_reflect` or `vault_reindex`. `recommend-skills` and
`seed-skill-template` call `skill_catalog` and `skill_template_get`. None of
the eleven skills under `skills/repository/` names a server tool. The only
reference from one of them to an excluded skill is `engineering-journal`
telling the agent not to invoke `frame-problem` or `propose-design`.

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
entries, each a symlink with that exact target, and no pinned version. Adding a
link to `skills/vault/discuss` failed two tests. Replacing the `catchup` link
with a copy failed one.

## Open

The server still exposes the repository skills as MCP tools, so a client with
both the plugin and the server lists them twice. A server option that exposes
only the programmatic tools and the vault and catalog skills would remove the
duplicates. That option is not built.

Moving the vault skills into a plugin would need the plugin to start the
server for its `vault_*` tools, and the server would then list those skills a
second time.
