use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(not(test))]
const SOURCE_REPOSITORY: &str = "https://github.com/iopsystems/skills-mcp";

#[cfg(not(test))]
fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    for path in git_watch_paths(&manifest_dir) {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let (commit, dirty) = resolve_provenance(
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

    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_REPOSITORY={SOURCE_REPOSITORY}");
    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_COMMIT={commit}");
    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_DIRTY={dirty}");
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
    let mut names = vec!["HEAD".to_owned(), "index".to_owned()];
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
    fn git_watch_paths_include_symbolic_branch_ref() {
        let repo = TempRepo::new();

        let paths = git_watch_paths(repo.path());

        assert!(paths.contains(&repo.path().join(".git/HEAD")));
        assert!(paths.contains(&repo.path().join(".git/index")));
        assert!(paths.contains(&repo.path().join(".git/refs/heads/review-test")));
    }
}
