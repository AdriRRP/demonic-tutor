# Slice Name

RemoveSummoningSickness

---

## Goal

Automatically remove summoning sickness from all creatures controlled by a player at the beginning of their turn, allowing those creatures to attack or use tap abilities.

---

## Why This Slice Exists Now

This slice is the natural next step after creature spells can enter the battlefield because:

1. Creatures currently enter with summoning sickness but it is never removed
2. Without this slice, creatures can never attack
3. It completes the creature lifecycle introduced when creature spells started resolving to the battlefield
4. It's a simple, automatic behavior that doesn't require complex combat logic

---

## Supported Behavior

- at the beginning of each player's turn, automatically remove summoning sickness from the active player's creatures
- this happens as part of the turn advancement (Beginning phase)
- creatures that entered the battlefield in previous turns become able to attack

---

## Invariants / Legality Rules

- all creatures controlled by the active player lose summoning sickness at turn start
- creatures that entered battlefield this turn retain summoning sickness
- this is an automatic behavior, not a command
- only creatures on the battlefield are affected

---

## Out of Scope

- attacking behavior
- combat damage
- blocking behavior
- declare attackers step
- declare blockers step
- combat phase progression
- using tap abilities
- +1/+1 counters or -1/-1 counters
- damage tracking or destruction
- triggered abilities on attack

---

## Domain Impact

### Aggregate Impact
- modify `Game::advance_turn` so the Untap phase updates only the active player's battlefield state
- or add a new method `remove_summoning_sickness` that is called at the start of the active player's turn

### Entity / Value Object Impact
- `CardInstance` already has `remove_summoning_sickness()` method
- may need a method on `Battlefield` or `Player` to iterate over their creatures

### Commands
- no new commands (this is automatic behavior)

### Events
- consider if an event should be emitted (e.g., `SummoningSicknessRemoved`) or keep it implicit
- implicit is acceptable since it's automatic and not player-initiated

### Errors
- no new errors (automatic behavior with no failure mode)

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:
- it involves turn-based automatic state transitions
- it affects creature state on the battlefield
- it's part of the turn progression logic
- it enforces a gameplay invariant

---

## Documentation Impact

- `docs/domain/current-state.md` - update capabilities to note summoning sickness is automatically removed
- `docs/domain/aggregate-game.md` - may need to note this automatic behavior
- `docs/architecture/vertical-slices.md` - add to slice evolution

---

## Test Impact

- creatures played in previous turn can attack after turn advances
- creatures played this turn still have summoning sickness
- multiple creatures all lose summoning sickness correctly
- works correctly across turn transitions (player 1 -> player 2 -> player 1)

---

## Rules Reference

- 302.6 — "A creature can't attack or use abilities that require tap until it has been under its controller's control since the beginning of their turn"

---

## Rules Support Statement

This slice implements the automatic removal of summoning sickness at the beginning of a player's turn, as required by rule 302.6. This allows creatures to legally attack or use tap abilities on subsequent turns. This slice does not model the combat phase, attacking behavior, or any other combat-related mechanics.

---

## Open Questions

- Should we emit an event for each creature, or is implicit behavior acceptable?
- Do we need to handle creatures that come under a player's control mid-turn (e.g., through summoning)?

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
