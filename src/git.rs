// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Git Integration
//!
//! Provides fully thread-safe, pure-[`gix`] operations for creating
//! version-bump commits and lightweight tags without spawning any subprocess.
//!
//! ## Thread Safety
//!
//! All functions accept a `&gix::Repository` reference.  `gix::Repository`
//! performs no global mutable state mutations; callers may share a single
//! repository handle across threads.
//!
//! ## Tree Construction
//!
//! [`commit_files`] updates only the root-level tree entries that correspond
//! to the modified files.  Each modified file's content is written as a new
//! blob; the root tree is rebuilt by replacing those entries while keeping all
//! other existing entries intact.  The new tree and commit objects are written
//! with [`gix::Repository::write_object`] and
//! [`gix::Repository::commit_as`] respectively.

use crate::error::BumpError;
use gix::Repository;
use gix::bstr::ByteSlice;
use gix::objs::Tree;
use gix::objs::tree::EntryKind;
use gix::refs::transaction::PreviousValue;
use std::time::{SystemTime, UNIX_EPOCH};

/// Reads the git author identity from the repository's local configuration.
///
/// Priority order:
/// 1. `user.name` / `user.email` from the repository's local `.git/config`.
/// 2. `GIT_AUTHOR_NAME` / `GIT_AUTHOR_EMAIL` environment variables.
/// 3. `GIT_COMMITTER_NAME` / `GIT_COMMITTER_EMAIL` environment variables.
///
/// # Arguments
///
/// * `repo` - An open [`gix::Repository`].
///
/// # Returns
///
/// A `(name, email)` tuple.
///
/// # Errors
///
/// Returns [`BumpError::GitError`] when neither git config nor environment
/// variables supply both values.
///
/// # Complexity
///
/// - **Time**: O(1): reads a small config file.
/// - **Space**: O(1).
pub fn get_git_author(repo: &Repository) -> Result<(String, String), BumpError> {
    let config = repo.config_snapshot();

    let name = config
        .string("user.name")
        .map(|s| s.to_string())
        .or_else(|| {
            std::env::var("GIT_AUTHOR_NAME")
                .or_else(|_| std::env::var("GIT_COMMITTER_NAME"))
                .ok()
        })
        .ok_or_else(|| {
            BumpError::GitError(
                "read author".into(),
                "user.name is not set in git config".into(),
            )
        })?;

    let email = config
        .string("user.email")
        .map(|s| s.to_string())
        .or_else(|| {
            std::env::var("GIT_AUTHOR_EMAIL")
                .or_else(|_| std::env::var("GIT_COMMITTER_EMAIL"))
                .ok()
        })
        .ok_or_else(|| {
            BumpError::GitError(
                "read author".into(),
                "user.email is not set in git config".into(),
            )
        })?;

    Ok((name, email))
}

/// Verifies that the git working directory has no uncommitted changes.
///
/// Uses the `gix` index to compare the on-disk work-tree against the staged
/// state.  If any tracked files differ, this function returns an error.
///
/// # Arguments
///
/// * `repo` - An open [`gix::Repository`].
///
/// # Returns
///
/// `Ok(())` when the working tree is clean.
///
/// # Errors
///
/// Returns [`BumpError::GitError`] when uncommitted changes are detected or
/// when the index cannot be opened.
///
/// # Complexity
///
/// - **Time**: O(F) where F = number of tracked files.
/// - **Space**: O(1).
pub fn assert_clean_working_tree(repo: &Repository) -> Result<(), BumpError> {
    let index = repo
        .open_index()
        .map_err(|e| BumpError::GitError("open_index".into(), e.to_string()))?;

    let workdir = repo
        .workdir()
        .ok_or_else(|| BumpError::GitError("workdir".into(), "Bare repository".into()))?;

    let mut has_changes = false;
    for entry in index.entries() {
        let rel_path = entry.path(&index);
        let abs_path = workdir.join(rel_path.to_str_lossy().as_ref());
        if let Ok(on_disk) = std::fs::read(&abs_path) {
            let blob_id = repo
                .write_blob(&on_disk)
                .map_err(|e| BumpError::GitError("write_blob(check)".into(), e.to_string()))?;
            if blob_id.detach() != entry.id {
                has_changes = true;
                break;
            }
        }
    }

    if has_changes {
        return Err(BumpError::GitError(
            "dirty check".into(),
            "Git working directory is not clean. Commit or stash your changes first.".into(),
        ));
    }

    Ok(())
}

/// Returns the current UTC Unix timestamp as a git-format time string.
///
/// The format is `"<unix_secs> +0000"`, which is what
/// [`gix::actor::SignatureRef`]'s `time` field expects.
///
/// # Returns
///
/// A `String` like `"1234567890 +0000"`.
///
/// # Complexity
///
/// - **Time**: O(1).
/// - **Space**: O(1).
fn now_utc_time_str() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{} +0000", secs)
}

