# Slice Proposal — Activated Ability Uses Priority Windows

## Goal

Ensure the first supported non-mana activated ability uses the same explicit priority-window legality model as spell casting.

## Why This Slice Exists Now

Once a mana-ability foundation exists, the next honest expansion is a non-mana activated ability that proves abilities also live inside the current stack-and-priority system.

## Supported Behavior

- a supported activated ability may be activated only by the current legal actor in a supported window
- the activation uses the stack unless it is a mana ability

## Invariants / Legality Rules

- mana and non-mana activated abilities remain semantically distinct
- stack-using activations follow the same priority and pass model as supported spells

## Out of Scope

- triggered abilities
- modes or targets on abilities unless directly required
- loyalty abilities

## Domain Impact

- widen the aggregate-owned stack model to another object family only if directly required

## Ownership Check

This remains inside the `Game` aggregate because priority, stack ordering, and legality are aggregate-owned concerns.

## Documentation Impact

- current-state
- aggregate-game if the stack object model materially changes
- implemented slice doc

## Test Impact

- unit and BDD coverage for one stack-using activated ability corridor

## Rules Reference

- 117
- 602
- 405

## Rules Support Statement

This slice introduces one supported non-mana activated ability corridor only. It does not imply broad activated-ability support.
