# Slice Name

ZeroToughnessCreatureDies

---

## Goal

Destroy a creature automatically when it enters the battlefield with toughness 0.

---

## Why This Slice Exists Now

This slice follows `CastSpell`, `CreatureDestruction`, and terminal game-state slices because:

1. the runtime already models creature toughness and graveyard movement
2. creature spells already enter the battlefield directly
3. zero-toughness death is a small but meaningful state-based behavior
4. it improves semantic truthfulness without requiring a general SBA engine

---

## Supported Behavior

- when a creature spell resolves to the battlefield with toughness 0, it immediately dies
- the creature leaves the battlefield and enters its controller's graveyard
- `SpellCast` is still emitted with `EnteredBattlefield`
- `CreatureDied` is emitted for the creature that dies

---

## Invariants / Legality Rules

- this slice only supports the current runtime representation of toughness `0`
- it does not model negative toughness because creature toughness is currently represented as `u32`
- the zero-toughness check is currently applied after creature-spell resolution
- the behavior remains automatic game logic, not a player command

---

## Out of Scope

- a general state-based action engine
- negative toughness
- continuous effects that change power and toughness
- counters
- creature death from combat damage beyond the existing combat slice
- triggered abilities that respond to death

---

## Domain Impact

### Aggregate Impact

- `CastSpell` may now produce both `SpellCast` and `CreatureDied`

### Entity / Value Object Impact

- `CardInstance` exposes zero-toughness detection

### Commands

- no new public command required

### Events

- reuse `CreatureDied`

### Errors

- no new public error required

---

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/aggregate-game.md`
- `docs/rules/rules-map.md`
- `docs/slices/implemented/cast-spell.md`
- `features/state-based-actions/zero_toughness_creature_dies.feature`
- this slice document

---

## Test Impact

- casting a 0-toughness creature spell emits `SpellCast`
- that creature does not remain on the battlefield
- that creature enters the graveyard
- `CreatureDied` is emitted
- ordinary creature and noncreature spell casting still behave as before

---

## Rules Reference

- 704.5f — A creature with toughness 0 or less is put into its owner's graveyard

---

## Rules Support Statement

This slice implements a narrow zero-toughness death check tied to the repository's current creature-spell resolution path. It does not yet implement a general state-based action engine, and because toughness is currently modeled as `u32`, it only represents the `0 toughness` portion of the full rule.
