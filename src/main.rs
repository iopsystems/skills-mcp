use std::{borrow::Cow, path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, File};
use rmcp::{
    model::{
        CallToolRequestParams, CallToolResult, Content, Implementation, InitializeResult,
        ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities, Tool,
    },
    service::RequestContext,
    transport::stdio,
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::Mutex;

mod vault;

use vault::{Direction, Index, SearchFilters};

static SKILLS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");

#[derive(Debug, Deserialize)]
struct Frontmatter {
    name: String,
    description: String,
}

struct LoadedSkill {
    name: String,
    description: String,
    body: String,
}

fn load_skills() -> Result<Vec<LoadedSkill>> {
    let mut out = Vec::new();
    for entry in SKILLS.dirs() {
        let Some(skill_file) = find_skill_md(entry) else {
            continue;
        };
        let raw = skill_file
            .contents_utf8()
            .with_context(|| format!("SKILL.md is not UTF-8: {}", skill_file.path().display()))?;
        let (fm, body) = split_frontmatter(raw).with_context(|| {
            format!(
                "SKILL.md missing YAML frontmatter: {}",
                skill_file.path().display()
            )
        })?;
        let meta: Frontmatter = serde_yaml::from_str(fm).with_context(|| {
            format!(
                "failed to parse frontmatter in {}",
                skill_file.path().display()
            )
        })?;
        out.push(LoadedSkill {
            name: meta.name,
            description: meta.description,
            body: body.trim_start_matches('\n').to_owned(),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

fn find_skill_md<'a>(dir: &'a Dir<'a>) -> Option<&'a File<'a>> {
    dir.files().find(|f| {
        f.path()
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("SKILL.md"))
            .unwrap_or(false)
    })
}

fn split_frontmatter(src: &str) -> Option<(&str, &str)> {
    let rest = src.strip_prefix("---")?;
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    let end = rest.find("\n---")?;
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after.strip_prefix('\n').unwrap_or(after);
    Some((fm, body))
}

fn empty_schema() -> Arc<serde_json::Map<String, Value>> {
    obj_schema(json!({
        "type": "object",
        "properties": {},
        "additionalProperties": false
    }))
}

fn obj_schema(v: Value) -> Arc<serde_json::Map<String, Value>> {
    match v {
        Value::Object(map) => Arc::new(map),
        _ => unreachable!("literal is an object"),
    }
}

/// Built-in programmatic tools — Rust-backed, with real input schemas,
/// returning structured JSON. Complements the instructional skills.
fn programmatic_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: Cow::Borrowed("vault_search"),
            title: None,
            description: Some(Cow::Borrowed(
                "Search the knowledge-iop vault for artifacts matching filters. \
                 At least one of type/status/author/topic must be provided. \
                 Returns up to `limit` rows (default 50, max 500). `topic` is \
                 a full-text-search term matched against title and body.",
            )),
            input_schema: obj_schema(json!({
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "description": "Artifact type: scope, arc, problem-brief, design-brief, decision, discussion, session-note, inquiry, exploration, synthesis, claim, schema"
                    },
                    "status": {"type": "string"},
                    "author": {"type": "string"},
                    "topic": {"type": "string", "description": "FTS5 search term over title + body"},
                    "limit": {"type": "integer", "minimum": 1, "maximum": 500}
                },
                "additionalProperties": false
            })),
            output_schema: None,
            annotations: None,
            execution: None,
            icons: None,
            meta: None,
        },
        Tool {
            name: Cow::Borrowed("vault_edges"),
            title: None,
            description: Some(Cow::Borrowed(
                "Return edges touching the given artifact id. Optionally \
                 filter by edge kind (frames, supersedes, superseded_by, \
                 relates_to, conflicts_with, depends_on, derived_from, arc, \
                 scopes, inquiry) and direction (outgoing, incoming, both — \
                 default both). Each row includes the neighbor artifact.",
            )),
            input_schema: obj_schema(json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The artifact id to pivot on"},
                    "kind": {"type": "string"},
                    "direction": {
                        "type": "string",
                        "enum": ["outgoing", "incoming", "both"]
                    }
                },
                "required": ["id"],
                "additionalProperties": false
            })),
            output_schema: None,
            annotations: None,
            execution: None,
            icons: None,
            meta: None,
        },
        Tool {
            name: Cow::Borrowed("vault_reindex"),
            title: None,
            description: Some(Cow::Borrowed(
                "Force a full rebuild of the vault's sqlite index. Normally \
                 unnecessary — queries auto-rebuild on staleness — but useful \
                 after a schema bump or to debug drift.",
            )),
            input_schema: empty_schema(),
            output_schema: None,
            annotations: None,
            execution: None,
            icons: None,
            meta: None,
        },
        Tool {
            name: Cow::Borrowed("vault_check_transition"),
            title: None,
            description: Some(Cow::Borrowed(
                "Evaluate a proposed status transition against the vault's \
                 resolution-propagation invariants. Returns allowed/blockers/\
                 warnings with machine-stable rule ids and artifact evidence. \
                 Use this before committing a status change to catch cascading \
                 violations (e.g. accepting a decision whose design-brief is \
                 still draft, closing an arc with open inquiries).",
            )),
            input_schema: obj_schema(json!({
                "type": "object",
                "properties": {
                    "id": {"type": "string", "description": "The artifact id to transition"},
                    "new_status": {"type": "string", "description": "The proposed new status"}
                },
                "required": ["id", "new_status"],
                "additionalProperties": false
            })),
            output_schema: None,
            annotations: None,
            execution: None,
            icons: None,
            meta: None,
        },
    ]
}

