<div align="center">

# ⬆️ Bump2version

[![bump2version logo](https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/logo.png)](https://github.com/wiseaidev/bump2version)

[![Crates.io](https://img.shields.io/crates/v/bump2version.svg)](https://crates.io/crates/bump2version)
[![Docs.rs](https://docs.rs/bump2version/badge.svg)](https://docs.rs/bump2version)
[![PyPI](https://img.shields.io/pypi/v/bump-rs.svg)](https://pypi.org/project/bump-rs)
[![npm](https://img.shields.io/npm/v/bump2version.svg)](https://www.npmjs.com/package/bump2version)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/wiseaidev/bump2version/blob/main/LICENSE)

> `bump2version` is a multi-language version bumper written entirely in **100% safe Rust**, with `no_std` support and native Python and Node.js bindings 🗿.

|                    🦀 Rust                    |                                    🐍 Python                                    |                                 🟩 Node.js                                  |
| :-------------------------------------------: | :-----------------------------------------------------------------------------: | :-------------------------------------------------------------------------: |
|           `cargo add bump2version`            |                              `pip install bump-rs`                              |                         `npm install bump2version`                          |
| [Documentation](https://docs.rs/bump2version) | [Read PYTHON.md](https://github.com/wiseaidev/bump2version/blob/main/PYTHON.md) | [Read NODE.md](https://github.com/wiseaidev/bump2version/blob/main/NODE.md) |

[![bump2version banner](https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/banner.png)](https://github.com/wiseaidev/bump2version)

</div>

## 🤔 What does this crate provide?

`bump2version` automates semantic version management for any project regardless of language. It:

- **Parses** version strings using a fully configurable regex (default: semver `major.minor.patch`).
- **Bumps** any named component (`major`, `minor`, `patch`, or custom cyclic stages).
- **Rewrites** version occurrences across multiple files, including multiline CHANGELOG patterns, using `(?ms)` DOTALL + MULTILINE semantics identical to Python's `re.MULTILINE | re.DOTALL`.
- **Commits and tags** via 100% pure `gix` (gitoxide); zero subprocess calls, zero `web-flow` ghost-author bugs.
- **Reads** author identity from the local git config.

## 🦀 Rust

The Rust crate is available on [crates.io](https://crates.io/crates/bump2version).
For a complete API reference, installation guide, and worked examples, visit the
**[Rust Usage Guide](https://github.com/wiseaidev/bump2version/blob/main/RUST.md)**.

The crate ships the following Cargo features:

| Feature  | Default | Description                                   |
| -------- | ------- | --------------------------------------------- |
| `std`    | ✅      | File I/O, git integration, regex stdlib cache |
| `cli`    | ❌      | Standalone `bump2version` binary via `clap`   |
| `python` | ❌      | Python extension module via PyO3/maturin      |
| `node`   | ❌      | Node.js native add-on via napi-rs             |

### Quick Start

```toml
[dependencies]
bump2version = "0.2.0"
```

```rust
use bump2version::{config::BumpConfig, version::{parse_version, bump_version, serialize_version}};

fn main() {
    let cfg = BumpConfig::default();
    let v   = parse_version("1.2.3", &cfg).unwrap();
    let v2  = bump_version(&v, "patch", &cfg).unwrap();
    println!("{}", serialize_version(&v2, &cfg)); // 1.2.4
}
```

### `no_std` Support

Core modules (`config`, `version`, `files`, `error`) compile in `no_std + alloc`:

```toml
# no_std (alloc required by the target):
bump2version = { version = "0.2.0", default-features = false }
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

The Python bindings are published to PyPI as **`bump-rs`** and can be installed with `pip install bump-rs`.
Built with [maturin](https://www.maturin.rs/), pre-compiled wheels for CPython 3.12+.

<!-- absolute url for docs.rs because PYTHON.md is not bundled in the crate -->

For installation instructions, full method signatures, and examples, read the
**[Python Usage Guide](https://github.com/wiseaidev/bump2version/blob/main/PYTHON.md)**.

```sh
pip install bump-rs
```

```python
from bump_rs import bump_version, apply_file_change, BumpConfig

print(bump_version("1.2.3", "patch"))   # "1.2.4"
print(bump_version("1.2.3", "minor"))   # "1.3.0"
print(bump_version("1.2.3", "major"))   # "2.0.0"

content = 'version = "1.0.0"\n'
print(apply_file_change(content, "1.0.0", "1.0.1"))
# 'version = "1.0.1"\n'

# Multiline CHANGELOG pattern
content = "## 1.0.0\nRelease notes\n\n## 0.9.0\nOld notes\n"
updated = apply_file_change(
    content,
    current_version="1.0.0",
    new_version="1.0.1",
    search="## {current_version}\nRelease notes",
    replace="## {new_version}\nRelease notes",
)
print(updated)  # "## 1.0.1\nRelease notes\n\n## 0.9.0\nOld notes\n"

# Custom parse/serialize config
cfg = BumpConfig(parse=r"(?P<major>\d+)\.(?P<minor>\d+)", serialize="{major}.{minor}")
print(bump_version("2.0", "minor", config=cfg))  # "2.1"
```

## 🟩 Node.js

The Node.js bindings are published to npm as **`bump2version`** and can be installed with `npm install bump2version`.
Built with [napi-rs](https://napi.rs/), pre-compiled `.node` add-on.

<!-- absolute url for docs.rs because NODE.md is not bundled in the crate -->

For installation instructions, TypeScript type definitions, and examples, read the
**[Node.js Usage Guide](https://github.com/wiseaidev/bump2version/blob/main/NODE.md)**.

```sh
npm install bump2version
```

```javascript
const { bumpVersion, applyFileChange } = require("bump2version");

console.log(bumpVersion("1.2.3", "patch")); // '1.2.4'
console.log(bumpVersion("1.2.3", "minor")); // '1.3.0'
```

## 💻 Command-line interface

```sh
cargo install bump2version --features rust-binary

bump2version --bump patch   # 1.0.0 → 1.0.1
bump2version --bump minor   # 1.0.0 → 1.1.0
bump2version --bump major   # 1.0.0 → 2.0.0
```

| Option               | Description                                     |
| -------------------- | ----------------------------------------------- |
| `--config-file`      | Config file path (default: `.bumpversion.toml`) |
| `--current-version`  | Override current version                        |
| `--bump`             | Component: `major`, `minor`, `patch`            |
| `--parse`            | Parse regex override                            |
| `--serialize`        | Serialize format override                       |
| `--dry-run` / `-n`   | Simulate without writing files                  |
| `--new-version`      | Explicit new version                            |
| `--commit` / `--tag` | Git commit + tag                                |

## ⚙️ Configuration File (`.bumpversion.toml`)

```toml
[bumpversion]
current_version = "1.0.0"
commit = true
tag = true

[bumpversion:file:Cargo.toml]
search = 'version = "{current_version}"'
replace = 'version = "{new_version}"'

[bumpversion:file:CHANGELOG.md]
search = """
## {current_version}
    Release notes line 1
    Release notes line 2"""
replace = """
## {new_version}
    Release notes line 1
    Release notes line 2"""
```

## 🔒 Safety

This crate enforces a zero-unsafe policy via `#![forbid(unsafe_code)]` at the crate root (except the Node.js FFI layer which requires `unsafe` for napi-rs interop). Every byte of the implementation, config parsing, regex matching, version bumping, git object creation, is written in safe Rust. The compiler will reject any future `unsafe` block introduced into the safe portions.

## 📊 Benchmarks

### Rust (`cargo bench`)

Run with `cargo bench`. Results on x86-64 Linux (rustc stable):

<details>
<summary><code>cargo bench</code> output</summary>

| **Benchmark**                   | **Time** |
| ------------------------------- | -------- |
| `config_parse/minimal`          | ~12 µs   |
| `config_parse/full_with_parts`  | ~18 µs   |
| `version_parse/1.0.0`           | ~439 µs  |
| `version_bump/patch`            | ~13.5 µs |
| `version_bump/minor`            | ~14.1 µs |
| `version_bump/major`            | ~6.0 µs  |
| `file_replace/100 lines`        | ~247 µs  |
| `file_replace/1 000 lines`      | ~1.35 ms |
| `file_replace/10 000 lines`     | ~14.4 ms |
| `multiline_replace` (CHANGELOG) | ~87 µs   |

</details>

### Python Nano-Benchmarks (`benchmarks/benchmark.py`)

Times measured via 3-sigma filtered `timeit` (CPython 3.12, x86-64 Linux). Run with:

```sh
pip install bump-rs bumpversion
python benchmarks/benchmark.py
```

#### Version Bumping: full round-trip (parse + bump + serialize)

| **Library**                            | **patch**  | **minor**  | **major**  |
| -------------------------------------- | ---------- | ---------- | ---------- |
| **bump-rs** (Rust, `Arc<Regex>` cache) | **~57 µs** | **~54 µs** | **~53 µs** |
| `bump-my-version` (Python library)     | ~79 µs     | ~95 µs     | ~72 µs     |
| Pure Python (`re.compile` + `int()`)   | ~3.6 µs    | ~2.2 µs    | ~2.2 µs    |
| `bump-my-version` CLI (subprocess)     | ~585 ms    | ~585 ms    | ~585 ms    |

bump-rs is **1.4-1.8× faster** than `bump-my-version`'s Python library and **~10 000× faster** than the CLI.

#### File Search/Replace

| **Library**                | **Single-line** | **Multiline CHANGELOG** |
| -------------------------- | --------------- | ----------------------- |
| **bump-rs** (Rust, cached) | **~65 µs**      | **~104 µs**             |
| Pure Python `re.sub`       | ~1.7 µs         | ~1.3 µs                 |

**When bump-rs wins:**

- **vs bump-my-version library**: 1.4-1.8× faster version bumping, correct multiline pattern semantics.
- **vs bump-my-version CLI**: ~10 000× faster, no subprocess startup.
- **Thread safety**: `#![forbid(unsafe_code)]` + no GIL constraint → scales across threads.
- **Full pipeline**: config + bump + git commit entirely in safe Rust.

**When pure Python wins:**

- Single in-memory arithmetic on a tiny string where ~50 µs PyO3 FFI overhead dominates: use `bump_rs` in batch or for full-pipeline work.

## 📚 Further Reading

- [Semantic Versioning 2.0.0](https://semver.org/): the canonical version scheme.
- [bump-my-version](https://github.com/callowayproject/bump-my-version): the Python tool this crate is feature-parity with.
- [gitoxide (gix)](http://github.com/GitoxideLabs/gitoxide): the pure-Rust git implementation powering our git integration.
- [PyO3](https://pyo3.rs/): Rust ↔ Python FFI framework.
- [napi-rs](https://napi.rs/): Rust ↔ Node.js FFI framework.

## 📄 License

Licensed under the [MIT License](https://github.com/wiseaidev/bump2version/blob/main/LICENSE).
