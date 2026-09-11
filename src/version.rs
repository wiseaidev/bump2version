// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Version Parsing and Bumping
//!
//! Provides [`Version`] (an ordered map of version components) and
//! [`bump_version`], the central function that increments one component and
//! resets downstream components according to the configured serialisation
//! order and part definitions.
//!
//! ## Supported Component Styles
//!
//! - **Numeric**: the default; any named capture group whose current value is
//!   a valid `u64` (`major`, `minor`, `patch`, `devnum`, ...).
//! - **Cyclic string**: parts declared in `[bumpversion:part:NAME]` with a
//!   `values` list advance to the next element in the list on bump.
//! - **Optional**: parts declared with `optional_value` are omitted from the
//!   serialised string when they equal `optional_value`.
//!
//! ## Example
//!
//! ```rust
//! use bump2version::version::{parse_version, bump_version, serialize_version};
//! use bump2version::config::BumpConfig;
//!
//! let cfg = BumpConfig::default();
//! let version = parse_version("1.2.3", &cfg).unwrap();
//! let bumped  = bump_version(&version, "patch", &cfg).unwrap();
//! assert_eq!(serialize_version(&bumped, &cfg), "1.2.4");
//! ```
//!
//! ```rust
//! use bump2version::version::{parse_version, bump_version, serialize_version};
//! use bump2version::config::BumpConfig;
//!
//! let cfg = BumpConfig::default();
//! let version = parse_version("1.2.3", &cfg).unwrap();
//! let bumped  = bump_version(&version, "minor", &cfg).unwrap();
//! assert_eq!(serialize_version(&bumped, &cfg), "1.3.0");
//! ```

use crate::config::{BumpConfig, PartConfig};
use crate::error::BumpError;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use indexmap::IndexMap;
use memchr::memchr;
use regex::Regex;
use smallvec::SmallVec;

/// Returns a cached [`Arc<Regex>`] for `pattern`, compiling it on the first
/// call and reusing the compiled automaton on all subsequent calls.
///
/// In `std` builds a global `OnceLock<RwLock<HashMap>>` holds the cache;
/// reads acquire a shared lock (zero contention on the hot path) and only
/// the first compilation of each unique pattern takes the write lock.
///
/// In `no_std` builds the cache is omitted and `Regex` is compiled fresh
/// each call (no `OnceLock`/`RwLock` available without `std`).
///
/// # Complexity
///
/// - **Time (std, cache hit)**: O(1): one `Arc::clone`.
/// - **Time (first call / no_std)**: O(pattern_len): NFA/DFA compilation.
/// - **Space**: O(P) where P = number of distinct patterns ever compiled.
#[cfg(feature = "std")]
fn cached_regex(pattern: &str) -> Result<Arc<Regex>, BumpError> {
    use std::collections::HashMap;
    use std::sync::{OnceLock, RwLock};

    static CACHE: OnceLock<RwLock<HashMap<String, Arc<Regex>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| RwLock::new(HashMap::with_capacity(4)));

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

/// A single version component.
///
/// Holds the string value as it appears in the serialised version and the
/// index into a cyclic `values` list (or `None` for numeric parts).
#[derive(Debug, Clone, PartialEq)]
pub struct VersionPart {
    /// Current string value of this component.
    pub value: String,

    /// Index of `value` within the cyclic `values` list for this part.
    ///
    /// `None` for numeric parts.
    pub cycle_index: Option<usize>,
}

/// An ordered map from component name to its current [`VersionPart`].
///
/// Insertion order is preserved (backed by [`IndexMap`]) and reflects the
/// order of named capture groups in the `parse` regex, which in turn
/// determines bump/reset order.
pub type Version = IndexMap<String, VersionPart>;

