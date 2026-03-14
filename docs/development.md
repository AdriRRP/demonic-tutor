# Development Guidelines

## Quality Standard

All code must pass:
- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::perf -W clippy::cargo -W clippy::unwrap_used -W clippy::expect_used -W clippy::panic -W clippy::todo -W clippy::unimplemented -W clippy::unreachable -A clippy::multiple_crate_versions -D warnings`

Quick check: `./scripts/check-all.sh`

## Versioning

This project follows Semantic Versioning (MAJOR.MINOR.PATCH).

- **Version**: Check `Cargo.toml` for current version
- **Changelog**: See `CHANGELOG.md` for release history

When making changes:
- Do NOT modify version in `Cargo.toml` unless explicitly requested
- Do NOT manually update `CHANGELOG.md` - it is generated at release time
- Focus on implementing the feature correctly; release process handles versioning

## Dependencies

Use exact versions (major.minor.patch) in `Cargo.toml` to track precise versions:
```toml
rand = "0.10.0"  # NOT "0.10"
```

## Panic-Free Policy

Production code (`src/`) must not contain:
- `unwrap()`, `expect()`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`

Use `Result` for error handling.

Test code (`tests/`) may use `unwrap()` where appropriate.

## Style

### Imports

- Use compact/grouped imports to minimize code size:
  ```rust
  use crate::domain::{
      commands::{Cmd1, Cmd2, Cmd3},
      errors::DomainError,
      events::Event,
  };
  ```
- Only use fully qualified paths when they provide semantic distinction:
  - Appropriate: `std::io::Result`, `std::result::Result`
  - Avoid: `std::sync::Arc` unless disambiguating from another `Arc`

### Code

- Prefer explicit code over clever abstractions
- Keep domain core small and deterministic
- Introduce new concepts only for the active slice
- Avoid speculative infrastructure
