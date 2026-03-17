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
- combat damage is assigned and marked on the creatures that receive it
- player life changes from unblocked combat damage are supported
- creatures with lethal marked damage are destroyed automatically and moved to graveyard
- surviving creatures keep damage marked as runtime state

## Out of Scope

- first strike
- double strike
- trample
- combat tricks on the stack
- cleanup-based damage removal
- a general state-based action engine

## Related Features

- `features/combat/combat_damage_marking.feature`
- `features/combat/creature_destruction.feature`
