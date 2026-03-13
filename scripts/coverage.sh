#!/bin/bash
# Coverage script - generates coverage report

set -e

echo "Generating coverage report..."
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
echo "Coverage report generated: lcov.info"
