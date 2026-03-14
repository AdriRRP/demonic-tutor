#!/bin/bash
# Dependency update check - verifies all direct dependencies are up to date

set -e

echo "=== Checking for outdated dependencies ==="

# Check only root dependencies (depth=1) to avoid noise from transitive deps
cargo outdated --depth 1 --exit-code 1 || {
    echo ""
    echo "ERROR: There are outdated direct dependencies. Please update them before merging."
    echo "Run 'cargo outdated --depth 1' to see available updates."
    exit 1
}

echo ""
echo "=== All direct dependencies are up to date ==="
