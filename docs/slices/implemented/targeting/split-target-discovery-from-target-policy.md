# Implemented Slice — Split Target Discovery From Target Policy

## Summary

Separate target lookup and target existence checks from rule-policy evaluation so targeting stays easier to extend semantically.

## Supported Behavior

- target discovery now resolves a typed target shape before rule-policy evaluation
- actor-relative legality stays unchanged for player, creature, and graveyard-card targets
- cast-time and resolution-time targeting still share the same legality corridor

## Invariants

- missing-target errors remain explicit
- target-kind mismatches remain explicit
- this slice does not expand supported Magic rules

## Implementation Notes

- the targeting corridor now resolves targets into a smaller internal semantic shape
- rule-policy evaluation consumes that resolved shape instead of rediscovering state inline
- the orchestration layer only translates lookup failures and rule outcomes

## Tests

- full repository validation remains green after the targeting refactor
