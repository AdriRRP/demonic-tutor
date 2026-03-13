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
echo "=== Security Audit ==="
cargo audit

echo ""
echo "=== All checks passed ==="
