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
│   ├── player.rs      # Aggregate-owned entity internals
│   ├── priority.rs    # Aggregate-owned priority state
│   └── stack.rs       # Aggregate-owned stack state
└── rules/
    ├── mod.rs
    ├── lifecycle.rs        # Start game, opening hands, mulligan
    ├── game_effects.rs     # Direct life and game-end helpers reused by rules
    ├── resource_actions.rs # Lands, mana, spells, creatures, life
    ├── state_based_actions.rs # Shared review of supported state-based actions
    ├── stack_priority.rs   # Minimal stack casting, passing, and top-object resolution
    └── combat.rs           # Attacking, blocking, combat damage
    └── turn_flow/
        ├── mod.rs
        ├── phase_behavior.rs
        ├── turn_progression.rs
        ├── draw_effects.rs
        └── cleanup.rs
```

---

## Why This Organization

- **Readability** — Each file focuses on a specific domain capability.
- **Maintainability** — Changes related to a capability stay localized.
- **Discoverability** — New developers can find relevant code quickly.
- **DDD alignment** — Modules reflect the domain language, not technical categories.
- **Rust coherence** — `mod.rs` stays small while internal modules separate aggregate state, rules, and invariants.
- **Semantic consistency** — Shared state-based action review and direct game effects stay explicit instead of being duplicated across turn flow, resource, and combat code.
- **Incremental stack evolution** — Stack and priority can grow from aggregate-owned model state through small explicit slices instead of a generic rules engine.

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
