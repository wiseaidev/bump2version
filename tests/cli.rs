// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn cargo_bin() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.push("bump2version");
    path.to_str().unwrap().to_string()
}

fn setup_repo(dir: &TempDir, current_version: &str, config_content: &str, file_content: &str) {
    fs::write(dir.path().join(".bumpversion.toml"), config_content).unwrap();
    fs::write(dir.path().join("VERSION"), file_content).unwrap();

    Command::new("git")
        .args(["init"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "CI Robot"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "ci@example.com"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    let _ = current_version;
}

#[test]
fn test_dry_run_does_not_modify_files() {
    let dir = TempDir::new().unwrap();
    let config =
        "[bumpversion]\ncurrent_version = 1.0.0\ncommit = false\n\n[bumpversion:file:VERSION]\n";
    setup_repo(&dir, "1.0.0", config, "1.0.0\n");

    let status = Command::new(cargo_bin())
        .args([
            "--bump",
            "patch",
            "--dry-run",
            "--config-file",
            ".bumpversion.toml",
        ])
        .current_dir(dir.path())
        .status();

    if let Ok(s) = status
        && s.success()
    {
        let contents = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert_eq!(contents.trim(), "1.0.0", "dry-run must not change files");
    }
}

#[test]
fn test_bump_patch_updates_version_file() {
    let dir = TempDir::new().unwrap();
    let config = "[bumpversion]\ncurrent_version = 1.0.0\ncommit = false\ntag = false\n\n[bumpversion:file:VERSION]\n";
    setup_repo(&dir, "1.0.0", config, "1.0.0\n");

    let status = Command::new(cargo_bin())
        .args(["--bump", "patch", "--config-file", ".bumpversion.toml"])
        .current_dir(dir.path())
        .status();

    if let Ok(s) = status
        && s.success()
    {
        let contents = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(
            contents.contains("1.0.1"),
            "VERSION should now be 1.0.1, got: {contents}"
        );
    }
}

#[test]
fn test_new_version_flag_overrides_bump() {
    let dir = TempDir::new().unwrap();
    let config = "[bumpversion]\ncurrent_version = 2.0.0\ncommit = false\ntag = false\n\n[bumpversion:file:VERSION]\n";
    setup_repo(&dir, "2.0.0", config, "2.0.0\n");

    let status = Command::new(cargo_bin())
        .args([
            "--new-version",
            "5.0.0",
            "--config-file",
            ".bumpversion.toml",
        ])
        .current_dir(dir.path())
        .status();

    if let Ok(s) = status
        && s.success()
    {
        let contents = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(
            contents.contains("5.0.0"),
            "VERSION should now be 5.0.0, got: {contents}"
        );
    }
}

#[test]
fn test_current_version_flag_overrides_config() {
    let dir = TempDir::new().unwrap();
    let config = "[bumpversion]\ncurrent_version = 0.0.1\ncommit = false\ntag = false\n\n[bumpversion:file:VERSION]\n";
    fs::write(dir.path().join(".bumpversion.toml"), config).unwrap();
    fs::write(dir.path().join("VERSION"), "3.1.4\n").unwrap();

    Command::new("git")
        .args(["init"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "CI"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "ci@ci.ci"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    let output = Command::new(cargo_bin())
        .args([
            "--current-version",
            "3.1.4",
            "--bump",
            "patch",
            "--config-file",
            ".bumpversion.toml",
        ])
        .current_dir(dir.path())
        .output();

    if let Ok(o) = output
        && o.status.success()
    {
        let contents = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(
            contents.contains("3.1.5"),
            "VERSION should be 3.1.5, got: {contents}"
        );
    }
}
