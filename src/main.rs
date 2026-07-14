use std::{borrow::Cow, path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, File};
use rmcp::{
    model::{
        CallToolRequestParams, CallToolResult, Content, Implementation, InitializeResult,
        ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities, Tool,
        ToolAnnotations,
    },
    service::RequestContext,
    transport::stdio,
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::Mutex;

mod templates;
mod vault;

use vault::{Direction, Index, SearchFilters};

static SKILLS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");
static TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

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
            name: Cow::Borrowed("skill_catalog"),
            title: None,
            description: Some(Cow::Borrowed(
                "List active skills and installable skill templates from the embedded catalog. Read-only; returns metadata only and never template contents.",
            )),
            input_schema: empty_schema(),
            output_schema: None,
            annotations: Some(ToolAnnotations::new().read_only(true)),
            execution: None,
            icons: None,
            meta: None,
        },
        Tool {
            name: Cow::Borrowed("skill_template_get"),
            title: None,
            description: Some(Cow::Borrowed(
                "Retrieve an embedded skill template by template_id, optionally limited to one declared path. Read-only; never reads arbitrary filesystem paths.",
            )),
            input_schema: obj_schema(json!({
                "type": "object",
                "properties": {
                    "template_id": {"type": "string"},
                    "path": {"type": "string"}
                },
                "required": ["template_id"],
                "additionalProperties": false
            })),
            output_schema: None,
            annotations: Some(ToolAnnotations::new().read_only(true)),
            execution: None,
            icons: None,
            meta: None,
        },
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
        Tool {
            name: Cow::Borrowed("vault_reflect"),
            title: None,
            description: Some(Cow::Borrowed(
                "Produce the Part A (graph hygiene) report the reconciler \
                 consumes: scope activity, arc momentum, orphan problem-briefs, \
                 stale design-briefs, pending syntheses, and stale open arcs. \
                 Returns structured JSON; the reconcile-vault skill layers \
                 strategic judgment (Part B) on top. All three window \
                 parameters are optional; defaults are 30 / 14 / 60 days.",
            )),
            input_schema: obj_schema(json!({
                "type": "object",
                "properties": {
                    "window_days": {
                        "type": "integer", "minimum": 1, "maximum": 3650,
                        "description": "How many days back counts as 'recent'. Default 30."
                    },
                    "min_days_stale_design": {
                        "type": "integer", "minimum": 1, "maximum": 3650,
                        "description": "A design-brief in draft/proposed longer than this is stale. Default 14."
                    },
                    "min_days_stale_arc": {
                        "type": "integer", "minimum": 1, "maximum": 3650,
                        "description": "An open arc with no activity for longer than this is stale. Default 60."
                    }
                },
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
    templates: Arc<templates::TemplateRegistry>,
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
    fn new(skills: Vec<LoadedSkill>, templates: templates::TemplateRegistry) -> Self {
        Self {
            skills: Arc::new(skills),
            templates: Arc::new(templates),
            vault: Arc::new(Mutex::new(VaultState::Unresolved)),
        }
    }

    fn find_skill(&self, name: &str) -> Option<&LoadedSkill> {
        self.skills.iter().find(|s| s.name == name)
    }

    fn listed_tools(&self) -> Vec<Tool> {
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
        tools
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
                "This server exposes three tool families. The instructional skill tools \
                 (catchup, plan-feature, ...) return instruction text to follow. The \
                 catalog and retrieval tools are read-only: skill_catalog lists active \
                 skill and installable template metadata, while skill_template_get returns \
                 declared embedded template files; neither fetches, writes, or exposes \
                 undeclared files. The vault programmatic tools query a SQLite index of the \
                 knowledge-iop vault and return structured JSON: vault_search searches \
                 artifacts, vault_edges traverses relationships, vault_reindex rebuilds the \
                 index, vault_check_transition validates proposed status changes, and \
                 vault_reflect produces graph-hygiene reports."
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult {
            tools: self.listed_tools(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        self.route_tool(request.name.as_ref(), request.arguments)
            .await
    }
}

impl SkillsServer {
    async fn route_tool(
        &self,
        name: &str,
        arguments: Option<serde_json::Map<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        match name {
            "skill_catalog" => {
                if arguments.as_ref().is_some_and(|args| !args.is_empty()) {
                    Ok(tool_err("skill_catalog does not accept arguments"))
                } else {
                    self.handle_skill_catalog().await
                }
            }
            "skill_template_get" => self.handle_skill_template_get(arguments).await,
            "vault_search" => self.handle_vault_search(arguments).await,
            "vault_edges" => self.handle_vault_edges(arguments).await,
            "vault_reindex" => self.handle_vault_reindex().await,
            "vault_check_transition" => self.handle_vault_check_transition(arguments).await,
            "vault_reflect" => self.handle_vault_reflect(arguments).await,
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

    async fn handle_skill_catalog(&self) -> Result<CallToolResult, McpError> {
        let mut items = self
            .skills
            .iter()
            .map(|skill| {
                json!({
                    "kind": "active-skill",
                    "id": skill.name,
                    "description": skill.description,
                })
            })
            .collect::<Vec<_>>();
        items.extend(self.templates.summaries().into_iter().map(|template| {
            json!({
                "kind": "template",
                "id": template.id,
                "version": template.version,
                "purpose": template.purpose,
                "compatibility": template.compatibility,
            })
        }));
        items.sort_by(|left, right| {
            left["id"]
                .as_str()
                .cmp(&right["id"].as_str())
                .then_with(|| left["kind"].as_str().cmp(&right["kind"].as_str()))
        });

        Ok(json_result(json!({
            "schema_version": 1,
            "items": items,
        })))
    }

    async fn handle_skill_template_get(
        &self,
        args: Option<serde_json::Map<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        let Some(args) = args else {
            return Ok(tool_err("missing required argument: template_id"));
        };
        if args.keys().any(|key| key != "template_id" && key != "path") {
            return Ok(tool_err("unexpected argument for skill_template_get"));
        }
        let Some(template_id) = args.get("template_id").and_then(Value::as_str) else {
            return Ok(tool_err(
                "missing or invalid required argument: template_id",
            ));
        };
        let path = match args.get("path") {
            Some(Value::String(path)) => Some(path.as_str()),
            Some(_) => return Ok(tool_err("invalid argument: path must be a string")),
            None => None,
        };

        match self.templates.get(template_id, path) {
            Ok(bundle) => match serde_json::to_value(bundle) {
                Ok(value) => Ok(json_result(value)),
                Err(error) => Ok(tool_err(format!("failed to serialize template: {error}"))),
            },
            Err(error) => Ok(tool_err(error.to_string())),
        }
    }

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

    async fn handle_vault_reflect(
        &self,
        args: Option<serde_json::Map<String, Value>>,
    ) -> Result<CallToolResult, McpError> {
        let args = args.unwrap_or_default();
        let window_days = args
            .get("window_days")
            .and_then(Value::as_u64)
            .map(|v| v as u32)
            .unwrap_or(30);
        let min_days_stale_design = args
            .get("min_days_stale_design")
            .and_then(Value::as_u64)
            .map(|v| v as u32)
            .unwrap_or(14);
        let min_days_stale_arc = args
            .get("min_days_stale_arc")
            .and_then(Value::as_u64)
            .map(|v| v as u32)
            .unwrap_or(60);

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
        match vault::reflect(
            index,
            window_days,
            min_days_stale_design,
            min_days_stale_arc,
        ) {
            Ok(report) => match serde_json::to_value(&report) {
                Ok(v) => Ok(json_result(v)),
                Err(e) => Ok(vault_err(format!("serialize failed: {e}"))),
            },
            Err(e) => Ok(vault_err(format!("reflect failed: {e:#}"))),
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

fn tool_err(msg: impl Into<String>) -> CallToolResult {
    CallToolResult::error(vec![Content::text(msg.into())])
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
    let templates = templates::TemplateRegistry::from_dir(&TEMPLATES)
        .context("failed to load embedded skill templates")?;
    tracing::info!(count = skills.len(), "loaded skills");

    let service = SkillsServer::new(skills, templates)
        .serve(stdio())
        .await
        .context("failed to start MCP stdio service")?;
    service.waiting().await.context("MCP service error")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

    fn test_server() -> SkillsServer {
        let skills = load_skills().expect("embedded skills should load");
        let templates = templates::TemplateRegistry::from_dir(&TEST_TEMPLATES)
            .expect("embedded templates should load");
        SkillsServer::new(skills, templates)
    }

    fn result_json(result: &CallToolResult) -> Value {
        let serialized = serde_json::to_value(result).expect("tool result should serialize");
        let text = serialized["content"][0]["text"]
            .as_str()
            .expect("tool result should contain text");
        serde_json::from_str(text).expect("tool result text should be JSON")
    }

    #[test]
    fn server_instructions_describe_every_tool_family_and_vault_tool() {
        let instructions = test_server()
            .get_info()
            .instructions
            .expect("server instructions should be present");

        for required in [
            "instructional skill tools",
            "skill_catalog",
            "skill_template_get",
            "vault_search",
            "vault_edges",
            "vault_reindex",
            "vault_check_transition",
            "vault_reflect",
        ] {
            assert!(
                instructions.contains(required),
                "server instructions missing {required}"
            );
        }
    }

    #[test]
    fn skill_template_tools_have_exact_read_only_descriptions_and_schemas() {
        let tools = test_server().listed_tools();
        let catalog = tools
            .iter()
            .find(|tool| tool.name == "skill_catalog")
            .expect("skill_catalog should be listed");
        assert_eq!(
            catalog.description.as_deref(),
            Some(
                "List active skills and installable skill templates from the embedded catalog. Read-only; returns metadata only and never template contents."
            )
        );
        assert_eq!(
            Value::Object(catalog.input_schema.as_ref().clone()),
            json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false
            })
        );
        assert_eq!(
            catalog.annotations.as_ref().and_then(|a| a.read_only_hint),
            Some(true)
        );

        let get = tools
            .iter()
            .find(|tool| tool.name == "skill_template_get")
            .expect("skill_template_get should be listed");
        assert_eq!(
            get.description.as_deref(),
            Some(
                "Retrieve an embedded skill template by template_id, optionally limited to one declared path. Read-only; never reads arbitrary filesystem paths."
            )
        );
        assert_eq!(
            Value::Object(get.input_schema.as_ref().clone()),
            json!({
                "type": "object",
                "properties": {
                    "template_id": {"type": "string"},
                    "path": {"type": "string"}
                },
                "required": ["template_id"],
                "additionalProperties": false
            })
        );
        assert_eq!(
            get.annotations.as_ref().and_then(|a| a.read_only_hint),
            Some(true)
        );
    }

    #[tokio::test]
    async fn skill_catalog_combines_metadata_without_bodies_or_template_contents() {
        let result = test_server().handle_skill_catalog().await.unwrap();
        assert_eq!(result.is_error, Some(false));
        let catalog = result_json(&result);
        assert_eq!(catalog["schema_version"], 1);

        let items = catalog["items"]
            .as_array()
            .expect("items should be an array");
        let ordering = items
            .iter()
            .map(|item| (item["id"].as_str().unwrap(), item["kind"].as_str().unwrap()))
            .collect::<Vec<_>>();
        assert!(ordering.windows(2).all(|pair| pair[0] <= pair[1]));

        let active = items
            .iter()
            .find(|item| item["kind"] == "active-skill")
            .expect("active skill metadata should be present");
        assert_eq!(
            active.as_object().unwrap().keys().collect::<Vec<_>>(),
            vec!["description", "id", "kind"]
        );
        assert!(active["description"]
            .as_str()
            .is_some_and(|v| !v.is_empty()));
        assert!(active.get("body").is_none());

        let template = items
            .iter()
            .find(|item| item["kind"] == "template")
            .expect("template metadata should be present");
        assert_eq!(
            template.as_object().unwrap().keys().collect::<Vec<_>>(),
            vec!["compatibility", "id", "kind", "purpose", "version"]
        );
        assert!(template["compatibility"].as_array().is_some());
        assert!(template.get("files").is_none());
        assert!(template.get("content").is_none());
    }

    #[tokio::test]
    async fn skill_template_get_returns_full_bundle_and_single_declared_path() {
        let server = test_server();
        let full = server
            .handle_skill_template_get(Some(serde_json::Map::from_iter([(
                "template_id".to_owned(),
                json!("engineering-journal-skill"),
            )])))
            .await
            .unwrap();
        assert_eq!(full.is_error, Some(false));
        let bundle = result_json(&full);
        assert_eq!(bundle["manifest"]["id"], "engineering-journal-skill");
        assert!(bundle["source"]["repository"].as_str().is_some());
        assert!(bundle["source"]["commit"].as_str().is_some());
        assert!(bundle["source"]["dirty"].is_boolean());
        assert_eq!(bundle["aggregate_sha256"].as_str().unwrap().len(), 64);
        let declared = bundle["manifest"]["files"].as_array().unwrap();
        let files = bundle["files"].as_array().unwrap();
        assert_eq!(files.len(), declared.len());
        let mut declared_paths = declared
            .iter()
            .map(|file| file["path"].as_str().unwrap())
            .collect::<Vec<_>>();
        declared_paths.sort_unstable();
        assert_eq!(
            files
                .iter()
                .map(|file| file["path"].as_str().unwrap())
                .collect::<Vec<_>>(),
            declared_paths
        );
        assert!(files.iter().all(|file| file["content"].is_string()));

        let one = server
            .handle_skill_template_get(Some(serde_json::Map::from_iter([
                ("template_id".to_owned(), json!("engineering-journal-skill")),
                ("path".to_owned(), json!("SKILL.md")),
            ])))
            .await
            .unwrap();
        assert_eq!(one.is_error, Some(false));
        let one = result_json(&one);
        assert_eq!(one["files"].as_array().unwrap().len(), 1);
        assert_eq!(one["files"][0]["path"], "SKILL.md");
    }

    #[tokio::test]
    async fn skill_template_get_invalid_requests_are_mcp_errors() {
        let server = test_server();
        let cases = [
            None,
            Some(serde_json::Map::new()),
            Some(serde_json::Map::from_iter([(
                "template_id".to_owned(),
                json!(42),
            )])),
            Some(serde_json::Map::from_iter([(
                "template_id".to_owned(),
                json!("unknown-template"),
            )])),
            Some(serde_json::Map::from_iter([
                ("template_id".to_owned(), json!("engineering-journal-skill")),
                ("path".to_owned(), json!(42)),
            ])),
            Some(serde_json::Map::from_iter([
                ("template_id".to_owned(), json!("engineering-journal-skill")),
                ("path".to_owned(), json!("undeclared.txt")),
            ])),
            Some(serde_json::Map::from_iter([
                ("template_id".to_owned(), json!("engineering-journal-skill")),
                ("unexpected".to_owned(), json!(true)),
            ])),
        ];

        for args in cases {
            let result = server.handle_skill_template_get(args).await.unwrap();
            assert_eq!(result.is_error, Some(true));
            assert!(!result.content.is_empty());
        }
    }

    #[tokio::test]
    async fn call_tool_routing_keeps_programmatic_tools_ahead_of_inert_template_ids() {
        let server = test_server();

        let catalog = server.route_tool("skill_catalog", None).await.unwrap();
        assert_eq!(catalog.is_error, Some(false));

        let template = server
            .route_tool("document-feature-skill", None)
            .await
            .unwrap();
        assert_eq!(template.is_error, Some(true));
        assert!(template.content.iter().any(|content| {
            serde_json::to_value(content)
                .ok()
                .and_then(|value| value["text"].as_str().map(str::to_owned))
                .is_some_and(|text| text.contains("unknown tool: document-feature-skill"))
        }));
    }

    #[test]
    fn skill_template_ids_are_inert_and_existing_tools_remain_listed() {
        let tools = test_server().listed_tools();
        let names = tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>();

        assert!(!names.contains(&"engineering-journal-skill"));
        assert!(!names.contains(&"document-feature-skill"));
        assert!(names.contains(&"engineering-journal"));
        assert!(names.contains(&"catchup"));
        assert!(names.contains(&"vault_search"));
        assert!(names.contains(&"vault_reflect"));
        assert!(names.contains(&"skill_catalog"));
        assert!(names.contains(&"skill_template_get"));
    }

    #[test]
    fn engineering_journal_skill_is_embedded() {
        let skills = load_skills().expect("embedded skills should load");
        let journal = skills
            .iter()
            .find(|skill| skill.name == "engineering-journal")
            .expect("engineering-journal should be served");

        assert!(journal.description.contains("non-trivial"));
        assert!(journal.body.contains("status: open"));
        assert!(journal.body.contains("status: shipped"));
        assert!(journal.body.contains("status: no-go"));
        assert!(journal.body.contains("status: superseded"));
    }

    #[test]
    fn engineering_journal_contract_covers_reconciliation_boundaries() {
        let skills = load_skills().expect("embedded skills should load");
        let body = &skills
            .iter()
            .find(|skill| skill.name == "engineering-journal")
            .expect("engineering-journal should be served")
            .body;

        for required in [
            "intent-first",
            "single-PR",
            "docs/journal/README.md",
            "Problem framing candidate",
            "Design reasoning candidate",
            "Do not invoke `frame-problem`",
            "Do not invoke `propose-design`",
        ] {
            assert!(
                body.contains(required),
                "missing contract marker: {required}"
            );
        }
    }

    #[test]
    fn engineering_journal_evals_cover_key_scenarios() {
        let raw = include_str!("../skills/engineering-journal/evals/trigger-evals.json");
        let value: serde_json::Value =
            serde_json::from_str(raw).expect("journal evals should be valid JSON");
        let evals = value["evals"]
            .as_array()
            .expect("journal evals should contain an evals array");

        assert_eq!(evals.len(), 10);
        for eval in evals {
            assert!(eval["name"].as_str().is_some());
            assert!(eval["prompt"].as_str().is_some());
            assert!(eval["expectations"]
                .as_array()
                .is_some_and(|v| !v.is_empty()));
        }
    }
}
