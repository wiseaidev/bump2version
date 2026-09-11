// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Multi-Language Version Auto-Detection
//!
//! Provides [`detect_and_bump`] for automatically discovering version strings
//! across any programming language's ecosystem and bumping them in a single
//! pass.
//!
//! ## Supported Languages
//!
//! | Language   | Manifest Files |
//! |------------|----------------|
//! | Rust       | `Cargo.toml` |
//! | Python     | `pyproject.toml`, `setup.cfg`, `setup.py` |
//! | JavaScript | `package.json` |
//! | Go         | `go.mod` |
//! | Java       | `pom.xml`, `build.gradle` |
//! | Ruby       | `Gemfile` |
//!
//! ## Example
//!
//! ```no_run
//! # #[cfg(feature = "detect")]
//! use bump2version::detect::detect_and_bump;
//! # #[cfg(feature = "detect")]
//! let updated = detect_and_bump(".", "1.0.0", "1.0.1", false).unwrap();
//! # #[cfg(feature = "detect")]
//! println!("Updated {} files", updated.len());
//! ```

use crate::error::BumpError;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Known manifest file names that may contain version strings.
static MANIFESTS: &[&str] = &[
    "Cargo.toml",
    "pyproject.toml",
    "setup.cfg",
    "package.json",
    "go.mod",
    "pom.xml",
    "build.gradle",
    "build.gradle.kts",
    "Gemfile",
];

/// Directories to skip while walking the project tree.
static SKIP_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    ".venv",
    "venv",
    "__pycache__",
    "dist",
    "build",
    ".tox",
];

/// Discovers all manifest files under `root_dir` that likely contain
/// version strings.
///
/// # Complexity
///
/// - **Time**: O(F) where F = total file count in the directory tree.
/// - **Space**: O(M) where M = number of matching manifests found.
pub fn discover_manifests(root_dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();

    for entry in WalkDir::new(root_dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|n| !SKIP_DIRS.contains(&n))
                .unwrap_or(true)
        })
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if MANIFESTS.contains(&file_name.as_ref()) {
            found.push(entry.path().to_owned());
        }
    }

    found
}

/// Returns `true` if `content` contains a recognisable version declaration
/// matching `version`.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `content.len()`.
/// - **Space**: O(1).
fn content_has_version(content: &str, version: &str) -> bool {
    content.contains(version)
}

/// Replaces all occurrences of `current_version` with `new_version` in
/// `content` without using regex, suitable for simple manifest substitution.
///
/// # Complexity
///
/// - **Time**: O(n) where n = `content.len()`.
/// - **Space**: O(n).
fn replace_version_in_content(content: &str, current: &str, new: &str) -> String {
    content.replace(current, new)
}

/// Detects manifest files in `root_dir` and replaces `current_version` with
/// `new_version` in every file that contains the version string.
///
/// When `dry_run` is `true`, files are not written but the list of files that
/// would have been updated is returned.
///
/// # Arguments
///
/// * `root_dir`        - Root directory of the project to scan.
/// * `current_version` - The version string to find.
/// * `new_version`     - The version string to substitute.
/// * `dry_run`         - When `true`, skip writing files.
///
/// # Returns
///
/// A list of paths for files that were (or would have been) updated.
///
/// # Errors
///
/// Returns [`BumpError::Io`] on file read/write failure.
///
/// # Complexity
///
/// - **Time**: O(M × S) where M = manifest count and S = average file size.
/// - **Space**: O(S) for the largest manifest file.
pub fn detect_and_bump(
    root_dir: &str,
    current_version: &str,
    new_version: &str,
    dry_run: bool,
) -> Result<Vec<String>, BumpError> {
    let manifests = discover_manifests(Path::new(root_dir));
    let mut updated: Vec<String> = Vec::new();

    for path in manifests {
        let content = fs::read_to_string(&path)?;
        if !content_has_version(&content, current_version) {
            continue;
        }
        let new_content = replace_version_in_content(&content, current_version, new_version);
        if new_content != content {
            if !dry_run {
                fs::write(&path, &new_content)?;
            }
            updated.push(path.to_string_lossy().into_owned());
        }
    }

    Ok(updated)
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
