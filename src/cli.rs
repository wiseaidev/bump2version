// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # CLI Argument Definitions
//!
//! Defines [`Cli`], the parsed command-line interface for `bump2version`.
//! All arguments map directly to `[bumpversion]` configuration keys and
//! override the values found in the config file.

use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Styles};

/// Returns the ANSI styles used for the `bump2version` CLI help text.
///
/// # Returns
///
/// A [`Styles`] instance with red headers/errors, blue literals, and green
/// placeholders.
///
/// # Complexity
///
/// - **Time**: O(1).
/// - **Space**: O(1).
fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

/// Command-line interface for the `bump2version` tool.
///
/// All fields correspond to configuration keys supported in `.bumpversion.toml`.
/// Command-line values take precedence over the config file.
#[derive(Parser, Debug, Clone)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "bump2version",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about = r#"
██████╗ ██╗   ██╗███╗   ███╗██████╗ ██████╗ ██╗   ██╗███████╗██████╗ ███████╗██╗ ██████╗ ███╗   ██╗
██╔══██╗██║   ██║████╗ ████║██╔══██╗╚════██╗██║   ██║██╔════╝██╔══██╗██╔════╝██║██╔═══██╗████╗  ██║
██████╔╝██║   ██║██╔████╔██║██████╔╝ █████╔╝██║   ██║█████╗  ██████╔╝███████╗██║██║   ██║██╔██╗ ██║
██╔══██╗██║   ██║██║╚██╔╝██║██╔═══╝ ██╔═══╝ ╚██╗ ██╔╝██╔══╝  ██╔══██╗╚════██║██║██║   ██║██║╚██╗██║
██████╔╝╚██████╔╝██║ ╚═╝ ██║██║     ███████╗ ╚████╔╝ ███████╗██║  ██║███████║██║╚██████╔╝██║ ╚████║
╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝
                                                                                                   
Bump2version CLI is a command-line tool for managing version numbers in your
projects. Easily update version strings, create commits, and manage version
control tags.

FEATURES:
  - Incremental Versioning: Bump major, minor, or patch versions with ease.
  - Multiline Search/Replace: Match multi-line patterns in any file.
  - Configurability: Use a configuration file or CLI options to customize.
  - Git Integration: Create commits and tags; uses your configured identity.

EXAMPLES:
  Bump patch version:
    bump2version --current-version 1.2.3 --bump patch

  Bump minor version and create a commit:
    bump2version --current-version 1.2.3 --bump minor --commit

For more information, visit: https://github.com/wiseaidev/bump2version
"#
)]
pub struct Cli {
    /// Path to the configuration file.
    ///
    /// Defaults to `.bumpversion.toml` in the current directory.  All
    /// config-file values can be overridden by the corresponding CLI flags.
    #[arg(
        short = 'c',
        long = "config-file",
        value_name = "FILE",
        default_value_t = String::from(".bumpversion.toml")
    )]
    pub config_file: String,

    /// The version string currently present in the project.
    ///
    /// When omitted, `bump2version` reads `current_version` from the config
    /// file.
    #[arg(long = "current-version", value_name = "VERSION")]
    pub current_version: Option<String>,

    /// The version component to increment.
    ///
    /// Must be a named capture group in the `--parse` regex.  Typical values
    /// are `major`, `minor`, and `patch`.  Defaults to `patch`.
    #[arg(
        long = "bump",
        value_name = "PART",
        default_value_t = String::from("patch")
    )]
    pub bump: String,

    /// Regular expression (with named capture groups) used to parse the
    /// version string into its components.
    ///
    /// Defaults to the standard semver regex with optional pre-release parts.
    #[arg(
        long = "parse",
        value_name = "REGEX",
        default_value_t = String::from(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)")
    )]
    pub parse: String,

    /// Python-style format string used to serialise the version back to a
    /// string after bumping.
    ///
    /// Placeholder names must match the capture group names in `--parse`.
    /// Defaults to `{major}.{minor}.{patch}`.
    #[arg(
        long = "serialize",
        value_name = "FORMAT",
        default_value_t = String::from("{major}.{minor}.{patch}")
    )]
    pub serialize: String,

    /// Simulate the bump without writing any files or creating any git
    /// objects.
    ///
    /// Useful for previewing what `bump2version` would do.
    #[arg(short = 'n', long = "dry-run", default_value_t = false)]
    pub dry_run: bool,

    /// Explicit new version string to use instead of computing it from
    /// `--bump`.
    ///
    /// When supplied, `--bump` is ignored.
    #[arg(long = "new-version", value_name = "VERSION")]
    pub new_version: Option<String>,

    /// Create a git commit containing all modified files after bumping.
    #[arg(long = "commit", default_value_t = true)]
    pub commit: bool,

    /// Create a lightweight git tag pointing to the version-bump commit.
    ///
    /// The tag name is derived from the `tag_name` field in the config file
    /// (default: `v{new_version}`).
    #[arg(long = "tag")]
    pub tag: bool,

    /// Template for the git commit message.
    ///
    /// Supports `{current_version}` and `{new_version}` placeholders.
    #[arg(
        short = 'm',
        long = "message",
        value_name = "COMMIT_MSG",
        default_value_t = String::from("Bump version: {current_version} → {new_version}")
    )]
    pub message: String,

    /// Additional files to update beyond those listed in the config file.
    ///
    /// Each file is searched for the current version string and replaced with
    /// the new version string using the global `search`/`replace` patterns.
    #[arg(value_name = "file")]
    pub files: Vec<String>,
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
