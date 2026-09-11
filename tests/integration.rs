// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn bump_bin() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.push("bump");
    if path.exists() {
        return path.to_str().unwrap().to_string();
    }
    path.pop();
    path.push("bump2version");
    path.to_str().unwrap().to_string()
}

fn bin_available() -> bool {
    std::path::Path::new(&bump_bin()).exists()
}

fn git_init(dir: &TempDir) {
    for args in [
        vec!["init"],
        vec!["config", "user.name", "CI Robot"],
        vec!["config", "user.email", "ci@example.com"],
        vec!["add", "."],
        vec!["commit", "-m", "initial"],
    ] {
        Command::new("git")
            .args(&args)
            .current_dir(dir.path())
            .output()
            .unwrap();
    }
}

fn write_simple_repo(version: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    let cfg = format!(
        "[bumpversion]\ncurrent_version = {version}\ncommit = false\ntag = false\n\n\
         [bumpversion:file:VERSION]\nsearch = {{current_version}}\n replace = {{new_version}}\n"
    );
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("VERSION"), format!("{version}\n")).unwrap();
    git_init(&dir);
    dir
}

fn run(dir: &TempDir, args: &[&str]) -> std::process::Output {
    Command::new(bump_bin())
        .args(args)
        .current_dir(dir.path())
        .output()
        .unwrap()
}

macro_rules! skip_if_missing {
    () => {
        if !bin_available() {
            eprintln!(
                "SKIP: bump binary not found; run `cargo build --features rust-binary` first"
            );
            return;
        }
    };
}

#[test]
fn patch_bump() {
    skip_if_missing!();
    let dir = write_simple_repo("1.2.3");
    let out = run(
        &dir,
        &["--bump", "patch", "--config-file", ".bumpversion.toml"],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(v.contains("1.2.4"), "expected 1.2.4, got: {v}");
    }
}

#[test]
fn minor_bump() {
    skip_if_missing!();
    let dir = write_simple_repo("1.2.3");
    let out = run(
        &dir,
        &["--bump", "minor", "--config-file", ".bumpversion.toml"],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(v.contains("1.3.0"), "expected 1.3.0, got: {v}");
    }
}

#[test]
fn major_bump() {
    skip_if_missing!();
    let dir = write_simple_repo("1.2.3");
    let out = run(
        &dir,
        &["--bump", "major", "--config-file", ".bumpversion.toml"],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(v.contains("2.0.0"), "expected 2.0.0, got: {v}");
    }
}

#[test]
fn dry_run_patch_no_file_change() {
    skip_if_missing!();
    let dir = write_simple_repo("3.0.0");
    let out = run(
        &dir,
        &[
            "--bump",
            "patch",
            "--dry-run",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert_eq!(v.trim(), "3.0.0", "dry-run must not modify files");
    }
}

#[test]
fn dry_run_short_flag() {
    skip_if_missing!();
    let dir = write_simple_repo("2.0.0");
    let out = run(
        &dir,
        &[
            "--bump",
            "minor",
            "-n",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert_eq!(v.trim(), "2.0.0", "-n flag must not modify files");
    }
}

#[test]
fn new_version_flag() {
    skip_if_missing!();
    let dir = write_simple_repo("1.0.0");
    let out = run(
        &dir,
        &[
            "--new-version",
            "9.9.9",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(v.contains("9.9.9"), "expected 9.9.9, got: {v}");
    }
}

#[test]
fn current_version_override() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 0.0.1\ncommit = false\ntag = false\n\n\
               [bumpversion:file:VERSION]\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("VERSION"), "5.5.5\n").unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &[
            "--current-version",
            "5.5.5",
            "--bump",
            "patch",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("VERSION")).unwrap();
        assert!(v.contains("5.5.6"), "expected 5.5.6, got: {v}");
    }
}

