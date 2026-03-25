# Creature Hexproof Targeting Foundation

## Summary

Supported targeted spells now reject opposing creatures with `Hexproof` during cast and again on resolution revalidation.

## Scope

- creature targets with `Hexproof` cannot be targeted by opponents
- the creature controller can still target its own `Hexproof` creature
- permanent-targeting effects inherit the same protection when the permanent is a `Hexproof` creature

## Notes

- this slice is intentionally limited to creature `Hexproof`
- it does not claim a broader implementation for every future permanent type or multiplayer nuance
