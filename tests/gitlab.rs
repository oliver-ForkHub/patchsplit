#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct Workspace(PathBuf);

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const PROJECT: &str = "zitzhen/coco-community-control";
const SHORT_HASH: &str = "de9ea1a";
const FULL_HASH: &str = "de9ea1a4a3f6ad6b0ade271b958bf05142f8be89";

fn mr_patch_body() -> String {
    format!(
        "From {first} Mon Sep 17 00:00:00 2001\n\
From: Test <test@example.com>\n\
Subject: [PATCH 1/2] First\n\
\n\
diff --git a/a.txt b/a.txt\n\
new file mode 100644\n\
--- /dev/null\n\
+++ b/a.txt\n\
@@ -0,0 +1 @@\n\
+a\n\
From {second} Mon Sep 17 00:00:00 2001\n\
From: Test <test@example.com>\n\
Subject: [PATCH 2/2] Second\n\
\n\
diff --git a/b.txt b/b.txt\n\
new file mode 100644\n\
--- /dev/null\n\
+++ b/b.txt\n\
@@ -0,0 +1 @@\n\
+b\n",
        first = "1".repeat(40),
        second = "2".repeat(40)
    )
}

fn mr_diff_body() -> String {
    "diff --git a/a.txt b/a.txt\n\
new file mode 100644\n\
--- /dev/null\n\
+++ b/a.txt\n\
@@ -0,0 +1 @@\n\
+a\n"
    .to_string()
}

fn commit_patch_body() -> String {
    format!(
        "From {FULL_HASH} Mon Sep 17 00:00:00 2001\n\
From: Test <test@example.com>\n\
Subject: [PATCH] Single commit\n\
\n\
diff --git a/a.txt b/a.txt\n\
new file mode 100644\n\
--- /dev/null\n\
+++ b/a.txt\n\
@@ -0,0 +1 @@\n\
+a\n"
    )
}

#[test]
fn gitlab_cli_downloads_mr_squash_and_commit_patches() {
    let root = Workspace(std::env::temp_dir().join(format!(
        "patchsplit-gitlab-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    )));
    let bin = root.0.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let mr_patch = root.0.join("mr.patch");
    let mr_diff = root.0.join("mr.diff");
    let commit_patch = root.0.join("commit.patch");
    fs::write(&mr_patch, mr_patch_body()).unwrap();
    fs::write(&mr_diff, mr_diff_body()).unwrap();
    fs::write(&commit_patch, commit_patch_body()).unwrap();

    // Substitute only the HTTP boundary; exercise the real CLI end to end.
    let curl = bin.join("curl");
    fs::write(
        &curl,
        "#!/bin/sh\n\
for arg do url=\"$arg\"; done\n\
case \"$url\" in\n\
  'https://gitlab.com/zitzhen/coco-community-control/-/merge_requests/363.patch')\n\
    cat \"$PATCHSPLIT_TEST_MR_PATCH\" ;;\n\
  'https://gitlab.com/zitzhen/coco-community-control/-/merge_requests/363.diff')\n\
    cat \"$PATCHSPLIT_TEST_MR_DIFF\" ;;\n\
  'https://gitlab.com/zitzhen/coco-community-control/-/commit/de9ea1a.patch')\n\
    cat \"$PATCHSPLIT_TEST_COMMIT\" ;;\n\
  *) echo \"unexpected url: $url\" >&2; exit 22 ;;\n\
esac\n",
    )
    .unwrap();
    fs::set_permissions(&curl, fs::Permissions::from_mode(0o755)).unwrap();
    let mut paths = vec![bin];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let path = std::env::join_paths(paths).unwrap();

    let run = |args: &[&str]| -> Output {
        Command::new(env!("CARGO_BIN_EXE_patchsplit"))
            .args(args)
            .env("PATH", &path)
            .env("PATCHSPLIT_TEST_MR_PATCH", &mr_patch)
            .env("PATCHSPLIT_TEST_MR_DIFF", &mr_diff)
            .env("PATCHSPLIT_TEST_COMMIT", &commit_patch)
            .env("PATCHSPLIT_LANGUAGE", "C")
            .output()
            .unwrap()
    };

    // Merge request: split one mailbox into per-commit patch files.
    let mr_dir = root.0.join("mr");
    let result = run(&[
        "--gitlab",
        PROJECT,
        "363",
        "--out",
        mr_dir.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let mut names: Vec<String> = fs::read_dir(&mr_dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    assert_eq!(names, vec!["0001-first.patch", "0002-second.patch"]);

    // Squash: one raw diff named after the merge request.
    let squash_dir = root.0.join("squash");
    let result = run(&[
        PROJECT,
        "363",
        "--gitlab",
        "--squash",
        "--out",
        squash_dir.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let squashed = squash_dir.join("mr-363.patch");
    assert_eq!(fs::read(&squashed).unwrap(), mr_diff_body().as_bytes());

    // Single commit by short hash.
    let commit_dir = root.0.join("commit");
    let result = run(&[
        "--gitlab",
        PROJECT,
        "--commit",
        SHORT_HASH,
        "--out",
        commit_dir.to_str().unwrap(),
    ]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        fs::read(commit_dir.join(format!("{SHORT_HASH}.patch"))).unwrap(),
        commit_patch_body().as_bytes()
    );

    // A project path without a namespace is rejected with exit code 2.
    let bad = run(&["--gitlab", "project-only", "363"]);
    assert_eq!(bad.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&bad.stderr).contains("namespace/project"),
        "{}",
        String::from_utf8_lossy(&bad.stderr)
    );
}
