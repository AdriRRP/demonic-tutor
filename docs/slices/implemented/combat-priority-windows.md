# Slice — Combat Priority Windows

## Goal

Open explicit priority windows after attackers and blockers are declared, so combat can host stack interaction before blockers and before combat damage.

## Supported Behavior

- entering `BeginningOfCombat` from `FirstMain` opens a priority window for the active player
- `DeclareAttackers` moves the game into `DeclareBlockers` and opens a priority window for the active player there
- `DeclareBlockers` moves the game into `CombatDamage` and opens a priority window for the active player there
- resolving combat damage moves the game into `EndOfCombat` and reopens a priority window for the active player when the game remains active
- those windows may be closed through consecutive `PassPriority` commands
- combat damage cannot be resolved until the current combat priority window is closed

## Explicit Limits

- this slice predates the explicit combat-subphase foundation and should now be read together with `combat-subphases-foundation.md`
- only the currently supported minimal stack semantics are available inside these windows
- response spells during combat are still limited to instants

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `BeginningOfCombat`
- `Game::declare_attackers()` now reopens `PriorityState` for the active player after attackers are locked in
- `Game::declare_blockers()` now reopens `PriorityState` for the active player after blockers are locked in
- `Game::resolve_combat_damage()` now reopens `PriorityState` for the active player after damage resolves if the game remains active
- combat-oriented test helpers now close empty combat priority windows explicitly before continuing to attackers, blockers, damage, or the move to `SecondMain`

## Rules Support Statement

This slice makes combat timing more semantically honest. With the later combat-subphase foundation in place, these windows now live in explicit combat moments instead of a single generic `Combat` phase.

## Tests

- entering `BeginningOfCombat` opens priority for the active player
- declaring attackers opens priority for the active player
- declaring blockers opens priority for the active player
- resolving combat damage reopens priority for the active player
- BDD coverage confirms those windows exist in the appropriate combat subphase
