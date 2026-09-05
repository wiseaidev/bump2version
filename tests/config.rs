// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bump2version::config::{
    BumpConfig, DEFAULT_PARSE, DEFAULT_SERIALIZE, parse_config, parse_config_file,
};

#[test]
fn test_parse_minimal_config() {
    let toml = "[bumpversion]\ncurrent_version = 1.2.3\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.current_version.as_deref(), Some("1.2.3"));
    assert!(cfg.commit);
    assert!(!cfg.tag);
}

#[test]
fn test_parse_commit_and_tag_flags() {
    let toml = "[bumpversion]\ncommit = false\ntag = true\ncurrent_version = 0.1.0\n";
    let cfg = parse_config(toml).unwrap();
    assert!(!cfg.commit);
    assert!(cfg.tag);
}

#[test]
fn test_parse_custom_parse_and_serialize() {
    let toml = "[bumpversion]\ncurrent_version = 2.0\nparse = (?P<major>\\d+)\\.(?P<minor>\\d+)\nserialize = {major}.{minor}\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.parse, r"(?P<major>\d+)\.(?P<minor>\d+)");
    assert_eq!(cfg.serialize, vec!["{major}.{minor}"]);
}

#[test]
fn test_parse_multiline_serialize() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\nserialize =\n\t{major}.{minor}.{patch}-{stage}.{devnum}\n\t{major}.{minor}.{patch}\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.serialize.len(), 2);
    assert_eq!(cfg.serialize[0], "{major}.{minor}.{patch}-{stage}.{devnum}");
    assert_eq!(cfg.serialize[1], "{major}.{minor}.{patch}");
}

#[test]
fn test_parse_file_section() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\n\n[bumpversion:file:Cargo.toml]\nsearch = version = \"{current_version}\"\nreplace = version = \"{new_version}\"\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.files.len(), 1);
    assert_eq!(cfg.files[0].path, "Cargo.toml");
    assert_eq!(
        cfg.files[0].search.as_deref(),
        Some("version = \"{current_version}\"")
    );
    assert_eq!(
        cfg.files[0].replace.as_deref(),
        Some("version = \"{new_version}\"")
    );
}

#[test]
fn test_parse_multiline_search_in_file_section() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\n\n[bumpversion:file:CHANGELOG.md]\nsearch = ## {current_version}\n    some context line\n    another line\nreplace = ## {new_version}\n";
    let cfg = parse_config(toml).unwrap();
    let fc = &cfg.files[0];
    let search = fc.search.as_deref().unwrap();
    assert!(search.contains('\n'), "search should be multiline");
    assert!(search.contains("some context line"));
    assert!(search.contains("another line"));
}

#[test]
fn test_parse_part_section() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\n\n[bumpversion:part:stage]\noptional_value = stable\nfirst_value = stable\nvalues =\n\talpha\n\tbeta\n\tstable\n";
    let cfg = parse_config(toml).unwrap();
    let stage = cfg.parts.get("stage").unwrap();
    assert_eq!(stage.optional_value.as_deref(), Some("stable"));
    assert_eq!(stage.first_value.as_deref(), Some("stable"));
    assert_eq!(stage.values, vec!["alpha", "beta", "stable"]);
}

#[test]
fn test_parse_multiple_file_sections() {
    let toml = "[bumpversion]\ncurrent_version = 0.0.1\n\n[bumpversion:file:Cargo.toml]\nsearch = \"{current_version}\"\nreplace = \"{new_version}\"\n\n[bumpversion:file:README.md]\nsearch = v{current_version}\nreplace = v{new_version}\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.files.len(), 2);
    assert_eq!(cfg.files[0].path, "Cargo.toml");
    assert_eq!(cfg.files[1].path, "README.md");
}

#[test]
fn test_parse_allow_dirty_flag() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\nallow_dirty = true\n";
    let cfg = parse_config(toml).unwrap();
    assert!(cfg.allow_dirty);
}

#[test]
fn test_parse_default_values_when_missing() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.parse, DEFAULT_PARSE);
    assert_eq!(cfg.serialize, vec![DEFAULT_SERIALIZE]);
    assert_eq!(cfg.search, "{current_version}");
    assert_eq!(cfg.replace, "{new_version}");
}

#[test]
fn test_config_not_found_returns_error() {
    let result = parse_config_file("/nonexistent/path/.bumpversion.toml");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("not found") || msg.contains("nonexistent"));
}

#[test]
fn test_comments_and_blank_lines_are_ignored() {
    let toml = "# this is a comment\n\n[bumpversion]\n# another comment\ncurrent_version = 3.0.0\n\n# blank lines above\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.current_version.as_deref(), Some("3.0.0"));
}

#[test]
fn test_ignore_missing_version_flag_in_file() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\n\n[bumpversion:file:optional.txt]\nignore_missing_version = true\n";
    let cfg = parse_config(toml).unwrap();
    assert!(cfg.files[0].ignore_missing_version);
}

#[test]
fn test_bump_config_default_is_sane() {
    let cfg = BumpConfig::default();
    assert!(cfg.current_version.is_none());
    assert!(!cfg.serialize.is_empty());
    assert!(!cfg.parse.is_empty());
}

#[test]
fn test_parse_strip_quotes() {
    let toml = "[bumpversion]\ncurrent_version = \"1.0.0\"\n\n[bumpversion:file:Cargo.toml]\nsearch = 'version = \"{current_version}\"'\nreplace = '''version = \"{new_version}\"'''\n";
    let cfg = parse_config(toml).unwrap();
    assert_eq!(cfg.current_version.as_deref(), Some("1.0.0"));
    assert_eq!(
        cfg.files[0].search.as_deref(),
        Some("version = \"{current_version}\"")
    );
    assert_eq!(
        cfg.files[0].replace.as_deref(),
        Some("version = \"{new_version}\"")
    );
}
