//! The `iop` Claude Code plugin is a tree of symlinks into `skills/repository/`.
//! The vault and catalog skills need tools only the MCP server provides, so they
//! stay out of it. The frontmatter checks repeat the ones `check-skills` in
//! `brayniac/custom-skills` makes, because Claude Code loads a malformed skill
//! without an error.

use std::collections::BTreeSet;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const PLUGIN_SKILLS: &str = "plugins/iop/skills";
const SOURCE_CLUSTER: &str = "skills/repository";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn entry_names(dir: &Path) -> BTreeSet<String> {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("read {}: {error}", dir.display()))
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        // Finder writes `.DS_Store`, which `.gitignore` hides.
        .filter(|name| name != ".DS_Store")
        .collect()
}

#[test]
fn plugin_links_every_repository_skill_and_nothing_else() {
    let linked = entry_names(&root().join(PLUGIN_SKILLS));
    let sources = entry_names(&root().join(SOURCE_CLUSTER));
    assert!(!sources.is_empty());
    assert_eq!(linked, sources);
}

#[test]
fn plugin_skills_are_relative_symlinks_to_their_repository_skill() {
    for name in entry_names(&root().join(PLUGIN_SKILLS)) {
        let link = root().join(PLUGIN_SKILLS).join(&name);
        let meta = fs::symlink_metadata(&link).unwrap();
        assert!(
            meta.file_type().is_symlink(),
            "{name} must be a symlink, not a copy"
        );
        let target = fs::read_link(&link).unwrap();
        assert_eq!(
            target,
            Path::new("../../../skills/repository").join(&name),
            "{name} links to the wrong place"
        );
        assert!(
            link.join("SKILL.md").is_file(),
            "{name} does not resolve to a skill"
        );
    }
}

#[test]
fn marketplace_lists_the_plugin_without_a_pinned_version() {
    let marketplace: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root().join(".claude-plugin/marketplace.json")).unwrap(),
    )
    .unwrap();
    let plugins = marketplace["plugins"].as_array().unwrap();
    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0]["name"], "iop");
    assert_eq!(plugins[0]["source"], "./plugins/iop");
    // A version here pins installs just as one in `plugin.json` does.
    assert!(plugins[0].get("version").is_none());

    let plugin: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root().join("plugins/iop/.claude-plugin/plugin.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(plugin["name"], "iop");
    // Without a version, an install's version is the commit SHA, so every commit
    // on the default branch is an update.
    assert!(plugin.get("version").is_none());
}

/// The tools the server lists that are not skills: the tools a plugin-only host
/// does not have. Asked of the built binary, so it covers every way a tool can
/// be registered.
fn server_tool_names() -> Vec<String> {
    let mut server = Command::new(env!("CARGO_BIN_EXE_skills-mcp"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = server.stdin.take().unwrap();
    for request in [
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"plugin-layout","version":"0"}}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#,
    ] {
        writeln!(stdin, "{request}").unwrap();
    }
    // Read on a thread so a server that never answers fails the test instead of
    // hanging it.
    let stdout = server.stdout.take().unwrap();
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let listing = BufReader::new(stdout)
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(&line).ok())
            .find(|message| message["id"] == 2);
        let _ = sender.send(listing);
    });
    let listing = receiver.recv_timeout(Duration::from_secs(30));
    server.kill().unwrap();
    server.wait().unwrap();
    let listing = listing
        .expect("no tools/list response within 30 seconds")
        .expect("server closed stdout before answering tools/list");

    assert!(
        listing.get("error").is_none(),
        "tools/list failed: {listing}"
    );
    let result = &listing["result"];
    assert!(
        result.get("nextCursor").is_none(),
        "tools/list is paginated"
    );
    let skills: BTreeSet<String> = ["skills/repository", "skills/vault", "skills/catalog"]
        .into_iter()
        .flat_map(skill_names)
        .collect();
    let names: Vec<String> = result["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap().to_owned())
        .filter(|name| !skills.contains(name))
        .collect();
    assert!(
        names.iter().any(|n| n == "vault_search") && names.iter().any(|n| n == "skill_catalog"),
        "server tools: {names:?}"
    );
    names
}

