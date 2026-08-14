//! Guards line-number citations in durable documents.
//!
//! A citation like `src/main.rs:38` in a journal entry is a fact that decays
//! silently: editing the cited file shifts the line, and nothing notices. This
//! test requires every such citation to be anchored by a phrase from the same
//! paragraph that appears verbatim at the cited line. When the phrase moves,
//! the number is re-derivable; when the phrase is gone, the claim itself needs
//! a human.
//!
//!   cargo test --locked citations          check (what CI runs)
//!   CITATIONS_FIX=1 cargo test citations   rewrite line numbers that moved
//!
//! A missing phrase fails in both modes. Nothing can be re-derived from an
//! anchor that no longer exists.
//!
//! Put `<!-- cite-ignore -->` on its own line before a paragraph whose
//! citation is illustrative rather than a claim about the repository.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const SCAN_ROOTS: &[&str] = &["docs", "skills", "templates"];
const SCAN_FILES: &[&str] = &["README.md"];
const IGNORE_MARKER: &str = "<!-- cite-ignore -->";
const MIN_ANCHOR_LEN: usize = 3;
/// Confirming an anchor sits at the cited line is safe with a short phrase.
/// Moving a citation on the strength of one is not: a seven-character token
/// occurring once is a coincidence, not evidence.
const MIN_RELOCATE_LEN: usize = 8;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// A `path:line` or `path:start-end` reference, or a bare `:line` that
/// inherits the path from the previous citation in its paragraph.
#[derive(Debug, PartialEq)]
struct Citation {
    token: String,
    path: Option<String>,
    start: usize,
    end: usize,
}

/// Parses the inside of a backtick span. Returns `None` for spans that are not
/// citations, which is most of them.
fn parse_citation(span: &str) -> Option<Citation> {
    if span.contains("://") || span.starts_with("http") {
        return None;
    }
    let (head, tail) = span.rsplit_once(':')?;
    let (first, last) = match tail.split_once('-') {
        Some((a, b)) => (a, b),
        None => (tail, tail),
    };
    let start: usize = first.parse().ok()?;
    let end: usize = last.parse().ok()?;
    if start == 0 || end < start {
        return None;
    }
    if head.is_empty() {
        return Some(Citation {
            token: span.to_owned(),
            path: None,
            start,
            end,
        });
    }
    // A path we can resolve has an extension and no whitespace.
    if !head.contains('.') || head.chars().any(char::is_whitespace) {
        return None;
    }
    Some(Citation {
        token: span.to_owned(),
        path: Some(head.to_owned()),
        start,
        end,
    })
}

/// Contents of every span delimited by `mark`, each with its byte offset in
/// `text`. Offsets let a citation be matched to the phrase written beside it,
/// which is what disambiguates two citations sharing one sentence.
fn delimited_spans(text: &str, mark: char) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let mut base = 0usize;
    let mut rest = text;
    while let Some(open) = rest.find(mark) {
        let after = &rest[open + 1..];
        match after.find(mark) {
            Some(close) => {
                out.push((after[..close].to_owned(), base + open + 1));
                base += open + 1 + close + 1;
                rest = &after[close + 1..];
            }
            None => break,
        }
    }
    out
}

fn backtick_spans(text: &str) -> Vec<String> {
    delimited_spans(text, '`')
        .into_iter()
        .map(|(span, _)| span)
        .collect()
}

fn quoted_spans(text: &str) -> Vec<String> {
    delimited_spans(text, '"')
        .into_iter()
        .map(|(span, _)| span)
        .collect()
}

/// True for a line that opens a new list item, so sibling bullets do not pool
/// their anchors. Without this, a citation in one bullet can be "anchored" by a
/// phrase belonging to an unrelated bullet three lines away.
fn starts_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .or_else(|| trimmed.strip_prefix("+ "))
    {
        return !rest.trim().is_empty();
    }
    let digits: String = trimmed.chars().take_while(char::is_ascii_digit).collect();
    !digits.is_empty() && trimmed[digits.len()..].starts_with(". ")
}

