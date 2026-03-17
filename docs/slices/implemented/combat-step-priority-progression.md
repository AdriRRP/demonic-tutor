# Slice — Combat Step Priority Progression

## Goal

Make the priority corridor between explicit combat subphases an explicit, traceable slice instead of a behavior inferred across several smaller combat timing slices.

## Supported Behavior

- `DeclareAttackers` advances combat into `DeclareBlockers`
- that transition reopens priority for the active player
- `DeclareBlockers` advances combat into `CombatDamage`
- that transition reopens priority for the active player
- resolving combat damage advances combat into `EndOfCombat`
- that transition reopens priority for the active player while the game remains active

## Explicit Limits

- this slice only formalizes priority progression between currently supported combat subphases
- it does not model triggered abilities, combat tricks with richer target semantics, or additional combat steps
- response spells inside those windows remain limited to the currently supported minimal instant-speed stack behavior

## Domain Changes

- no new public command is introduced
- the explicit combat-step model now has a documented priority corridor from `DeclareAttackers` through `EndOfCombat`
- combat timing slices that predate explicit subphases should be read together with this slice

## Rules Support Statement

This slice makes the internal combat corridor easier to reason about. Instead of treating post-attack, post-block, and post-damage interaction as isolated windows, the runtime now exposes a coherent progression of explicit combat subphases, each with its own priority handoff.

## Tests

- BDD coverage confirms that declaring attackers advances into `DeclareBlockers` with priority
- BDD coverage confirms that declaring blockers advances into `CombatDamage` with priority
- BDD coverage confirms that resolving combat damage advances into `EndOfCombat` with priority
