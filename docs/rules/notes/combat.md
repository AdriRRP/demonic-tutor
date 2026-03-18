# Rules Notes — Combat

## Purpose

Summarize the rule areas DemonicTutor currently uses to model combat.

This is a repository-owned interpretation note, not a copy of the Comprehensive Rules.

## Relevant Rules

- 506 — Combat phase
- 508 — Declare attackers
- 509 — Declare blockers
- 510 — Combat damage step

## Current DemonicTutor Interpretation

- combat currently uses explicit subphases:
  - `BeginningOfCombat`
  - `DeclareAttackers`
  - `DeclareBlockers`
  - `CombatDamage`
  - `EndOfCombat`
- attackers are declared in `DeclareAttackers`
- blockers are declared in `DeclareBlockers`
- entering `BeginningOfCombat` opens a priority window for the active player before attackers are declared
- declaring attackers opens a priority window for the active player
- declaring blockers opens a priority window for the active player
- resolving combat damage moves the game into `EndOfCombat` and reopens a priority window for the active player when the game remains active
- blocker-to-attacker assignments are stored in runtime combat state and reused during damage resolution
- the current combat model supports at most one blocker per attacker
- combat damage is assigned and marked on the creatures that receive it
- player life changes from unblocked combat damage are supported through the same life-change semantics used by explicit targeted life effects elsewhere in the aggregate
- unblocked combat damage can end the game when it reduces a defending player to 0 life
- creatures with lethal marked damage are destroyed automatically through the shared state-based action review and moved to graveyard
- surviving creatures keep damage marked until the turn ends, then lose it automatically

## Out of Scope

- first strike
- double strike
- trample
- multiple blockers per attacker
- rules-complete combat timing beyond the currently supported explicit subphases
- a general state-based action engine

## Related Features

- `features/combat/combat_priority_windows.feature`
- `features/combat/beginning_of_combat_priority_window.feature`
- `features/combat/post_combat_damage_priority_window.feature`
- `features/combat/combat_damage_marking.feature`
- `features/combat/creature_destruction.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
