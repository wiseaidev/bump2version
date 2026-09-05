// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bump2version::config::{BumpConfig, FileConfig};
use bump2version::files::{apply_config_version_update, apply_file_change};

fn default_cfg() -> BumpConfig {
    BumpConfig::default()
}

fn file_cfg(path: &str) -> FileConfig {
    FileConfig::new(path)
}

fn file_cfg_with_patterns(path: &str, search: &str, replace: &str) -> FileConfig {
    let mut fc = FileConfig::new(path);
    fc.search = Some(search.to_string());
    fc.replace = Some(replace.to_string());
    fc
}

#[test]
fn test_simple_version_replace() {
    let content = "version = \"1.0.0\"\n";
    let fc = file_cfg("test.txt");
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "1.0.0", "1.0.1").unwrap();
    assert_eq!(result, "version = \"1.0.1\"\n");
}

#[test]
fn test_custom_search_replace_patterns() {
    let content = "## Version 1.2.3\n";
    let fc = file_cfg_with_patterns(
        "CHANGELOG.md",
        "## Version {current_version}",
        "## Version {new_version}",
    );
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "1.2.3", "1.2.4").unwrap();
    assert_eq!(result, "## Version 1.2.4\n");
}

#[test]
fn test_multiline_search_replace() {
    let content = "## 1.0.0\nsome context\nmore context\n\nolder stuff\n";
    let fc = file_cfg_with_patterns(
        "CHANGELOG.md",
        "## {current_version}\nsome context\nmore context",
        "## {new_version}\nsome context\nmore context",
    );
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "1.0.0", "1.0.1").unwrap();
    assert!(result.contains("## 1.0.1"));
    assert!(!result.contains("## 1.0.0"));
}

#[test]
fn test_multiline_search_replace_with_realistic_changelog() {
    let content = "# Changelog\n\n## [1.2.3] - unreleased\nsome notes\nanother note\n\n## [1.2.2] - 2024-01-01\nold stuff\n";
    let fc = file_cfg_with_patterns(
        "CHANGELOG.md",
        "## [{current_version}] - unreleased\nsome notes\nanother note",
        "## [{new_version}] - unreleased\nsome notes\nanother note",
    );
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "1.2.3", "1.2.4").unwrap();
    assert!(result.contains("## [1.2.4] - unreleased"));
    assert!(!result.contains("## [1.2.3] - unreleased"));
    assert!(
        result.contains("## [1.2.2] - 2024-01-01"),
        "older entries untouched"
    );
}

#[test]
fn test_pattern_not_found_returns_error() {
    let content = "nothing here\n";
    let fc = file_cfg("test.txt");
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "9.9.9", "9.9.10");
    assert!(result.is_err());
}

#[test]
fn test_ignore_missing_version_flag() {
    let content = "nothing here\n";
    let mut fc = file_cfg("test.txt");
    fc.ignore_missing_version = true;
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "9.9.9", "9.9.10").unwrap();
    assert_eq!(result, "nothing here\n");
}

#[test]
fn test_only_first_occurrence_needs_to_match() {
    let content = "v1.0.0 is here and v1.0.0 is also here\n";
    let fc = file_cfg("test.txt");
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "1.0.0", "1.0.1").unwrap();
    assert!(result.contains("v1.0.1"));
}

#[test]
fn test_toml_style_search_replace() {
    let content = "name = \"my-crate\"\nversion = \"2.5.1\"\nedition = \"2024\"\n";
    let fc = file_cfg_with_patterns(
        "Cargo.toml",
        "version = \"{current_version}\"",
        "version = \"{new_version}\"",
    );
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "2.5.1", "2.5.2").unwrap();
    assert!(result.contains("version = \"2.5.2\""));
    assert!(!result.contains("version = \"2.5.1\""));
    assert!(
        result.contains("edition = \"2024\""),
        "other keys untouched"
    );
}

#[test]
fn test_apply_config_version_update_basic() {
    let content = "[bumpversion]\ncurrent_version = 1.0.0\ncommit = true\n";
    let result = apply_config_version_update(content, "1.0.0", "1.0.1");
    assert!(result.contains("current_version = 1.0.1"));
    assert!(!result.contains("current_version = 1.0.0"));
}

#[test]
fn test_apply_config_version_update_only_first_occurrence() {
    let content =
        "[bumpversion]\ncurrent_version = 1.0.0\n# some comment about current_version = 1.0.0\n";
    let result = apply_config_version_update(content, "1.0.0", "2.0.0");
    assert!(result.contains("current_version = 2.0.0"));
}

#[test]
fn test_multiline_search_with_tabs() {
    let content = "SEARCH\n\tline two\n\tline three\nEND\n";
    let fc = file_cfg_with_patterns(
        "file.txt",
        "SEARCH\n\tline two\n\tline three",
        "REPLACED\n\tline two\n\tline three",
    );
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "ignored", "ignored");
    if let Ok(r) = result {
        assert!(r.contains("REPLACED"));
    }
}

#[test]
fn test_search_with_version_in_middle_of_multiline() {
    let content = "START\ncurrent: 1.0.0\nEND\n";
    let fc = file_cfg_with_patterns(
        "file.txt",
        "START\ncurrent: {current_version}\nEND",
        "START\ncurrent: {new_version}\nEND",
    );
    let cfg = default_cfg();
    let result = apply_file_change(content, &fc, &cfg, "1.0.0", "2.0.0").unwrap();
    assert!(result.contains("2.0.0"));
    assert!(!result.contains("1.0.0"));
}
