# Development Guidelines

## Quality standard

All code must pass:

- `cargo fmt --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::perf -W clippy::cargo -A clippy::multiple_crate_versions -D warnings`

## Style

- Prefer explicit code over clever abstractions.
- Keep the domain core small and deterministic.
- Introduce new concepts only when required by the active slice.
- Avoid speculative infrastructure.

## Slice policy

Each slice should be:
- small
- testable
- reviewable
- explicit about supported behavior and non-goals

## Panic-free production code policy

Code under `src/` must not contain:

- `unwrap()`
- `expect()`
- `panic!`
- `todo!`
- `unimplemented!`
- `unreachable!`

Production code must pass strict clippy checks enforcing this rule.

Test code under `tests/` may still use `unwrap()` where appropriate.
