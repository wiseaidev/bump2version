// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Error Types
//!
//! Defines [`BumpError`], the single typed error enum used throughout
//! `bump2version`. Every public function that can fail returns
//! `Result<_, BumpError>`.
//!
//! ## No-std compatibility
//!
//! This module is fully `no_std + alloc`. The [`BumpError::Io`] variant is
//! only present when the `std` feature is enabled, since `std::io::Error`
//! is unavailable in `no_std` contexts.

use alloc::string::String;
use thiserror::Error;

/// The top-level error type for the `bump2version` library.
///
/// Every variant carries a human-readable message and, where applicable,
/// the underlying cause wrapped via [`thiserror`].
///
/// # Examples
///
/// ```rust
/// use bump2version::error::BumpError;
///
/// let err = BumpError::VersionNotFound("1.0.0".into(), "README.md".into());
/// assert!(err.to_string().contains("1.0.0"));
/// ```
#[derive(Debug, Error)]
pub enum BumpError {
    /// The configuration file could not be found at the specified path.
    ///
    /// Field 0 is the path that was requested.
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),

    /// The configuration file exists but contained malformed content.
    ///
    /// Field 0 is a description of what went wrong.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// The search pattern for a version string was not found in a file.
    ///
    /// Field 0 is the search pattern, field 1 is the file path.
    #[error("Did not find '{0}' in file '{1}'")]
    VersionNotFound(String, String),

    /// The `parse` regex failed to compile.
    ///
    /// Field 0 is the raw regex string, field 1 is the underlying [`regex::Error`].
    #[error("Invalid parse regex '{0}': {1}")]
    InvalidRegex(String, #[source] regex::Error),

    /// The requested version component (e.g. `"patch"`) does not exist in
    /// the parsed version.
    ///
    /// Field 0 is the component name.
    #[error("Unknown version component: '{0}'")]
    UnknownComponent(String),

    /// A numeric version component could not be incremented because its
    /// current value is not a valid integer.
    ///
    /// Field 0 is the component value.
    #[error("Version component '{0}' is not a valid integer")]
    InvalidComponentValue(String),

    /// A Git operation failed.
    ///
    /// Field 0 is a description of the operation, field 1 is the cause.
    #[error("Git error during '{0}': {1}")]
    GitError(String, String),

    /// A generic I/O error (file reads/writes, process spawning, etc.).
    ///
    /// Only available with the `std` feature.
    #[cfg(feature = "std")]
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An error that doesn't fit any of the above categories.
    #[error("{0}")]
    Other(String),
}

#[cfg(feature = "std")]
impl From<anyhow::Error> for BumpError {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
