# skills-mcp

A single-binary [Model Context Protocol](https://modelcontextprotocol.io) server
that ships a bundle of Claude skills embedded at compile time. Each skill under
`skills/<name>/SKILL.md` is exposed as an MCP tool whose invocation returns the
skill body for the model to follow.

## Layout

```
skills-mcp/
├── Cargo.toml
├── src/
│   └── main.rs
└── skills/
    └── <skill-name>/
        └── SKILL.md   # YAML frontmatter (name, description) + markdown body
```

`SKILL.md` frontmatter:

```markdown
---
name: <tool-name>
description: <one-line description used as the MCP tool description>
---

<body — returned verbatim to the model when the tool is invoked>
```

## Build

```
cargo build --release
```

The result is a single static-ish binary at `target/release/skills-mcp`; all
skill content is baked in via `include_dir!`, so no sidecar files are needed.

## Run (stdio)

Configure your MCP client (e.g. Claude Code) to spawn the binary:

```json
{
  "mcpServers": {
    "skills": {
      "command": "/path/to/skills-mcp"
    }
  }
}
```

## Adding a skill

1. Create `skills/<your-skill>/SKILL.md` with the frontmatter above.
2. Rebuild. The skill is picked up automatically.
