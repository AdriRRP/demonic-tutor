# Sorcery-Speed Spells Share One Timing Rule

## Goal

Make the current sorcery-speed timing rule explicit in the model so the casting code reads in
domain terms instead of as an ad-hoc boolean combination.

## Scope

In scope:

- introducing a `CardType` helper that identifies sorcery-speed spell types
- using that helper inside stack casting legality checks
- adding unit coverage for the helper semantics

Out of scope:

- changing gameplay behavior
- introducing a broader timing policy framework

## Notes

- This is a semantic cleanup slice: the runtime behavior stays the same.
- The goal is readability and maintainability, not a new rule.
