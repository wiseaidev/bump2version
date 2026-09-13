// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Utility Helpers
//!
//! Miscellaneous helpers that do not belong to a single module. Currently
//! re-exports the most-used public APIs from [`crate::config`],
//! [`crate::version`], and [`crate::files`] so that callers can import from
//! a single location.

use crate::config::{BumpConfig, FileConfig, parse_config_file};
use crate::error::BumpError;
use crate::version::{BumpPart, bump_version, parse_version, serialize_version};
use std::collections::HashSet;

/// Reads file paths declared in `[bumpversion:file:PATH]` sections of the
/// config file at `config_path`.
///
/// # Arguments
///
/// * `config_path` - Filesystem path to the `.bumpversion.toml` file.
///
/// # Returns
///
/// A [`HashSet<String>`] containing every declared file path, or a
/// [`BumpError`] if the config cannot be read.
///
/// # Errors
///
/// Returns [`BumpError::ConfigNotFound`] when the file does not exist and
/// [`BumpError::InvalidConfig`] for parse failures.
///
/// # Complexity
///
/// - **Time**: O(L) where L = total number of lines in the config file.
/// - **Space**: O(F) where F = number of `[bumpversion:file:...]` sections.
pub fn read_files_from_config(config_path: &str) -> Result<HashSet<String>, BumpError> {
    let cfg = parse_config_file(config_path)?;
    Ok(cfg.files.into_iter().map(|f| f.path).collect())
}

/// Computes the new version string by bumping the part named in `args.bump`.
///
/// Reads the current version from the config file, parses it with the
/// configured regex, bumps the requested component, then serialises the result
/// back to a string using the configured `serialize` format list.
///
/// # Arguments
///
/// * `config_path` - Filesystem path to the `.bumpversion.toml` file.
/// * `bump`        - The [`BumpPart`] identifying which component to increment.
/// * `current_version_override` - Optional version string that overrides the
///   value stored in the config file.
/// * `parse_re`    - Optional regex override from the CLI; when `None` the
///   value from the config file is used.
/// * `serialize_fmt` - Optional serialise format override from the CLI.
///
/// # Returns
///
/// The new version string, or `None` when the bump cannot be performed.
///
/// # Complexity
///
/// - **Time**: O(L + V) where L = config file line count and V = version
///   string length.
/// - **Space**: O(L) for the config parse.
pub fn compute_new_version(
    config_path: &str,
    bump: &BumpPart,
    current_version_override: Option<&str>,
    parse_re: Option<&str>,
    serialize_fmt: Option<&str>,
) -> Option<String> {
    let mut cfg: BumpConfig = parse_config_file(config_path).ok()?;

    if let Some(re) = parse_re {
        cfg.parse = re.to_string();
    }
    if let Some(fmt) = serialize_fmt {
        cfg.serialize = vec![fmt.to_string()];
    }

    let current = current_version_override.or(cfg.current_version.as_deref())?;
    let version = parse_version(current, &cfg).ok()?;
    let bumped = bump_version(&version, bump, &cfg).ok()?;
    Some(serialize_version(&bumped, &cfg))
}

/// Returns a [`BumpConfig`] built from the config file at `path`, with CLI
/// overrides applied.
///
/// # Arguments
///
/// * `path`      - Path to the `.bumpversion.toml` file.
/// * `parse_re`  - Optional `--parse` CLI override.
/// * `serialize` - Optional `--serialize` CLI override.
///
/// # Returns
///
/// A fully populated [`BumpConfig`], or a [`BumpError`] on failure.
///
/// # Errors
///
/// Propagates any error from [`parse_config_file`].
///
/// # Complexity
///
/// - **Time**: O(L) where L = config line count.
/// - **Space**: O(L).
pub fn load_config(
    path: &str,
    parse_re: Option<&str>,
    serialize: Option<&str>,
) -> Result<BumpConfig, BumpError> {
    let mut cfg = parse_config_file(path)?;
    if let Some(re) = parse_re {
        cfg.parse = re.to_string();
    }
    if let Some(fmt) = serialize {
        cfg.serialize = vec![fmt.to_string()];
    }
    Ok(cfg)
}

/// Builds the list of [`FileConfig`] entries to update, merging files from
/// the config with extra files supplied on the command line.
///
/// Command-line files use the global `search`/`replace` patterns from `cfg`.
///
/// # Arguments
///
/// * `cfg`        - The active [`BumpConfig`].
/// * `extra_files` - Additional file paths passed directly on the CLI.
///
/// # Returns
///
/// A `Vec<FileConfig>` ordered with config-file entries first, followed by
/// extra CLI files.
///
/// # Complexity
///
/// - **Time**: O(F + E) where F = number of config files and E = number of
///   extra CLI files.
/// - **Space**: O(F + E).
pub fn collect_file_configs(cfg: &BumpConfig, extra_files: &[String]) -> Vec<FileConfig> {
    let mut all = cfg.files.clone();
    for path in extra_files {
        if !all.iter().any(|f| &f.path == path) {
            all.push(FileConfig::new(path));
        }
    }
    all
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
