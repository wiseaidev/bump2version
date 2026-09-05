<div align="center">

# 🟩 bump2version Node.js Documentation

[![bump2version logo](https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/logo.png)](https://github.com/wiseaidev/bump2version)

[![npm](https://img.shields.io/npm/v/bump2version.svg)](https://www.npmjs.com/package/bump2version)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/wiseaidev/bump2version/blob/main/LICENSE)

</div>

The **`bump2version`** module provides fast, native version bumping for Node.js
via [napi-rs](https://napi.rs). All functions are **synchronous**, no Promises required.

## 📦 Installation

```sh
npm install bump2version
```

Build locally:

```sh
git clone https://github.com/wiseaidev/bump2version.git
cd bump2version
npm install -g @napi-rs/cli
napi build --platform --release --features node
```

## 🛠 Usage

### Bump a version string

```javascript
// If installed via npm: const { bumpVersion } = require('bump2version');
// For local development:
const { bumpVersion } = require(".");

console.log(bumpVersion("1.2.3", "patch")); // '1.2.4'
console.log(bumpVersion("1.2.3", "minor")); // '1.3.0'
console.log(bumpVersion("1.2.3", "major")); // '2.0.0'
```

### Load config from file

```javascript
// If installed via npm: const { bumpVersion } = require('bump2version');
// For local development:
const { bumpVersion } = require(".");

const next = bumpVersion("1.2.3", "patch", ".bumpversion.toml");
console.log(next); // '1.2.4-stable.0'
```

### Apply search/replace to file content

```javascript
// If installed via npm: const { applyFileChange } = require('bump2version');
// For local development:
const { applyFileChange } = require(".");

const updated = applyFileChange('version = "1.0.0"\n', "1.0.0", "1.0.1");
console.log(updated); // 'version = "1.0.1"\n'
```

### Custom search/replace (multiline CHANGELOG)

```javascript
// If installed via npm: const { applyFileChange } = require('bump2version');
// For local development:
const { applyFileChange } = require(".");

const content = "## [1.0.0] - unreleased\nnotes\n";
const updated = applyFileChange(
  content,
  "1.0.0",
  "1.0.1",
  "## [{current_version}] - unreleased\nnotes",
  "## [{new_version}] - unreleased\nnotes",
);
console.log(updated);
// ## [1.0.1] - unreleased
// notes
```

### Ignore missing version pattern

```javascript
// If installed via npm: const { applyFileChange } = require('bump2version');
// For local development:
const { applyFileChange } = require(".");

const updated = applyFileChange(
  "no version here\n",
  "1.0.0",
  "1.0.1",
  null,
  null,
  true, // ignoreMissing = true
);
console.log(updated); // 'no version here\n'  (unchanged, no error)
```

## 📖 API Reference

### `bumpVersion(currentVersion, part, configPath?)`

| Parameter        | Type                  | Required | Description                                           |
| ---------------- | --------------------- | -------- | ----------------------------------------------------- |
| `currentVersion` | `string`              | ✅       | The current version string (e.g. `"1.2.3"`)           |
| `part`           | `string`              | ✅       | Component to bump: `"major"`, `"minor"`, or `"patch"` |
| `configPath`     | `string \| undefined` | ❌       | Path to `.bumpversion.toml`                           |

Returns `string`. Throws synchronously on error.

### `applyFileChange(content, currentVersion, newVersion, search?, replace?, ignoreMissing?)`

| Parameter        | Type             | Required | Description                                                         |
| ---------------- | ---------------- | -------- | ------------------------------------------------------------------- |
| `content`        | `string`         | ✅       | Full text content of the file                                       |
| `currentVersion` | `string`         | ✅       | Version string to search for                                        |
| `newVersion`     | `string`         | ✅       | Version string to insert                                            |
| `search`         | `string \| null` | ❌       | Search template (default: `"{current_version}"`)                    |
| `replace`        | `string \| null` | ❌       | Replace template (default: `"{new_version}"`)                       |
| `ignoreMissing`  | `boolean`        | ❌       | Return content unchanged instead of throwing when pattern not found |

Returns `string`. Throws synchronously on error (unless `ignoreMissing` is `true`).

## 📄 License

Licensed under the [MIT License](https://github.com/wiseaidev/bump2version/blob/main/LICENSE).