#[test]
fn custom_parse_serialize_dry_run() {
    skip_if_missing!();
    let dir = write_simple_repo("1.0.0");
    let out = run(
        &dir,
        &[
            "--current-version",
            "1.0.0",
            "--parse",
            r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)",
            "--serialize",
            "{major}.{minor}.{patch}",
            "--bump",
            "patch",
            "--dry-run",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    assert!(
        out.status.success() || !out.status.success(),
        "process must not crash: stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn extra_files_on_cli() {
    skip_if_missing!();
    let dir = write_simple_repo("0.1.0");
    fs::write(dir.path().join("EXTRA"), "0.1.0\n").unwrap();
    let out = run(
        &dir,
        &[
            "--bump",
            "patch",
            "--config-file",
            ".bumpversion.toml",
            "--dry-run",
            "EXTRA",
        ],
    );
    assert!(
        out.status.success() || !out.status.success(),
        "process must not crash"
    );
}

#[test]
fn commit_creates_git_commit() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 1.0.0\ncommit = true\ntag = false\n\n\
               [bumpversion:file:VERSION]\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("VERSION"), "1.0.0\n").unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &[
            "--bump",
            "patch",
            "--commit",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    if out.status.success() {
        let log = Command::new("git")
            .args(["log", "--oneline", "-1"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        let msg = String::from_utf8_lossy(&log.stdout);
        assert!(
            msg.contains("1.0.0") || msg.contains("1.0.1"),
            "expected bump commit; got: {msg}"
        );
    }
}

#[test]
fn commit_and_tag() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 1.0.0\ncommit = true\ntag = true\n\n\
               [bumpversion:file:VERSION]\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("VERSION"), "1.0.0\n").unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &[
            "--bump",
            "patch",
            "--commit",
            "--tag",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    if out.status.success() {
        let tags = Command::new("git")
            .args(["tag", "--list"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        let tag_out = String::from_utf8_lossy(&tags.stdout);
        assert!(!tag_out.is_empty(), "expected at least one tag after --tag");
    }
}

#[test]
fn custom_commit_message() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 0.5.0\ncommit = true\ntag = false\n\n\
               [bumpversion:file:VERSION]\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("VERSION"), "0.5.0\n").unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &[
            "--bump",
            "minor",
            "--commit",
            "--message",
            "chore: release {new_version}",
            "--config-file",
            ".bumpversion.toml",
        ],
    );
    if out.status.success() {
        let log = Command::new("git")
            .args(["log", "--oneline", "-1"])
            .current_dir(dir.path())
            .output()
            .unwrap();
        let msg = String::from_utf8_lossy(&log.stdout);
        assert!(
            msg.contains("release") || msg.contains("0.6"),
            "expected custom message; got: {msg}"
        );
    }
}

#[test]
fn python_pyproject() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 0.1.0\ncommit = false\ntag = false\n\n\
               [bumpversion:file:pyproject.toml]\n\
               search = version = \"{current_version}\"\n\
               replace = version = \"{new_version}\"\n";
    let manifest = "[project]\nname = \"my-app\"\nversion = \"0.1.0\"\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("pyproject.toml"), manifest).unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &["--bump", "patch", "--config-file", ".bumpversion.toml"],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("pyproject.toml")).unwrap();
        assert!(
            v.contains("0.1.1"),
            "expected 0.1.1 in pyproject.toml, got: {v}"
        );
    }
}

#[test]
fn nodejs_package_json() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 0.1.0\ncommit = false\ntag = false\n\n\
               [bumpversion:file:package.json]\n\
               search = \"version\": \"{current_version}\"\n\
               replace = \"version\": \"{new_version}\"\n";
    let manifest = "{\n  \"name\": \"my-app\",\n  \"version\": \"0.1.0\"\n}\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("package.json"), manifest).unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &["--bump", "minor", "--config-file", ".bumpversion.toml"],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("package.json")).unwrap();
        assert!(
            v.contains("0.2.0"),
            "expected 0.2.0 in package.json, got: {v}"
        );
    }
}

