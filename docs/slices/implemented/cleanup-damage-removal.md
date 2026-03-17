# Slice Name

CleanupDamageRemoval

---

## Goal

Clear marked damage from surviving creatures when the turn ends.

---

## Why This Slice Exists Now

This slice completes the current simplified combat lifecycle because:

1. combat damage is already marked on creatures
2. lethal damage already destroys creatures automatically
3. surviving creatures should not keep marked damage across turns
4. the runtime already has a natural end-of-turn transition point

---

## Supported Behavior

- clear marked damage from surviving creatures when the game advances from `EndStep` to the next player's `Untap`
- keep lethal-damage destruction separate from cleanup removal
- apply damage cleanup automatically as turn progression, not as a player command

---

## Invariants / Legality Rules

- cleanup-based damage removal is automatic game behavior
- only creatures that remain on the battlefield are affected
- destroyed creatures are not restored or otherwise changed by cleanup
- damage cleanup happens when the turn ends, before the next turn begins

---

## Out of Scope

- a full cleanup step model
- end-of-turn triggered abilities
- until-end-of-turn continuous effects
- mana cleanup semantics beyond the current simplified pool reset
- state-based actions unrelated to lethal damage

---

## Domain Impact

### Aggregate Impact

- `Game` clears marked damage during end-of-turn phase progression

### Entity / Value Object Impact

- existing `CardInstance::clear_damage()` becomes a live gameplay behavior

### Commands

- no new public command required

### Events

- no new event required

### Errors

- no new public error required

---

## Ownership Check

This behavior belongs to the `Game` aggregate because it:

- updates authoritative runtime combat state
- happens automatically from turn progression
- is part of gameplay state cleanup, not infrastructure

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/notes/combat.md`
- `docs/rules/notes/turn-flow.md`
- `features/turn-flow/cleanup_damage_removal.feature`
- this slice document

---

## Test Impact

- marked damage on a surviving creature is cleared when the turn ends
- cleanup happens automatically without a player command

---

## Rules Reference

- 120.6 — Damage marked on a permanent remains until the cleanup step
- 514 — Cleanup step

---

## Rules Support Statement

This slice introduces a minimal cleanup behavior that removes marked damage from surviving creatures at the end of the turn. It does not model the full cleanup step or other end-of-turn rules interactions.
