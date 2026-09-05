// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Python Bindings
//!
//! Exposes the `bump2version` library to Python via [`pyo3`].
//! All types and functions are gated behind the `python` Cargo feature.
//!
//! ## Installation
//!
//! Build with [maturin](https://github.com/PyO3/maturin):
//!
//! ```sh
//! pip install maturin
//! maturin develop --features python
//! ```
//!
//! Or install the pre-built wheel:
//!
//! ```sh
//! pip install bump-rs
//! ```
//!
//! ## Usage
//!
//! ```python
//! from bump_rs import bump_version, BumpConfig
//!
//! # Bump the patch component of "1.2.3"
//! new_ver = bump_version("1.2.3", "patch")
//! print(new_ver)  # "1.2.4"
//!
//! # Bump with custom config
//! cfg = BumpConfig(parse=r"(?P<major>\d+)\.(?P<minor>\d+)", serialize="{major}.{minor}")
//! new_ver = bump_version("2.0", "minor", config=cfg)
//! print(new_ver)  # "2.1"
//! ```
//!
//! ## See Also
//!
//! - [`crate::version::bump_version`]
//! - [`crate::config::BumpConfig`]
//! - [PyO3 documentation](https://pyo3.rs)

use crate::config::{self, FileConfig as RustFileConfig};
use crate::error::BumpError;
use crate::files::apply_file_change;
use crate::version::{bump_version, parse_version, serialize_version};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

/// Python-facing version of [`crate::config::BumpConfig`].
///
/// Provides the core configuration for parsing and bumping version strings.
///
/// Examples:
///
/// ```python
/// >>> from bump_rs import BumpConfig
/// >>> cfg = BumpConfig()
/// >>> cfg.parse
/// '(?P<major>\\d+)\\.(?P<minor>\\d+)\\.(?P<patch>\\d+)'
/// ```
#[pyclass(name = "BumpConfig")]
pub struct PyBumpConfig {
    inner: config::BumpConfig,
}

#[pymethods]
impl PyBumpConfig {
    /// Create a new :class:`BumpConfig` with default settings.
    ///
    /// Args:
    ///     parse:     Optional regex with named groups (default: standard semver).
    ///     serialize: Optional format string (default: ``"{major}.{minor}.{patch}"``).
    ///     search:    Optional search pattern (default: ``"{current_version}"``).
    ///     replace:   Optional replace pattern (default: ``"{new_version}"``).
    #[new]
    #[pyo3(signature = (parse=None, serialize=None, search=None, replace=None))]
    pub fn new(
        parse: Option<String>,
        serialize: Option<String>,
        search: Option<String>,
        replace: Option<String>,
    ) -> Self {
        let mut inner = config::BumpConfig::default();
        if let Some(p) = parse {
            inner.parse = p;
        }
        if let Some(s) = serialize {
            inner.serialize = vec![s];
        }
        if let Some(s) = search {
            inner.search = s;
        }
        if let Some(r) = replace {
            inner.replace = r;
        }
        Self { inner }
    }

    /// The parse regex string.
    #[getter]
    pub fn parse(&self) -> &str {
        &self.inner.parse
    }

    /// The first serialise format string.
    #[getter]
    pub fn serialize(&self) -> &str {
        self.inner
            .serialize
            .first()
            .map(String::as_str)
            .unwrap_or("")
    }

    /// The search pattern template.
    #[getter]
    pub fn search(&self) -> &str {
        &self.inner.search
    }

    /// The replace pattern template.
    #[getter]
    pub fn replace(&self) -> &str {
        &self.inner.replace
    }

    pub fn __repr__(&self) -> String {
        format!(
            "BumpConfig(parse={:?}, serialize={:?})",
            self.inner.parse,
            self.inner.serialize.first().unwrap_or(&String::new())
        )
    }
}