/// Paragraphs of a markdown file, with the 1-based line each one starts on.
/// Fenced code blocks are dropped: an example inside a fence is not a claim.
fn paragraphs(markdown: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    let mut start = 0usize;
    let mut fenced = false;

    for (index, line) in markdown.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fenced = !fenced;
            if !current.is_empty() {
                out.push((start, current.join("\n")));
                current.clear();
            }
            continue;
        }
        if fenced {
            continue;
        }
        if line.trim().is_empty() {
            if !current.is_empty() {
                out.push((start, current.join("\n")));
                current.clear();
            }
            continue;
        }
        if starts_list_item(line) && !current.is_empty() {
            out.push((start, current.join("\n")));
            current.clear();
        }
        if current.is_empty() {
            start = index + 1;
        }
        current.push(line);
    }
    if !current.is_empty() {
        out.push((start, current.join("\n")));
    }
    out
}

fn markdown_files() -> Vec<PathBuf> {
    let mut files = BTreeSet::new();
    for name in SCAN_FILES {
        let path = root().join(name);
        if path.is_file() {
            files.insert(path);
        }
    }
    for dir in SCAN_ROOTS {
        let base = root().join(dir);
        if !base.is_dir() {
            continue;
        }
        for entry in WalkDir::new(&base).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|e| e == "md") {
                files.insert(path.to_path_buf());
            }
        }
    }
    files.into_iter().collect()
}

fn relative(path: &Path) -> String {
    path.strip_prefix(root())
        .unwrap_or(path)
        .display()
        .to_string()
}

enum Outcome {
    Ok,
    Moved { to: usize },
    Problem { detail: String },
}

fn check_citation(cite: &Citation, at: usize, path: &str, anchors: &[(String, usize)]) -> Outcome {
    let target = root().join(path);
    if !target.is_file() {
        return Outcome::Problem {
            detail: format!("cited file `{path}` does not exist"),
        };
    }
    let body = match fs::read_to_string(&target) {
        Ok(body) => body,
        Err(error) => {
            return Outcome::Problem {
                detail: format!("cannot read `{path}`: {error}"),
            }
        }
    };
    let lines: Vec<&str> = body.lines().collect();
    if cite.end > lines.len() {
        return Outcome::Problem {
            detail: format!(
                "`{}` points past the end of `{path}` ({} lines)",
                cite.token,
                lines.len()
            ),
        };
    }

    // Nearest phrase first: the one written beside a citation is the one
    // describing it.
    let mut ordered: Vec<&(String, usize)> = anchors
        .iter()
        .filter(|(a, _)| a.trim().len() >= MIN_ANCHOR_LEN)
        .collect();
    ordered.sort_by_key(|(_, pos)| pos.abs_diff(at));
    let usable: Vec<&String> = ordered.iter().map(|(anchor, _)| anchor).collect();
    if usable.is_empty() {
        return Outcome::Problem {
            detail: format!(
                "`{}` has no anchor phrase in its paragraph. Quote a phrase from \
                 the cited line so the number stays re-derivable.",
                cite.token
            ),
        };
    }

    let cited_text = lines[cite.start - 1..cite.end].join("\n");
    if usable.iter().any(|a| cited_text.contains(a.as_str())) {
        return Outcome::Ok;
    }

    // The anchor is not where the citation says. Relocate on the nearest phrase
    // that is long enough to be evidence and occurs exactly once in the file.
    let relocation = usable
        .iter()
        .filter(|a| a.len() >= MIN_RELOCATE_LEN)
        .find_map(|anchor| {
            let hits: Vec<usize> = lines
                .iter()
                .enumerate()
                .filter(|(_, line)| line.contains(anchor.as_str()))
                .map(|(index, _)| index + 1)
                .collect();
            (hits.len() == 1).then_some(hits[0])
        });
    match relocation {
        Some(to) => Outcome::Moved { to },
        None => {
            // Say where each anchor actually is. "Not found" sends the author
            // hunting; "now at 29 and 39" lets them just pick one.
            let mut sightings: Vec<String> = Vec::new();
            for anchor in &usable {
                let hits: Vec<String> = lines
                    .iter()
                    .enumerate()
                    .filter(|(_, line)| line.contains(anchor.as_str()))
                    .map(|(index, _)| (index + 1).to_string())
                    .collect();
                sightings.push(match hits.len() {
                    0 => format!("{anchor:?} appears nowhere"),
                    _ => format!("{anchor:?} appears at {}", hits.join(", ")),
                });
            }
            Outcome::Problem {
                detail: format!(
                    "`{}` is not anchored to line {} of `{path}`: {}. Quote a phrase \
                     unique to the line you mean.",
                    cite.token,
                    cite.start,
                    sightings.join("; ")
                ),
            }
        }
    }
}

