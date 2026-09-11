# Python Example

Demonstrates `bump2version` managing a Python project version across `pyproject.toml` and `setup.cfg`.

## Usage

```sh
# Dry-run preview
bump --config-file examples/python/.bumpversion.toml --bump patch --dry-run

# Bump patch
bump --config-file examples/python/.bumpversion.toml --bump patch

# Bump minor
bump --config-file examples/python/.bumpversion.toml --bump minor

# Use detect mode to auto-find the pyproject.toml
bump --bump patch --detect --current-version 0.1.0 --dry-run
```
