# bump2version Rust Documentation 🦀

The `bump2version` Rust crate provides a fully thread-safe, library-quality
version bumper. All logic is available both as a CLI binary and as a library
importable in other Rust projects.

## 📦 Installation (CLI)

```sh
cargo install bump2version --features rust-binary
```

> **Note**: This installs `bump`, `cargo-bump`, and the original `bump2version` executable. The latter is retained for backward compatibility with older tutorials.

## 📦 Library Usage

```toml
[dependencies]
bump2version = { version = "0.2.2", default-features = false }
```

## 🛠 Usage Overview

### Parse a version

```rust
use bump2version::config::BumpConfig;
use bump2version::version::parse_version;

let cfg = BumpConfig::default();
let v = parse_version("1.2.3", &cfg).unwrap();
assert_eq!(v["major"].value, "1");
assert_eq!(v["patch"].value, "3");
```

### Bump a version

```rust
use bump2version::config::BumpConfig;
use bump2version::version::{BumpPart, bump_version, serialize_version, parse_version};

let cfg = BumpConfig::default();
let v = parse_version("1.2.3", &cfg).unwrap();
let bumped = bump_version(&v, &BumpPart::Minor, &cfg).unwrap();
assert_eq!(serialize_version(&bumped, &cfg), "1.3.0");
```

### Parse config file

```rust
use bump2version::config::parse_config_file;

let cfg = parse_config_file(".bumpversion.toml").unwrap();
println!("{:?}", cfg.current_version);
```

### Apply file search/replace

```rust
use bump2version::config::{BumpConfig, FileConfig};
use bump2version::files::apply_file_change;

let cfg = BumpConfig::default();
let mut fc = FileConfig::new("Cargo.toml");
fc.search = Some(r#"name = "bump2version"\nversion = "{current_version}""#.to_string());
fc.replace = Some(r#"name = "bump2version"\nversion = "{new_version}""#.to_string());

let content = r#"name = "bump2version"\nversion = "1.0.0""#.to_string();
let updated = apply_file_change(&content, &fc, &cfg, "1.0.0", "1.0.1").unwrap();
assert!(updated.contains("1.0.1"));
```

### Read git author from local config

```rust
use bump2version::git::get_git_author;
use gix::open;

let repo = open(".").unwrap();
let (name, email) = get_git_author(&repo).unwrap();
println!("{name} <{email}>");
```

### Installation check

```sh
bump --version
# bump 0.2.1

bump2version --version
# bump2version 0.2.1  (backward-compatible alias)

cargo bump --help
# bump 0.2.1 : high-performance version bumper
```

### 1. Patch / Minor / Major bump

```sh
bump --bump patch          # 0.2.1 → 0.2.2
bump --bump minor          # 0.2.1 → 0.3.0
bump --bump major          # 0.2.1 → 1.0.0
```

### 2. Dry-run preview (no files written)

```sh
bump --bump patch --dry-run
bump --bump minor -n
# Output: [dry-run] Would commit 1 file(s) with message: Bump version: 0.2.1 → 0.2.2
```

### 3. Explicit new version (skip bump calculation)

```sh
bump --new-version 2.0.0
bump --new-version 1.0.0-beta.1
```

### 4. Override current version

```sh
bump --current-version 1.0.0 --bump patch
```

### 5. Custom config file

```sh
bump --config-file path/to/.bumpversion.toml --bump patch
bump --config-file examples/python/.bumpversion.toml --bump minor --dry-run
```

### 6. Git commit

```sh
bump --bump patch --commit
```

### 7. Git commit + tag

```sh
bump --bump patch --commit --tag
```

### 8. Custom commit message

```sh
bump --bump patch --commit --message "chore: release {new_version}"
```

### 9. Extra files on the command line

```sh
bump --bump patch README.md CHANGELOG.md
bump --bump patch --dry-run src/main.rs
```

### 10. Custom parse regex

```sh
bump --bump patch \
  --parse '(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)'
```

### 11. Custom serialize format

```sh
bump --bump patch --serialize '{major}.{minor}.{patch}-dev'
```

### 12. Combined: custom parse + serialize + dry-run