fn skill_names(cluster: &str) -> BTreeSet<String> {
    entry_names(&root().join(cluster))
}

fn files_under(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files_under(&path, out);
        } else {
            out.push(path);
        }
    }
}

fn names_word(text: &str, word: &str) -> bool {
    let is_ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    text.match_indices(word).any(|(at, _)| {
        let before = &text[..at];
        let after = text[at + word.len()..].chars().next();
        // `mcp__<server>__<tool>` is how Claude Code names an MCP tool.
        let starts = !before.chars().next_back().is_some_and(is_ident) || before.ends_with("__");
        starts && !after.is_some_and(is_ident)
    })
}

/// The skill each backticked span on `line` would invoke: `name`, `/name`,
/// `/name <args>`, `plugin:name` and `mcp__<server>__name` all name `name`.
fn backticked_skills(line: &str) -> Vec<&str> {
    line.split('`')
        .skip(1)
        .step_by(2)
        .filter_map(|span| span.split_whitespace().next())
        .map(|token| token.trim_start_matches('/'))
        .map(|token| token.rsplit(':').next().unwrap())
        // `mcp__<server>__<skill>`: the server serves each skill as a tool.
        .map(|token| token.rsplit("__").next().unwrap())
        .collect()
}

#[test]
fn repository_skills_need_nothing_only_the_server_provides() {
    // A plugin skill that calls a server tool, or hands off to a skill the
    // plugin leaves out, fails on a host with only the plugin installed.
    let tools = server_tool_names();
    let excluded: BTreeSet<String> = skill_names("skills/vault")
        .into_iter()
        .chain(skill_names("skills/catalog"))
        .collect();
    for name in entry_names(&root().join(SOURCE_CLUSTER)) {
        let mut files = Vec::new();
        files_under(&root().join(SOURCE_CLUSTER).join(&name), &mut files);
        for file in files {
            let Ok(text) = fs::read_to_string(&file) else {
                continue;
            };
            for (number, line) in text.lines().enumerate() {
                let at = format!("{}:{}", file.display(), number + 1);
                for tool in &tools {
                    assert!(!names_word(line, tool), "{at} names the server tool {tool}");
                }
                // "Do not invoke `name`" tells the agent not to hand off. A
                // substring test cannot tell what the rest of the sentence says.
                let mut line = line.to_owned();
                for skill in &excluded {
                    line = line.replace(&format!("Do not invoke `{skill}`"), "");
                }
                for skill in backticked_skills(&line) {
                    assert!(
                        !excluded.contains(skill),
                        "{at} hands off to {skill}, which only the server serves"
                    );
                }
            }
        }
    }
}

/// The keys `check-skills` in `brayniac/custom-skills` allows. Claude Code and
/// `claude plugin validate --strict` ignore a key they do not know, so a
/// misspelled one would pass unnoticed.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Frontmatter {
    name: String,
    description: String,
}

/// Claude Code truncates one skill's listed description past this many
/// characters.
const MAX_DESCRIPTION_CHARS: usize = 1536;

/// The skill a `SKILL.md` declares, or why Claude Code would load it wrongly.
/// Frontmatter the server cannot parse already fails `load_skills`; the checks
/// after the parse are the ones nothing else makes.
fn check_skill(dir_name: &str, text: &str) -> Result<Frontmatter, String> {
    let yaml = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .map(|(yaml, _)| yaml)
        .ok_or("no YAML frontmatter")?;
    let meta: Frontmatter = serde_yaml::from_str(yaml).map_err(|error| {
        format!("frontmatter must be YAML with exactly name and description: {error}")
    })?;
    let valid_name = (1..=64).contains(&meta.name.len())
        && meta
            .name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_');
    if !valid_name {
        return Err(format!(
            "name {:?} must match [a-z0-9_-]{{1,64}}",
            meta.name
        ));
    }
    // The plugin lists a skill by its directory; the server serves it by `name`.
    if meta.name != dir_name {
        return Err(format!("name {:?} differs from its directory", meta.name));
    }
    let chars = meta.description.trim().chars().count();
    if !(1..=MAX_DESCRIPTION_CHARS).contains(&chars) {
        return Err(format!(
            "description is {chars} characters; it must be 1 to {MAX_DESCRIPTION_CHARS}"
        ));
    }
    Ok(meta)
}

