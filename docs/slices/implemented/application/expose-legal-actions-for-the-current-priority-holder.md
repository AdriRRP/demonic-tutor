# Expose Legal Actions For The Current Priority Holder

## Goal

Expose the current supported action surface as a public menu so clients do not have to infer legality from phase and card state.

## Why This Slice Existed Now

Without a legal-action surface, any UI would need to rebuild turn, priority, and card legality logic outside the aggregate.

## Supported Behavior

- expose pass-priority when a priority window is open
- expose playable lands, tappable mana sources, castable spells, and activatable abilities for the current holder
- expose attack, block, combat-damage, turn-advance, and cleanup-discard actions in the supported non-priority states
- expose blocker options in a client-friendly form

## Out Of Scope

- exhaustive combinatorial attack/block assignment generation
- speculative future actions
- hidden convenience shortcuts that bypass aggregate validation

## Rules Support Statement

This slice does not widen gameplay support.

It exposes only the actions that the currently supported subset can already execute.
