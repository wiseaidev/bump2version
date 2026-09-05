#!/usr/bin/env python3
# Copyright 2026 Mahmoud Harmouch.
#
# Licensed under the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

"""
Nano-benchmark: bump-rs (Rust) vs bump-my-version (Python) vs pure Python.

Measures timing with statistical filtering (minimum + 3-sigma outlier removal)
and reports time-per-operation alongside the standard deviation around the
minimum. Each benchmark runs for at least ``BENCH_TIME`` seconds.

Usage::

    python benchmarks/benchmark.py          # full run (~3 s per benchmark)
    python benchmarks/benchmark.py 1        # one-shot (faster, CI mode)

Requirements::

    pip install bump-rs bumpversion
"""

import re
import sys
import timeit
from math import sqrt
from time import time

try:
    import bump_rs as rust_lib

    RUST_AVAILABLE = True
except ImportError:
    rust_lib = None
    RUST_AVAILABLE = False

BMV_AVAILABLE = False
try:
    from bumpversion.config import get_configuration as _get_cfg
    from bumpversion.bump import get_next_version as _bmv_next
    from bumpversion.versioning.version_config import VersionConfig as _VC

    _bmv_cfg = _get_cfg()
    _bmv_vc = _VC(
        parse=_bmv_cfg.parse,
        serialize=tuple(_bmv_cfg.serialize),
        search=_bmv_cfg.search,
        replace=_bmv_cfg.replace,
        part_configs=_bmv_cfg.parts,
    )

    def _bmv_bump(version_str, part):
        parsed = _bmv_vc.parse(version_str)
        nxt = _bmv_next(parsed, _bmv_cfg, part, None)
        return _bmv_vc.serialize(nxt, {})

    BMV_AVAILABLE = True
except Exception as _e:
    _bmv_bump = None

_SEMVER = re.compile(r"(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)")


def _py_bump(version_str, part):
    m = _SEMVER.match(version_str)
    maj, mn, pat = int(m["major"]), int(m["minor"]), int(m["patch"])
    if part == "major":
        return f"{maj + 1}.0.0"
    if part == "minor":
        return f"{maj}.{mn + 1}.0"
    return f"{maj}.{mn}.{pat + 1}"



BENCH_TIME = 2.0
PRINT_TPL = (
    "Name: {name:<38} Library: {lib:<22} Time: {time:.2e} s;  Sigma: {sigma:.0e} s"
)


def mindev(data, xbar=None):
    """Standard deviation relative to the minimum of *data*."""
    if not data:
        raise ValueError("No data")
    if xbar is None:
        xbar = min(data)
    return sqrt(sum((x - xbar) ** 2 for x in data) / max(len(data) - 1, 1))


def autorange(stmt, globs=None, ratio=500, bench_time=BENCH_TIME, number=None):
    """Adaptive timing with 3-sigma outlier rejection."""
    if globs is None:
        globs = {}
    t = timeit.Timer(stmt=stmt, globals=globs)
    break_immediately = False

    if number is None:
        a = t.autorange()
        number = max(int(a[0] / ratio), 1)
        repeat = max(int(a[0] / number), 1)
    else:
        repeat = 1
        break_immediately = True

    data = list(t.repeat(number=number, repeat=repeat))
    bench_start = time()
    while True:
        data.extend(t.repeat(number=number, repeat=repeat))
        if break_immediately or time() - bench_start > bench_time:
            break

    data.sort()
    xbar = data[0]
    i = 0
    while i < len(data):
        i = len(data)
        sigma = mindev(data, xbar=xbar)
        for i2 in range(2, len(data)):
            if data[i2] - xbar > 3 * sigma:
                break
        k = max(i, 5)
        del data[k:]

    return min(data) / number, mindev(data, xbar=xbar) / number


def print_result(name, lib, bench_res):
    print(
        PRINT_TPL.format(
            name=f"`{name}`",
            lib=lib,
            time=bench_res[0],
            sigma=bench_res[1],
        )
    )


FILE_CONTENT_SIMPLE = 'version = "1.2.3"\n'
FILE_CONTENT_MULTILINE = (
    "## 1.2.3\nRelease notes line 1\nRelease notes line 2\n\n## 1.2.2\nOlder notes\n"
)


