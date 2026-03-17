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
* the public domain model should prefer **canonical game semantics** over convenience APIs
* duplicate commands or events representing the same real-world concept should be removed once the canonical form is clear
* broad semantic refactors should close with repository curation so code, canonical docs, ADRs, and agent guidance agree before commit or release

When in doubt, prefer **clear domain modeling over technical shortcuts**.

---

# Vertical Slice Development

The project evolves through **small vertical slices**.

Each slice should:

* introduce a coherent domain behavior
* remain minimal and reviewable
* include tests for observable behavior
* update documentation when necessary

For rule-heavy areas, a slice may also be preceded by a focused Gherkin feature under `features/` as long as the feature:

* describes observable behavior rather than copied rules text
* references the relevant rules and slices
* stays truthful about out-of-scope behavior

Avoid introducing large frameworks or infrastructure before they are required by a slice.

---

# Code Style

## Imports

Use grouped imports to keep modules readable.

Example:

```rust
use crate::domain::play::{
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
* semantically misleading shortcuts that are easier to code but harder to justify in the ubiquitous language

New concepts should only be introduced when required by the active slice.

---

# Testing

Important domain behavior must be testable in isolation.

Tests should:

* validate observable behavior
* remain focused and readable
* avoid testing implementation details

Repository test layout:

* `tests/unit.rs` as the single conventional test target aggregating files under `tests/unit/`
* `tests/unit/` for conventional Rust integration and behavior test modules, grouped by domain area such as `lifecycle`, `turn_flow`, `resource_actions`, `combat`, `infrastructure`, and `regressions`
* `tests/bdd/` for executable Cucumber acceptance test runners

Prefer small tests tied to domain behaviors over large integration scaffolding.

BDD-style tests may be introduced when gameplay flows become complex.

When `features/` are executable, keep them focused on acceptance-level behavior and continue using ordinary Rust tests for fine-grained invariants and edge cases.

Current executable BDD pilot:

```bash
cargo test --test bdd_turn_progression
```

---

# Development Philosophy

The repository should evolve incrementally.

Prefer:

* small, reviewable changes
* narrow vertical slices
* explicit modeling decisions
* closing broad cleanups by synchronizing canonical docs, superseding stale history honestly, and updating reusable agent guidance when a lesson is likely to recur
* using focused scenario files to clarify semantics before adding complex rule-heavy slices

Avoid:

* speculative architecture
* premature abstraction
* large refactors without clear benefit

When a refactor is justified, prefer refactors that:

* eliminate semantic duplication
* improve replayability and event clarity
* preserve stable public APIs while simplifying internal structure

---

# Code Organization

When an `impl`, trait, or module grows to affect readability or maintainability, it should be split by **domain capability**, not by generic utilities.

Prefer:

- modules organized by domain behavior or aggregate concern (e.g., `rules/resource_actions.rs`, `rules/combat.rs`, `invariants.rs`)
- focused files with clear responsibilities

Avoid:

- monolithic files that mix unrelated behaviors
- generic `helpers.rs`, `utils.rs`, or `common.rs` modules without domain context

This applies especially to the `Game` aggregate: dividing its implementation into internal modules does not change the aggregate boundary.

---

# Event Design

Domain events should be semantically useful outside the aggregate.

Prefer payloads that make replay, logging, and analytics understandable without forcing consumers to reconstruct basic intent from hidden state.

In practice, this means events should usually carry enough information to answer:

- what happened
- who caused it
- what kind of domain object was involved
- what meaningful result or outcome occurred

Avoid splitting one domain fact into multiple technical delta events unless those deltas are independently meaningful.

---

# Runtime Representation

Internal data structures may be optimized for memory and locality when the model grows, but those optimizations should remain behind stable, explicit domain methods.

Prefer:

- compact internal state when it reduces repeated allocation or per-entity footprint
- shared storage for frequently cloned identifiers
- encapsulation that preserves readable domain APIs

Avoid:

- leaking memory-oriented encodings into the public domain interface
- bit-level or packed representations that make domain behavior harder to review without a clear payoff
