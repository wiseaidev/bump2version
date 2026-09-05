// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Configuration Parsing
//!
//! Parses `.bumpversion.toml` configuration strings as used by `bumpversion`,
//! `bump2version`, and `bump-my-version`. Fully `no_std + alloc`: the
//! filesystem helper [`parse_config_file`] requires the `std` feature.
//!
//! ## Multiline Values
//!
//! Multi-line values are written with indented continuation lines:
//!
//! ```text
//! [bumpversion:file:CHANGELOG.md]
//! search = ## {current_version}
//!     release notes
//!     more notes
//! replace = ## {new_version}
//! ```

use crate::error::BumpError;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Default parse regex used when none is provided in the config.
pub const DEFAULT_PARSE: &str =
    r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(-(?P<stage>[^.]*?)\.(?P<devnum>\d+))?";

/// Default serialise format used when none is provided.
pub const DEFAULT_SERIALIZE: &str = "{major}.{minor}.{patch}";

/// Default commit message template.
pub const DEFAULT_MESSAGE: &str = "Bump version: {current_version} \u{2192} {new_version}";

/// Per-file search/replace configuration parsed from a
/// `[bumpversion:file:PATH]` section.
#[derive(Debug, Clone, PartialEq)]
pub struct FileConfig {
    /// Path of the file to update, relative to the repository root.
    pub path: String,

    /// Optional override of the search pattern for this file.
    ///
    /// Supports multi-line values written with indented continuation lines.
    pub search: Option<String>,

    /// Optional override of the replacement pattern for this file.
    ///
    /// Supports multi-line values written with indented continuation lines.
    pub replace: Option<String>,

    /// Optional override of the parse regex for this file.
    pub parse: Option<String>,

    /// Optional override of the serialisation format list for this file.
    pub serialize: Option<Vec<String>>,

    /// If `true`, the tool will not raise an error when the version string is
    /// not found in the file.
    pub ignore_missing_version: bool,

    /// If `true`, the tool will not raise an error when the file does not
    /// exist.
    pub ignore_missing_file: bool,
}

impl FileConfig {
    /// Creates a [`FileConfig`] with all optional fields set to `None`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file relative to the project root.
    ///
    /// # Returns
    ///
    /// A new [`FileConfig`] for `path` with all overrides unset.
    ///
    /// # Complexity
    ///
    /// - **Time**: O(1).
    /// - **Space**: O(n) where n = `path.len()`.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            search: None,
            replace: None,
            parse: None,
            serialize: None,
            ignore_missing_version: false,
            ignore_missing_file: false,
        }
    }
}

/// Per-version-part configuration, parsed from a `[bumpversion:part:NAME]`
/// section.
#[derive(Debug, Clone, PartialEq)]
pub struct PartConfig {
    /// Name of the version component (e.g. `"major"`, `"stage"`).
    pub name: String,

    /// First value the part takes after being reset.
    pub first_value: Option<String>,

    /// The "invisible" value that is omitted from the serialised version.
    pub optional_value: Option<String>,

    /// Ordered list of string values that the part cycles through.
    pub values: Vec<String>,
}

/// Top-level configuration read from `.bumpversion.toml`.
#[derive(Debug, Clone, PartialEq)]
pub struct BumpConfig {
    /// The current version string as stored in the config file.
    pub current_version: Option<String>,

    /// Whether to create a git commit after bumping.
    pub commit: bool,

    /// Whether to create a git tag after bumping.
    pub tag: bool,

    /// Regex (with named groups) used to parse the version string into
    /// components. Defaults to [`DEFAULT_PARSE`].
    pub parse: String,

    /// Ordered list of serialisation format strings.
    pub serialize: Vec<String>,

    /// Default search pattern (may contain `{current_version}`).
    pub search: String,

    /// Default replacement pattern (may contain `{new_version}`).
    pub replace: String,

    /// Commit message template.
    pub message: String,

    /// Template for the git tag name.
    pub tag_name: String,

    /// If `true` the working directory does not need to be clean.
    pub allow_dirty: bool,

    /// Per-file configuration blocks.
    pub files: Vec<FileConfig>,

    /// Per-part configuration blocks, keyed by part name.
    pub parts: BTreeMap<String, PartConfig>,
}

