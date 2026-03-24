# Implemented Slice — Move Combat Runtime Links To Internal Card Refs

## Summary

Combat-time creature links no longer store the attacked creature as a public `CardInstanceId`.
The blocking relationship now keeps the attacked creature through the attacker's internal `PlayerCardHandle`.

## What Changed

- `CreatureRuntime.blocking_target` now uses `PlayerCardHandle`
- blocker assignment resolves the attacker handle at declaration time
- combat damage participant collection materializes the outward attacker id only when needed for the current combat calculations and events

## Outcome

- combat runtime state is more compact and more internal-reference-driven
- outward combat behavior and the current one-blocker model remain unchanged
- future combat slices can build on internal refs instead of textual card links
