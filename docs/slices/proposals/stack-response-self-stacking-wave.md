# Slice Proposal — StackResponseSelfStackingWave

## Goal

Define the next coherent wave of stack slices after active-player casting, opponent instant responses, and active-player self-stacking have already been landed across the currently supported priority windows.

This wave focuses on one missing interaction pattern:

- the non-active player receives priority
- casts an instant response
- keeps priority
- casts a second instant before passing

The goal is to complete the minimal stack model without opening new spell classes, triggered abilities, activated abilities, or full combat-step timing.

---

## Why This Wave Exists Now

The runtime already supports three stable building blocks:

- active-player instant casting in supported windows
- non-active instant response after the active player passes
- active-player self-stacking while retaining priority

What remains asymmetrical is self-stacking by the responding player.

Closing that gap provides high semantic value with low architectural risk because it reuses the same `CastSpell`, `PassPriority`, `SpellPutOnStack`, and `StackTopResolved` flow already proved elsewhere.

---

## Proposed Slice Order

### 1. Respond With Second Instant On Existing Stack

Baseline pattern slice.

Behavior:
- Bob receives priority with Alice's spell already on the stack
- Bob casts an instant
- Bob keeps priority
- Bob casts a second instant before passing
- the second response resolves first
- Bob's first response remains on the stack above Alice's original spell

Feature:
- `features/stack/respond_with_second_instant_spell.feature`

Planned implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-spell.md`

---

### 2. Respond With Second Instant In Upkeep Window

Behavior:
- after Alice passes the empty `Upkeep` window, Bob may cast two instants consecutively before passing

Feature:
- `features/stack/respond_with_second_instant_in_upkeep_window.feature`

Planned implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-upkeep-window.md`

---

### 3. Respond With Second Instant In Draw Window

Behavior:
- after Alice passes the post-draw `Draw` window, Bob may cast two instants consecutively before passing

Feature:
- `features/stack/respond_with_second_instant_in_draw_window.feature`

Planned implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-draw-window.md`

---

### 4. Respond With Second Instant In First Main Window

Behavior:
- after Alice passes the empty `FirstMain` window, Bob may cast two instants consecutively before passing

Feature:
- `features/stack/respond_with_second_instant_in_first_main_window.feature`

Planned implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-first-main-window.md`

---

### 5. Respond With Second Instant In Second Main Window

Behavior:
- after Alice passes the empty `SecondMain` window, Bob may cast two instants consecutively before passing

Feature:
- `features/stack/respond_with_second_instant_in_second_main_window.feature`

Planned implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-second-main-window.md`

---

### 6. Respond With Second Instant In End Step Window

Behavior:
- after Alice passes the empty `EndStep` window, Bob may cast two instants consecutively before passing

Feature:
- `features/stack/respond_with_second_instant_in_end_step_window.feature`

Planned implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-end-step-window.md`

---

### 7. Respond With Second Instant In Beginning Of Combat Window

Behavior:
- after Alice passes the beginning-of-combat window, Bob may cast two instants consecutively before passing

Feature:
- `features/stack/respond_with_second_instant_in_beginning_of_combat_window.feature`

Planned implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-beginning-of-combat-window.md`

---

### 8. Respond With Second Instant In Declare Blockers Window

Behavior:
- after Alice passes from the post-attacker handoff, Bob may cast two instants consecutively before passing in `DeclareBlockers`

Feature:
- `features/stack/respond_with_second_instant_in_declare_blockers_window.feature`

Implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-declare-blockers-window.md`

---

### 9. Respond With Second Instant In Combat Damage Window

Behavior:
- after Alice passes from the post-blocker handoff, Bob may cast two instants consecutively before passing in `CombatDamage`

Feature:
- `features/stack/respond_with_second_instant_in_combat_damage_window.feature`

Implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-combat-damage-window.md`

---

### 10. Respond With Second Instant In End Of Combat Window

Behavior:
- after Alice passes the `EndOfCombat` window, Bob may cast two instants consecutively before passing

Feature:
- `features/stack/respond_with_second_instant_in_end_of_combat_window.feature`

Implemented slice doc:
- `docs/slices/implemented/respond-with-second-instant-in-end-of-combat-window.md`

---

## Shared Invariants / Legality Rules

Every slice in this wave keeps the same minimal legality model:

- only the current priority holder may cast
- response spells remain limited to instants
- the responding player retains priority immediately after casting
- each additional response is a normal stack object and resolves LIFO
- when the top response resolves and the game remains active, priority reopens to the active player

---

## Explicitly Out Of Scope

- non-instant response spells
- activated abilities on the stack
- triggered abilities
- targets, modes, and replacement effects
- richer multiplayer priority handling
- full combat-step decomposition beyond the currently supported windows

---

## Ownership Check

This entire wave belongs to the `Game` aggregate inside the `play` bounded context.

Reason:
- priority ownership
- stack legality
- stack ordering
- resolution order

are all gameplay legality concerns already owned by the aggregate.

---

## Documentation Impact When Implementing

Each slice in this wave should update only the directly affected truth owners:

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md` only if aggregate-owned responsibilities materially change
- `docs/rules/rules-map.md`
- `features/README.md`
- the implemented slice doc itself

If a later slice makes an earlier stack slice historical or superseded, mark it explicitly.

---

## Rules Reference

- 117
- 505
- 506
- 507
- 508
- 509
- 510
- 511
- 601
- 608

---

## Rules Support Statement

This wave does not broaden DemonicTutor into a full Magic stack engine.

It only completes the already-supported minimal timing model by allowing the responding player to self-stack instants while they still hold priority in the currently supported windows.
