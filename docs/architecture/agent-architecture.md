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

---

### Skills

Skills encapsulate reusable workflows.

They reduce prompt size and enforce consistent execution of common tasks.

Skills should:

- remain narrow in scope
- reference canonical documentation
- produce reviewable outputs

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

## Canonical Working Templates

The repository may include canonical templates for recurring project artifacts.

Examples include:

- `docs/architecture/slice-template.md`
- `docs/architecture/adr-template.md`

These templates define the expected structure of those artifacts.

Skills may use them, but must not duplicate them.

---

# Evolution Policy

The architecture is designed to evolve safely.

Stable elements are:

- single entrypoint
- canonical documentation
- one core agent
- skills as the extension mechanism

Future evolution should follow this order:

1. improve canonical documentation
2. refine skills
3. introduce automation or tools
4. introduce additional agents only when clearly justified

Architectural evolution should favor simplification over expansion whenever possible.
