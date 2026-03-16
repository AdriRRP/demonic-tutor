# Slice Name

DeclareAttackers

---

## Goal

Introduce a combat phase that allows the active player to declare attacking creatures. Creatures must satisfy requirements (not tapped, not summoning sickness) to attack.

---

## Why This Slice Exists Now

This slice is the natural next step after `RemoveSummoningSickness` because:

1. Creatures now have summoning sickness removed at turn start
2. We need a way to use creatures in gameplay (attacking)
3. It unlocks the core combat mechanic of Magic
4. It's prerequisite for blocking and damage

---

## Supported Behavior

- introduce a `DeclareAttackersCommand` that specifies which creatures attack
- verify the player is the active player
- verify the attack happens in the correct phase (Beginning of Combat)
- verify each attacking creature:
  - is on the battlefield
  - is controlled by the active player
  - is not tapped
  - does not have summoning sickness
- move attacking creatures to the "attacking" state
- emit `AttackersDeclared` event with the list of attackers

---

## Invariants / Legality Rules

- only the active player may declare attackers
- attackers must be creatures on the battlefield controlled by the attacker
- creatures cannot attack the turn they enter (summoning sickness)
- creatures cannot attack if they are tapped
- attacking creatures become tapped after attacking
- this slice does NOT resolve combat damage (future slice)

---

## Out of Scope

- combat damage dealing
- blocking behavior
- declare blockers step
- combat damage resolution
- state-based actions for damage destruction
- double strike / first strike
- trample
- creature abilities
- +1/+1 counters
- attacking planeswalkers
- combat phase steps beyond declare attackers

---

## Domain Impact

### Aggregate Impact
- extend `Game` with `declare_attackers` behavior
- add combat phase validation

### Entity / Value Object Impact
- `CardInstance` - add `is_attacking` field
- `Phase` - may need Combat sub-phases or new phases

### Commands
- add `DeclareAttackersCommand`

### Events
- add `AttackersDeclared`

### Errors
- add error variants for:
  - creature already tapped
  - creature has summoning sickness
  - creature not on battlefield
  - creature not controlled by attacker

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:
- it involves turn-based combat logic
- it enforces combat legality rules
- it affects creature state on the battlefield
- it produces domain events

---

## Documentation Impact

- `docs/domain/current-state.md` - update capabilities
- `docs/domain/aggregate-game.md` - extend aggregate responsibilities
- `docs/domain/DOMAIN_GLOSSARY.md` - add combat terms
- `docs/architecture/vertical-slices.md` - add to slice evolution

---

## Test Impact

- declare attackers succeeds for valid untapped creatures without summoning sickness
- declare attackers fails for tapped creatures
- declare attackers fails for creatures with summoning sickness
- declare attackers fails for creatures not on battlefield
- declare attackers fails for opponent's creatures
- declare attackers fails for non-creature cards
- attackers become tapped after declaration
- `AttackersDeclared` event is emitted with correct data

---

## Rules Reference

- 508.1 — Declare attackers step
- 508.2 — Attackers must be controlled by attacker
- 508.3 — Creature cannot attack if tapped
- 508.4 — Creature cannot attack if summoning sickness
- 508.5 — Tapping attacking creatures
- 302.6 — Summoning sickness rule

---

## Rules Support Statement

This slice introduces the declare attackers step of the combat phase. It allows creatures to attack but does not resolve combat damage. Combat damage resolution and state-based actions are future slices.

---

## Open Questions

- Should we introduce explicit Combat phase, or use sub-phases within Beginning?
- Do we need to track which player/planeswalker is being attacked?

---

## Review Checklist

- [x] Is the slice minimal?
- [x] Does it introduce one coherent behavior?
- [x] Are the legality rules explicit?
- [x] Is out-of-scope behavior stated clearly?
- [x] Does it avoid implying unsupported rules?
- [x] Is ownership clear?
- [x] Does it preserve bounded context and aggregate boundaries?
- [x] Are documentation updates limited to changed truth owners?
- [x] Is the slice easy to review and test?
