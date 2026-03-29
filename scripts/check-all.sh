#!/bin/bash
# Check all - runs all quality checks

set -e

echo "=== Running all checks ==="

echo ""
echo "=== Formatting ==="
cargo fmt --all -- --check

echo ""
echo "=== Tests ==="
cargo test

echo ""
echo "=== Clippy (strict) ==="
cargo clippy --all-targets --all-features -- \
    -W clippy::all \
    -W clippy::pedantic \
    -W clippy::nursery \
    -W clippy::perf \
    -W clippy::cargo \
    -W clippy::unwrap_used \
    -W clippy::expect_used \
    -W clippy::panic \
    -W clippy::todo \
    -W clippy::unimplemented \
    -W clippy::unreachable \
    -A clippy::multiple_crate_versions \
    -D warnings

echo ""
echo "=== Wasm Compatibility ==="
if ! rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
    echo "error: missing rust target wasm32-unknown-unknown; run 'rustup target add wasm32-unknown-unknown'" >&2
    exit 1
fi
cargo check --target wasm32-unknown-unknown

echo ""
echo "=== Security Audit ==="
cargo audit

echo ""
echo "=== Dependency Update Check ==="
./scripts/deps.sh

echo ""
echo "=== All checks passed ==="
