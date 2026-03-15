# Development Guidelines — DemonicTutor

This document defines the practical development rules for working in the repository.

It complements:

- `PROJECT.md` (vision)
- `CONSTRAINTS.md` (system limits)
- `docs/architecture/` (system design)

These guidelines focus on **code quality, development workflow and implementation discipline**.

---

# Quality Standard

All code must pass the project's quality checks.

Required checks:

- `cargo fmt --check`
- `cargo test`
- `cargo clippy`

Full clippy configuration:

```

cargo clippy --all-targets --all-features -- 
-W clippy::all 
-W clippy::pedantic 
-W clippy::nursery 
-W clippy::perf 
-W clippy::cargo 
-W clippy::unwrap_used 
-W clippy::expect_used 
-W clippy::panic 
-W clippy::todo 
-W clippy::unimplemented 
-W clippy::unreachable 
-A clippy::multiple_crate_versions 
-D warnings

```

Quick validation:

```

./scripts/check-all.sh

```

Code that fails these checks must not be merged.

---

# Versioning

The project follows **Semantic Versioning**:

```
MAJOR.MINOR.PATCH

```

Release management is automated.

When contributing:

- **Do not modify the version** in `Cargo.toml` unless explicitly requested.
- **Do not manually update `CHANGELOG.md`**.
- Focus on implementing the change correctly.

The release process will handle versioning and changelog updates.

---

# Dependencies

Dependencies should remain predictable and explicit.

Rules:

- Prefer **exact versions** in `Cargo.toml`.

Example:

```toml
rand = "0.10.0"
````

Avoid:

```toml
rand = "0.10"
```

Dependencies should only be introduced when they clearly simplify implementation.

Avoid unnecessary libraries.

---

# Error Handling

Production code must avoid panics.

The following must not appear in `src/`:

* `unwrap()`
* `expect()`
* `panic!`
* `todo!`
* `unimplemented!`
* `unreachable!`

Use `Result` and explicit error types.

Test code may use `unwrap()` when appropriate.

---

# Architectural Discipline

Code must respect the architecture defined in `docs/architecture/`.

In particular:

* the domain core must remain **deterministic**
* the domain core must not depend on **UI, storage or network concerns**
* aggregates enforce **domain invariants**
* infrastructure must remain **separate from domain logic**

When in doubt, prefer **clear domain modeling over technical shortcuts**.

---

# Vertical Slice Development

The project evolves through **small vertical slices**.

Each slice should:

* introduce a coherent domain behavior
* remain minimal and reviewable
* include tests for observable behavior
* update documentation when necessary

Avoid introducing large frameworks or infrastructure before they are required by a slice.

---

# Code Style

## Imports

Use grouped imports to keep modules readable.

Example:

```rust
use crate::domain::{
    commands::{Cmd1, Cmd2},
    errors::DomainError,
    events::Event,
};
```

Fully qualified paths should only be used when they improve clarity.

---

## Implementation Style

Prefer:

* explicit code
* deterministic logic
* simple control flow

Avoid:

* clever abstractions
* speculative infrastructure
* unnecessary generalization

New concepts should only be introduced when required by the active slice.

---

# Testing

Important domain behavior must be testable in isolation.

Tests should:

* validate observable behavior
* remain focused and readable
* avoid testing implementation details

Prefer small tests tied to domain behaviors over large integration scaffolding.

BDD-style tests may be introduced when gameplay flows become complex.

---

# Development Philosophy

The repository should evolve incrementally.

Prefer:

* small, reviewable changes
* narrow vertical slices
* explicit modeling decisions

Avoid:

* speculative architecture
* premature abstraction
* large refactors without clear benefit
