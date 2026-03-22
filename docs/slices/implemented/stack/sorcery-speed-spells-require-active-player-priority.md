# Sorcery-Speed Spells Require Active-Player Priority

## Goal

Make explicit that sorcery-speed spells still require the active player's priority even when the
current main-phase window is open and the stack is empty.

## Scope

In scope:

- rejecting a non-active player's artifact cast in an empty `FirstMain` priority window
- executable unit coverage proving the same restriction for creatures, sorceries, enchantments, and
  planeswalkers

Out of scope:

- introducing a richer timing-specific error taxonomy
- broadening non-active access to sorcery-speed timing

## Notes

- The current minimal stack model allows sorcery-speed spells only for the active player in
  `FirstMain` or `SecondMain` while the stack is empty.
- This slice turns that cross-cutting timing rule into explicit repository truth instead of leaving
  it implied by `cast_spell` internals.