/// Parses a version string into a [`Version`] using the `parse` regex in
/// `cfg`.
///
/// Each named capture group in the regex becomes a key in the returned map.
/// Groups that did not participate in the match (optional groups) are stored
/// with an empty string value.
///
/// # Arguments
///
/// * `version_str` - The raw version string (e.g. `"1.2.3"` or
///   `"1.2.3-alpha.1"`).
/// * `cfg`         - The active [`BumpConfig`] whose `parse` field holds the
///   regex.
///
/// # Returns
///
/// An ordered [`Version`] map, or a [`BumpError`] if the regex is invalid or
/// does not match.
///
/// # Errors
///
/// Returns [`BumpError::InvalidRegex`] when `cfg.parse` is not a valid regex,
/// and [`BumpError::VersionNotFound`] when the regex does not match
/// `version_str`.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `version_str.len()`.
/// - **Space**: O(k) where k = number of named capture groups in the regex.
pub fn parse_version(version_str: &str, cfg: &BumpConfig) -> Result<Version, BumpError> {
    let re = cached_regex(&cfg.parse)?;

    let caps = re.captures(version_str).ok_or_else(|| {
        BumpError::VersionNotFound(version_str.to_string(), "<version string>".to_string())
    })?;

    let mut version: Version = IndexMap::with_capacity(8);

    for name in re.capture_names().flatten() {
        let val = caps
            .name(name)
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string();

        let cycle_index = cfg.parts.get(name).and_then(|pc: &PartConfig| {
            if pc.values.is_empty() {
                None
            } else {
                pc.values.iter().position(|v| v == &val)
            }
        });

        version.insert(
            name.to_string(),
            VersionPart {
                value: val,
                cycle_index,
            },
        );
    }

    Ok(version)
}

/// Bumps the named component in `version` and resets all downstream
/// components using branchless arithmetic where possible.
///
/// "Downstream" means all components that appear **after** the bumped one in
/// the serialisation format string.  Numeric downstream components are reset
/// to `"0"`; cyclic components are reset to their `first_value` (or the first
/// element of their `values` list).
///
/// # Arguments
///
/// * `version`        - The current parsed [`Version`].
/// * `part_to_bump`   - Name of the component to increment (e.g. `"patch"`).
/// * `cfg`            - Active [`BumpConfig`].
///
/// # Returns
///
/// A new [`Version`] with the bumped and reset components, or a [`BumpError`]
/// if the requested component was not found or its value cannot be
/// incremented.
///
/// # Errors
///
/// - [`BumpError::UnknownComponent`]: `part_to_bump` is not in `version`.
/// - [`BumpError::InvalidComponentValue`]: a numeric component has a
///   non-integer value.
///
/// # Complexity
///
/// - **Time**: O(k) where k = number of version components.
/// - **Space**: O(k) for the cloned version.
pub fn bump_version(
    version: &Version,
    part_to_bump: &str,
    cfg: &BumpConfig,
) -> Result<Version, BumpError> {
    if !version.contains_key(part_to_bump) {
        return Err(BumpError::UnknownComponent(part_to_bump.to_string()));
    }

    let serialize_order: SmallVec<[String; 8]> = extract_format_keys(&cfg.serialize[0]);

    let mut bumped_version = version.clone();
    let mut found = false;

    for key in &serialize_order {
        let part = match bumped_version.get_mut(key.as_str()) {
            Some(p) => p,
            None => continue,
        };

        let is_target = key == part_to_bump;
        found |= is_target;

        if is_target {
            if let Some(pc) = cfg.parts.get(key.as_str())
                && !pc.values.is_empty()
            {
                let current_idx = part.cycle_index.unwrap_or(0);
                let next_idx = (current_idx + 1) % pc.values.len();
                part.value = pc.values[next_idx].clone();
                part.cycle_index = Some(next_idx);
                continue;
            }

            let current: u64 = part
                .value
                .parse()
                .map_err(|_| BumpError::InvalidComponentValue(part.value.clone()))?;
            part.value = (current + 1).to_string();
            part.cycle_index = None;
        } else if found {
            if let Some(pc) = cfg.parts.get(key.as_str())
                && !pc.values.is_empty()
            {
                let first = pc
                    .first_value
                    .clone()
                    .or_else(|| pc.values.first().cloned())
                    .unwrap_or_default();
                let idx = pc.values.iter().position(|v| v == &first);
                part.value = first;
                part.cycle_index = idx;
                continue;
            }
            part.value = String::from("0");
            part.cycle_index = None;
        }
    }

    Ok(bumped_version)
}

