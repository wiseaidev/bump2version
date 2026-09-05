# Copyright 2026 Mahmoud Harmouch.
#
# Licensed under the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

"""
bump_rs: fast, Rust-powered version bumping for Python.

This package re-exports the native extension built from the `bump2version`
Rust crate via maturin. Import directly from ``bump_rs``:

.. code-block:: python

    from bump_rs import bump_version, apply_file_change, BumpConfig
"""

from bump_rs._bump_rs import (  # noqa: F401
    BumpConfig,
    apply_file_change,
    bump_version,
)

__all__ = ["BumpConfig", "apply_file_change", "bump_version"]