/// Writes new blob objects for the given files, rebuilds the root tree, and
/// creates a signed commit: all using pure [`gix`] object-store APIs with
/// no subprocess calls.
///
/// Only root-level tree entries are updated; files must be relative to the
/// repository root. The author and committer are set to `author_name /
/// author_email` rather than copying from the HEAD commit.
///
/// # Arguments
///
/// * `repo`         - An open [`gix::Repository`].
/// * `files`        - Paths of files to stage (relative to repository root).
/// * `message`      - The commit message.
/// * `author_name`  - Author display name.
/// * `author_email` - Author email address.
///
/// # Returns
///
/// The [`gix::ObjectId`] of the newly created commit.
///
/// # Errors
///
/// Returns [`BumpError::GitError`] for any object-store failure and
/// [`BumpError::Io`] for file-system errors.
///
/// # Complexity
///
/// - **Time**: O(N × S) where N = number of files and S = average file size.
/// - **Space**: O(N) for the updated entry list.
pub fn commit_files(
    repo: &Repository,
    files: &[String],
    message: &str,
    author_name: &str,
    author_email: &str,
) -> Result<gix::ObjectId, BumpError> {
    let head_commit = repo
        .head_commit()
        .map_err(|e| BumpError::GitError("head_commit".into(), e.to_string()))?;
    let parent_id = head_commit.id;

    let current_tree = head_commit
        .tree()
        .map_err(|e| BumpError::GitError("head_tree".into(), e.to_string()))?;

    let mut tree_entries: Vec<gix::objs::tree::Entry> = current_tree
        .iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| BumpError::GitError("tree_iter".into(), e.to_string()))?
        .into_iter()
        .map(|e| gix::objs::tree::Entry {
            mode: e.mode(),
            filename: e.filename().to_owned(),
            oid: e.oid().into(),
        })
        .collect();

    let mut file_blobs: Vec<(gix::bstr::BString, gix::ObjectId)> = Vec::new();

    for path_str in files {
        let content = std::fs::read(path_str)?;
        let blob_id = repo
            .write_blob(&content)
            .map_err(|e| BumpError::GitError("write_blob".into(), e.to_string()))?;

        let file_name = std::path::Path::new(path_str)
            .file_name()
            .ok_or_else(|| {
                BumpError::GitError("filename".into(), format!("Invalid path: {path_str}"))
            })?
            .as_encoded_bytes()
            .as_bstr()
            .to_owned();

        if let Some(existing) = tree_entries.iter_mut().find(|e| e.filename == file_name) {
            existing.oid = blob_id.detach();
        } else {
            tree_entries.push(gix::objs::tree::Entry {
                mode: EntryKind::Blob.into(),
                filename: file_name.clone(),
                oid: blob_id.detach(),
            });
        }
        file_blobs.push((file_name, blob_id.detach()));
    }

    tree_entries.sort_by(|a, b| a.filename.cmp(&b.filename));

    let new_tree_id = repo
        .write_object(&Tree {
            entries: tree_entries,
        })
        .map_err(|e| BumpError::GitError("write_tree".into(), e.to_string()))?;

    let time_str = now_utc_time_str();
    let sig = gix::actor::SignatureRef {
        name: author_name.as_bytes().as_bstr(),
        email: author_email.as_bytes().as_bstr(),
        time: &time_str,
    };

    let commit_id = repo
        .commit_as(sig, sig, "HEAD", message, new_tree_id, [parent_id])
        .map_err(|e| BumpError::GitError("commit_as".into(), e.to_string()))?;

    let mut index = repo
        .open_index()
        .map_err(|e| BumpError::GitError("open_index".into(), e.to_string()))?;

    for (file_name, blob_id) in &file_blobs {
        for (entry, path) in index.entries_mut_with_paths() {
            if path == file_name.as_bstr() {
                entry.id = *blob_id;
                break;
            }
        }
    }

    index
        .write(gix::index::write::Options::default())
        .map_err(|e| BumpError::GitError("write_index".into(), e.to_string()))?;

    Ok(commit_id.detach())
}

/// Creates a lightweight git tag pointing to `commit_id`.
///
/// A lightweight tag is simply a named reference pointing directly at the
/// commit object.  Use `git tag` for annotated tags if needed.
///
/// # Arguments
///
/// * `repo`      - An open [`gix::Repository`].
/// * `tag_name`  - The tag name (e.g. `"v1.2.3"`).
/// * `commit_id` - The [`gix::ObjectId`] of the commit to tag.
///
/// # Returns
///
/// `Ok(())` on success.
///
/// # Errors
///
/// Returns [`BumpError::GitError`] if the tag reference cannot be written.
///
/// # Complexity
///
/// - **Time**: O(1).
/// - **Space**: O(1).
pub fn create_tag(
    repo: &Repository,
    tag_name: &str,
    commit_id: gix::ObjectId,
) -> Result<(), BumpError> {
    repo.tag_reference(tag_name, commit_id, PreviousValue::Any)
        .map_err(|e| BumpError::GitError("tag_reference".into(), e.to_string()))?;
    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
