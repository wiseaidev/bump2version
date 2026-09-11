# Node.js Example

Demonstrates `bump2version` managing a Node.js project across `package.json`.

## Usage

```sh
# Dry-run preview
bump --config-file examples/nodejs/.bumpversion.toml --bump patch --dry-run

# Bump patch
bump --config-file examples/nodejs/.bumpversion.toml --bump patch

# Bump minor, override current version
bump --config-file examples/nodejs/.bumpversion.toml --bump minor --current-version 0.1.0
```