#[derive(Clone)]
struct SkillsServer {
    skills: Arc<Vec<LoadedSkill>>,
    /// Lazily-opened vault index, shared across tool calls. `None` means
    /// no vault was located; programmatic vault tools will error out.
    vault: Arc<Mutex<VaultState>>,
}

enum VaultState {
    /// Never attempted to locate the vault yet.
    Unresolved,
    /// Vault located and index opened; holds the connection.
    Ready(Index),
    /// Vault location attempt failed; cached so we don't retry every call.
    Unavailable(String),
}

impl SkillsServer {
    fn new(skills: Vec<LoadedSkill>) -> Self {
        Self {
            skills: Arc::new(skills),
            vault: Arc::new(Mutex::new(VaultState::Unresolved)),
        }
    }

    fn find_skill(&self, name: &str) -> Option<&LoadedSkill> {
        self.skills.iter().find(|s| s.name == name)
    }

    /// Get a refreshed index, locating the vault on first use.
    async fn vault_index(&self) -> Result<tokio::sync::OwnedMutexGuard<VaultState>, String> {
        let guard = self.vault.clone().lock_owned().await;
        match &*guard {
            VaultState::Ready(_) => return Ok(guard),
            VaultState::Unavailable(msg) => return Err(msg.clone()),
            VaultState::Unresolved => {}
        }
        drop(guard);
        let start = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let open_result = tokio::task::spawn_blocking(move || {
            let root = vault::locate_vault(&start)?;
            Index::open(&root)
        })
        .await
        .map_err(|e| format!("vault open task panicked: {e}"))?;
        let mut guard = self.vault.clone().lock_owned().await;
        match open_result {
            Ok(idx) => {
                *guard = VaultState::Ready(idx);
                Ok(guard)
            }
            Err(err) => {
                let msg = format!("{err:#}");
                *guard = VaultState::Unavailable(msg.clone());
                Err(msg)
            }
        }
    }
}

impl ServerHandler for SkillsServer {
    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                title: None,
                description: Some(env!("CARGO_PKG_DESCRIPTION").to_string()),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "This server exposes two kinds of tools. Skill tools \
                 (catchup, requirements-architect, ...) return instruction text \
                 to follow. Vault tools (vault_search, vault_edges, vault_reindex) \
                 query a SQLite index of the knowledge-iop vault and return \
                 structured JSON."
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let empty = empty_schema();
        let mut tools: Vec<Tool> = self
            .skills
            .iter()
            .map(|s| Tool {
                name: Cow::Owned(s.name.clone()),
                title: None,
                description: Some(Cow::Owned(s.description.clone())),
                input_schema: empty.clone(),
                output_schema: None,
                annotations: None,
                execution: None,
                icons: None,
                meta: None,
            })
            .collect();
        tools.extend(programmatic_tools());
        tools.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(ListToolsResult {
            tools,
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        match request.name.as_ref() {
            "vault_search" => self.handle_vault_search(request.arguments).await,
            "vault_edges" => self.handle_vault_edges(request.arguments).await,
            "vault_reindex" => self.handle_vault_reindex().await,
            "vault_check_transition" => self.handle_vault_check_transition(request.arguments).await,
            name => match self.find_skill(name) {
                Some(skill) => Ok(CallToolResult::success(vec![Content::text(
                    skill.body.clone(),
                )])),
                None => Ok(CallToolResult::error(vec![Content::text(format!(
                    "unknown tool: {name}"
                ))])),
            },
        }
    }
}

impl SkillsServer {
    async fn handle_vault_search(
        &self,
        args: Option<serde_json::Map<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        let args = args.unwrap_or_default();
        let mut guard = match self.vault_index().await {
            Ok(g) => g,
            Err(msg) => return Ok(vault_err(msg)),
        };
        let VaultState::Ready(index) = &mut *guard else {
            return Ok(vault_err("vault unavailable".to_string()));
        };
        if let Err(e) = index.refresh_if_stale() {
            return Ok(vault_err(format!("refresh failed: {e:#}")));
        }

        let r#type = args.get("type").and_then(Value::as_str);
        let status = args.get("status").and_then(Value::as_str);
        let author = args.get("author").and_then(Value::as_str);
        let topic = args.get("topic").and_then(Value::as_str);
        let limit = args.get("limit").and_then(Value::as_u64).map(|v| v as u32);
        let filters = SearchFilters {
            r#type,
            status,
            author,
            topic,
            limit,
        };
        let rows = match vault::search(index, &filters) {
            Ok(r) => r,
            Err(e) => return Ok(vault_err(format!("search failed: {e:#}"))),
        };
        Ok(json_result(json!({
            "count": rows.len(),
            "matches": rows,
        })))
    }

