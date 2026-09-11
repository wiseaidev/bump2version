# bump2version Command Line Interface ⬆️

The standalone `bump` binary translates CLI arguments into optimized version-bump operations powered entirely by the Rust engine. It can also be invoked as a `cargo bump` subcommand after installation.

## 📦 Installation

To enable the CLI binary compilation, install the crate with the `rust-binary` feature:

```bash
cargo install bump2version --features rust-binary
```

This installs three executables:

- `bump`: the new, preferred, shorter standalone CLI
- `cargo-bump`: the Cargo subcommand integration
- `bump2version`: retained indefinitely for backward compatibility with older tutorials and scripts

## 🛠 Available Commands & Options

### 1. Basic Patch Bump (most common)

Increments the `patch` component of the version found in `.bumpversion.toml`.

```bash
bump --bump patch
```

### 2. Minor Bump

Increments `minor`, resets `patch` to `0`.

```bash
bump --bump minor
```

### 3. Major Bump

Increments `major`, resets `minor` and `patch` to `0`.

```bash
bump --bump major
```

### 4. Dry Run (preview only, no files written)

```bash
bump --bump patch --dry-run
bump --bump minor -n
```

### 5. Explicit New Version

Skip the bump calculation and set the version directly.

```bash
bump --new-version 2.0.0
bump --new-version 1.0.0-beta.1
```

### 6. Override Current Version

```bash
bump --current-version 1.0.0 --bump patch
```

### 7. Custom Config File

```bash
bump --config-file /path/to/.bumpversion.toml --bump patch
```

### 8. Bump + Git Commit

Create a git commit containing all modified files.

```bash
bump --bump patch --commit
```

### 9. Bump + Git Commit + Tag

Create a commit and a lightweight tag (`v{new_version}`).

```bash
bump --bump patch --commit --tag
```

### 10. Custom Commit Message

```bash
bump --bump patch --commit --message "Release {new_version}"
```

### 11. Extra Files on Command Line

Update additional files not listed in the config.

```bash
bump --bump patch README.md CHANGELOG.md
```

### 12. Custom Parse Regex

Use a custom regex for version parsing.

```bash
bump --bump patch --parse '(?P<major>\d+)\.(?P<minor>\d+)'
```

### 13. Custom Serialize Format

```bash
bump --bump patch --serialize '{major}.{minor}.{patch}-dev'
```

### 14. Combined: Custom parse + serialize + dry-run

```bash
bump \
  --current-version "1.0.0-alpha.1" \
  --parse '(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)-(?P<stage>[a-z]+)\.(?P<devnum>\d+)' \
  --serialize '{major}.{minor}.{patch}-{stage}.{devnum}' \
  --bump devnum \
  --dry-run
```

### 15. Cargo Subcommand Mode

After installation, `bump` is accessible as a `cargo` subcommand:

```bash
cargo bump --bump patch
cargo bump --bump minor --dry-run
cargo bump --new-version 2.0.0 --commit --tag
```

### 16. Docker Usage

```bash
docker run --rm -v "$(pwd):/workspace" wiseaidev/bump2version \
  --config-file /workspace/.bumpversion.toml \
  --bump patch \
  --dry-run
```

### 17. Watch Mode (requires `watch` feature)

Monitors files for changes and auto-bumps on save.

```bash
bump --watch --bump patch
```

### 18. Multi-Language Auto-Detect (requires `detect` feature)

Scans the directory tree and bumps all discovered manifests.

```bash
bump --bump patch --detect
bump --bump patch --detect --dry-run
```

## 🎛 Options Reference

| Flag                | Short | Default                                           | Description              |
| ------------------- | ----- | ------------------------------------------------- | ------------------------ |
| `--config-file`     | `-c`  | `.bumpversion.toml`                               | Path to config file      |
| `--current-version` |       | _(from config)_                                   | Override current version |
| `--bump`            |       | `patch`                                           | Component to bump        |
| `--parse`           |       | semver regex                                      | Regex with named groups  |
| `--serialize`       |       | `{major}.{minor}.{patch}`                         | Format string            |
| `--dry-run`         | `-n`  | `false`                                           | Preview without writing  |
| `--new-version`     |       | _(computed)_                                      | Explicit new version     |
| `--commit`          |       | `true`                                            | Create git commit        |
| `--tag`             |       | `false`                                           | Create git tag           |
| `--message`         | `-m`  | `Bump version: {current_version} → {new_version}` | Commit message template  |
| `--watch`           |       | `false`                                           | Enter watch mode         |
| `--detect`          |       | `false`                                           | Multi-language detection |
| `files...`          |       | `[]`                                              | Extra files to update    |

## 🔁 Commands Examples

| Scenario         | Command                                                                |
| ---------------- | ---------------------------------------------------------------------- |
| Min config patch | `bump --bump patch --dry-run`                                          |
| Min config minor | `bump --bump minor --dry-run`                                          |
| Min config major | `bump --bump major --dry-run`                                          |
| Explicit version | `bump --new-version 9.9.9 --dry-run`                                   |
| Override current | `bump --current-version 0.1.0 --bump patch --dry-run`                  |
| Commit only      | `bump --bump patch --commit`                                           |
| Commit + tag     | `bump --bump patch --commit --tag`                                     |
| Custom message   | `bump --bump patch --commit --message "v{new_version} release"`        |
| Extra files      | `bump --bump patch README.md --dry-run`                                |
| Custom parse     | `bump --parse '(?P<major>\d+)\.(?P<minor>\d+)' --bump minor --dry-run` |
| Custom serialize | `bump --serialize '{major}.{minor}' --bump patch --dry-run`            |
| Cargo subcommand | `cargo bump --bump patch --dry-run`                                    |
| Detect mode      | `bump --bump patch --detect --dry-run`                                 |
| Watch mode       | `bump --watch --bump patch`                                            |

## 🍃 Alias

Add a convenient alias to your shell profile (`~/.bashrc` or `~/.zshrc`):

```sh
alias bv="bump"
```
