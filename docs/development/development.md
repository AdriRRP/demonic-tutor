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

Release management is explicit and curated when a release is being cut.

When contributing:

- **Do not modify the version** in `Cargo.toml` unless explicitly requested.
- **Do not update `CHANGELOG.md`** unless the user is explicitly preparing a release.
- Focus on implementing the change correctly.

When a release is explicitly requested, use the repository's release workflow to update version, changelog, validation, and tags together.

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

## Practical Slice Flow

Default sequence:

1. define the capability in one sentence
2. load only the minimum canonical context
3. decide the smallest observable behavior worth landing
4. add or refine the `.feature` only if the behavior is acceptance-level or rule-heavy
5. add or refine the slice doc
6. implement the domain behavior in the owning capability module
7. add unit tests for invariants, edge cases, and regressions
8. add BDD only for the end-to-end gameplay corridor
9. sync canonical docs only if their owned truth changed
10. run `./scripts/check-all.sh`
11. commit semantically

Preferred order:

`feature/slice -> domain -> unit tests -> BDD -> docs sync -> check-all -> commit`

## Small Example

Example: `Forest` produces `Green`.

1. feature intent:
   `tapping a Forest adds Green mana`
2. domain impact:
   extend card-face mana color, mana pool, and land-tap resolution
3. unit coverage:
   `forest adds green mana`
   `mana event reports green`
4. executable BDD only if the slice also proves a real corridor such as:
   `Alice casts a green instant in FirstMain`
5. docs sync:
   `current-state`
   glossary only if a new term becomes canonical
   implemented slice doc

This is the default bar: one real behavior, one small model extension, focused tests, honest docs.

---

# Code Style

## Imports

Use grouped imports to keep modules readable.

This is mandatory in this repository:

- every Rust module file under `src/` and `tests/` must start with a brief module rustdoc comment using `//!`
- top-level imports in each Rust module must follow repository `rustfmt` settings rather than per-file ad hoc formatting
- when multiple imports can be grouped semantically, prefer compact grouped imports

These rules are repository policy and must be preserved in every new file and every edited file.

Example:

```rust
//! Supports spell casting orchestration.

use {
    crate::domain::play::{
        commands::{Cmd1, Cmd2},
        errors::DomainError,
        events::Event,
    },
    std::sync::Arc,
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
* modeling finite rule spaces as combinations of optional fields when a closed enum can express the supported cases directly
* cloning full aggregate-owned runtime objects in validation paths when a smaller capability snapshot matches the supported invariant

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

## Test Strategy

Use unit tests by default.

Add unit tests for:

- legality and invariants
- state transitions inside one capability
- edge cases and regressions
- negative paths

Add executable BDD for:

- end-to-end gameplay corridors
- rule-heavy flows that are easier to review in ubiquitous language
- interactions across stack, priority, mana, combat, targeting, or turn flow

Do not move fine-grained invariant testing into BDD.
Do not use BDD to replace unit coverage.

## Test Placement

- add new unit files inside the nearest existing capability directory under `tests/unit/`
- add new BDD step files inside the nearest existing behavior family under `tests/bdd/`
- add shared BDD setup only under `tests/bdd/world/`
- add shared unit helpers only under `tests/unit/support/`

If a file becomes crowded, split it by capability or scenario family, not by arbitrary size.

Examples:

- new mana payment edge case:
  `tests/unit/resource_actions/`
- new combat legality rule:
  `tests/unit/combat/`
- new targeting gameplay corridor:
  `tests/bdd/spell_casting/`
- repeated combat setup helper:
  `tests/bdd/world/setup_combat_windows/`

Prefer small tests tied to domain behaviors over large integration scaffolding.

BDD-style tests may be introduced when gameplay flows become complex.

When `features/` are executable, keep them focused on acceptance-level behavior and continue using ordinary Rust tests for fine-grained invariants and edge cases.

## Feature Rules

- put executable features under the nearest capability directory in `features/`
- keep one coherent gameplay behavior per feature
- name scenarios by observable gameplay result
- use features to describe behavior
- use slice docs to describe scope and implementation boundaries
- if a feature becomes historical or reference-only, mark it honestly

Current executable BDD suite:

```bash
cargo test --test bdd
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

- modules organized by domain behavior or aggregate concern (e.g., `rules/resource_actions.rs`, `rules/combat/`, `invariants.rs`)
- focused files with clear responsibilities

Avoid:

- monolithic files that mix unrelated behaviors
- generic `helpers.rs`, `utils.rs`, or `common.rs` modules without domain context

This applies especially to the `Game` aggregate: dividing its implementation into internal modules does not change the aggregate boundary.

## New Modules

Prefer adding behavior to an existing capability module first.

Create a new module only when:

- the file is becoming hard to navigate
- the new behavior is a stable capability of its own
- the helper logic is reused inside the same capability family

Good:

- `src/domain/play/game/rules/stack_priority/`
- `src/domain/play/game/rules/combat/`
- `tests/bdd/world/setup_combat_windows/`

Avoid:

- `helpers.rs` with unrelated logic
- broad framework extraction for one slice
- public modules created only to hide temporary complexity

## Utility Placement

Add a utility only after the second real use.

Place it as close as possible to the owning behavior:

- stack-only helper:
  under `src/domain/play/game/rules/stack_priority/`
- combat-only helper:
  under `src/domain/play/game/rules/combat/`
- BDD setup helper:
  under `tests/bdd/world/`
- unit helper:
  under `tests/unit/support/`

Keep helpers:

- small
- deterministic
- private unless another module truly needs them
- named in ubiquitous language

Avoid cross-cutting utility files for unrelated concerns.

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

Current repository choice:

- domain ids currently centralize their shared string backing behind a single `SharedIdStr` alias in `src/domain/play/ids.rs`
- the runtime intentionally uses `Arc<str>` today because ids are cloned pervasively into events, tests, and stack objects
- this choice should not be changed casually to `Rc<str>` or interning without profiling the real workload first
- if future measurements show identifier cloning or atomic refcounting as a meaningful hotspot, revisit the alias rather than rewriting each id type independently

## Memory Heuristics

Prefer the smallest representation that matches the supported rule space.

Prefer:

- closed enums for finite rule sets
- small snapshots in validation paths
- face/runtime separation when mutable state grows
- explicit value objects for legal state

Avoid:

- cloning full runtime objects just to inspect one capability
- collections more general than the currently supported invariant
- representational combinations that allow impossible states

If a new slice needs more state, first ask whether the state belongs to:

- immutable card face data
- mutable runtime state
- transient validation snapshot
- or a narrower capability-local helper
