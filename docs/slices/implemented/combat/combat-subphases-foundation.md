# Slice — Combat Subphases Foundation

## Goal

Replace the old single `Combat` phase with explicit combat subphases so turn flow, legality, and priority windows can align with the actual moments the runtime already models.

## Supported Behavior

- the phase model now distinguishes:
  - `BeginningOfCombat`
  - `DeclareAttackers`
  - `DeclareBlockers`
  - `CombatDamage`
  - `EndOfCombat`
- advancing from `FirstMain` enters `BeginningOfCombat`
- closing the empty beginning-of-combat window advances into `DeclareAttackers`
- `DeclareAttackers` now moves the game into `DeclareBlockers` and opens priority there
- `DeclareBlockers` now moves the game into `CombatDamage` and opens priority there
- `ResolveCombatDamage` now moves the game into `EndOfCombat` and opens priority there while the game remains active
- closing the empty end-of-combat window advances into `SecondMain`

## Explicit Limits

- the combat model is still a simplified subset of Magic timing
- no separate attacker-declaration priority before attackers beyond the explicit `BeginningOfCombat` window
- no first strike, double strike, trample, or multiple blockers per attacker
- no triggered abilities tied to combat-step transitions

## Domain Changes

- `Phase` now models explicit combat subphases instead of a single `Combat`
- combat legality now keys off the specific subphase:
  - `declare_attackers` in `DeclareAttackers`
  - `declare_blockers` in `DeclareBlockers`
  - `resolve_combat_damage` in `CombatDamage`
- combat-related setup helpers and BDD worlds now advance through those explicit subphases instead of assuming a generic `Combat` phase

## Rules Support Statement

This slice makes combat timing substantially more honest without attempting a full rules-complete combat engine. The runtime now exposes the combat moments it already cared about semantically, which lowers ambiguity in legality checks, priority windows, and future combat growth.

## Tests

- turn progression traverses explicit combat subphases
- declaring attackers leaves the game in `DeclareBlockers` with priority for the active player
- declaring blockers leaves the game in `CombatDamage` with priority for the active player
- resolving combat damage leaves the game in `EndOfCombat` with priority for the active player while the game remains active
- executable combat and stack BDD scenarios continue to pass against the explicit subphase model
