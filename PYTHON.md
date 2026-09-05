<div align="center">

# 🐍 bump-rs Python Documentation

[![bump2version logo](https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/logo.png)](https://github.com/wiseaidev/bump2version)

[![PyPI](https://img.shields.io/pypi/v/bump-rs.svg)](https://pypi.org/project/bump-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/wiseaidev/bump2version/blob/main/LICENSE)

</div>

The **`bump-rs`** package provides blazing-fast version bumping for Python,
powered by a 100% safe Rust core. All functions are **synchronous** - no `asyncio` required.

## 📦 Installation

```sh
pip install bump-rs
```

Build locally (requires [maturin](https://github.com/PyO3/maturin)):

```sh
git clone https://github.com/wiseaidev/bump2version.git
cd bump2version
python3 -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop --features python
```

## 🛠 Usage

### Bump a version string

```python
from bump_rs import bump_version

print(bump_version("1.2.3", "patch"))   # "1.2.4"
print(bump_version("1.2.3", "minor"))   # "1.3.0"
print(bump_version("1.2.3", "major"))   # "2.0.0"
```

### Load config from file

```python
from bump_rs import bump_version

# reads parse/serialize/parts from your .bumpversion.toml
new_ver = bump_version("1.2.3", "patch", config_path=".bumpversion.toml")
print(new_ver)  # e.g. "1.2.4"
```

### Custom `BumpConfig`

```python
from bump_rs import bump_version, BumpConfig

cfg = BumpConfig(
    parse=r"(?P<major>\d+)\.(?P<minor>\d+)",
    serialize="{major}.{minor}",
)
print(bump_version("2.0", "minor", config=cfg))  # "2.1"
```

### Apply search/replace to file content

```python
from bump_rs import apply_file_change

content = 'version = "1.0.0"\n'
print(apply_file_change(content, "1.0.0", "1.0.1"))
# 'version = "1.0.1"\n'
```

### Multiline CHANGELOG pattern

```python
from bump_rs import apply_file_change

content = "## 1.0.0\nRelease notes\n\n## 0.9.0\nOld notes\n"
updated = apply_file_change(
    content,
    current_version="1.0.0",
    new_version="1.0.1",
    search="## {current_version}\nRelease notes",
    replace="## {new_version}\nRelease notes",
)
print(updated)
# ## 1.0.1
# Release notes
#
# ## 0.9.0
# Old notes
```

### Ignore missing pattern

```python
from bump_rs import apply_file_change

updated = apply_file_change(
    "no version here\n",
    "1.0.0",
    "1.0.1",
    ignore_missing=True,
)
print(updated)  # "no version here\n"  (unchanged, no error)
```

## 📖 API Reference

### Functions

| Function                                                                                                       | Returns | Description                       |
| -------------------------------------------------------------------------------------------------------------- | ------- | --------------------------------- |
| `bump_version(current_version, part, *, config=None, config_path=None)`                                        | `str`   | Compute the next version string   |
| `apply_file_change(content, current_version, new_version, *, search=None, replace=None, ignore_missing=False)` | `str`   | Apply version replacement to text |

### `BumpConfig` class

| Parameter   | Type          | Description                                          |
| ----------- | ------------- | ---------------------------------------------------- |
| `parse`     | `str \| None` | Regex with named groups (default: semver)            |
| `serialize` | `str \| None` | Format string (default: `"{major}.{minor}.{patch}"`) |
| `search`    | `str \| None` | Search template (default: `"{current_version}"`)     |
| `replace`   | `str \| None` | Replace template (default: `"{new_version}"`)        |

| Property     | Type  | Description            |
| ------------ | ----- | ---------------------- |
| `.parse`     | `str` | Active parse regex     |
| `.serialize` | `str` | First serialize format |
| `.search`    | `str` | Search template        |
| `.replace`   | `str` | Replace template       |

## 📊 Running Benchmarks

```sh
pip install bump-rs bumpversion
python benchmarks/benchmark.py
```

See the full comparison table in the
[README](https://github.com/wiseaidev/bump2version/blob/main/README.md#-benchmarks).

## 📄 License

Licensed under the [MIT License](https://github.com/wiseaidev/bump2version/blob/main/LICENSE).
