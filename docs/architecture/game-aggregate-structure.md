# Game Aggregate Structure — DemonicTutor

This document describes the recommended internal organization of the `Game` aggregate.

---

## Overview

`Game` is the **aggregate root** of the `play` bounded context. Its implementation may be organized into internal modules by domain capability without changing the aggregate boundary.

---

## Guiding Principles

- **Aggregate boundary remains unchanged** — The `Game` aggregate root stays cohesive regardless of internal file organization.
- **Modules follow domain capabilities** — Group code by behavior and internal aggregate concerns, not by generic categories (`helpers`, `utils`).
- **The file structure is a guideline** — It may evolve as the system grows. Do not treat it as a rigid constraint.

---

## Recommended Structure

```
src/domain/play/game/
├── mod.rs             # Aggregate facade and command entrypoints
├── invariants.rs      # Aggregate legality checks and internal lookups
├── model/
│   ├── mod.rs
│   └── player.rs      # Aggregate-owned entity internals
└── rules/
    ├── mod.rs
    ├── lifecycle.rs        # Start game, opening hands, mulligan
    ├── turn_flow.rs        # Phases, draws, turn progression
    ├── resource_actions.rs # Lands, mana, spells, creatures, life
    └── combat.rs           # Attacking, blocking, combat damage
```

---

## Why This Organization

- **Readability** — Each file focuses on a specific domain capability.
- **Maintainability** — Changes related to a capability stay localized.
- **Discoverability** — New developers can find relevant code quickly.
- **DDD alignment** — Modules reflect the domain language, not technical categories.
- **Rust coherence** — `mod.rs` stays small while internal modules separate aggregate state, rules, and invariants.

---

## When to Refactor

Consider splitting or reorganizing when:

- A file exceeds ~200 lines
- Unrelated behaviors are mixed in one file
- Finding code becomes difficult
- The domain language suggests a new capability

---

## Important Reminders

- **Do not create new aggregates** just because code is split into modules.
- **Keep the aggregate root cohesive** — all commands must flow through `Game`.
- **Preserve invariants** — State transitions remain explicit and deterministic.
- **Do not add infrastructure concerns** (persistence, networking) inside the aggregate.

See also:

- `docs/domain/aggregate-game.md` — Aggregate responsibilities and invariants
- `docs/architecture/vertical-slices.md` — How slices evolve the system
