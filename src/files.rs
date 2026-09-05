// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # File Search and Replace
//!
//! Applies version-string search-and-replace operations to individual files.
//! Handles both single-line and multi-line search/replace patterns, rendering
//! `{current_version}` and `{new_version}` placeholders before matching.
//!
//! ## Multiline Support
//!
//! Multi-line patterns require both `MULTILINE` (`(?m)`) and `DOTALL`
//! (`(?s)`) semantics.  In Rust's `regex` crate, `(?s)` makes `.` match
//! newlines and `(?m)` makes `^`/`$` match line boundaries.  This module
//! applies `(?ms)` automatically when the rendered search string contains a
//! newline character.
//!
//! ## Example
//!
//! ```rust
//! use bump2version::config::{BumpConfig, FileConfig};
//! use bump2version::files::apply_file_change;
//!
//! let content = "## 1.0.0\nsome text\n";
//! let mut fc = FileConfig::new("CHANGELOG.md");
//! fc.search = Some("## {current_version}".to_string());
//! fc.replace = Some("## {new_version}".to_string());
//! let cfg = BumpConfig::default();
//! let result = apply_file_change(content, &fc, &cfg, "1.0.0", "1.0.1").unwrap();
//! assert_eq!(result, "## 1.0.1\nsome text\n");
//! ```

use crate::config::{BumpConfig, FileConfig};
use crate::error::BumpError;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use regex::Regex;

/// Returns a cached [`Arc<Regex>`] for `pattern`.
///
/// Identical in design to `version::cached_regex`; see that function's
/// documentation for the full complexity analysis.
#[cfg(feature = "std")]
fn cached_regex(pattern: &str) -> Result<Arc<Regex>, BumpError> {
    use std::collections::HashMap;
    use std::sync::{OnceLock, RwLock};

    static CACHE: OnceLock<RwLock<HashMap<String, Arc<Regex>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| RwLock::new(HashMap::new()));

    {
        let guard = cache.read().unwrap_or_else(|p| p.into_inner());
        if let Some(re) = guard.get(pattern) {
            return Ok(Arc::clone(re));
        }
    }

    let re = Regex::new(pattern).map_err(|e| BumpError::InvalidRegex(pattern.to_string(), e))?;
    let re = Arc::new(re);
    cache
        .write()
        .unwrap_or_else(|p| p.into_inner())
        .insert(pattern.to_string(), Arc::clone(&re));
    Ok(re)
}

#[cfg(not(feature = "std"))]
fn cached_regex(pattern: &str) -> Result<Arc<Regex>, BumpError> {
    Regex::new(pattern)
        .map_err(|e| BumpError::InvalidRegex(pattern.to_string(), e))
        .map(Arc::new)
}

/// Renders a pattern template by substituting `{current_version}` and
/// `{new_version}` placeholders.
///
/// # Arguments
///
/// * `template`        - The raw template from the config (search or replace
///   field).
/// * `current_version` - The current version string.
/// * `new_version`     - The new version string.
///
/// # Returns
///
/// The rendered string with both placeholders substituted.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `template.len()`.
/// - **Space**: O(n) for the output string.
fn render_pattern(template: &str, current_version: &str, new_version: &str) -> String {
    template
        .replace("{current_version}", current_version)
        .replace("{new_version}", new_version)
}

/// Escapes a string for use as a literal regex pattern.
///
/// Wraps [`regex::escape`] so callers do not need to import the `regex` crate
/// directly.
///
/// # Arguments
///
/// * `text` - The raw string to escape.
///
/// # Returns
///
/// A `String` safe for use as a `regex::Regex` pattern that matches `text`
/// literally.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `text.len()`.
/// - **Space**: O(n).
fn escape_literal(text: &str) -> String {
    regex::escape(text)
}

/// Applies a single search-and-replace operation to `content`.
///
/// The search pattern is taken from `file_cfg.search` (or `cfg.search` if
/// absent), with `{current_version}` rendered into the actual version string.
/// The replacement is taken from `file_cfg.replace` (or `cfg.replace`), with
/// `{new_version}` rendered.
///
/// When the rendered search string contains a newline the pattern is compiled
/// with `(?ms)` flags (MULTILINE + DOTALL) to enable multi-line matching.
/// Otherwise it is escaped and compiled as a literal pattern.
///
/// # Arguments
///
/// * `content`         - The full text content of the file.
/// * `file_cfg`        - Per-file configuration (search/replace overrides).
/// * `cfg`             - Global [`BumpConfig`] supplying defaults.
/// * `current_version` - The version string being replaced.
/// * `new_version`     - The version string to insert.
///
/// # Returns
///
/// The updated file content, or a [`BumpError`] if the search pattern is not
/// found or the regex is invalid.
///
/// # Errors
///
/// - [`BumpError::VersionNotFound`]: the rendered search pattern did not
///   match anywhere in `content`.
/// - [`BumpError::InvalidRegex`]  : the pattern could not be compiled.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `content.len()`.
/// - **Space**: O(n) for the returned string.
pub fn apply_file_change(
    content: &str,
    file_cfg: &FileConfig,
    cfg: &BumpConfig,
    current_version: &str,
    new_version: &str,
) -> Result<String, BumpError> {
    let search_template = file_cfg.search.as_deref().unwrap_or(&cfg.search);

    let replace_template = file_cfg.replace.as_deref().unwrap_or(&cfg.replace);

    let rendered_search = render_pattern(search_template, current_version, new_version);
    let rendered_replace = render_pattern(replace_template, current_version, new_version);

    let is_multiline = rendered_search.contains('\n');

    let pattern = if is_multiline {
        let escaped = escape_literal(&rendered_search);
        format!("(?ms){}", escaped)
    } else {
        escape_literal(&rendered_search)
    };

    let re = cached_regex(&pattern)?;

    if !re.is_match(content) {
        if !file_cfg.ignore_missing_version {
            return Err(BumpError::VersionNotFound(
                rendered_search,
                file_cfg.path.clone(),
            ));
        }
        return Ok(content.to_string());
    }

    Ok(re
        .replace_all(content, regex::NoExpand(&rendered_replace))
        .into_owned())
}

/// Applies version-string search-and-replace to the `current_version` key in
/// the global config section of a `.bumpversion.toml` file.
///
/// Replaces the first occurrence of `current_version = <old>` with
/// `current_version = <new>`.
///
/// # Arguments
///
/// * `content`         - The raw text of the config file.
/// * `current_version` - The version string currently stored.
/// * `new_version`     - The version string to write.
///
/// # Returns
///
/// The updated config file content.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `content.len()`.
/// - **Space**: O(n).
pub fn apply_config_version_update(
    content: &str,
    current_version: &str,
    new_version: &str,
) -> String {
    let candidates = [
        (
            format!("current_version = \"{}\"", current_version),
            format!("current_version = \"{}\"", new_version),
        ),
        (
            format!("current_version = '{}'", current_version),
            format!("current_version = '{}'", new_version),
        ),
        (
            format!("current_version = {}", current_version),
            format!("current_version = {}", new_version),
        ),
    ];
    for (old, new) in &candidates {
        if content.contains(old.as_str()) {
            return content.replacen(old.as_str(), new.as_str(), 1);
        }
    }
    content.to_string()
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
