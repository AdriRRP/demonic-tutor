# Cast Planeswalker In Main Window

## Goal

Allow the active player to cast a planeswalker spell during an empty `FirstMain` or `SecondMain`
priority window and resolve it onto the battlefield.

## Scope

In scope:

- casting a planeswalker spell from hand while the active player holds priority in `FirstMain`
- casting a planeswalker spell from hand while the active player holds priority in `SecondMain`
- resolving the planeswalker spell from the stack to the battlefield

Out of scope:

- loyalty abilities
- counters
- planeswalker combat targeting

## Notes

- This slice formalizes planeswalkers as another sorcery-speed permanent spell in the current
  minimal stack model.
- The planeswalker is put onto the stack first and only enters the battlefield after stack
  resolution.