/// Every skill directory the server embeds and the plugin links, with the
/// directory name each is known by.
fn skill_dirs() -> Vec<(String, PathBuf)> {
    let mut dirs = Vec::new();
    for cluster in [
        "skills/repository",
        "skills/vault",
        "skills/catalog",
        PLUGIN_SKILLS,
    ] {
        for name in entry_names(&root().join(cluster)) {
            let dir = root().join(cluster).join(&name);
            dirs.push((name, dir));
        }
    }
    dirs
}

#[test]
fn skill_frontmatter_loads_in_claude_code() {
    let dirs = skill_dirs();
    assert!(dirs.len() > entry_names(&root().join(PLUGIN_SKILLS)).len());
    for (name, dir) in dirs {
        let path = dir.join("SKILL.md");
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        if let Err(problem) = check_skill(&name, &text) {
            panic!("{}: {problem}", path.display());
        }
    }
}

#[test]
fn skill_check_rejects_what_claude_code_loads_wrongly() {
    let skill = |name: &str, extra: &str, description: &str| {
        format!("---\nname: {name}\n{extra}description: {description}\n---\n\nBody.\n")
    };
    assert!(check_skill("a-b_1", &skill("a-b_1", "", "Does a thing.")).is_ok());
    assert!(check_skill("a", &skill("a", "", &"é".repeat(MAX_DESCRIPTION_CHARS))).is_ok());
    for (dir, text) in [
        ("a", "name: a\ndescription: x\n".to_owned()),
        ("a", skill("a", "", "\"unterminated")),
        // Claude Code re-quotes this value and loads it; the server's parser
        // does not.
        ("a", skill("a", "", "Use when: a colon")),
        ("a", skill("a", "allowed-tools: Bash\n", "x")),
        ("a", skill("a", "", "")),
        ("a", skill("a", "", &"x".repeat(MAX_DESCRIPTION_CHARS + 1))),
        ("b", skill("a", "", "x")),
        ("Upper", skill("Upper", "", "x")),
        ("a.b", skill("a.b", "", "x")),
        (&"a".repeat(65), skill(&"a".repeat(65), "", "x")),
    ] {
        assert!(check_skill(dir, &text).is_err(), "accepted {text:?}");
    }
}

/// Claude Code 2.1.308 sizes the skill listing at
/// `(contextWindow ?? 200000) * 4 * 0.01` characters: 8,000 for a 200,000-token
/// window and 40,000 for 1,000,000. `skillListingBudgetFraction` and
/// `SLASH_COMMAND_TOOL_CHAR_BUDGET` override it. Every skill on the host shares
/// the listing, and past the budget Claude Code drops the descriptions of the
/// least-used skills. Past this bound, the plugin alone overflows the listing
/// on a 200,000-token model.
const PLUGIN_LISTING_BUDGET: usize = 8_000;

/// One listing entry is `- iop:<name>: <description>` and a newline; Claude Code
/// counts it as `iop:<name>`, four characters, the description, and one
/// separator.
fn listing_entry_chars(name: &str, description: &str) -> usize {
    "iop:".len() + name.len() + 4 + description.trim().chars().count() + 1
}

#[test]
fn plugin_listing_fits_the_budget_of_a_200k_window() {
    let mut total = 0;
    for name in entry_names(&root().join(PLUGIN_SKILLS)) {
        let path = root().join(PLUGIN_SKILLS).join(&name).join("SKILL.md");
        let meta = check_skill(&name, &fs::read_to_string(&path).unwrap())
            .unwrap_or_else(|problem| panic!("{}: {problem}", path.display()));
        total += listing_entry_chars(&meta.name, &meta.description);
    }
    eprintln!("plugin skill listing: {total} characters of {PLUGIN_LISTING_BUDGET}");
    assert!(
        total <= PLUGIN_LISTING_BUDGET,
        "plugin skill listing totals {total} characters, past {PLUGIN_LISTING_BUDGET}"
    );
}
