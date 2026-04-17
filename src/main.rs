use std::{borrow::Cow, sync::Arc};

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, File};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, CallToolResult, Content, Implementation, InitializeResult,
        ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities, Tool,
    },
    service::RequestContext,
    transport::stdio,
};
use serde::Deserialize;
use serde_json::json;

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

fn empty_schema() -> Arc<serde_json::Map<String, serde_json::Value>> {
    let value = json!({
        "type": "object",
        "properties": {},
        "additionalProperties": false
    });
    match value {
        serde_json::Value::Object(map) => Arc::new(map),
        _ => unreachable!("literal is an object"),
    }
}

#[derive(Clone)]
struct SkillsServer {
    skills: Arc<Vec<LoadedSkill>>,
}

impl SkillsServer {
    fn new(skills: Vec<LoadedSkill>) -> Self {
        Self {
            skills: Arc::new(skills),
        }
    }

    fn find(&self, name: &str) -> Option<&LoadedSkill> {
        self.skills.iter().find(|s| s.name == name)
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
                "Each tool corresponds to a Claude skill. Invoke a tool to receive the skill's \
                 instructions; then follow them for the remainder of the task."
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let schema = empty_schema();
        let tools = self
            .skills
            .iter()
            .map(|s| Tool {
                name: Cow::Owned(s.name.clone()),
                title: None,
                description: Some(Cow::Owned(s.description.clone())),
                input_schema: schema.clone(),
                output_schema: None,
                annotations: None,
                execution: None,
                icons: None,
                meta: None,
            })
            .collect();
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
        match self.find(&request.name) {
            Some(skill) => Ok(CallToolResult::success(vec![Content::text(
                skill.body.clone(),
            )])),
            None => Ok(CallToolResult::error(vec![Content::text(format!(
                "unknown skill: {}",
                request.name
            ))])),
        }
    }
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