/// Bumps a version string by the specified component.
///
/// Args:
///     current_version: The current version string (e.g. ``"1.2.3"``).
///     part:            The component to increment (``"major"``, ``"minor"``,
///                      or ``"patch"``).
///     config:          Optional :class:`BumpConfig` supplying the parse regex
///                      and serialise format. Uses the default semver config
///                      when omitted.
///     config_path:     Optional path to a ``.bumpversion.toml`` file. When
///                      supplied, it is parsed and merged with ``config``.
///
/// Returns:
///     The new version string.
///
/// Raises:
///     ValueError:  If ``part`` is not a known version component.
///     RuntimeError: If the parse regex does not match ``current_version``.
///
/// Examples:
///
/// ```python
/// >>> from bump_rs import bump_version
/// >>> bump_version("1.2.3", "patch")
/// '1.2.4'
/// >>> bump_version("1.2.3", "minor")
/// '1.3.0'
/// >>> bump_version("1.2.3", "major")
/// '2.0.0'
/// ```
#[pyfunction]
#[pyo3(name = "bump_version", signature = (current_version, part, config=None, config_path=None))]
pub fn bump_version_py(
    current_version: &str,
    part: &str,
    config: Option<&PyBumpConfig>,
    config_path: Option<&str>,
) -> PyResult<String> {
    let mut cfg = if let Some(path) = config_path {
        config::parse_config_file(path).map_err(|e| PyRuntimeError::new_err(e.to_string()))?
    } else {
        config::BumpConfig::default()
    };

    if let Some(py_cfg) = config {
        cfg.parse = py_cfg.inner.parse.clone();
        cfg.serialize = py_cfg.inner.serialize.clone();
        cfg.search = py_cfg.inner.search.clone();
        cfg.replace = py_cfg.inner.replace.clone();
    }

    let version =
        parse_version(current_version, &cfg).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    let bumped = bump_version(&version, part, &cfg).map_err(|e| match e {
        BumpError::UnknownComponent(c) => PyValueError::new_err(format!("Unknown component: {c}")),
        other => PyRuntimeError::new_err(other.to_string()),
    })?;

    Ok(serialize_version(&bumped, &cfg))
}

/// Applies search-and-replace to the text content of a file and returns the
/// updated content.
///
/// Args:
///     content:         The full text content of the file.
///     current_version: The version string to search for.
///     new_version:     The version string to replace with.
///     search:          Optional search pattern override (default:
///                      ``"{current_version}"``).
///     replace:         Optional replace pattern override (default:
///                      ``"{new_version}"``).
///     ignore_missing:  When ``True``, return ``content`` unchanged rather
///                      than raising if the pattern is not found.
///
/// Returns:
///     The updated file content.
///
/// Raises:
///     RuntimeError: If the pattern is not found and ``ignore_missing`` is
///                   ``False``.
///
/// Examples:
///
/// ```python
/// >>> from bump_rs import apply_file_change
/// >>> apply_file_change("version = 1.0.0\n", "1.0.0", "1.0.1")
/// 'version = 1.0.1\n'
/// ```
#[pyfunction]
#[pyo3(name = "apply_file_change", signature = (content, current_version, new_version, search=None, replace=None, ignore_missing=false))]
pub fn apply_file_change_py(
    content: &str,
    current_version: &str,
    new_version: &str,
    search: Option<&str>,
    replace: Option<&str>,
    ignore_missing: bool,
) -> PyResult<String> {
    let cfg = config::BumpConfig::default();
    let mut fc = RustFileConfig::new("<python-caller>");
    fc.ignore_missing_version = ignore_missing;
    if let Some(s) = search {
        fc.search = Some(s.to_string());
    }
    if let Some(r) = replace {
        fc.replace = Some(r.to_string());
    }

    apply_file_change(content, &fc, &cfg, current_version, new_version)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

/// Register all Python-exposed types and functions into the `_bump_rs`
/// module.
///
/// Called from the `#[pymodule]` entry point in `lib.rs`.
pub fn register_python_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBumpConfig>()?;
    m.add_function(wrap_pyfunction!(bump_version_py, m)?)?;
    m.add_function(wrap_pyfunction!(apply_file_change_py, m)?)?;
    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