impl Default for BumpConfig {
    fn default() -> Self {
        Self {
            current_version: None,
            commit: true,
            tag: false,
            parse: DEFAULT_PARSE.to_string(),
            serialize: alloc::vec![DEFAULT_SERIALIZE.to_string()],
            search: "{current_version}".to_string(),
            replace: "{new_version}".to_string(),
            message: DEFAULT_MESSAGE.to_string(),
            tag_name: "v{new_version}".to_string(),
            allow_dirty: false,
            files: Vec::new(),
            parts: BTreeMap::new(),
        }
    }
}

/// Strips one level of leading indentation from a continuation line.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `line.len()`.
/// - **Space**: O(1).
fn strip_continuation(line: &str) -> Option<&str> {
    if line.starts_with('\t') || line.starts_with("    ") || line.starts_with("  ") {
        Some(line.trim())
    } else {
        None
    }
}

/// Collects a multi-line value starting from its first line.
///
/// # Complexity
///
/// - **Time**: O(k) where k = number of continuation lines.
/// - **Space**: O(k).
fn collect_multiline(first: &str, rest: &[&str]) -> (String, usize) {
    let first_trimmed = first.trim();
    let mut lines: Vec<String> = if first_trimmed.is_empty() {
        Vec::new()
    } else {
        alloc::vec![first_trimmed.to_string()]
    };
    let mut consumed = 0usize;

    let is_multiline_double = first_trimmed.starts_with("\"\"\"")
        && (first_trimmed.len() < 6 || !first_trimmed[3..].ends_with("\"\"\""));
    let is_multiline_single = first_trimmed.starts_with("'''")
        && (first_trimmed.len() < 6 || !first_trimmed[3..].ends_with("'''"));

    if is_multiline_double || is_multiline_single {
        for line in rest {
            lines.push(line.to_string());
            consumed += 1;
            let trimmed_line = line.trim_end();
            if (is_multiline_double && trimmed_line.ends_with("\"\"\""))
                || (is_multiline_single && trimmed_line.ends_with("'''"))
            {
                break;
            }
        }
    } else {
        for line in rest {
            match strip_continuation(line) {
                Some(cont) => {
                    lines.push(cont.to_string());
                    consumed += 1;
                }
                None => break,
            }
        }
    }

    let joined = lines.join("\n");
    (strip_quotes(&joined), consumed)
}

/// Strips standard TOML quotes from a string value.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `val.len()`.
/// - **Space**: O(n).
fn strip_quotes(val: &str) -> String {
    let mut s = val;
    if (s.starts_with("\"\"\"") && s.ends_with("\"\"\"") && s.len() >= 6)
        || (s.starts_with("'''") && s.ends_with("'''") && s.len() >= 6)
    {
        s = &s[3..s.len() - 3];
        if s.starts_with('\n') {
            s = &s[1..];
        } else if s.starts_with("\r\n") {
            s = &s[2..];
        }
    } else if (s.starts_with('"') && s.ends_with('"') && s.len() >= 2)
        || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2)
    {
        s = &s[1..s.len() - 1];
    }
    s.to_string()
}

