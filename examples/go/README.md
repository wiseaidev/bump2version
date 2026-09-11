# Go Example

Demonstrates `bump2version` managing a Go project version in `VERSION` and a `main.go` constant.

## Usage

```sh
# Dry-run preview
bump --config-file examples/go/.bumpversion.toml --bump patch --dry-run

# Bump patch (updates VERSION + main.go)
bump --config-file examples/go/.bumpversion.toml --bump patch

# Custom serialize to include a build suffix
bump --config-file examples/go/.bumpversion.toml \
  --serialize '{major}.{minor}.{patch}' \
  --bump patch --dry-run
```