    async fn handle_vault_edges(
        &self,
        args: Option<serde_json::Map<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        let args = args.unwrap_or_default();
        let Some(id) = args.get("id").and_then(Value::as_str) else {
            return Ok(vault_err("missing required argument: id".to_string()));
        };
        let id = id.to_string();
        let kind = args.get("kind").and_then(Value::as_str).map(str::to_string);
        let direction = match args.get("direction").and_then(Value::as_str) {
            Some("outgoing") => Direction::Outgoing,
            Some("incoming") => Direction::Incoming,
            _ => Direction::Both,
        };
        let mut guard = match self.vault_index().await {
            Ok(g) => g,
            Err(msg) => return Ok(vault_err(msg)),
        };
        let VaultState::Ready(index) = &mut *guard else {
            return Ok(vault_err("vault unavailable".to_string()));
        };
        if let Err(e) = index.refresh_if_stale() {
            return Ok(vault_err(format!("refresh failed: {e:#}")));
        }
        let rows = match vault::edges_of(index, &id, kind.as_deref(), direction) {
            Ok(r) => r,
            Err(e) => return Ok(vault_err(format!("edges query failed: {e:#}"))),
        };
        Ok(json_result(json!({
            "id": id,
            "count": rows.len(),
            "edges": rows,
        })))
    }

    async fn handle_vault_reindex(&self) -> Result<CallToolResult, McpError> {
        let mut guard = match self.vault_index().await {
            Ok(g) => g,
            Err(msg) => return Ok(vault_err(msg)),
        };
        let VaultState::Ready(index) = &mut *guard else {
            return Ok(vault_err("vault unavailable".to_string()));
        };
        match index.rebuild() {
            Ok(()) => Ok(json_result(json!({"status": "rebuilt"}))),
            Err(e) => Ok(vault_err(format!("rebuild failed: {e:#}"))),
        }
    }

    async fn handle_vault_check_transition(
        &self,
        args: Option<serde_json::Map<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        let args = args.unwrap_or_default();
        let Some(id) = args.get("id").and_then(Value::as_str) else {
            return Ok(vault_err("missing required argument: id".to_string()));
        };
        let Some(new_status) = args.get("new_status").and_then(Value::as_str) else {
            return Ok(vault_err(
                "missing required argument: new_status".to_string(),
            ));
        };
        let id = id.to_string();
        let new_status = new_status.to_string();

        let mut guard = match self.vault_index().await {
            Ok(g) => g,
            Err(msg) => return Ok(vault_err(msg)),
        };
        let VaultState::Ready(index) = &mut *guard else {
            return Ok(vault_err("vault unavailable".to_string()));
        };
        if let Err(e) = index.refresh_if_stale() {
            return Ok(vault_err(format!("refresh failed: {e:#}")));
        }
        match vault::check_transition(index, &id, &new_status) {
            Ok(result) => match serde_json::to_value(&result) {
                Ok(v) => Ok(json_result(v)),
                Err(e) => Ok(vault_err(format!("serialize failed: {e}"))),
            },
            Err(e) => Ok(vault_err(format!("check_transition failed: {e:#}"))),
        }
    }
}

fn json_result(v: Value) -> CallToolResult {
    let text = serde_json::to_string_pretty(&v).unwrap_or_else(|_| v.to_string());
    CallToolResult::success(vec![Content::text(text)])
}

fn vault_err(msg: String) -> CallToolResult {
    CallToolResult::error(vec![Content::text(msg)])
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let skills = load_skills().context("failed to load embedded skills")?;
    tracing::info!(count = skills.len(), "loaded skills");

    let service = SkillsServer::new(skills)
        .serve(stdio())
        .await
        .context("failed to start MCP stdio service")?;
    service.waiting().await.context("MCP service error")?;
    Ok(())
}
