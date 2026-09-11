# Ruby Example

Demonstrates `bump2version` managing a Ruby gem version across `.gemspec` and `Gemfile`.

## Usage

```sh
# Dry-run preview
bump --config-file examples/ruby/.bumpversion.toml --bump patch --dry-run

# Bump patch (updates gemspec + Gemfile)
bump --config-file examples/ruby/.bumpversion.toml --bump patch

# Bump major
bump --config-file examples/ruby/.bumpversion.toml --bump major
```
