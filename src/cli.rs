use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug, Clone)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "bump2version",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{before-help}{name} {version}
{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
about=r#"
🚀 Bump2version CLI
===================

Bump2version CLI is a command-line tool for managing version numbers in your projects.
Easily update version strings, create commits, and manage version control tags.

FEATURES:
  - Incremental Versioning: Bump major, minor, or patch versions with ease.
  - Configurability: Use a configuration file or command-line options to customize behavior.
  - Git Integration: Create commits and tags in your version control system.

USAGE:
  bump2version [OPTIONS]

EXAMPLES:
  Bump patch version:
    bump2version --current-version 1.2.3 --bump patch

  Bump minor version and create a commit:
    bump2version --current-version 1.2.3 --bump minor --commit

For more information, visit: https://github.com/wiseaidev/bump2version
"#
)]
pub struct Cli {
    /// Config file to read most of the variables from.
    #[arg(
        short = 'c',
        long = "config-file",
        value_name = "FILE",
        default_value_t = String::from(".bumpversion.toml")
    )]
    pub config_file: String,

    /// Version that needs to be updated.
    #[arg(long = "current-version", value_name = "VERSION")]
    pub current_version: String,

    /// Part of the version to be bumped.
    #[arg(
        long = "bump",
        value_name = "PART",
        default_value_t = String::from("patch")
    )]
    pub bump: String,

    /// Regex parsing the version string.
    #[arg(
        long = "parse",
        value_name = "REGEX",
        default_value_t = String::from(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)")
    )]
    pub parse: String,

    /// How to format what is parsed back to a version.
    #[arg(
        long = "serialize",
        value_name = "FORMAT",
        default_value_t = String::from("{major}.{minor}.{patch}")
    )]
    pub serialize: String,

    /// Don't write any files, just pretend.
    #[arg(short = 'n', long = "dry-run")]
    pub dry_run: bool,

    /// New version that should be in the files.
    #[arg(long = "new-version", value_name = "VERSION")]
    pub new_version: String,

    /// Create a commit in version control.
    #[arg(long = "commit", default_value_t = true)]
    pub commit: bool,

    /// Create a tag in version control.
    #[arg(long = "tag")]
    pub tag: bool,

    /// Commit message.
    #[arg(
        short = 'm',
        long = "message",
        value_name = "COMMIT_MSG",
        default_value_t = String::from("Bump version: {current_version} → {new_version}")
    )]
    pub message: String,

    /// Files to change.
    #[arg(value_name = "file")]
    pub files: Vec<String>,
}
