// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bump2version::config::{BumpConfig, parse_config};
use bump2version::version::{
    bump_version, extract_format_keys, parse_version, render_format, serialize_version,
};

fn default_cfg() -> BumpConfig {
    BumpConfig::default()
}

#[test]
fn test_parse_version_simple() {
    let cfg = default_cfg();
    let v = parse_version("1.2.3", &cfg).unwrap();
    assert_eq!(v["major"].value, "1");
    assert_eq!(v["minor"].value, "2");
    assert_eq!(v["patch"].value, "3");
}

#[test]
fn test_parse_version_large_numbers() {
    let cfg = default_cfg();
    let v = parse_version("100.200.300", &cfg).unwrap();
    assert_eq!(v["major"].value, "100");
    assert_eq!(v["minor"].value, "200");
    assert_eq!(v["patch"].value, "300");
}

#[test]
fn test_parse_version_zero() {
    let cfg = default_cfg();
    let v = parse_version("0.0.0", &cfg).unwrap();
    assert_eq!(v["major"].value, "0");
    assert_eq!(v["minor"].value, "0");
    assert_eq!(v["patch"].value, "0");
}

#[test]
fn test_bump_patch() {
    let cfg = default_cfg();
    let v = parse_version("1.2.3", &cfg).unwrap();
    let bumped = bump_version(&v, "patch", &cfg).unwrap();
    assert_eq!(bumped["patch"].value, "4");
    assert_eq!(bumped["minor"].value, "2");
    assert_eq!(bumped["major"].value, "1");
}

#[test]
fn test_bump_minor_resets_patch() {
    let cfg = default_cfg();
    let v = parse_version("1.2.9", &cfg).unwrap();
    let bumped = bump_version(&v, "minor", &cfg).unwrap();
    assert_eq!(bumped["minor"].value, "3");
    assert_eq!(bumped["patch"].value, "0");
    assert_eq!(bumped["major"].value, "1");
}

#[test]
fn test_bump_major_resets_minor_and_patch() {
    let cfg = default_cfg();
    let v = parse_version("1.99.8", &cfg).unwrap();
    let bumped = bump_version(&v, "major", &cfg).unwrap();
    assert_eq!(bumped["major"].value, "2");
    assert_eq!(bumped["minor"].value, "0");
    assert_eq!(bumped["patch"].value, "0");
}

#[test]
fn test_bump_patch_from_zero() {
    let cfg = default_cfg();
    let v = parse_version("0.0.0", &cfg).unwrap();
    let bumped = bump_version(&v, "patch", &cfg).unwrap();
    assert_eq!(serialize_version(&bumped, &cfg), "0.0.1");
}

#[test]
fn test_serialize_version_uses_format() {
    let cfg = default_cfg();
    let v = parse_version("3.14.159", &cfg).unwrap();
    let s = serialize_version(&v, &cfg);
    assert_eq!(s, "3.14.159");
}

#[test]
fn test_extract_format_keys_standard() {
    let keys = extract_format_keys("{major}.{minor}.{patch}");
    assert_eq!(keys, vec!["major", "minor", "patch"]);
}

#[test]
fn test_extract_format_keys_with_stage() {
    let keys = extract_format_keys("{major}.{minor}.{patch}-{stage}.{devnum}");
    assert_eq!(keys, vec!["major", "minor", "patch", "stage", "devnum"]);
}

#[test]
fn test_bump_unknown_component_returns_error() {
    let cfg = default_cfg();
    let v = parse_version("1.2.3", &cfg).unwrap();
    let result = bump_version(&v, "nonexistent", &cfg);
    assert!(result.is_err());
}

#[test]
fn test_cyclic_stage_bump() {
    let toml = "[bumpversion]\ncurrent_version = 1.0.0\nparse = (?P<major>\\d+)\\.(?P<minor>\\d+)\\.(?P<patch>\\d+)(-(?P<stage>[a-z]+))?\nserialize =\n\t{major}.{minor}.{patch}-{stage}\n\t{major}.{minor}.{patch}\n\n[bumpversion:part:stage]\noptional_value = stable\nfirst_value = alpha\nvalues =\n\talpha\n\tbeta\n\tstable\n";
    let cfg = parse_config(toml).unwrap();
    let v = parse_version("1.0.0", &cfg).unwrap();
    let bumped = bump_version(&v, "stage", &cfg).unwrap();
    let stage_val = &bumped.get("stage").map(|p| p.value.as_str()).unwrap_or("");
    assert!(
        !stage_val.is_empty(),
        "stage should have been bumped to a cycle value"
    );
}

#[test]
fn test_render_format_substitutes_placeholders() {
    let cfg = default_cfg();
    let v = parse_version("2.4.6", &cfg).unwrap();
    let rendered = render_format("{major}.{minor}.{patch}", &v);
    assert_eq!(rendered, "2.4.6");
}

#[test]
fn test_full_bump_and_serialize_roundtrip() {
    let cfg = default_cfg();
    for (input, part, expected) in &[
        ("1.0.0", "patch", "1.0.1"),
        ("1.0.9", "patch", "1.0.10"),
        ("1.9.9", "minor", "1.10.0"),
        ("9.9.9", "major", "10.0.0"),
    ] {
        let v = parse_version(input, &cfg).unwrap();
        let bumped = bump_version(&v, part, &cfg).unwrap();
        let result = serialize_version(&bumped, &cfg);
        assert_eq!(&result, expected, "Failed: {input} --{part}--> {expected}");
    }
}
