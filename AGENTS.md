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

When rule-heavy behavior already has a feature specification, also load:

- relevant `.feature` files under `features/`
- `docs/architecture/gherkin-features.md`
- relevant notes under `docs/rules/`

When the user asks for an end-to-end slice workflow, also consider the orchestrator skill:

- `.agents/skills/slice-implementation-flow/SKILL.md`

### Development workflow

Load:

- `docs/development/development.md`

### Repository curation and release preparation

Load:

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- relevant implemented slice documentation
- relevant ADRs
- `.agents/context/core-agent.md`
- relevant skills under `.agents/skills/`

These routing rules are examples.  
Agents must always prefer **minimal context loading**.

---

# Domain Discipline

Agents must:

- model only rules required by the active slice
- never imply unsupported Magic rules
- preserve aggregate boundaries
- maintain ubiquitous language consistency
- prefer the **domain-canonical action** over temporary convenience commands
- remove duplicate domain entrypoints when one concept is the real source of truth
- keep domain events expressive enough for replay and analysis without reconstructing basic intent from hidden state
- treat Gherkin features as behavior specifications derived from project truth, not as a replacement for canonical documentation or the full rulebook
- treat partial stack/priority support as a real constraint: new gameplay actions must either integrate with the currently supported priority windows or reject execution while one is open
- when widening an existing priority window, check explicitly whether the repository now needs active-player casting, non-active instant response, and active-player self-stacking in that window
- prefer positive, canonical game terminology over negated convenience wording
- mark historical slices and proposals honestly when later slices supersede them instead of leaving them sounding current

When domain truth is unclear, agents must defer to canonical documentation rather than infer behavior.

---

# Aggregate Implementation

When the `Game` aggregate grows, new behaviors should preferably be added as **internal modules by domain capability**, not by expanding a monolithic file.

Dividing the aggregate's implementation into modules does **not** create new aggregates. The aggregate boundary remains unchanged.

Agents should not infer new aggregates just because code is split into modules.

Internal representation may be optimized for memory or locality, but those optimizations must preserve:

- explicit domain APIs
- reviewability
- deterministic behavior
- ubiquitous language at the boundary of the model

When implementation structure needs to evolve, prefer:

- modules grouped by domain capability
- small explicit state machines where gameplay state has clear phases or holders
- enums and deterministic transitions over generic rule engines or trait-object dispatch
- thin application orchestration with explicit outcome-to-event mapping helpers instead of speculative abstraction layers

Avoid introducing generic frameworks when a small explicit module split keeps the code more legible.

---

# Features And BDD

The repository uses two kinds of Gherkin features:

- **executable features** used by `cucumber-rs`
- **implemented reference features** that remain truthful documentation even when not executed directly

Agents must keep feature headers honest:

- `implemented` only when the described behavior is actually supported
- `proposed` only while the behavior is still design work
- `historical` when the feature records an earlier state that is no longer live truth

When a feature stops being the live source for a behavior, do not silently leave it looking current.

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

When a session uncovers a repeated design mistake, inconsistency pattern, or closing workflow that is likely to recur, agents should prefer updating an existing skill or adding a new one before ending the work.

When broad refactors, stack growth, or repeated semantic corrections land, agents should also consider whether:

- `AGENTS.md`
- `.agents/context/core-agent.md`
- `.agents/skills/README.md`
- one or more existing skills

need synchronized updates so the repository keeps learning from the work.
