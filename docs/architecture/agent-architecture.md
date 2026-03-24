# Agent Architecture — DemonicTutor

## Purpose

This document defines the architecture used for agent-assisted development in DemonicTutor.

The goal is to enable agent assistance while preserving:

- correctness
- domain integrity
- low context usage
- human reviewability
- long-term maintainability

Agents assist development but do not define project truth.

---

# Scope

This architecture is designed to:

- support agent-assisted development
- preserve Domain-Driven Design boundaries
- minimize context loading
- keep the repository understandable for humans

It does not attempt to:

- encode the full domain model in prompts
- replace architectural judgement
- optimize for agent autonomy over correctness
- introduce unnecessary agent orchestration

---

# Core Principles

## Single Entry Point

All agent interaction begins with:

`AGENTS.md`

This file defines how context should be discovered and loaded.

Agents should not scan the repository indiscriminately.

---

## Progressive Context Loading

Agents should load **only the context required for the current task**.

The full documentation tree is never the default context.

Additional documents are loaded only when relevant.

---

## Domain-Driven Design First

The domain model defines the architecture.

Agents must respect:

- bounded contexts
- aggregates
- ubiquitous language
- explicit invariants

Agents assist implementation but do not redefine the domain.

**Note on modularization:** Internal code organization may evolve independently from domain boundaries. The aggregate boundary defines domain responsibility, while file/module boundaries are implementation details. Dividing an aggregate's implementation into internal modules does not create new aggregates.

---

## Canonical Documentation

Canonical documentation defines stable project truth.

Examples include:

- product vision
- constraints
- domain language
- architecture
- runtime abstraction explanations
- bounded contexts
- aggregate responsibilities
- current system state

Only canonical documentation may define stable project truth.

Operational agent context must reference canonical sources rather than duplicate them.

Domain rules must never be defined outside canonical documentation.

---

# Authority Model

When sources conflict, precedence is:

1. Code
2. Accepted ADRs
3. Canonical documentation
4. Operational agent context
5. Skills

This ensures that project truth always remains explicit and reviewable.

---

# Documentation Layers

The repository contains four conceptual documentation layers.

### Canonical Documentation

Defines stable project truth.

Examples:

- domain model
- architecture
- constraints
- current system state

Canonical documentation should remain few, stable, and authoritative.

---

### Operational Agent Context

Supports correct agent behavior.

Examples:

- entrypoint instructions
- agent working posture
- workflow routing guidance

Operational context must not redefine domain knowledge.

---

### Historical Documentation

Explains why decisions were made.

Examples:

- ADRs
- changelog

Historical documentation informs reasoning but does not override current truth.

When historical documents are superseded but still valuable, they should be marked explicitly instead of silently left looking current.

---

### Skills

Skills encapsulate reusable workflows.

They reduce prompt size and enforce consistent execution of common tasks.

Skills should:

- remain narrow in scope
- reference canonical documentation
- produce reviewable outputs
- capture repeated repository workflows and guardrails that reduce future drift
- stay synchronized with stable repository practices such as stack/priority guardrails, truthful feature statuses, and honest historical marking

Skills must not redefine architecture or domain rules.

---

# Agent Model

The repository uses a **single-agent architecture**.

## Core Agent

The core agent:

- interprets tasks within repository context
- loads minimal documentation
- respects domain boundaries
- produces small reviewable changes
- invokes skills when needed
- helps keep code, docs, and agent context synchronized when stable design lessons emerge
- prefers turning repeated semantic corrections into durable repository guidance before closing broad refactors or release-preparation work

The agent operates within rules defined by canonical documentation.

---

# Skills vs Agents

Specialization should occur through **skills first**.

New agents should only be introduced when:

- a workflow is recurrent
- skills cannot keep context small enough
- specialization materially reduces error or review burden

Architectural evolution should prefer refinement over expansion.

---

# Repository Mapping

The architecture maps conceptually to the repository layout:

```

/
├── AGENTS.md
├── PROJECT.md
├── CONSTRAINTS.md
├── SECURITY.md
├── CHANGELOG.md
│
├── docs/
│   ├── domain/
│   ├── architecture/
│   ├── slices/
│   └── development/
│
├── .agents/
│   ├── context/
│   └── skills/

```

The architecture is not defined by the directory tree, but the structure reflects the architectural model.

For the human-oriented explanation of the engine's current compact runtime abstractions, see:

- `docs/architecture/runtime-abstractions.md`

## Canonical Working Templates

The repository may include canonical templates for recurring project artifacts.

Examples include:

- `docs/architecture/slice-template.md`
- `docs/architecture/adr-template.md`

These templates define the expected structure of those artifacts.

Skills may use them, but must not duplicate them.

## Stable Development Patterns

The agent architecture should reinforce the repository's preferred implementation style:

- grow the `Game` aggregate through internal modules by capability, not new aggregates by default
- prefer explicit enums, small state structs, and deterministic transition helpers over generic engines
- prefer closed rule representations over optional-field cross-products when the supported cases are finite
- when the runtime becomes denser or more optimized, preserve a plain-language explanation in `docs/architecture/runtime-abstractions.md`
- architecture docs should reduce cognitive load for humans before sending them into low-level storage or identity code
- prefer small semantic snapshots over cloning full runtime objects in validation-heavy paths
- keep partial rule support honest in code, features, slices, and current-state docs
- when widening priority-window support, synchronize the behavior triangle explicitly: active-player casting, non-active instant response, and active-player self-stacking
- prefer broader truthful summaries in canonical docs once many narrow stack slices exist, rather than leaving public docs to underrepresent current support
- treat executable and non-executable features as different documentation roles that still require truthful status metadata
- mark proposals and historical slices explicitly once they no longer describe the live source of truth

These are implementation and curation guardrails, not separate sources of domain truth.

---

# Evolution Policy

The architecture is designed to evolve safely.

Stable elements are:

- single entrypoint
- canonical documentation
- one core agent
- skills as the extension mechanism

Operational refinements may still occur, especially when:

- repeated review findings expose avoidable design drift
- repository cleanup workflows become recurrent
- agents need clearer guidance to preserve semantics, consistency, and long-term maintainability

Future evolution should follow this order:

1. improve canonical documentation
2. refine skills
3. introduce automation or tools
4. introduce additional agents only when clearly justified

Architectural evolution should favor simplification over expansion whenever possible.
