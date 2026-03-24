# Implemented Slice — Finish Dual-Layer Identity At True Boundaries

## Summary

Push more of the runtime core onto player indices, card handles, and stack object numbers so public string-backed ids are materialized mainly at commands, events, errors, and tests.

## Supported Behavior

- gameplay behavior remains unchanged
- outward-facing ids stay deterministic and reviewable
- internal turn flow, resource actions, stack targeting, and combat links now rely more on compact runtime references

## Invariants

- public ids remain stable at aggregate boundaries
- internal identity stays explicit through indices, handles, and object numbers
- this slice does not expand supported Magic rules

## Implementation Notes

- resource actions and turn-flow effects now consume player indices once known by the aggregate instead of re-resolving `PlayerId` inside hot rules
- stack runtime objects and combat runtime links now carry internal references first and materialize public ids only when crossing outward-facing boundaries
- aggregate location lookup and player-owned card access continue the shift toward handle-first runtime semantics

## Tests

- full repository validation remains green after the identity-boundary refactor
