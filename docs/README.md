# Documentation Map — DemonicTutor

This directory contains the canonical documentation of the DemonicTutor project.

The documentation is organized by responsibility to keep the system understandable as it evolves.

Agents should rely on `AGENTS.md` for routing and context loading.  
This document primarily helps human contributors navigate the documentation.

---

# Project-Level Documents

These documents live in the repository root.

| Document | Purpose |
|--------|--------|
| `PROJECT.md` | Defines the vision and identity of the project |
| `CONSTRAINTS.md` | Defines architectural and modeling limits |
| `SECURITY.md` | Security reporting and threat model |
| `AGENTS.md` | Entry point for agents working in the repository |

---

# Domain Documentation

Location:

```
docs/domain/

```

These documents describe the **domain model and ubiquitous language**.

| Document | Purpose |
|--------|--------|
| `DOMAIN_GLOSSARY.md` | Ubiquitous language for the domain |
| `context-map.md` | Relationships between bounded contexts |
| `aggregate-game.md` | Design of the core `Game` aggregate |
| `current-state.md` | Snapshot of the currently implemented domain |

These documents represent **domain truth** and must remain consistent with the code.

---

# System Architecture

Location:

```
docs/architecture/

```

These documents describe **how the system is structured**.

| Document | Purpose |
|--------|--------|
| `system-overview.md` | High-level architecture of the system |
| `vertical-slices.md` | Strategy for incremental feature evolution |
| `agent-architecture.md` | Architecture of the agent system |
| `slice-template.md` | Canonical structure for slice documentation |
| `adr-template.md` | Template for architecture decisions |
| `adr/` | Accepted architecture decision records |

---

# Vertical Slices

Location:

```
docs/slices/

```

Slices describe **incremental capabilities added to the system**.

| Directory | Purpose |
|--------|--------|
| `implemented/` | Documented slices already present in the system |
| `proposals/` | Proposed future slices |

Slices represent the main mechanism for evolving the system safely.

---

# Development Guidelines

Location:

```
docs/development/

```

| Document | Purpose |
|--------|--------|
| `development.md` | Code quality rules and development workflow |

---

# How Documentation Evolves

Documentation should evolve incrementally together with the code.

Typical workflow:


```
Design slice
↓
Implement slice
↓
Update domain documentation
↓
Update architecture documentation (if needed)
↓
Record architectural decisions (ADR) when necessary

```

Documentation should remain:

- explicit
- minimal
- consistent with the domain model

---

# Documentation Philosophy

The documentation follows several principles:

- **single responsibility per document**
- **explicit domain language**
- **minimal duplication**
- **incremental evolution through slices**

The goal is to keep the repository understandable even as the system grows.