#[test]
fn line_citations_stay_anchored_to_the_text_they_cite() {
    let fixing = std::env::var_os("CITATIONS_FIX").is_some();
    let mut problems: Vec<String> = Vec::new();
    let mut repaired: Vec<String> = Vec::new();
    let mut checked = 0usize;

    for file in markdown_files() {
        let original = match fs::read_to_string(&file) {
            Ok(body) => body,
            Err(error) => {
                problems.push(format!("{}: cannot read: {error}", relative(&file)));
                continue;
            }
        };
        let mut updated = original.clone();
        let mut skip_next = false;

        for (line_number, paragraph) in paragraphs(&original) {
            // The marker suppresses the paragraph it introduces, whether a
            // blank line separates the two or not.
            let first_line = paragraph.lines().next().unwrap_or_default().trim();
            if paragraph.trim() == IGNORE_MARKER {
                skip_next = true;
                continue;
            }
            if skip_next || first_line == IGNORE_MARKER {
                skip_next = false;
                continue;
            }

            let spans = delimited_spans(&paragraph, '`');
            let citations: Vec<(Citation, usize)> = spans
                .iter()
                .filter_map(|(span, at)| parse_citation(span).map(|cite| (cite, *at)))
                .collect();
            if citations.is_empty() {
                continue;
            }
            let anchors: Vec<(String, usize)> = spans
                .iter()
                .filter(|(span, _)| parse_citation(span).is_none())
                .cloned()
                .chain(delimited_spans(&paragraph, '"'))
                .collect();

            let mut inherited: Option<String> = None;
            for (cite, at) in &citations {
                let path = match cite.path.clone().or_else(|| inherited.clone()) {
                    Some(path) => path,
                    None => {
                        problems.push(format!(
                            "{}:{line_number}: `{}` has no path and no earlier \
                             citation to inherit one from",
                            relative(&file),
                            cite.token
                        ));
                        continue;
                    }
                };
                inherited = Some(path.clone());
                checked += 1;

                match check_citation(cite, *at, &path, &anchors) {
                    Outcome::Ok => {}
                    Outcome::Moved { to } => {
                        let old = format!("`{}`", cite.token);
                        let new = match cite.path {
                            Some(ref p) => format!("`{p}:{to}`"),
                            None => format!("`:{to}`"),
                        };
                        if fixing {
                            updated = updated.replace(&old, &new);
                            repaired
                                .push(format!("{}:{line_number}: {old} -> {new}", relative(&file)));
                        } else {
                            problems.push(format!(
                                "{}:{line_number}: {old} moved; its anchor is now at \
                                 line {to}. Re-run with CITATIONS_FIX=1 to update it.",
                                relative(&file)
                            ));
                        }
                    }
                    Outcome::Problem { detail } => {
                        problems.push(format!("{}:{line_number}: {detail}", relative(&file)));
                    }
                }
            }
        }

        if fixing && updated != original {
            if let Err(error) = fs::write(&file, updated) {
                problems.push(format!("{}: cannot write fix: {error}", relative(&file)));
            }
        }
    }

    assert!(checked > 0, "found no line citations to check at all");

    if !repaired.is_empty() {
        eprintln!("repaired {} citation(s):", repaired.len());
        for line in &repaired {
            eprintln!("  {line}");
        }
    }
    assert!(
        problems.is_empty(),
        "{} citation problem(s):\n{}",
        problems.len(),
        problems.join("\n")
    );
}

