# Slice Name

DeclareBlockers

---

## Goal

Introduce the declare blockers step of combat. The defending player may assign creatures to block attacking creatures.

---

## Why This Slice Exists Now

This slice follows `DeclareAttackers` because:

1. Attackers are declared, now defenders need a way to respond
2. It completes the basic combat interaction
3. It enables damage resolution in future slices

---

## Supported Behavior

- introduce a `DeclareBlockersCommand` that specifies blocking assignments
- verify the player is the defending player (not active player)
- verify the phase is Main (combat phase)
- verify each blocking creature:
  - is on the battlefield
  - is controlled by defending player
  - is not tapped
- verify each target attacker exists and is attacking
- assign blockers to attackers (one creature can block multiple attackers in Magic, but we'll start with one-to-one)
- mark blocking creatures as blocked
- emit `BlockersDeclared` event

---

## Invariants / Legality Rules

- only defending player may declare blockers
- blockers must be untapped creatures controlled by defending player
- cannot block creatures that are not attacking
- blocking creatures become tapped after blocking
- this slice does NOT resolve combat damage (future slice)

---

## Out of Scope

- combat damage resolution
- multiple blockers per attacker
- trample
- first strike / double strike
- deathtouch
- lifelink
- indestructible
- damage prevention
- regenerate
- blocking creatures taking damage

---

## Domain Impact

### Aggregate Impact
- extend `Game` with `declare_blockers` behavior

### Entity / Value Object Impact
- `CardInstance` - add `is_blocking` field

### Commands
- add `DeclareBlockersCommand` - contains pairs of (blocker, target attacker)

### Events
- add `BlockersDeclared`

### Errors
- add error variants for:
  - player not the defending player
  - blocker already tapped
  - blocker not on battlefield
  - attacker not attacking

---

## Ownership Check

This behavior belongs to the `Game` aggregate because:
- it involves combat phase logic
- it enforces blocking legality
- it modifies creature state
- it produces domain events

---

## Documentation Impact

- `docs/domain/current-state.md` - update capabilities
- `docs/domain/aggregate-game.md` - extend aggregate
- `docs/architecture/vertical-slices.md` - add to evolution

---

## Test Impact

- declare blockers succeeds for valid untapped creatures
- declare blockers fails for attacking player
- declare blockers fails for tapped creatures
- declare blockers fails for non-attacking targets
- blockers become tapped after declaration
- `BlockersDeclared` event is emitted

---

## Rules Reference

- 509.1 — Declare blockers step
- 509.2 — Blockers must be controlled by defending player
- 509.3 — Creature can only block one creature (simplified)
- 509.4 — Tapping blocking creatures

---

## Rules Support Statement

This slice introduces the declare blockers step. It allows defenders to block attackers but does not resolve combat damage. Damage resolution is a future slice.

---

## Open Questions

- Should we track which specific attacker each blocker is assigned to?
- Should we allow multiple blockers per attacker (full rules)?

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
