#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct Workspace(PathBuf);

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

#[test]
fn squash_cli_writes_an_applicable_net_diff_and_protects_existing_files() {
    let root = Workspace(std::env::temp_dir().join(format!(
        "patchsplit-squash-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    )));
    let repo = root.0.join("repo");
    let bin = root.0.join("bin");
    fs::create_dir_all(&repo).unwrap();
    fs::create_dir_all(&bin).unwrap();
    git(&repo, &["init", "-q"]);
    git(&repo, &["config", "user.name", "Test"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    fs::write(repo.join("edited"), "original\n").unwrap();
    fs::write(repo.join("reverted"), "unchanged\n").unwrap();
    fs::write(repo.join("old-name"), "rename me\n").unwrap();
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-qm", "base"]);
    fs::write(repo.join("edited"), "intermediate\n").unwrap();
    fs::write(repo.join("reverted"), "temporary\n").unwrap();
    fs::write(repo.join("temporary-file"), "temporary\n").unwrap();
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-qm", "first"]);
    fs::write(repo.join("edited"), "final without newline").unwrap();
    fs::write(repo.join("reverted"), "unchanged\n").unwrap();
    fs::remove_file(repo.join("temporary-file")).unwrap();
    fs::rename(repo.join("old-name"), repo.join("new-name")).unwrap();
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-qm", "second"]);
    let expected_tree = git(&repo, &["rev-parse", "HEAD^{tree}"]);
    let diff = git(
        &repo,
        &["diff", "--no-ext-diff", "--no-textconv", "HEAD~2", "HEAD"],
    );
    let diff_text = String::from_utf8_lossy(&diff);
    assert!(!diff_text.contains("intermediate"));
    assert!(!diff_text.contains("reverted"));
    assert!(!diff_text.contains("temporary-file"));
    let fixture = root.0.join("response.diff");
    fs::write(&fixture, &diff).unwrap();
    // Substitute only the HTTP boundary; exercise the real CLI and Git application.
    let curl = bin.join("curl");
    fs::write(&curl, "#!/bin/sh\nfor arg do url=\"$arg\"; done\n[ \"$url\" = 'https://github.com/owner/repo/pull/42.diff' ] || exit 22\ncat \"$PATCHSPLIT_TEST_DIFF\"\n").unwrap();
    fs::set_permissions(&curl, fs::Permissions::from_mode(0o755)).unwrap();
    let mut paths = vec![bin];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let path = std::env::join_paths(paths).unwrap();
    let output_dir = root.0.join("patches");
    let run = |extra: &[&str]| -> Output {
        Command::new(env!("CARGO_BIN_EXE_patchsplit"))
            .args(["owner/repo", "42", "--squash", "--out"])
            .arg(&output_dir)
            .args(extra)
            .env("PATH", &path)
            .env("PATCHSPLIT_TEST_DIFF", &fixture)
            .env("PATCHSPLIT_LANGUAGE", "C")
            .output()
            .unwrap()
    };
    assert!(run(&[]).status.success());
    let patch = output_dir.join("pr-42.patch");
    assert_eq!(fs::read_dir(&output_dir).unwrap().count(), 1);
    assert_eq!(fs::read(&patch).unwrap(), diff);
    git(&repo, &["checkout", "--detach", "HEAD~2"]);
    git(&repo, &["apply", "--index", patch.to_str().unwrap()]);
    assert_eq!(git(&repo, &["write-tree"]), expected_tree);

    fs::write(&patch, "user edits").unwrap();
    let result = run(&[]);
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stderr).contains("refusing to overwrite"));
    assert_eq!(fs::read_to_string(&patch).unwrap(), "user edits");
    assert!(run(&["--force"]).status.success());
    assert_eq!(fs::read(&patch).unwrap(), diff);

    fs::remove_file(&patch).unwrap();
    for (body, error) in [
        (" \n", "downloaded patch is empty"),
        ("<!DOCTYPE html>", "not a Git diff"),
    ] {
        fs::write(&fixture, body).unwrap();
        let result = run(&[]);
        assert_eq!(result.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&result.stderr).contains(error));
        assert!(!patch.exists());
    }
}
