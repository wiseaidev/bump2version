//! # ⬆️ Bump2version
//!
//! Bump2version is a command-line tool for managing version numbers in your projects.
//! Easily update version strings, create commits, and manage version control tags.
//!
//! ## Features
//!
//! - **Incremental Versioning:** Bump major, minor, or patch versions with ease.
//! - **Configurability:** Use a configuration file or command-line options to customize behavior.
//! - **Git Integration:** Create commits and tags in your version control system.
//!
//! ## Quick Start
//!
//! Get started with the `bump2version` CLI by following these simple steps:
//!
//! 1. Install the `bump2version` tool using Cargo:
//!
//! ```bash
//! cargo install bump2version
//! ```
//!
//! 2. Use the following options to manage version numbers and customize the behavior:
//!
//! ```bash
//! bump2version --current-version 1.2.3 --bump patch
//! ```
//!
//! ## Options
//!
//! | Option                 | Description                                                       |
//! |------------------------|-------------------------------------------------------------------|
//! | `--config-file`        | Config file to read most of the variables from.                   |
//! | `--current-version`    | Version that needs to be updated.                                 |
//! | `--bump`               | Part of the version to be bumped (default: patch).                |
//! | `--parse`              | Regex parsing the version string (default: \d+\.\d+\.\d+).        |
//! | `--serialize`          | How to format what is parsed back to a version (default: {major}.{minor}.{patch}). |
//! | `--dry-run`            | Don't write any files, just pretend.                               |
//! | `--new-version`        | New version that should be in the files.                           |
//! | `--commit`             | Create a commit in version control (default: true).                |
//! | `--tag`                | Create a tag in version control.                                   |
//! | `--message`            | Commit message (default: Bump version: {current_version} → {new_version}). |
//! | `file`                 | Files to change.                                                  |
//!
//! ## GitHub Repository
//!
//! You can access the source code for this CLI tool on [GitHub](https://github.com/wiseaidev/bump2version).
//!
//! ## Contributing
//!
//! Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement,
//! please engage with the project on [GitHub](https://github.com/wiseaidev/bump2version).
//! Your contributions help improve this CLI tool for the community.
//!
//! **Manage your project versions with ease! 🚀**

pub mod cli;
pub mod utils;
