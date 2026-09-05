# bump2version Rust Documentation 🦀

The `bump2version` Rust crate provides a fully thread-safe, library-quality
version bumper. All logic is available both as a CLI binary and as a library
importable in other Rust projects.

## 📦 Installation (CLI)

```sh
cargo install bump2version --features rust-binary
```

## 📦 Library Usage

```toml
[dependencies]
bump2version = { version = "0.2.0", default-features = false }
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
use bump2version::version::{parse_version, bump_version, serialize_version};

let cfg = BumpConfig::default();
let v = parse_version("1.2.3", &cfg).unwrap();
let bumped = bump_version(&v, "minor", &cfg).unwrap();
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
fc.search = Some(r#"version = "{current_version}""#.to_string());
fc.replace = Some(r#"version = "{new_version}""#.to_string());

let content = r#"version = "1.0.0""#.to_string();
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

## 📖 Module Overview

| Module    | Description                                                                        |
| --------- | ---------------------------------------------------------------------------------- |
| `config`  | Parse `.bumpversion.toml` into `BumpConfig`, `FileConfig`, `PartConfig`            |
| `version` | Parse version strings, bump components, serialize back to string                   |
| `files`   | Apply single/multiline search-replace with `{current_version}` tokens              |
| `git`     | Thread-safe git commit + tag via `gix`; reads author from git config               |
| `error`   | Typed `BumpError` enum                                                             |
| `utils`   | Higher-level helpers: `load_config`, `compute_new_version`, `collect_file_configs` |
| `cli`     | `clap`-based CLI argument struct (requires `cli` feature)                          |
| `python`  | PyO3 bindings (requires `python` feature)                                          |
| `node`    | napi-rs bindings (requires `node` feature)                                         |

## 🔗 See Also

- [docs.rs/bump2version](https://docs.rs/bump2version)
- [GitHub](https://github.com/wiseaidev/bump2version)
