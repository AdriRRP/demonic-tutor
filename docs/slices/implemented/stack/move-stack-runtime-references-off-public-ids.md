# Implemented Slice — Move Stack Runtime References Off Public Ids

## Summary

The stack runtime now stores controller and target identity through internal references instead of public ids.
`StackObject` carries `controller_index`, and spell targets inside the stack are kept as `StackTargetRef` values backed by player indices and handles.

## What Changed

- `StackObject` moved from `PlayerId` controller ownership to `controller_index`
- `SpellOnStack` now stores `StackTargetRef` instead of public-id-based `SpellTarget`
- casting converts external targets into internal refs before stack insertion
- resolution materializes public ids only when the boundary needs them again

## Outcome

- one of the hottest runtime structures no longer depends on public ids as its working identity
- event and command surfaces remain stable
- stack semantics remain unchanged
