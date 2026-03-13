#!/bin/bash
# Format check - verifies code formatting

set -e

echo "Checking formatting..."
cargo fmt --all -- --check