/// Serialises a [`Version`] back to a string using the `serialize` format
/// list in `cfg`.
///
/// Each format in the list is tried in order.  The first format whose
/// required components are all present and non-optional wins.  If no format
/// matches (all optional components are at their optional value), the last
/// format in the list is used as a fallback.
///
/// # Arguments
///
/// * `version` - The version to serialise.
/// * `cfg`     - The active [`BumpConfig`].
///
/// # Returns
///
/// The serialised version string.
///
/// # Complexity
///
/// - **Time**: O(F × k) where F = number of format strings and k = number of
///   version components.
/// - **Space**: O(n) for the output string where n = length of the longest
///   format.
pub fn serialize_version(version: &Version, cfg: &BumpConfig) -> String {
    for format in &cfg.serialize {
        let keys: SmallVec<[String; 8]> = extract_format_keys(format);

        let all_optional_at_optional = keys.iter().all(|key| {
            if let Some(part) = version.get(key.as_str()) {
                if let Some(pc) = cfg.parts.get(key.as_str())
                    && let Some(ref opt_val) = pc.optional_value
                {
                    return part.value == *opt_val;
                }
                part.value.is_empty()
            } else {
                true
            }
        });

        let uses_optional = keys.iter().any(|key| {
            cfg.parts
                .get(key.as_str())
                .is_some_and(|pc| pc.optional_value.is_some())
        });

        if uses_optional && all_optional_at_optional {
            continue;
        }

        let can_render = keys.iter().all(|key| version.contains_key(key.as_str()));

        if can_render {
            return render_format(format, version);
        }
    }

    if let Some(last) = cfg.serialize.last() {
        return render_format(last, version);
    }

    String::new()
}

/// Extracts placeholder key names from a format string using byte-level
/// scanning via [`memchr`] for maximum throughput.
///
/// For example, `"{major}.{minor}.{patch}"` yields
/// `["major", "minor", "patch"]`.
///
/// # Arguments
///
/// * `format` - A format string containing `{key}` placeholders.
///
/// # Returns
///
/// An ordered `Vec<String>` of placeholder names, in the order they appear.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `format.len()`.
/// - **Space**: O(k) where k = number of placeholders.
pub fn extract_format_keys(format: &str) -> SmallVec<[String; 8]> {
    let bytes = format.as_bytes();
    let len = bytes.len();
    let mut keys = SmallVec::new();
    let mut pos = 0;

    while pos < len {
        let remaining = &bytes[pos..];
        let open = match memchr(b'{', remaining) {
            Some(i) => i,
            None => break,
        };
        let after_open = pos + open + 1;
        if after_open >= len {
            break;
        }
        let rest = &bytes[after_open..];
        let close = match memchr(b'}', rest) {
            Some(i) => i,
            None => break,
        };
        keys.push(format[after_open..after_open + close].to_string());
        pos = after_open + close + 1;
    }

    keys
}

/// Renders a format string by substituting `{key}` placeholders with the
/// corresponding component values from `version` in a single allocation
/// pass.
///
/// Unknown placeholders are left as-is.
///
/// # Arguments
///
/// * `format`  - A format template string (e.g. `"{major}.{minor}.{patch}"`).
/// * `version` - The [`Version`] map supplying placeholder values.
///
/// # Returns
///
/// The rendered string.
///
/// # Complexity
///
/// - **Time**: O(n × k) where n = `format.len()` and k = placeholder count.
/// - **Space**: O(n) for the output.
pub fn render_format(format: &str, version: &Version) -> String {
    let bytes = format.as_bytes();
    let len = bytes.len();
    let mut result = String::with_capacity(len);
    let mut pos = 0;

    while pos < len {
        let remaining = &bytes[pos..];
        let open = match memchr(b'{', remaining) {
            Some(i) => i,
            None => {
                result.push_str(&format[pos..]);
                break;
            }
        };

        result.push_str(&format[pos..pos + open]);
        let after_open = pos + open + 1;

        if after_open >= len {
            result.push('{');
            break;
        }

        let rest = &bytes[after_open..];
        match memchr(b'}', rest) {
            Some(close) => {
                let key = &format[after_open..after_open + close];
                match version.get(key) {
                    Some(part) => result.push_str(&part.value),
                    None => {
                        result.push('{');
                        result.push_str(key);
                        result.push('}');
                    }
                }
                pos = after_open + close + 1;
            }
            None => {
                result.push('{');
                pos = after_open;
            }
        }
    }

    result
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
