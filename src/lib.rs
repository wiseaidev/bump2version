// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), doc = "")]
#![cfg_attr(feature = "std", doc = include_str!("../README.md"))]
#![cfg_attr(feature = "std", doc = include_str!("../RUST.md"))]
#![cfg_attr(feature = "std", doc = include_str!("../WASM.md"))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/wiseaidev/bump2version/refs/heads/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "node"), forbid(unsafe_code))]

extern crate alloc;

pub mod config;
pub mod error;
pub mod files;
pub mod version;

#[cfg(feature = "git")]
pub mod git;

#[cfg(feature = "std")]
pub mod utils;

#[cfg(all(feature = "cli", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "cli", feature = "std"))))]
pub mod cli;

#[cfg(all(feature = "cli", feature = "std", feature = "watch"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "cli", feature = "std", feature = "watch")))
)]
pub mod watch;

#[cfg(all(feature = "cli", feature = "std", feature = "detect"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "cli", feature = "std", feature = "detect")))
)]
pub mod detect;

#[cfg(feature = "std")]
pub mod workspace;

#[cfg(all(feature = "python", not(feature = "node"), feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "python", feature = "std"))))]
pub mod python;

#[cfg(all(feature = "node", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "node", feature = "std"))))]
pub mod node;

#[cfg(all(feature = "python", not(feature = "node"), feature = "std"))]
use pyo3::prelude::*;

#[cfg(all(feature = "python", not(feature = "node"), feature = "std"))]
#[pymodule]
fn _bump_rs(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    crate::python::register_python_module(py, m)?;
    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
