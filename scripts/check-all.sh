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
echo "=== Web Frontend ==="
if ! command -v npm >/dev/null 2>&1; then
    echo "error: missing npm; install Node.js to run web checks" >&2
    exit 1
fi
if [ ! -d apps/web/node_modules ]; then
    echo "error: missing apps/web/node_modules; run 'cd apps/web && npm install'" >&2
    exit 1
fi
npm --prefix apps/web run format:check
npm --prefix apps/web run lint
npm --prefix apps/web run build
npm --prefix apps/web run audit
npm --prefix apps/web run deps:check

echo ""
echo "=== Security Audit ==="
cargo audit

echo ""
echo "=== Dependency Update Check ==="
./scripts/deps.sh

echo ""
echo "=== All checks passed ==="
