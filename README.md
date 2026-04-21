# iop-skills

A single-binary [Model Context Protocol](https://modelcontextprotocol.io) server
that ships a bundle of Claude skills embedded at compile time. Each skill under
`skills/<name>/SKILL.md` is exposed as an MCP tool whose invocation returns the
skill body for the model to follow.

## Layout

```
iop-skills/
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

The result is a single static-ish binary at `target/release/iop-skills`; all
skill content is baked in via `include_dir!`, so no sidecar files are needed.

## Run (stdio)

Configure your MCP client (e.g. Claude Code) to spawn the binary. The JSON
key is the display name — pick whatever you want:

```json
{
  "mcpServers": {
    "iop-skills": {
      "command": "/path/to/iop-skills"
    }
  }
}
```

## Adding a skill

1. Create `skills/<your-skill>/SKILL.md` with the frontmatter above.
2. Rebuild. The skill is picked up automatically.

## Manual testing

The server speaks MCP over stdio, so any MCP-capable client can drive it. A
few ways to exercise it by hand:

### Option 1: MCP Inspector (recommended for exploration)

The official [MCP Inspector](https://github.com/modelcontextprotocol/inspector)
gives you a browser UI over the server — list tools, inspect schemas, and
invoke them with arbitrary arguments:

```
npx @modelcontextprotocol/inspector /path/to/iop-skills
```

Open the URL it prints, click **Connect**, then **List Tools** to see every
skill and vault tool. Pick one and hit **Run** to see the raw response.

### Option 2: Raw JSON-RPC over stdio

Every MCP request is a single line of JSON written to stdin; responses come
back on stdout. Useful for scripting or debugging without extra tooling.

Initialize, list tools, and call the `catchup` skill in one session:

```
(
  printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"manual","version":"0"}}}'
  printf '%s\n' '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  printf '%s\n' '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
  printf '%s\n' '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"catchup","arguments":{}}}'
) | ./target/release/iop-skills
```

Each response is a JSON-RPC message on its own line. Pipe through `jq` to
pretty-print:

```
... | ./target/release/iop-skills | jq -c 'select(.id)'
```

Calling a vault tool works the same way — pass arguments under `params.arguments`:

```json
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"vault_search","arguments":{"type":"decision","limit":5}}}
```

### Option 3: Claude Code

Register the binary as an MCP server in your client config (see [Run
(stdio)](#run-stdio) above), then in a session run `/mcp` to confirm
`iop-skills` is connected and its tools are listed. Invoking any skill tool
returns the skill body as a model message.
