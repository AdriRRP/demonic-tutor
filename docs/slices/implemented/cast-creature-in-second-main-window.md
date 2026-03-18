# Cast Creature In Second Main Window

## Goal

Make explicit that the active player may cast and resolve a creature spell during the empty
`SecondMain` priority window.

## Scope

In scope:

- casting a creature spell from hand while the active player holds priority in `SecondMain`
- resolving the creature spell from the stack onto the battlefield

Out of scope:

- extending creature timing beyond the current sorcery-speed model
- creature responses on an already open stack

## Notes

- This slice does not introduce new runtime timing. It fixes executable and documentary truth for a
  creature case that the current stack model already supports.
- The creature still goes onto the stack first and only enters the battlefield after stack
  resolution.
