# Slice — Combat Priority Windows

## Goal

Open explicit priority windows after attackers and blockers are declared, so combat can host stack interaction before blockers and before combat damage.

## Supported Behavior

- entering `Combat` from `FirstMain` opens a priority window for the active player
- `DeclareAttackers` opens a priority window for the active player
- `DeclareBlockers` opens a priority window for the active player
- resolving combat damage reopens a priority window for the active player when the game remains active
- those windows may be closed through consecutive `PassPriority` commands
- combat damage cannot be resolved until the current combat priority window is closed

## Explicit Limits

- the combat model still uses a single `Combat` phase rather than full combat substeps
- end-of-combat windows are still not modeled separately
- only the currently supported minimal stack semantics are available inside these windows
- response spells during combat are still limited to instants

## Domain Changes

- `advance_turn` now opens `PriorityState` when entering `Combat`
- `Game::declare_attackers()` now reopens `PriorityState` for the active player after attackers are locked in
- `Game::declare_blockers()` now reopens `PriorityState` for the active player after blockers are locked in
- `Game::resolve_combat_damage()` now reopens `PriorityState` for the active player after damage resolves if the game remains active
- combat-oriented test helpers now close empty combat priority windows explicitly before continuing to attackers, blockers, damage, or the move to `SecondMain`

## Rules Support Statement

This slice makes combat timing more semantically honest without yet introducing full combat-step modeling. Entering `Combat`, declaring attackers, declaring blockers, and resolving combat damage now open or reopen priority windows for the active player, allowing the current minimal instant-speed stack interaction before the next combat action.

## Tests

- entering `Combat` opens priority for the active player
- declaring attackers opens priority for the active player
- declaring blockers opens priority for the active player
- resolving combat damage reopens priority for the active player
- BDD coverage confirms those windows exist while the game remains in `Combat`
