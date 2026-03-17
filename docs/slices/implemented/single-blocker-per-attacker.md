# Slice Name

SingleBlockerPerAttacker

---

## Goal

Make the current combat model explicit by allowing at most one blocker per attacking creature.

---

## Why This Slice Exists Now

The repository already supports declaring blockers and resolving combat damage, but it does not model attacker-side damage assignment order for multiple blockers. Restricting the model to one blocker per attacker keeps combat semantically honest until that richer behavior exists.

---

## Supported Behavior

- reject `DeclareBlockers` when two different blockers are assigned to the same attacker
- keep single-blocker combat unchanged
- preserve combat damage resolution based on stored blocker-to-attacker runtime state

---

## Invariants / Legality Rules

- each blocking creature may appear at most once in the assignment list
- each attacking creature may have at most one blocker assigned
- combat damage continues to assume a single blocker per attacker in the current model

---

## Out of Scope

- multiple blockers per attacker
- attacker-side damage assignment order
- trample over multiple blockers
- first strike / double strike

---

## Domain Impact

### Aggregate Impact

- tighten `Game::declare_blockers` legality

### Commands

- no command shape changes required

### Events

- no new events required

### Errors

- add an explicit error for assigning more than one blocker to the same attacker

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/rules/notes/combat.md`
- `docs/slices/implemented/declare-blockers.md`
- `docs/slices/implemented/combat-damage.md`

---

## Test Impact

- unit test rejects two blockers on one attacker
- BDD feature specifies the supported combat simplification explicitly

---

## Rules Reference

- 509.1
- 509.2
- 509.3
- 510.2

---

## Rules Support Statement

DemonicTutor currently supports only single-blocker assignments per attacker. Multiple blockers remain out of scope until attacker-side damage assignment order is modeled.
