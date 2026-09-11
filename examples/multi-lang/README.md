# Multi-Language Workspace

A single project that contains manifests for **five** languages. `bump2version --detect`
discovers and bumps all of them in one pass.

## Structure

| File                    | Language   |
| ----------------------- | ---------- |
| `Cargo.toml`            | Rust       |
| `pyproject.toml`        | Python     |
| `package.json`          | JavaScript |
| `go.mod` / `VERSION`    | Go         |
| `pom.xml`               | Java       |
| `Gemfile` / `*.gemspec` | Ruby       |

## Usage

```sh
# Dry-run: see what would be updated
bump --current-version 0.1.0 --bump patch --detect --dry-run

# Bump patch across all manifests
bump --current-version 0.1.0 --bump patch --detect

# Bump minor, commit all changed files
bump --current-version 0.1.0 --bump minor --detect --commit

# Use the config file (sets current_version automatically)
bump --config-file examples/multi-lang/.bumpversion.toml --bump patch --detect
```
