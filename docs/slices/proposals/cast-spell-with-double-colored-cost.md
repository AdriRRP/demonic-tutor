# Slice Proposal — Cast Spell With Double Colored Cost

## Goal

Support spell costs that require two mana of the same color.

## Why This Slice Exists Now

Once mixed costs exist, the next important pressure test is repeated colored requirements such as `GG`. That closes an important gap in payment semantics without yet opening multicolor complexity.

## Supported Behavior

- a spell face may declare a cost such as `GG`
- casting succeeds only if the mana pool contains at least two mana of that color
- payment consumes the exact colored amount

## Invariants / Legality Rules

- generic mana cannot substitute for missing colored requirements
- off-color mana cannot satisfy the repeated colored symbols
- the pool is reduced only on successful payment

## Out of Scope

- multi-color double symbols like `WU`
- hybrid or phyrexian costs
- cost reductions

## Domain Impact

- extend mana-cost profiles to carry repeated colored requirements
- harden payment logic for multiple symbols of the same color

## Ownership Check

This belongs to the same aggregate-owned mana payment corridor as current cost validation and spending.

## Documentation Impact

- `docs/domain/current-state.md`
- `docs/domain/DOMAIN_GLOSSARY.md`
- the implemented slice doc for this capability

## Test Impact

- unit tests for successful and rejected double-colored costs
- executable BDD for at least one positive gameplay cast

## Rules Reference

- 106
- 107.4
- 202
- 601.2f

## Rules Support Statement

This slice adds repeated same-color requirements to the minimal mana-cost model. It still avoids broader symbol families and alternative cost rules.
