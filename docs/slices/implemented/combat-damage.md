# Slice Name

CombatDamage

---

## Goal

Resolve combat damage between attacking and blocking creatures. After attackers and blockers are declared, this slice calculates damage dealt based on power/toughness and marks damage on creatures.

---

## Why This Slice Exists Now

This slice follows `DeclareBlockers` because:

1. Attackers and blockers are already declared
2. Creatures have power and toughness modeled
3. Without damage resolution, combat has no meaningful outcome
4. It enables meaningful combat consequences and creature destruction

---

## Supported Behavior

- accept a `ResolveCombatDamageCommand`
- verify attackers have been declared
- resolve blocking assignments from aggregate runtime state established during `DeclareBlockers`
- for each attacking creature:
  - if blocked, deal damage equal to power to the blocking creature(s)
  - if unblocked, deal damage equal to power to the defending player
- for each blocking creature:
  - deal damage equal to power to the attacking creature it blocks
- route player life loss from unblocked combat damage through the same shared life semantics used by `AdjustLife`
- mark damage on creatures
- emit `CombatDamageResolved` event with damage details
- emit `LifeChanged` when combat damage changes a player's life total
- emit `GameEnded` when unblocked combat damage reduces a player to 0 life
- clear attacking and blocking combat flags after damage resolves
- allow a later narrow destruction pass to move lethally damaged creatures out of the battlefield

---

## Invariants / Legality Rules

- only the active player may trigger damage resolution
- damage resolution happens in Combat phase after attackers and blockers declared
- damage is calculated from creature's power at time of resolution
- each attacking creature can only deal damage once
- blocking creatures deal damage to their assigned attacker
- combat damage resolution fails if no attackers were declared

---

## Out of Scope

- first strike / double strike (simplified damage timing)
- trample (damage to player when unblocked)
- deathtouch (instant destruction)
- lifelink (damage dealt gains life)
- indestructible (prevents destruction)
- damage prevention / replacement effects
- multiple blockers per attacker
- general state-based actions beyond lethal damage destruction
- combat phase steps / priority
- stack

---

## Domain Impact

### Aggregate Impact
- extend `Game` with `resolve_combat_damage` behavior

### Entity / Value Object Impact
- `CardInstance` - add `damage` field to track marked damage
- rely on blocker-to-attacker runtime assignments persisted by the blocking slice

### Commands
- add `ResolveCombatDamageCommand`

### Events
- add `CombatDamageResolved` - contains list of (attacker, target, damage_amount)
- reuse `LifeChanged` / `GameEnded` when combat damage changes player life

### Errors
- add error variants for:
  - no attackers declared
  - phase invalid for combat

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:

- it involves combat phase logic
- it enforces combat legality rules
- it modifies creature state on the battlefield
- it produces domain events

---

## Documentation Impact

- `docs/domain/current-state.md` - update capabilities to include combat damage
- `docs/domain/aggregate-game.md` - extend aggregate responsibilities
- `docs/rules/rules-map.md` - add combat damage rules mapping

---

## Test Impact

- combat damage is dealt to blocking creatures
- combat damage is dealt to player when unblocked
- unblocked lethal combat damage ends the game through `ZeroLife`
- creatures with damage >= toughness marked appropriately
- combat participation flags are cleared after damage resolves
- fails when no attackers declared
- `CombatDamageResolved` event emitted correctly

---

## Rules Reference

- 510.1 — Combat damage step
- 510.2 — Damage assignment order
- 510.3 — Damage dealing
- 510.4 — Damage marked on creatures
- 510.5 — Unblocked attackers deal damage to player
- 704.5f — Creature with toughness 0 or less destroyed
- 704.5g — Creature with damage >= toughness destroyed

---

## Relationship With CreatureDestruction

This slice resolves and marks combat damage. The follow-on `CreatureDestruction` slice consumes that marked damage to destroy creatures whose damage is lethal and move them to graveyard.

---

## Open Questions

1. Should we model damage as a transient value or persistent?
2. When and how should attacker-side damage assignment order be introduced if multiple blockers per attacker become supported?

---

## Review Checklist

- [x] Is the slice minimal?
- [x] Does it introduce one coherent behavior?
- [x] Are the legality rules explicit?
- [x] Is out-of-scope behavior stated clearly?
- [x] Does it avoid implying unsupported rules?
- [x] Is ownership clear?
- [x] Does it preserve bounded context and aggregate boundaries?
- [x] Is the slice easy to review and test?
