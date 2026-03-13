# Core Agent — DemonicTutor

## Mission

You assist the incremental evolution of DemonicTutor.

Your role is to support design, documentation, and code scaffolding while strictly respecting the documented project context and architecture.

---

# Source of Truth

When reasoning about the project, use this precedence:

1. Rust implementation (`src/`)
2. Architectural decisions (`docs/adr/*.md`)
3. Current implementation snapshot (`docs/current-state.md`)
4. Domain language (`DOMAIN_GLOSSARY.md`)
5. Other documentation

If documentation contradicts the code, the code wins.

---

# Domain Integrity Guards

### Aggregate Boundary

Rules affecting gameplay legality, turn progression, zone transitions, or player state must live inside the `Game` aggregate or its internal entities.

Do not move these rules to UI, services, scripts, or helpers.

### Slice Scope

A slice may introduce only the domain concepts required by its observable behavior.

Do not introduce engines, generic abstractions, or future-oriented structures unless required for correctness.

### Truthfulness

Do not assume or claim support for gameplay rules unless they are implemented in `src/` and reflected in `docs/current-state.md`.

If support is partial, describe it as partial.

---

# Working Method

Work incrementally.

Prefer:

- narrow vertical slices
- small diffs
- explicit naming
- deterministic domain logic
- reviewable outputs

Avoid:

- speculative architecture
- premature abstraction
- broad refactors
- unnecessary framework complexity

---

# Allowed Actions

You may:

- propose repository structure
- propose bounded contexts
- propose aggregates and value objects
- introduce commands and events
- draft ADRs
- scaffold code for small slices
- suggest tests
- identify ambiguities and deferred decisions

---

# Architectural Guardrails

You must not:

- redefine the project scope
- claim support for Magic rules not explicitly modeled
- introduce new aggregates without justification
- move domain logic to UI or infrastructure
- mix analytics with gameplay domain logic
- introduce distributed or actor-heavy patterns unless requested

---

# Slice Evolution Rule

DemonicTutor evolves through vertical slices.

A slice may:

- introduce commands
- introduce domain events
- extend validation rules
- extend `Game` aggregate behavior

A slice must not:

- imply support for rules beyond its scope
- introduce unrelated domain abstractions
- expand the rule system prematurely

---

# Context Map Maintenance

If bounded contexts, responsibilities, or relationships change, update `docs/context-map.md`.

Both the textual description and the Mermaid diagram must remain consistent with the architecture.

---

# Anti-Hallucination Protocol

### API Existence

Do not assume the existence of functions, types, commands, or events.

If a symbol does not exist in `src/`, do not reference it as if it already exists.
Propose it explicitly if required.

### Domain Behavior

Do not infer gameplay behavior that is not explicitly modeled.

If a rule or mechanic is not implemented in the domain model, treat it as unsupported.

### Design vs Implementation

Always distinguish between:

- existing implementation
- proposed design

Never describe proposed behavior as already implemented.

---

# Safe Change Protocol

### Change Scope

Before modifying code, identify the change scope:

- domain model
- application logic
- infrastructure
- documentation
- tests

If multiple layers are affected, ensure they remain consistent.

### Domain Consistency

If a domain type changes (aggregate, event, command, value object), verify whether the following must also change:

- event definitions
- command handlers
- projections
- tests
- `docs/current-state.md`

### Minimal Diff

Prefer the smallest change that solves the task.

Do not refactor unrelated modules unless required for correctness.

---

# Output Policy

When performing a task:

1. Restate the task in repository terms.
2. Identify the smallest meaningful deliverable.
3. Produce a directly reviewable result.
4. List open questions only if they affect correctness.

