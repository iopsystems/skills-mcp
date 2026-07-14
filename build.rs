use std::{env, path::PathBuf, process::Command};

const SOURCE_REPOSITORY: &str = "https://github.com/iopsystems/skills-mcp";

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    emit_git_path_marker(&manifest_dir, "HEAD");
    emit_git_path_marker(&manifest_dir, "index");

    let commit = git_output(&manifest_dir, &["rev-parse", "HEAD"])
        .filter(|value| is_commit(value))
        .or_else(|| env::var("GITHUB_SHA").ok().filter(|value| is_commit(value)))
        .unwrap_or_else(|| "unknown".to_owned());
    let dirty = git_output(
        &manifest_dir,
        &[
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--",
            "templates",
        ],
    )
    .is_some_and(|output| !output.is_empty());

    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_REPOSITORY={SOURCE_REPOSITORY}");
    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_COMMIT={commit}");
    println!("cargo:rustc-env=IOP_SKILLS_SOURCE_DIRTY={dirty}");
}

fn emit_git_path_marker(manifest_dir: &PathBuf, name: &str) {
    if let Some(path) = git_output(manifest_dir, &["rev-parse", "--git-path", name]) {
        let path = PathBuf::from(path);
        let path = if path.is_absolute() {
            path
        } else {
            manifest_dir.join(path)
        };
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn git_output(manifest_dir: &PathBuf, args: &[&str]) -> Option<String> {
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
