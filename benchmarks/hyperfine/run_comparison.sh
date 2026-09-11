#!/usr/bin/env bash
# Copyright 2026 Mahmoud Harmouch.
#
# Licensed under the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FIXTURES_DIR="$SCRIPT_DIR/fixtures"
RESULTS_DIR="$SCRIPT_DIR/results"

mkdir -p "$FIXTURES_DIR" "$RESULTS_DIR"

cat > "$FIXTURES_DIR/.bumpversion.toml" <<'EOF'
[bumpversion]
current_version = 1.2.3
commit = false
tag = false

[bumpversion:file:fixtures/version.txt]
search = {current_version}
replace = {new_version}
EOF

echo "1.2.3" > "$FIXTURES_DIR/version.txt"

RUST_BIN="$PROJECT_ROOT/target/release/bump"
if [[ ! -f "$RUST_BIN" ]]; then
    echo "Building bump2version (release)..."
    cd "$PROJECT_ROOT"
    RUSTFLAGS="-C target-cpu=native" cargo build --release --features=rust-binary
fi

PYTHON_CMD=""
for cmd in bump-my-version bumpversion; do
    if command -v "$cmd" &>/dev/null; then
        PYTHON_CMD="$cmd"
        break
    fi
done

echo ""
echo "============================================="
echo "  bump2version Comparative Benchmark Suite"
echo "============================================="
echo ""

echo "--- Scenario 1: Rust CLI --dry-run (best case) ---"
hyperfine \
    --warmup 3 \
    --runs 20 \
    --export-json "$RESULTS_DIR/rust_dryrun.json" \
    --export-markdown "$RESULTS_DIR/rust_dryrun.md" \
    "$RUST_BIN \
        --config-file $FIXTURES_DIR/.bumpversion.toml \
        --current-version 1.2.3 \
        --bump patch \
        --dry-run"

if [[ -n "$PYTHON_CMD" ]]; then
    echo ""
    echo "--- Scenario 2: Python CLI ($PYTHON_CMD) --dry-run ---"
    hyperfine \
        --warmup 3 \
        --runs 20 \
        --export-json "$RESULTS_DIR/python_dryrun.json" \
        --export-markdown "$RESULTS_DIR/python_dryrun.md" \
        "$PYTHON_CMD \
            --config-file $FIXTURES_DIR/.bumpversion.toml \
            --current-version 1.2.3 \
            --dry-run \
            bump patch"
fi

echo ""
echo "--- Scenario 3: Rust CLI with 10-file config ---"
BIG_CFG="$FIXTURES_DIR/.bumpversion_big.toml"
cat > "$BIG_CFG" <<'EOF'
[bumpversion]
current_version = 1.2.3
commit = false
tag = false
EOF
for i in $(seq 1 10); do
    echo -e "\n[bumpversion:file:fixtures/file${i}.txt]\nsearch = {current_version}\nreplace = {new_version}" >> "$BIG_CFG"
    echo "1.2.3" > "$FIXTURES_DIR/file${i}.txt"
done

hyperfine \
    --warmup 3 \
    --runs 20 \
    --export-json "$RESULTS_DIR/rust_10files.json" \
    --export-markdown "$RESULTS_DIR/rust_10files.md" \
    "$RUST_BIN \
        --config-file $BIG_CFG \
        --current-version 1.2.3 \
        --bump patch \
        --dry-run"

echo ""
echo "--- Results Summary ---"
echo ""
echo "Rust CLI dry-run:"
cat "$RESULTS_DIR/rust_dryrun.md" 2>/dev/null || echo "(no results)"

if [[ -n "$PYTHON_CMD" ]]; then
    echo ""
    echo "Python CLI dry-run:"
    cat "$RESULTS_DIR/python_dryrun.md" 2>/dev/null || echo "(no results)"
fi

echo ""
echo "Benchmark data saved to: $RESULTS_DIR/"
