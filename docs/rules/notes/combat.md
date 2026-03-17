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

- attackers are declared in `Combat`
- blockers are declared in `Combat`
- blocker-to-attacker assignments are stored in runtime combat state and reused during damage resolution
- combat damage is assigned and marked on the creatures that receive it
- player life changes from unblocked combat damage are supported through the same life-adjustment semantics used elsewhere in the aggregate
- unblocked combat damage can end the game when it reduces a defending player to 0 life
- creatures with lethal marked damage are destroyed automatically through the shared state-based action review and moved to graveyard
- surviving creatures keep damage marked until the turn ends, then lose it automatically

## Out of Scope

- first strike
- double strike
- trample
- combat tricks on the stack
- a general state-based action engine

## Related Features

- `features/combat/combat_damage_marking.feature`
- `features/combat/creature_destruction.feature`
- `features/turn-flow/cleanup_damage_removal.feature`
