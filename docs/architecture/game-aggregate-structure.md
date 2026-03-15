# Game Aggregate Structure — DemonicTutor

This document describes the recommended internal organization of the `Game` aggregate.

---

## Overview

`Game` is the **aggregate root** of the `play` bounded context. Its implementation may be organized into internal modules by domain capability without changing the aggregate boundary.

---

## Guiding Principles

- **Aggregate boundary remains unchanged** — The `Game` aggregate root stays cohesive regardless of internal file organization.
- **Modules follow domain capabilities** — Group code by behavior (lands, mana, combat) not by generic categories (helpers, utils).
- **The file structure is a guideline** — It may evolve as the system grows. Do not treat it as a rigid constraint.

---

## Recommended Structure

```
src/domain/game/
├── mod.rs           # Re-exports and aggregates
├── aggregate.rs     # Main impl Game block
├── state.rs         # State transitions and invariants
├── phases.rs        # Phase-specific logic
├── start_game.rs    # Game initialization
├── opening_hands.rs # Dealing opening hands
├── mulligan.rs      # Mulligan handling
├── draw.rs          # Drawing cards
├── lands.rs         # Playing and managing lands
├── mana.rs          # Mana production and usage
├── spells.rs        # Casting spells
├── creatures.rs     # Creature-specific behavior
├── combat.rs        # Attacking and blocking
├── turns.rs         # Turn progression
└── life.rs          # Life total management
```

---

## Why This Organization

- **Readability** — Each file focuses on a specific domain capability.
- **Maintainability** — Changes related to a capability stay localized.
- **Discoverability** — New developers can find relevant code quickly.
- **DDD alignment** — Modules reflect the domain language, not technical categories.

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
