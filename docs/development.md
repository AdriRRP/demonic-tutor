# Development Guidelines

## Quality Standard

All code must pass:
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::perf -W clippy::cargo -W clippy::unwrap_used -W clippy::expect_used -W clippy::panic -W clippy::todo -W clippy::unimplemented -W clippy::unreachable -A clippy::multiple_crate_versions -D warnings`

Quick check: `./scripts/check-all.sh`

## Panic-Free Policy

Production code (`src/`) must not contain:
- `unwrap()`, `expect()`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`

Use `Result` for error handling.

Test code (`tests/`) may use `unwrap()` where appropriate.

## Style

- Prefer explicit code over clever abstractions
- Keep domain core small and deterministic
- Introduce new concepts only for the active slice
- Avoid speculative infrastructure
