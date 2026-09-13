// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Node.js Bindings
//!
//! Exposes `bump2version` to Node.js via [napi-rs](https://napi.rs).
//! All functions are **synchronous**: no Promise boilerplate required.
//!
//! ## Installation
//!
//! ```sh
//! npm install bump2version
//! ```
//!
//! Or build locally:
//!
//! ```sh
//! npm install -g @napi-rs/cli
//! napi build --platform --release --features node
//! ```
//!
//! ## Usage
//!
//! ```javascript
//! const { bumpVersion, applyFileChange } = require('bump2version');
//!
//! // Compute a new version
//! const next = bumpVersion('1.2.3', 'patch');
//! console.log(next); // '1.2.4'
//!
//! // Update a file's content in-memory
//! const updated = applyFileChange('version = 1.0.0\n', '1.0.0', '1.0.1');
//! console.log(updated); // 'version = 1.0.1\n'
//! ```
//!
//! ## See Also
//!
//! - [`crate::version::bump_version`]
//! - [napi-rs documentation](https://napi.rs/)

use crate::config::{self, FileConfig as RustFileConfig};
use crate::files::apply_file_change;
use crate::version::{BumpPart, bump_version, parse_version, serialize_version};
use napi_derive::napi;

/// Bumps a version string by the specified component.
///
/// ``currentVersion``: The current version string (e.g. ``"1.2.3"``).
/// ``part``:           The component to increment (``"major"``, ``"minor"``,
///                     or ``"patch"``).
/// ``configPath``:     Optional path to a ``.bumpversion.toml`` file.
///
/// Returns the new version string, or throws an ``Error`` on failure.
///
/// ```javascript
/// const { bumpVersion } = require('.');
/// console.log(bumpVersion('1.2.3', 'patch'));  // '1.2.4'
/// console.log(bumpVersion('1.2.3', 'minor'));  // '1.3.0'
/// console.log(bumpVersion('1.2.3', 'major'));  // '2.0.0'
/// ```
#[napi(js_name = "bumpVersion")]
pub fn bump_version_node(
    current_version: String,
    part: String,
    config_path: Option<String>,
) -> napi::Result<String> {
    let cfg = if let Some(path) = config_path {
        config::parse_config_file(&path).map_err(|e| napi::Error::from_reason(e.to_string()))?
    } else {
        config::BumpConfig::default()
    };

    let version = parse_version(&current_version, &cfg)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    let bump_part = BumpPart::from(part.as_str());
    let bumped = bump_version(&version, &bump_part, &cfg)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(serialize_version(&bumped, &cfg))
}

/// Applies search-and-replace to a file's text content and returns the
/// updated string.
///
/// ``content``:        The full text content of the file.
/// ``currentVersion``: The version string to search for.
/// ``newVersion``:     The version string to insert.
/// ``search``:         Optional search pattern template (default:
///                     ``"{current_version}"``).
/// ``replace``:        Optional replace pattern template (default:
///                     ``"{new_version}"``).
/// ``ignoreMissing``:  When ``true``, return ``content`` unchanged instead of
///                     throwing if the pattern is not found.
///
/// Returns the updated content string, or throws an ``Error`` when the
/// pattern is not found (and ``ignoreMissing`` is ``false``).
///
/// ```javascript
/// const { applyFileChange } = require('.');
/// const updated = applyFileChange('version = 1.0.0\n', '1.0.0', '1.0.1');
/// console.log(updated); // 'version = 1.0.1\n'
/// ```
#[napi(js_name = "applyFileChange")]
pub fn apply_file_change_node(
    content: String,
    current_version: String,
    new_version: String,
    search: Option<String>,
    replace: Option<String>,
    ignore_missing: Option<bool>,
) -> napi::Result<String> {
    let cfg = config::BumpConfig::default();
    let mut fc = RustFileConfig::new("<node-caller>");
    fc.ignore_missing_version = ignore_missing.unwrap_or(false);
    fc.search = search;
    fc.replace = replace;

    apply_file_change(&content, &fc, &cfg, &current_version, &new_version)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