/// Parses a `.bumpversion.toml` configuration string into a [`BumpConfig`].
///
/// Handles global `[bumpversion]`, per-file `[bumpversion:file:PATH]`, and
/// per-part `[bumpversion:part:NAME]` sections with multiline value support.
///
/// ## No-std compatibility
///
/// Only requires `alloc`. For filesystem-based parsing, see
/// [`parse_config_file`] (requires `std` feature).
///
/// # Arguments
///
/// * `content` - The raw text content of the `.bumpversion.toml` file.
///
/// # Returns
///
/// A [`BumpConfig`], or a [`BumpError`] if malformed.
///
/// # Complexity
///
/// - **Time**: O(L) where L = total number of lines.
/// - **Space**: O(L).
pub fn parse_config(content: &str) -> Result<BumpConfig, BumpError> {
    let mut cfg = BumpConfig::default();

    #[derive(Debug, Clone, PartialEq)]
    enum Section {
        Global,
        File(String),
        Part(String),
        Unknown,
    }

    let mut current_section = Section::Unknown;
    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0usize;

    while idx < lines.len() {
        let line = lines[idx];
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            idx += 1;
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let section = &trimmed[1..trimmed.len() - 1];
            current_section = if section == "bumpversion" {
                Section::Global
            } else if let Some(rest) = section.strip_prefix("bumpversion:file:") {
                let file_path = rest.trim().to_string();
                cfg.files.push(FileConfig::new(&file_path));
                Section::File(file_path)
            } else if let Some(rest) = section.strip_prefix("bumpversion:part:") {
                let part_name = rest.trim().to_string();
                cfg.parts.insert(
                    part_name.clone(),
                    PartConfig {
                        name: part_name.clone(),
                        first_value: None,
                        optional_value: None,
                        values: Vec::new(),
                    },
                );
                Section::Part(part_name)
            } else {
                Section::Unknown
            };
            idx += 1;
            continue;
        }

        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let after_eq = &trimmed[eq_pos + 1..];
            let (value, extra) = collect_multiline(after_eq, &lines[idx + 1..]);
            idx += extra;

            match &current_section {
                Section::Global => match key.as_str() {
                    "current_version" => cfg.current_version = Some(value.trim().to_string()),
                    "commit" => {
                        cfg.commit =
                            value.trim().eq_ignore_ascii_case("true") || value.trim() == "1"
                    }
                    "tag" => {
                        cfg.tag = value.trim().eq_ignore_ascii_case("true") || value.trim() == "1"
                    }
                    "parse" => cfg.parse = value.trim().to_string(),
                    "serialize" => {
                        cfg.serialize = value
                            .lines()
                            .map(str::trim)
                            .filter(|l| !l.is_empty())
                            .map(str::to_string)
                            .collect();
                    }
                    "search" => cfg.search = value,
                    "replace" => cfg.replace = value,
                    "message" => cfg.message = value.trim().to_string(),
                    "tag_name" => cfg.tag_name = value.trim().to_string(),
                    "allow_dirty" => {
                        cfg.allow_dirty =
                            value.trim().eq_ignore_ascii_case("true") || value.trim() == "1"
                    }
                    _ => {}
                },
                Section::File(path) => {
                    if let Some(fc) = cfg.files.iter_mut().find(|f| f.path == *path) {
                        match key.as_str() {
                            "search" => fc.search = Some(value),
                            "replace" => fc.replace = Some(value),
                            "parse" => fc.parse = Some(value.trim().to_string()),
                            "serialize" => {
                                fc.serialize = Some(
                                    value
                                        .lines()
                                        .map(str::trim)
                                        .filter(|l| !l.is_empty())
                                        .map(str::to_string)
                                        .collect(),
                                )
                            }
                            "ignore_missing_version" => {
                                fc.ignore_missing_version =
                                    value.trim().eq_ignore_ascii_case("true") || value.trim() == "1"
                            }
                            "ignore_missing_file" => {
                                fc.ignore_missing_file =
                                    value.trim().eq_ignore_ascii_case("true") || value.trim() == "1"
                            }
                            _ => {}
                        }
                    }
                }
                Section::Part(part) => {
                    if let Some(pc) = cfg.parts.get_mut(part) {
                        match key.as_str() {
                            "first_value" => pc.first_value = Some(value.trim().to_string()),
                            "optional_value" => pc.optional_value = Some(value.trim().to_string()),
                            "values" => {
                                pc.values = value
                                    .lines()
                                    .map(str::trim)
                                    .filter(|l| !l.is_empty())
                                    .map(str::to_string)
                                    .collect();
                            }
                            _ => {}
                        }
                    }
                }
                Section::Unknown => {}
            }
        }

        idx += 1;
    }

    if cfg.serialize.is_empty() {
        cfg.serialize = alloc::vec![DEFAULT_SERIALIZE.to_string()];
    }

    Ok(cfg)
}

/// Reads and parses a `.bumpversion.toml` file from disk.
///
/// Requires the `std` feature.
///
/// # Arguments
///
/// * `path` - Filesystem path to the configuration file.
///
/// # Errors
///
/// Returns [`BumpError::ConfigNotFound`] when the file does not exist.
///
/// # Complexity
///
/// - **Time**: O(L) where L = total lines.
/// - **Space**: O(L).
#[cfg(feature = "std")]
pub fn parse_config_file(path: &str) -> Result<BumpConfig, BumpError> {
    let content =
        std::fs::read_to_string(path).map_err(|_| BumpError::ConfigNotFound(path.to_string()))?;
    parse_config(&content)
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
