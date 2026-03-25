# Slice Proposal - Support Scry On Explicit Card Profiles

## Goal

Support a bounded `scry` subset so the acting player can inspect the top card(s) of their library and choose keep-on-top versus move-to-bottom ordering.

## Why

- `scry` is one of the highest-value library-manipulation mechanics for a limited-style set
- it forces the public contract to support top-of-library hidden-information prompts
- it gives the future UI a canonical pattern for inspect-and-order choices

## In Scope

- `scry 1` and, only if trivial after that, `scry 2`
- explicit card profiles that invoke scry on resolution
- prompt surface that reveals only the acting player's looked-at cards
- decisions:
  - keep on top
  - move to bottom
  - for `scry 2`, supported top/bottom ordering within the bounded subset

## Out of Scope

- shuffle effects
- search-library effects
- surveil
- generic arbitrary library reordering

## Acceptance

- the acting player can inspect the top card(s)
- the chosen keep/bottom outcome changes the future draw order accordingly
- hidden information stays hidden from the opponent in the public projection

## Notes

- this slice should introduce the first explicit hidden-information choice response carried through the public game contract
