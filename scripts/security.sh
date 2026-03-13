#!/bin/bash
# Security audit - checks for vulnerable dependencies

set -e

echo "Running security audit..."
cargo audit
