# AGENTS.md — DemonicTutor

## Purpose

This file is the entry point for agents working in the DemonicTutor repository.

It defines:

- how agents enter the repository
- how context should be loaded
- how work should be routed to the correct documentation

This document is intentionally minimal.

For full agent architecture see:

`docs/architecture/agent-architecture.md`

For operational working behavior see:

`.agents/context/core-agent.md`

---

# Initial Context

When beginning work, agents must read:

1. `CONSTRAINTS.md`
2. `docs/architecture/agent-architecture.md`
3. `.agents/context/core-agent.md`

After this initial context, agents must load **only the documentation required for the task**.

Agents must **not scan the repository indiscriminately**.

Context should expand **incrementally and only when needed**.

---

# Source of Truth

If multiple sources conflict, the following precedence applies:

1. **Code (`src/`)**
2. **Accepted ADRs**
3. **Canonical documentation**
4. **Operational agent context**
5. **Skills**

Operational context and skills must **never override canonical documentation**.

---

# Canonical Documentation

The following documents define project truth.

### Project and constraints

- `PROJECT.md`
- `CONSTRAINTS.md`

### Domain model

- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/context-map.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/current-state.md`

### System architecture

- `docs/architecture/system-overview.md`
- `docs/architecture/vertical-slices.md`

### Architecture decisions

- `docs/architecture/adr/`

---

# Context Routing

Agents should load the **minimum documentation necessary** depending on the task.

### Domain reasoning

Load:

- `docs/domain/DOMAIN_GLOSSARY.md`
- `docs/domain/context-map.md`
- `docs/domain/aggregate-game.md`
- `docs/domain/current-state.md`

### System architecture work

Load:

- `docs/architecture/system-overview.md`
- `docs/architecture/vertical-slices.md`
- relevant ADRs

### Slice implementation

Load:

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/architecture/vertical-slices.md`
- relevant slice documentation in `docs/slices/`

### Development workflow

Load:

- `docs/development/development.md`

These routing rules are examples.  
Agents must always prefer **minimal context loading**.

---

# Domain Discipline

Agents must:

- model only rules required by the active slice
- never imply unsupported Magic rules
- preserve aggregate boundaries
- maintain ubiquitous language consistency

When domain truth is unclear, agents must defer to canonical documentation rather than infer behavior.

---

# Skills

Skills encapsulate reusable workflows.

They should be used when work is:

- repeatable
- narrow in scope
- workflow driven

Skills live under:

`.agents/skills/`

Prefer creating a **skill** rather than introducing a new agent.

Skills must reference canonical documentation instead of duplicating domain knowledge.
