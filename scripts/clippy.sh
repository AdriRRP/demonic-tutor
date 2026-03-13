#!/bin/bash
# Strict clippy - runs all clippy lints

set -e

echo "Running strict clippy..."
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
