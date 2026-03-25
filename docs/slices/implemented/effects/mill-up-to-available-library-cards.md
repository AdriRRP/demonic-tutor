# Slice Implemented - Mill Up To Available Library Cards

## Outcome

The supported mill corridor now moves as many cards as possible from library to graveyard instead of failing when fewer than `N` cards remain.

## Supported Behavior

- self-mill and target-player mill both mill up to `N`
- if the library has fewer than `N` cards, all remaining cards are milled
- milling zero cards because the library is already empty remains a no-op

## Notes

- this closes a correctness gap in the first mill foundation
- it keeps mill distinct from draw-based lose-the-game behavior
