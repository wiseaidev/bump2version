<div align="center">

# ⬆️ Bump2version

[![bump2version logo](https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/logo.png)](https://github.com/wiseaidev/bump2version)

[![Crates.io](https://img.shields.io/crates/v/bump2version.svg)](https://crates.io/crates/bump2version)
[![Docs.rs](https://docs.rs/bump2version/badge.svg)](https://docs.rs/bump2version)
[![PyPI](https://img.shields.io/pypi/v/bump-rs.svg)](https://pypi.org/project/bump-rs)
[![npm](https://img.shields.io/npm/v/bump2version.svg)](https://www.npmjs.com/package/bump2version)
[![Docker](https://img.shields.io/docker/v/wiseaidev/bump2version?label=docker)](https://hub.docker.com/r/wiseaidev/bump2version)
[![GitHub Marketplace](https://img.shields.io/badge/Marketplace-bump2version-blue?logo=github)](https://github.com/marketplace/actions/bump-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/wiseaidev/bump2version/blob/main/LICENSE)

> `bump2version` is the world's fastest version bumper written entirely in **100% safe Rust**, with `no_std` support, native Python and Node.js bindings, and a `cargo bump` subcommand 🗿.

[![bump2version banner](https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/new-banner.png)](https://github.com/wiseaidev/bump2version)

</div>

## 🚀 Installation

| Platform             | Command                                                                                        |
| -------------------- | ---------------------------------------------------------------------------------------------- |
| **Rust binary**      | `cargo install bump2version --features rust-binary`                                            |
| **Cargo subcommand** | `cargo bump --help`                                                                            |
| **Docker**           | `docker pull wiseaidev/bump2version`                                                           |
| **Debian/Ubuntu**    | Download `.deb` from [GitHub Releases](https://github.com/wiseaidev/bump2version/releases)     |
| **RHEL/Fedora**      | Download `.rpm` from [GitHub Releases](https://github.com/wiseaidev/bump2version/releases)     |
| **Windows**          | Download `bump.exe` from [GitHub Releases](https://github.com/wiseaidev/bump2version/releases) |
| **GitHub Action**    | See [action.yml](https://github.com/wiseaidev/bump2version/blob/main/action.yml)               |
| **Python**           | `pip install bump-rs`                                                                          |
| **Node.js**          | `npm install bump2version`                                                                     |

> [!NOTE]
> Installing via `cargo` installs both `bump` and `bump2version` binaries. The original `bump2version` binary is retained indefinitely for backward compatibility with existing tutorials, CI/CD pipelines, and automation scripts.

## 🤔 What does this crate provide?

`bump2version` automates semantic version management for any project regardless of language. It:

- **Parses** version strings using a fully configurable regex (default: semver `major.minor.patch`).
- **Bumps** any named component (`major`, `minor`, `patch`, or custom cyclic stages like `alpha → beta → rc → stable`).
- **Rewrites** version occurrences across multiple files, including multiline CHANGELOG patterns.
- **Commits and tags** via 100% pure `gix` (gitoxide); zero subprocess calls.
- **Detects** version strings across any language's manifest files (Rust, Python, JS, Go, Java, Ruby).
- **Watches** for file changes and bumps automatically on save (one bump per save, with debounce).
- **Bumps workspaces** atomically across all Cargo workspace members.

## 💻 Command-line Interface

```sh
# Install
cargo install bump2version --features rust-binary

# Use directly
bump --bump patch          # 0.2.1 → 0.2.2
bump --bump minor          # 0.2.1 → 0.3.0
bump --bump major          # 0.2.1 → 1.0.0
bump --bump patch --dry-run  # preview only

# Use as cargo subcommand
cargo bump --bump patch
cargo bump --bump minor --dry-run

# Docker
docker run --rm -v $(pwd):/workspace wiseaidev/bump2version --bump patch --dry-run

# Multi-language auto-detect
bump --bump patch --detect
```

| Option               | Description                                              |
| -------------------- | -------------------------------------------------------- |
| `--config-file`      | Config file path (default: `.bumpversion.toml`)          |
| `--current-version`  | Override current version                                 |
| `--bump`             | Component: `major`, `minor`, `patch`, or any custom part |
| `--parse`            | Parse regex override                                     |
| `--serialize`        | Serialize format override                                |
| `--dry-run` / `-n`   | Simulate without writing files                           |
| `--new-version`      | Explicit new version (skips bump calculation)            |
| `--commit` / `--tag` | Git commit + lightweight tag                             |

## 🔭 Features

| Feature  | Default | Description                                   |
| -------- | ------- | --------------------------------------------- |
| `std`    | ✅      | File I/O, git integration, regex stdlib cache |
| `cli`    | ❌      | Standalone `bump` binary via `clap`           |
| `watch`  | ❌      | File-system watcher (bump on file save)       |
| `detect` | ❌      | Multi-language manifest auto-detection        |
| `python` | ❌      | Python extension module via PyO3/maturin      |
| `node`   | ❌      | Node.js native add-on via napi-rs             |

## 🦀 Rust

The Rust crate is available on [crates.io](https://crates.io/crates/bump2version). For a complete API reference visit **[RUST.md](https://github.com/wiseaidev/bump2version/blob/main/RUST.md)**.

### Quick Start

```toml
[dependencies]
bump2version = "0.2.1"
```

```rust
use bump2version::{config::BumpConfig, version::{BumpPart, parse_version, bump_version, serialize_version}};

fn main() {
    let cfg = BumpConfig::default();
    let v   = parse_version("1.2.3", &cfg).unwrap();
    let v2  = bump_version(&v, &BumpPart::Patch, &cfg).unwrap();
    println!("{}", serialize_version(&v2, &cfg)); // 1.2.4
}
```

### `no_std` Support

Core modules (`config`, `version`, `files`, `error`) compile in `no_std + alloc`:

```toml
bump2version = { version = "0.2.1", default-features = false }
```

| Module           | `no_std+alloc` | `std` |
| ---------------- | -------------- | ----- |
| `config`         | ✅             | ✅    |
| `version`        | ✅             | ✅    |
| `files`          | ✅             | ✅    |
| `error`          | ✅             | ✅    |
| `git`            | ❌             | ✅    |
| CLI              | ❌             | ✅    |
| Python / Node.js | ❌             | ✅    |

## 🐍 Python

```sh
pip install bump-rs
```

```python
from bump_rs import bump_version, apply_file_change, BumpConfig

print(bump_version("1.2.3", "patch"))   # "1.2.4"
print(bump_version("1.2.3", "minor"))   # "1.3.0"

cfg = BumpConfig(parse=r"(?P<major>\d+)\.(?P<minor>\d+)", serialize="{major}.{minor}")
print(bump_version("2.0", "minor", config=cfg))  # "2.1"
```

For full docs see **[PYTHON.md](https://github.com/wiseaidev/bump2version/blob/main/PYTHON.md)**.

## 🟩 Node.js

```sh
npm install bump2version
```

```javascript
const { bumpVersion, applyFileChange } = require("bump2version");
console.log(bumpVersion("1.2.3", "patch")); // '1.2.4'
```

For full docs see **[NODE.md](https://github.com/wiseaidev/bump2version/blob/main/NODE.md)**.

## ⚙️ Configuration File (`.bumpversion.toml`)

```toml
[bumpversion]
current_version = "1.0.0"
commit = true
tag = true

[bumpversion:file:Cargo.toml]
search = name = "my-crate"
    version = "{current_version}"
replace = name = "my-crate"
    version = "{new_version}"

[bumpversion:file:CHANGELOG.md]
search = """
## {current_version}
    Release notes line 1"""
replace = """
## {new_version}
    Release notes line 1"""
```

### Pre-release Cycling

```toml
[bumpversion]
current_version = "1.0.0-alpha.1"
parse = (?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)-(?P<stage>[a-z]+)\.(?P<devnum>\d+)
serialize =
    {major}.{minor}.{patch}-{stage}.{devnum}
    {major}.{minor}.{patch}

[bumpversion:part:stage]
optional_value = stable
first_value = alpha
values =
    alpha
    beta
    rc
    stable
```

### Cargo Workspace Bumping

```sh
# Atomically bump all workspace member crates
bump --bump patch
```

The `workspace` module discovers all `[workspace]` members and bumps every `Cargo.toml` in one pass.

## 🔭 GitHub Action

The action is published as **[`bump-rs`](https://github.com/marketplace/actions/bump-rs)** on the GitHub Marketplace.

```yaml
- name: bump-rs
  uses: wiseaidev/bump2version@v0.2.1
  with:
    release_type: patch # 'major', 'minor', or 'patch' - omit to auto-detect from git tags
    commit: "true"
    tag: "true"
    dry-run: "false"
    working-directory: "."
    config-file: ".bumpversion.toml"
```

| Output         | Description                                       |
| -------------- | ------------------------------------------------- |
| `new_version`  | The new version string after bumping              |
| `old_version`  | The previous version string before bumping        |
| `release_type` | The component that was bumped (major/minor/patch) |

## 🔒 Safety

This crate enforces `#![forbid(unsafe_code)]` at the crate root (except the Node.js FFI layer which requires `unsafe` for napi-rs interop). Every byte of the implementation: config parsing, regex matching, version bumping, git object creation, is written in safe Rust.

## 📊 Benchmarks

### CLI vs CLI: `bump` vs `bump-my-version` (hyperfine)

Measured with `hyperfine --runs 5 --warmup 2 -N` on x86-64 Linux (target-cpu=native release build):

| Scenario   | Tool                       | Mean       | Min      | Max      |
| ---------- | -------------------------- | ---------- | -------- | -------- |
| patch bump | **`bump` (Rust)**          | **5.6 ms** | 2.6 ms   | 10.5 ms  |
| minor bump | **`bump` (Rust)**          | **3.9 ms** | 2.8 ms   | 6.2 ms   |
| major bump | **`bump` (Rust)**          | **2.5 ms** | 1.9 ms   | 3.2 ms   |
| any bump   | `bump-my-version` (Python) | 482.3 ms   | 473.8 ms | 488.0 ms |

**CLI speedup: ~90-200× faster** depending on scenario (major bump is fastest: ~193×).

At the **library function level** (pure parse+bump+serialize, no process startup):

| Benchmark                      | Rust (`bump2version`) | Python (`bump-my-version`) | Speedup   |
| ------------------------------ | --------------------- | -------------------------- | --------- |
| parse + bump + serialize       | ~13 µs                | ~79 µs                     | ~6×       |
| Config parse (minimal)         | ~12 µs                | ~4 800 µs                  | ~400×     |
| File replace, 10K lines        | ~14 ms                | -                          | -         |
| Full pipeline (CLI cold start) | **2.4 ms**            | **482 ms**                 | **~200×** |

Run the comparison yourself:

```sh
# Rust
hyperfine --runs 10 -N "bump --config-file .bumpversion.toml --bump patch --dry-run"

# Python (if installed)
hyperfine --runs 10 "bump-my-version bump patch --dry-run"

# Full comparative suite
chmod +x benchmarks/hyperfine/run_comparison.sh
./benchmarks/hyperfine/run_comparison.sh
```

### Internal Benchmarks (`cargo bench`)

Run with `cargo bench`. Results on x86-64 Linux after optimization (LTO=fat, opt-level=3, target-cpu=native):

| Benchmark                      | Time    |
| ------------------------------ | ------- |
| `config_parse/minimal`         | ~12 µs  |
| `config_parse/full_with_parts` | ~18 µs  |
| `version_parse/1.0.0`          | ~430 ns |
| `version_bump/patch`           | ~13 µs  |
| `version_bump/minor`           | ~14 µs  |
| `version_bump/major`           | ~6 µs   |
| `prerelease_bump/stage`        | ~15 µs  |
| `file_replace/100 lines`       | ~240 µs |
| `file_replace/1 000 lines`     | ~1.3 ms |
| `file_replace/10 000 lines`    | ~14 ms  |
| `file_replace/100 000 lines`   | ~140 ms |
| `worst_case/10 000 lines`      | ~14 ms  |
| `multiline_replace/100x`       | ~85 µs  |

## 📚 Further Reading

- [CLI.md](https://github.com/wiseaidev/bump2version/blob/main/CLI.md): Full CLI command dictionary
- [RUST.md](https://github.com/wiseaidev/bump2version/blob/main/RUST.md): Rust API guide
- [PYTHON.md](https://github.com/wiseaidev/bump2version/blob/main/PYTHON.md): Python bindings guide
- [NODE.md](https://github.com/wiseaidev/bump2version/blob/main/NODE.md): Node.js bindings guide
- [DOCKER.md](https://github.com/wiseaidev/bump2version/blob/main/DOCKER.md): Docker usage guide
- [WASM.md](https://github.com/wiseaidev/bump2version/blob/main/WASM.md): WebAssembly + Yew guide
- [PACKAGING.md](https://github.com/wiseaidev/bump2version/blob/main/PACKAGING.md): Debian/RPM packaging
- [examples/yew-app](https://github.com/wiseaidev/bump2version/tree/main/examples/yew-app): Browser-side version bumper (Yew + WASM)
- [Semantic Versioning 2.0.0](https://semver.org/)
- [bump-my-version](https://github.com/callowayproject/bump-my-version): the Python tool this crate is feature-parity with
- [gitoxide (gix)](https://github.com/GitoxideLabs/gitoxide): the pure-Rust git implementation

## 📄 License

Licensed under the [MIT License](https://github.com/wiseaidev/bump2version/blob/main/LICENSE).
