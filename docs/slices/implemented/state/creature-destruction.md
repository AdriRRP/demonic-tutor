# Slice Name

CreatureDestruction

---

## Goal

Destroy creatures automatically when they have lethal damage marked on them, moving them from the battlefield to the graveyard. This completes the current simplified combat loop without introducing the full state-based action system.

---

## Why This Slice Exists Now

This slice follows `CombatDamage` because:

1. combat already marks damage on creatures
2. the runtime already tracks toughness and graveyard zones
3. combat still needs lasting battlefield consequences
4. the behavior is observable, narrow, and semantically important

---

## Supported Behavior

- check creatures with lethal damage through the shared state-based action review after relevant actions
- destroy creatures whose marked damage is greater than or equal to toughness
- move destroyed creatures from battlefield to graveyard
- emit `CreatureDied` once per creature that dies
- keep creatures with nonlethal damage on the battlefield

---

## Invariants / Legality Rules

- only creatures on the battlefield are eligible for destruction by this slice
- destruction is automatic game behavior, not a player command
- lethal damage means marked damage is greater than or equal to toughness
- a destroyed creature changes zone exactly once
- creature destruction in this slice is tied only to already-marked damage

---

## Out of Scope

- a general state-based action engine
- destruction from spells or abilities
- toughness 0-or-less checks unrelated to marked damage
- regeneration
- indestructible
- replacement effects
- death triggers
- token ceasing-to-exist handling
- cleanup-based damage removal

---

## Domain Impact

### Aggregate Impact

- extend `Game` with shared state-based action review that includes lethal-damage creature destruction

### Entity / Value Object Impact

- `CardInstance` exposes lethal-damage semantics
- `Battlefield` supports removing destroyed permanents cleanly

### Commands

- no new public command required

### Events

- add `CreatureDied`

### Errors

- no new public error required

---

## Ownership Check

This behavior belongs to the `Game` aggregate because it:

- enforces combat consequences inside the play domain
- moves cards between owned zones
- applies automatically from current game state
- emits domain events describing the resulting state change

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- `docs/rules/notes/combat.md`
- `features/combat/creature_destruction.feature`
- this slice document

---

## Test Impact

- destroys a creature with lethal marked damage
- keeps a creature with nonlethal marked damage alive
- moves destroyed creatures to graveyard
- emits `CreatureDied` for each creature that dies
- does not require a separate player command

---

## Rules Reference

- 704 — State-based actions
- 704.5g — A creature with lethal damage is destroyed

---

## Rules Support Statement

This slice introduces a narrow automatic destruction rule for creatures with lethal damage already marked on them through the repository's shared state-based action review. It does not implement a general state-based action system, and it does not model regeneration, indestructible, or other rule modifications to destruction.