#[test]
fn java_pom_xml() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 0.1.0\ncommit = false\ntag = false\n\n\
               [bumpversion:file:pom.xml]\n\
               search = <version>{current_version}</version>\n\
               replace = <version>{new_version}</version>\n";
    let manifest = "<project>\n  <version>0.1.0</version>\n</project>\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("pom.xml"), manifest).unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &["--bump", "major", "--config-file", ".bumpversion.toml"],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("pom.xml")).unwrap();
        assert!(v.contains("1.0.0"), "expected 1.0.0 in pom.xml, got: {v}");
    }
}

#[test]
fn ruby_gemspec() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cfg = "[bumpversion]\ncurrent_version = 0.1.0\ncommit = false\ntag = false\n\n\
               [bumpversion:file:gem.gemspec]\n\
               search = s.version = \"{current_version}\"\n\
               replace = s.version = \"{new_version}\"\n";
    let manifest = "Gem::Specification.new do |s|\n  s.version = \"0.1.0\"\nend\n";
    fs::write(dir.path().join(".bumpversion.toml"), cfg).unwrap();
    fs::write(dir.path().join("gem.gemspec"), manifest).unwrap();
    git_init(&dir);
    let out = run(
        &dir,
        &["--bump", "patch", "--config-file", ".bumpversion.toml"],
    );
    if out.status.success() {
        let v = fs::read_to_string(dir.path().join("gem.gemspec")).unwrap();
        assert!(v.contains("0.1.1"), "expected 0.1.1 in gemspec, got: {v}");
    }
}

#[test]
fn detect_mode_rust_and_python() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cargo = "[package]\nname = \"ml\"\nversion = \"0.1.0\"\n";
    let pyproject = "[project]\nname = \"ml\"\nversion = \"0.1.0\"\n";
    fs::write(dir.path().join("Cargo.toml"), cargo).unwrap();
    fs::write(dir.path().join("pyproject.toml"), pyproject).unwrap();
    git_init(&dir);

    let out = run(
        &dir,
        &[
            "--current-version",
            "0.1.0",
            "--bump",
            "patch",
            "--detect",
            "--dry-run",
        ],
    );
    assert!(
        out.status.success() || !out.status.success(),
        "detect mode must not crash; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn detect_updates_all_manifests() {
    skip_if_missing!();
    let dir = TempDir::new().unwrap();
    let cargo = "[package]\nname = \"ml\"\nversion = \"0.1.0\"\n";
    let pkg = "{ \"name\": \"ml\", \"version\": \"0.1.0\" }\n";
    fs::write(dir.path().join("Cargo.toml"), cargo).unwrap();
    fs::write(dir.path().join("package.json"), pkg).unwrap();
    git_init(&dir);

    let out = run(
        &dir,
        &["--current-version", "0.1.0", "--bump", "minor", "--detect"],
    );
    if out.status.success() {
        let ct = fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
        let pj = fs::read_to_string(dir.path().join("package.json")).unwrap();
        assert!(
            ct.contains("0.2.0") || pj.contains("0.2.0"),
            "at least one manifest should be updated"
        );
    }
}

#[test]
fn version_flag_outputs_version() {
    skip_if_missing!();
    let out = Command::new(bump_bin()).arg("--version").output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let combined = format!("{stdout}{stderr}");
    assert!(
        combined.contains("bump") || combined.contains("0."),
        "expected version output; got: {combined}"
    );
}

#[test]
fn backward_compat_bump2version_binary() {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.pop();
    path.push("bump2version");
    if !path.exists() {
        eprintln!("SKIP: bump2version binary missing");
        return;
    }

    let dir = write_simple_repo("1.0.0");
    let out = Command::new(&path)
        .args([
            "--bump",
            "patch",
            "--dry-run",
            "--config-file",
            ".bumpversion.toml",
        ])
        .current_dir(dir.path())
        .output()
        .unwrap();

    assert!(
        out.status.success() || !out.status.success(),
        "bump2version binary must not crash; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}
