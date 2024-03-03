# ⬆️ Bump2version

[![Crates.io](https://img.shields.io/crates/v/bump2version.svg)](https://crates.io/crates/bump2version)
[![docs](https://docs.rs/bump2version/badge.svg)](https://docs.rs/bump2version/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> 🚀 **bump2version**: A command-line tool for managing version numbers in your projects.

## 📖 Table of Contents

- [Quick Start](#-quick-start)
- [Configuration File](#-configuration-file)
- [Features](#-features)
- [Options](#-options)
- [GitHub Repository](#-github-repository)
- [Contributing](#-contributing)

## 🚀 Quick Start

Get started with the `bump2version` CLI by following these simple steps:

1. Install the `bump2version` tool using Cargo:

   ```bash
   cargo install bump2version
   ```

1. Use the following options to manage version numbers and customize the behavior:

   ```bash
   bump2version --bump patch # or minor or major
   ```

## 🛠️ Configuration File

To customize the behavior of `bump2version`, you can use a configuration file named `.bumpversion.toml`. This file should be present in the root of your project. If it's not found, the cli will use default values.

Here's an example `.bumpversion.toml` configuration file you can use in your project:

```toml
[bumpversion]
current_version = 0.1.0 # Specify the current version of your project, samne value as Cargo.toml version

# Specify how to search for and replace the version in the 'Cargo.toml' file.
[bumpversion:file:Cargo.toml]
search = 'version = "{current_version}"'
replace = 'version = "{new_version}"'
```

## ✨ Features

- **Incremental Versioning:** Bump major, minor, or patch versions with ease.
- **Configurability:** Use a configuration file or command-line options to customize behavior.
- **Git Integration:** Create commits and tags in your version control system.

## 🎨 Options

| Option                 | Description                                                       |
|------------------------|-------------------------------------------------------------------|
| `--config-file`        | Config file to read most of the variables from (default: `.bumpversion.toml`). |
| `--current-version`    | Version that needs to be updated.                                 |
| `--bump`               | Part of the version to be bumped (default: patch).                |
| `--parse`              | Regex parsing the version string (default: \d+\.\d+\.\d+).        |
| `--serialize`          | How to format what is parsed back to a version (default: {major}.{minor}.{patch}). |
| `--dry-run`            | Don't write any files, just pretend.                               |
| `--new-version`        | New version that should be in the files.                           |
| `--commit`             | Create a commit in version control (default: true).                |
| `--tag`                | Create a tag in version control.                                   |
| `--message`            | Commit message (default: Bump version: {current_version} → {new_version}). |
| `file`                 | Files to change.                                                  |

## 🌐 GitHub Repository

You can access the source code for this CLI tool on [GitHub](https://github.com/wiseaidev/bump2version).

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/bump2version). Your contributions help improve this CLI tool for the community.
