use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(not(test))]
fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    for path in git_watch_paths(&manifest_dir) {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let (repository, commit, dirty) = resolve_build_provenance(
        git_output(&manifest_dir, &["config", "--get", "remote.origin.url"]),
        git_output(&manifest_dir, &["rev-parse", "HEAD"]),
        env::var("GITHUB_SHA").ok(),
        git_output(
            &manifest_dir,
            &[
                "status",
                "--porcelain",
                "--untracked-files=all",
                "--",
                "templates",
            ],
        ),
    );

    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_REPOSITORY={repository}");
    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_COMMIT={commit}");
    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_DIRTY={dirty}");
}

fn resolve_build_provenance(
    origin: Option<String>,
    git_head: Option<String>,
    github_sha: Option<String>,
    git_status: Option<String>,
) -> (String, String, bool) {
    let repository = resolve_source_repository(origin);
    let (commit, template_dirty) = resolve_provenance(git_head, github_sha, git_status);
    let dirty = template_dirty || repository.is_none();
    (
        repository.unwrap_or_else(|| "UNKNOWN".to_owned()),
        commit,
        dirty,
    )
}

fn resolve_source_repository(origin: Option<String>) -> Option<String> {
    let origin = origin?;
    let normalized = origin.trim().trim_end_matches('/');
    let path = normalized
        .strip_prefix("https://github.com/")
        .or_else(|| normalized.strip_prefix("git@github.com:"))
        .or_else(|| normalized.strip_prefix("ssh://git@github.com/"))?;
    let path = path.strip_suffix(".git").unwrap_or(path);
    let mut components = path.split('/');
    let owner = components.next()?;
    let repository = components.next()?;
    if components.next().is_some()
        || owner.is_empty()
        || repository.is_empty()
        || !owner
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        || !repository
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return None;
    }
    Some(format!("https://github.com/{owner}/{repository}"))
}

fn resolve_provenance(
    git_head: Option<String>,
    github_sha: Option<String>,
    git_status: Option<String>,
) -> (String, bool) {
    let commit = git_head
        .filter(|value| is_commit(value))
        .or_else(|| github_sha.filter(|value| is_commit(value)))
        .unwrap_or_else(|| "unknown".to_owned());
    let dirty = git_status.is_none_or(|output| !output.is_empty());
    (commit, dirty)
}

fn git_watch_paths(manifest_dir: &Path) -> Vec<PathBuf> {
    let mut names = vec!["HEAD".to_owned(), "index".to_owned(), "config".to_owned()];
    if let Some(branch_ref) = git_output(manifest_dir, &["symbolic-ref", "-q", "HEAD"]) {
        names.push(branch_ref);
    }

    names
        .iter()
        .filter_map(|name| git_path(manifest_dir, name))
        .collect()
}

fn git_path(manifest_dir: &Path, name: &str) -> Option<PathBuf> {
    git_output(manifest_dir, &["rev-parse", "--git-path", name]).map(|path| {
        let path = PathBuf::from(path);
        if path.is_absolute() {
            path
        } else {
            manifest_dir.join(path)
        }
    })
}

fn git_output(manifest_dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(manifest_dir)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()
        .map(|value| value.trim().to_owned())
}

fn is_commit(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    struct TempRepo(PathBuf);

    impl TempRepo {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = env::temp_dir().join(format!(
                "iop-skills-build-provenance-{}-{unique}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            let output = Command::new("git")
                .args(["init", "--initial-branch=review-test"])
                .current_dir(&path)
                .output()
                .unwrap();
            assert!(output.status.success());
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn failed_status_is_dirty_with_valid_fallback_commit() {
        let fallback = "0123456789abcdef0123456789abcdef01234567";

        assert_eq!(
            resolve_provenance(None, Some(fallback.to_owned()), None),
            (fallback.to_owned(), true)
        );
    }

    #[test]
    fn source_repository_normalizes_verifiable_github_origins() {
        for origin in [
            "https://github.com/iopsystems/skills-mcp",
            "https://github.com/iopsystems/skills-mcp.git",
            "git@github.com:iopsystems/skills-mcp.git",
            "ssh://git@github.com/iopsystems/skills-mcp.git",
        ] {
            assert_eq!(
                resolve_source_repository(Some(origin.to_owned())),
                Some("https://github.com/iopsystems/skills-mcp".to_owned())
            );
        }

        assert_eq!(
            resolve_source_repository(Some("https://github.com/example/skills-mcp.git".to_owned())),
            Some("https://github.com/example/skills-mcp".to_owned())
        );
        assert_eq!(
            resolve_source_repository(Some("git@github.com:example/skills-mcp.git".to_owned())),
            Some("https://github.com/example/skills-mcp".to_owned())
        );
        for origin in [
            "https://gitlab.com/example/skills-mcp.git",
            "file:///tmp/skills-mcp",
        ] {
            assert_eq!(resolve_source_repository(Some(origin.to_owned())), None);
        }
        assert_eq!(resolve_source_repository(None), None);
    }

    #[test]
    fn unverifiable_origin_is_unknown_and_dirty_while_fork_identity_is_preserved() {
        let commit = "0123456789abcdef0123456789abcdef01234567";
        assert_eq!(
            resolve_build_provenance(
                Some("file:///tmp/skills-mcp".to_owned()),
                Some(commit.to_owned()),
                None,
                Some(String::new()),
            ),
            ("UNKNOWN".to_owned(), commit.to_owned(), true)
        );
        assert_eq!(
            resolve_build_provenance(
                Some("https://github.com/example/skills-mcp.git".to_owned()),
                Some(commit.to_owned()),
                None,
                Some(String::new()),
            ),
            (
                "https://github.com/example/skills-mcp".to_owned(),
                commit.to_owned(),
                false,
            )
        );
    }

    #[test]
    fn git_watch_paths_include_symbolic_branch_ref() {
        let repo = TempRepo::new();

        let paths = git_watch_paths(repo.path());

        assert!(paths.contains(&repo.path().join(".git/HEAD")));
        assert!(paths.contains(&repo.path().join(".git/index")));
        assert!(paths.contains(&repo.path().join(".git/refs/heads/review-test")));
    }
}