```sh
bump \
  --current-version "1.0.0-alpha.1" \
  --parse '(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)-(?P<stage>[a-z]+)\.(?P<devnum>\d+)' \
  --serialize '{major}.{minor}.{patch}-{stage}.{devnum}' \
  --bump devnum \
  --dry-run
# [dry-run] Would bump 1.0.0-alpha.1 → 1.0.0-alpha.2
```

### 13. Cargo subcommand

```sh
cargo bump --bump patch
cargo bump --bump minor --dry-run
cargo bump --new-version 2.0.0 --commit --tag
```

### 14. Docker

```sh
docker run --rm -v "$(pwd):/workspace" wiseaidev/bump2version \
  --config-file /workspace/.bumpversion.toml \
  --bump patch \
  --dry-run
```

### 15. Watch mode (requires `watch` feature)

```sh
# Monitors .bumpversion.toml and registered files; auto-bumps on save
bump --watch --bump patch
```

### 16. Multi-language auto-detect (requires `detect` feature)

```sh
# Scans the directory tree for Cargo.toml, pyproject.toml, package.json, etc.
bump --bump patch --detect
bump --bump minor --detect --dry-run
```

### 17. Backward-compatible `bump2version` binary

```sh
# All commands work identically on the bump2version alias:
bump2version --bump patch --dry-run
bump2version --bump minor
bump2version --new-version 2.0.0 --commit --tag
```

## 🗂 Examples

All examples live under `examples/` with pre-built `.bumpversion.toml` configs:

| Directory                                     | Language      | Key Files                     |
| --------------------------------------------- | ------------- | ----------------------------- |
| [`examples/rust-lib`](examples/rust-lib/)     | Rust          | `Cargo.toml`                  |
| [`examples/python`](examples/python/)         | Python        | `pyproject.toml`, `setup.cfg` |
| [`examples/nodejs`](examples/nodejs/)         | Node.js       | `package.json`                |
| [`examples/go`](examples/go/)                 | Go            | `VERSION`, `main.go`          |
| [`examples/java`](examples/java/)             | Java (Maven)  | `pom.xml`                     |
| [`examples/ruby`](examples/ruby/)             | Ruby          | `.gemspec`, `Gemfile`         |
| [`examples/multi-lang`](examples/multi-lang/) | All 6 at once | all of the above              |
| [`examples/yew-app`](examples/yew-app/)       | Yew WASM      | browser UI                    |

### Running any example

```sh
# From the project root:
bump --config-file examples/python/.bumpversion.toml --bump patch --dry-run

# From inside the example directory:
cd examples/nodejs
bump --bump minor --dry-run
```

### Detect mode across the multi-lang workspace

```sh
cd examples/multi-lang
bump --current-version 0.1.0 --bump patch --detect --dry-run
# [dry-run] Would update: Cargo.toml, pyproject.toml, package.json, VERSION, pom.xml, my-gem.gemspec
```

## 📖 Module Overview

| Module      | Description                                                                        |
| ----------- | ---------------------------------------------------------------------------------- |
| `config`    | Parse `.bumpversion.toml` into `BumpConfig`, `FileConfig`, `PartConfig`            |
| `version`   | Parse version strings, bump components, serialize back to string                   |
| `files`     | Apply single/multiline search-replace with `{current_version}` tokens              |
| `git`       | Thread-safe git commit + tag via `gix`; reads author from git config               |
| `error`     | Typed `BumpError` enum                                                             |
| `utils`     | Higher-level helpers: `load_config`, `compute_new_version`, `collect_file_configs` |
| `workspace` | Atomic workspace-aware bumping across all Cargo workspace members                  |
| `detect`    | Multi-language manifest scanner (requires `detect` feature)                        |
| `watch`     | File-system watcher loop for auto-bumping (requires `watch` feature)               |
| `cli`       | `clap`-based CLI argument struct (requires `cli` feature)                          |
| `python`    | PyO3 bindings (requires `python` feature)                                          |
| `node`      | napi-rs bindings (requires `node` feature)                                         |

## 🔗 See Also

- [docs.rs/bump2version](https://docs.rs/bump2version)
- [GitHub](https://github.com/wiseaidev/bump2version)
- [CLI Reference](https://github.com/wiseaidev/bump2version/blob/main/CLI.md)
- [Docker Guide](https://github.com/wiseaidev/bump2version/blob/main/DOCKER.md)
- [WASM / Yew Guide](https://github.com/wiseaidev/bump2version/blob/main/WASM.md)
