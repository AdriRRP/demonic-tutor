# Slice Proposal — Cast Spell With Generic Plus Colored Cost

## Goal

Support spell costs that require both generic mana and one colored mana symbol.

## Why This Slice Exists Now

The current colored mana foundation proves single-color costs. The next stable step is a mixed cost such as `1G`, because many later slices depend on costs that are not purely generic or purely single-colored.

## Supported Behavior

- a spell face may declare a mixed cost with one colored requirement plus generic mana
- casting succeeds when the mana pool satisfies both the colored requirement and the remaining generic amount
- colored mana may be consumed during that payment

## Invariants / Legality Rules

- required colored symbols must be paid before generic remainder is considered satisfied
- a payment attempt fails if the exact colored requirement is missing
- successful payment reduces the mana pool accordingly

## Out of Scope

- double-colored costs
- hybrid mana
- alternative or reduced costs

## Domain Impact

- extend mana-cost profiles beyond pure generic and pure single-color shortcuts
- extend payment logic to separate colored requirements from generic remainder

## Ownership Check

Casting legality and mana payment are aggregate-owned concerns because they determine whether a spell may enter the stack.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- the implemented slice doc for this capability

## Test Impact

- unit tests for successful mixed-cost payment
- executable BDD for a cast corridor that spends a mixed cost honestly

## Rules Reference

- 106
- 107.4
- 202
- 601.2f

## Rules Support Statement

This slice introduces a minimal mixed-cost model with one colored symbol plus generic mana. It does not yet imply full symbol parsing or broad multicolor support.