def run_benchmarks(number):
    sep = "#" * 80
    sep_minor = "/" * 80
    print(sep)
    print("# bump2version Nano-Benchmark Suite")
    print("# Libraries: bump-rs (Rust) | bump-my-version (Python) | Pure Python")
    print(sep)

    print(sep_minor)
    print("## Version bumping: parse + bump + serialize (full round-trip)")
    print(sep_minor)

    for part in ("patch", "minor", "major"):
        label = f"bump_version('1.2.3', '{part}')"

        if RUST_AVAILABLE:
            res = autorange(
                f"rust_lib.bump_version('1.2.3', '{part}')",
                globs={"rust_lib": rust_lib},
                number=number,
            )
            print_result(label, "bump-rs (Rust, cached)", res)

        if BMV_AVAILABLE:
            res = autorange(
                f"_bmv_bump('1.2.3', '{part}')",
                globs={"_bmv_bump": _bmv_bump},
                number=number,
            )
            print_result(label, "bump-my-version (Py)", res)

        res = autorange(
            f"_py_bump('1.2.3', '{part}')",
            globs={"_py_bump": _py_bump},
            number=number,
        )
        print_result(label, "Pure Python (baseline)", res)

        print()

    print(sep_minor)
    print("## File search/replace: single-line pattern")
    print(sep_minor)

    if RUST_AVAILABLE:
        res = autorange(
            "rust_lib.apply_file_change(c, '1.2.3', '1.2.4')",
            globs={"rust_lib": rust_lib, "c": FILE_CONTENT_SIMPLE},
            number=number,
        )
        print_result("apply_file_change (single-line)", "bump-rs (Rust, cached)", res)

    _LITERAL_RE = re.compile(re.escape('version = "1.2.3"'))

    def _py_replace_single(content, old, new):
        return _LITERAL_RE.sub(f'version = "{new}"', content, count=1)

    res = autorange(
        "_py_replace_single(c, '1.2.3', '1.2.4')",
        globs={"_py_replace_single": _py_replace_single, "c": FILE_CONTENT_SIMPLE},
        number=number,
    )
    print_result("apply_file_change (single-line)", "Pure Python re.sub", res)

    print()

    print(sep_minor)
    print("## File search/replace: multiline pattern (CHANGELOG style)")
    print(sep_minor)

    _ML_SEARCH = "## {current_version}\nRelease notes line 1\nRelease notes line 2"
    _ML_REPLACE = "## {new_version}\nRelease notes line 1\nRelease notes line 2"

    if RUST_AVAILABLE:
        res = autorange(
            "rust_lib.apply_file_change(c, '1.2.3', '1.2.4', search=s, replace=r)",
            globs={
                "rust_lib": rust_lib,
                "c": FILE_CONTENT_MULTILINE,
                "s": _ML_SEARCH,
                "r": _ML_REPLACE,
            },
            number=number,
        )
        print_result("apply_file_change (multiline)", "bump-rs (Rust, cached)", res)

    _ml_pattern = re.compile(
        re.escape("## 1.2.3\nRelease notes line 1\nRelease notes line 2"),
        re.MULTILINE | re.DOTALL,
    )
    _ml_repl = "## 1.2.4\nRelease notes line 1\nRelease notes line 2"

    def _py_replace_multi(content):
        return _ml_pattern.sub(_ml_repl, content, count=1)

    res = autorange(
        "_py_replace_multi(c)",
        globs={"_py_replace_multi": _py_replace_multi, "c": FILE_CONTENT_MULTILINE},
        number=number,
    )
    print_result("apply_file_change (multiline)", "Pure Python re.sub", res)

    print(sep)


if __name__ == "__main__":
    NUM = None
    ARGV = sys.argv
    LEN_ARGV = len(ARGV)
    MAX_POSITIONAL_ARGS = 1
    MAX_LEN_ARGV = MAX_POSITIONAL_ARGS + 1

    if LEN_ARGV > MAX_LEN_ARGV:
        raise ValueError(
            f"{__name__} must not accept more than "
            f"{MAX_POSITIONAL_ARGS} positional command-line parameters"
        )
    if LEN_ARGV == MAX_LEN_ARGV:
        NUM = int(ARGV[MAX_POSITIONAL_ARGS])

    run_benchmarks(NUM)
