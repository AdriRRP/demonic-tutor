# Slice Name

Generalize Tap Activated Abilities

## Goal

Support a broader family of tap-cost activated abilities beyond the first hard-coded life-gain example.

## Why This Slice Exists Now

The first stack-using activated ability corridor already exists. Generalizing it unlocks many practical permanents without needing a new timing model.

## Supported Behavior

- allow supported permanents to activate tap abilities with one explicit effect profile
- support explicit target requirements when needed
- put the activated ability on the stack as a first-class stack object

## Invariants / Legality Rules

- the source permanent must be untapped and under the correct controller
- tapping is part of the activation cost
- activating the ability must use the same priority rules already modeled for non-mana activations

## Out of Scope

- untap-symbol abilities
- modal activations
- activated abilities with variable `X` costs

## Domain Impact

### Aggregate Impact

- extend non-mana activated ability profiles and activation legality

## Ownership Check

This belongs to the `Game` aggregate because activation legality, tap costs, and stack insertion are gameplay-domain rules.

## Documentation Impact

- `docs/domain/current-state.md`
- this slice doc

## Test Impact

- activate a supported tap ability with no target
- activate a supported tap ability with target
- reject activation while tapped

## Rules Reference

- 602
- 117
- 405

## Rules Support Statement

This slice broadens the current supported tap-ability corridor but still stays in an explicit profile-based subset, not full free-form activated abilities.

