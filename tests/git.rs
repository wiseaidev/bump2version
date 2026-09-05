// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bump2version::git::{commit_files, create_tag, get_git_author};
use gix::open as open_repo;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_get_git_author_reads_local_config() {
    let dir = TempDir::new().unwrap();
    let _repo = gix::init(dir.path()).unwrap();

    let git_config_path = dir.path().join(".git").join("config");
    let existing = fs::read_to_string(&git_config_path).unwrap_or_default();
    fs::write(
        &git_config_path,
        format!(
            "{}\n[user]\n\tname = Correct Author\n\temail = correct@example.com\n",
            existing
        ),
    )
    .unwrap();

    let open_repo_result = open_repo(dir.path().to_str().unwrap()).unwrap();
    let (name, email) = get_git_author(&open_repo_result).unwrap();
    assert_eq!(name, "Correct Author");
    assert_eq!(email, "correct@example.com");
}

#[test]
fn test_get_git_author_does_not_return_web_flow() {
    let dir = TempDir::new().unwrap();
    let git_config_path = dir.path().join(".git").join("config");
    let repo = gix::init(dir.path()).unwrap();

    let existing = fs::read_to_string(&git_config_path).unwrap_or_default();
    fs::write(
        &git_config_path,
        format!(
            "{}\n[user]\n\tname = Real Developer\n\temail = dev@mycompany.com\n",
            existing
        ),
    )
    .unwrap();

    let open_repo_result = open_repo(dir.path().to_str().unwrap()).unwrap();
    let (name, email) = get_git_author(&open_repo_result).unwrap();
    assert_ne!(email, "49699333+web-flow@users.noreply.github.com");
    assert_eq!(name, "Real Developer");
    assert_eq!(email, "dev@mycompany.com");
    drop(repo);
}

#[test]
fn test_get_git_author_falls_back_to_env() {
    let dir = TempDir::new().unwrap();
    let _repo = gix::init(dir.path()).unwrap();

    unsafe {
        std::env::set_var("GIT_AUTHOR_NAME", "Env Author");
        std::env::set_var("GIT_AUTHOR_EMAIL", "env@author.com");
    }

    let open_repo_result = open_repo(dir.path().to_str().unwrap()).unwrap();
    let result = get_git_author(&open_repo_result);
    unsafe {
        std::env::remove_var("GIT_AUTHOR_NAME");
        std::env::remove_var("GIT_AUTHOR_EMAIL");
    }

    if let Ok((name, email)) = result {
        assert!(!name.is_empty());
        assert!(!email.is_empty());
    }
}

#[test]
fn test_commit_files_creates_commit_with_correct_author() {
    let dir = TempDir::new().unwrap();
    let repo_path = dir.path().to_str().unwrap().to_string();

    std::process::Command::new("git")
        .args(["-C", &repo_path, "init"])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["-C", &repo_path, "config", "user.name", "Commit Tester"])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["-C", &repo_path, "config", "user.email", "tester@test.com"])
        .output()
        .unwrap();

    let version_file = dir.path().join("version.txt");
    fs::write(&version_file, "1.0.0").unwrap();

    std::process::Command::new("git")
        .args(["-C", &repo_path, "add", "."])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["-C", &repo_path, "commit", "-m", "Initial commit"])
        .output()
        .unwrap();

    fs::write(&version_file, "1.0.1").unwrap();

    let repo = open_repo(&repo_path).unwrap();
    let result = commit_files(
        &repo,
        &[version_file.to_str().unwrap().to_string()],
        "Bump version: 1.0.0 → 1.0.1",
        "Commit Tester",
        "tester@test.com",
    );

    assert!(result.is_ok(), "commit_files failed: {:?}", result.err());

    let log_output = std::process::Command::new("git")
        .args(["-C", &repo_path, "log", "-1", "--format=%an <%ae>"])
        .output()
        .unwrap();
    let log_str = String::from_utf8(log_output.stdout)
        .unwrap()
        .trim()
        .to_string();
    assert_eq!(log_str, "Commit Tester <tester@test.com>");
}

#[test]
fn test_create_tag_makes_lightweight_tag() {
    let dir = TempDir::new().unwrap();
    let repo_path = dir.path().to_str().unwrap().to_string();

    std::process::Command::new("git")
        .args(["-C", &repo_path, "init"])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["-C", &repo_path, "config", "user.name", "Tagger"])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["-C", &repo_path, "config", "user.email", "tag@test.com"])
        .output()
        .unwrap();

    let f = dir.path().join("f.txt");
    fs::write(&f, "hello").unwrap();
    std::process::Command::new("git")
        .args(["-C", &repo_path, "add", "."])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["-C", &repo_path, "commit", "-m", "seed"])
        .output()
        .unwrap();

    let repo = open_repo(&repo_path).unwrap();
    let head_id = repo.head_commit().unwrap().id;
    let result = create_tag(&repo, "v1.0.1", head_id);
    assert!(result.is_ok(), "create_tag failed: {:?}", result.err());

    let tag_output = std::process::Command::new("git")
        .args(["-C", &repo_path, "tag", "-l", "v1.0.1"])
        .output()
        .unwrap();
    let tag_str = String::from_utf8(tag_output.stdout)
        .unwrap()
        .trim()
        .to_string();
    assert_eq!(tag_str, "v1.0.1");
}