#[cfg(test)]
mod parsing {
    use super::*;

    #[test]
    fn recognises_paths_lines_and_ranges() {
        let full = parse_citation("src/main.rs:38").expect("path citation");
        assert_eq!(full.path.as_deref(), Some("src/main.rs"));
        assert_eq!((full.start, full.end), (38, 38));

        let range = parse_citation("skills/x/SKILL.md:10-14").expect("range citation");
        assert_eq!((range.start, range.end), (10, 14));

        let bare = parse_citation(":37").expect("bare citation");
        assert_eq!(bare.path, None);
        assert_eq!((bare.start, bare.end), (37, 37));
    }

    #[test]
    fn rejects_spans_that_only_look_like_citations() {
        assert!(parse_citation("MIT OR Apache-2.0").is_none());
        assert!(parse_citation("cargo test --locked").is_none());
        assert!(parse_citation("https://example.com:8080").is_none());
        assert!(parse_citation("no_extension:12").is_none());
        assert!(parse_citation("src/main.rs:0").is_none());
        assert!(parse_citation("src/main.rs:14-9").is_none());
        assert!(parse_citation("some text:notanumber").is_none());
    }

    #[test]
    fn paragraphs_drop_fenced_blocks() {
        let markdown = "intro line\n\n```sh\n`src/main.rs:1` inside a fence\n```\n\nafter";
        let found = paragraphs(markdown);
        let joined: Vec<&str> = found.iter().map(|(_, text)| text.as_str()).collect();
        assert_eq!(joined, vec!["intro line", "after"]);
    }

    #[test]
    fn sibling_list_items_do_not_pool_their_anchors() {
        let markdown = "- first `alpha`\n- second `beta`\n- third `gamma`";
        let found = paragraphs(markdown);
        assert_eq!(found.len(), 3);
        assert_eq!(found[1].0, 2);
        assert_eq!(backtick_spans(&found[1].1), vec!["beta".to_owned()]);
    }

    #[test]
    fn list_item_continuation_stays_with_its_item() {
        let markdown = "- first line\n  wrapped continuation\n- second";
        let found = paragraphs(markdown);
        assert_eq!(found.len(), 2);
        assert!(found[0].1.contains("wrapped continuation"));
    }

    #[test]
    fn dashes_that_are_not_list_items_do_not_split() {
        assert!(!starts_list_item("some -- prose"));
        assert!(!starts_list_item("-"));
        assert!(starts_list_item("- item"));
        assert!(starts_list_item("  - nested"));
        assert!(starts_list_item("3. numbered"));
        assert!(!starts_list_item("3.no space"));
    }

    #[test]
    fn ignore_marker_works_attached_or_detached() {
        let attached = "<!-- cite-ignore -->\nthe `a/b.rs:1` example";
        let found = paragraphs(attached);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0].1.lines().next().unwrap().trim(),
            "<!-- cite-ignore -->"
        );

        let detached = "<!-- cite-ignore -->\n\nthe `a/b.rs:1` example";
        let found = paragraphs(detached);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].1.trim(), "<!-- cite-ignore -->");
    }

    #[test]
    fn paragraphs_report_starting_line() {
        let markdown = "one\n\ntwo\nstill two";
        let found = paragraphs(markdown);
        assert_eq!(found[0].0, 1);
        assert_eq!(found[1].0, 3);
    }

    #[test]
    fn extracts_backtick_and_quoted_anchors() {
        let text = "the `include_dir!` macro says \"treat this as beta\" here";
        assert_eq!(backtick_spans(text), vec!["include_dir!".to_owned()]);
        assert_eq!(quoted_spans(text), vec!["treat this as beta".to_owned()]);
    }
}
