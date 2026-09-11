// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Workspace-Aware Bumping
//!
//! Provides [`bump_workspace`] for atomically updating all member crates in a
//! Cargo workspace in a single pass.  Discovers workspace members from the
//! root `Cargo.toml`, applies the version bump to each member's `Cargo.toml`,
//! and updates the workspace root's own version if present.
//!
//! ## Example
//!
//! ```no_run
//! use bump2version::workspace::bump_workspace;
//!
//! // Atomically bump all workspace crates from 1.0.0 → 1.0.1
//! bump_workspace(".", "1.0.0", "1.0.1").unwrap();
//! ```

use crate::error::BumpError;
use crate::files::apply_config_version_update;
use std::fs;
use std::path::{Path, PathBuf};

/// Discovers all member `Cargo.toml` paths in a Cargo workspace.
///
/// Reads the root `Cargo.toml` at `root`, looks for a `[workspace]`
/// section with a `members` array, and returns the resolved path for each
/// member crate's `Cargo.toml`.
///
/// # Errors
///
/// Returns [`BumpError::Io`] when reading fails and [`BumpError::InvalidConfig`]
/// when the workspace section is missing.
///
/// # Complexity
///
/// - **Time**: O(M) where M = number of workspace members.
/// - **Space**: O(M).
pub fn discover_workspace_members(root: &Path) -> Result<Vec<PathBuf>, BumpError> {
    let cargo_toml = root.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml)?;
    let mut members: Vec<PathBuf> = Vec::new();

    let mut in_workspace = false;
    let mut in_members = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[workspace]" {
            in_workspace = true;
            in_members = false;
            continue;
        }
        if in_workspace && trimmed.starts_with('[') {
            in_workspace = false;
            in_members = false;
            continue;
        }
        if in_workspace && trimmed.starts_with("members") {
            in_members = true;
            continue;
        }
        if in_members {
            if trimmed.starts_with(']') {
                in_members = false;
                continue;
            }
            let member = trimmed.trim_matches(|c| c == '"' || c == '\'' || c == ',');
            if !member.is_empty() && !member.starts_with('#') {
                let member_path = root.join(member).join("Cargo.toml");
                if member_path.exists() {
                    members.push(member_path);
                }
            }
        }
    }

    if members.is_empty() && !content.contains("[workspace]") {
        members.push(cargo_toml);
    }

    Ok(members)
}

/// Atomically bumps the version in all Cargo workspace member crates.
///
/// Reads each member's `Cargo.toml`, replaces `current_version` with
/// `new_version`, and writes the file back.  The root workspace `Cargo.toml`
/// is also updated if it contains a `version` key.
///
/// # Arguments
///
/// * `root`            - Path to the workspace root directory.
/// * `current_version` - The version string currently in use.
/// * `new_version`     - The version string to write.
///
/// # Errors
///
/// Returns [`BumpError::Io`] on any file I/O failure.
///
/// # Complexity
///
/// - **Time**: O(M × S) where M = members and S = average `Cargo.toml` size.
/// - **Space**: O(S) for the largest member file.
pub fn bump_workspace(
    root: &str,
    current_version: &str,
    new_version: &str,
) -> Result<Vec<String>, BumpError> {
    let root_path = Path::new(root);
    let members = discover_workspace_members(root_path)?;
    let mut updated_paths: Vec<String> = Vec::with_capacity(members.len());

    for member_path in &members {
        let content = fs::read_to_string(member_path)?;
        let updated = apply_config_version_update(&content, current_version, new_version);
        if updated != content {
            fs::write(member_path, &updated)?;
            updated_paths.push(member_path.to_string_lossy().into_owned());
        }
    }

    Ok(updated_paths)
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
