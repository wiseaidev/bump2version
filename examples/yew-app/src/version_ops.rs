// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bump2version::config::BumpConfig;
use bump2version::version::{BumpPart, bump_version, parse_version, serialize_version};

/// The outcome of a single bump operation.
#[derive(Debug, Clone, PartialEq)]
pub struct BumpResult {
    /// The version string produced after bumping.
    pub new_version: String,
}

pub fn execute_bump(
    version_str: &str,
    part: &str,
    parse_regex: &str,
    serialize_fmt: &str,
) -> Result<BumpResult, String> {
    let mut cfg = BumpConfig::default();

    if !parse_regex.trim().is_empty() {
        cfg.parse = parse_regex.to_string();
    }
    if !serialize_fmt.trim().is_empty() {
        cfg.serialize = vec![serialize_fmt.to_string()];
    }

    let parsed = parse_version(version_str, &cfg).map_err(|e| format!("Parse error: {e}"))?;
    let bump_part = BumpPart::from(part);
    let bumped = bump_version(&parsed, &bump_part, &cfg).map_err(|e| format!("Bump error: {e}"))?;
    let new_version = serialize_version(&bumped, &cfg);

    Ok(BumpResult { new_version })
}
