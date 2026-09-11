# Java Example

Demonstrates `bump2version` managing a Maven project version in `pom.xml`.

## Usage

```sh
# Dry-run preview
bump --config-file examples/java/.bumpversion.toml --bump patch --dry-run

# Bump patch
bump --config-file examples/java/.bumpversion.toml --bump patch

# Jump to an explicit version
bump --config-file examples/java/.bumpversion.toml --new-version 1.0.0
```
