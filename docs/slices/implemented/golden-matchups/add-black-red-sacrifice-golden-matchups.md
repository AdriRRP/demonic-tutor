# Add Black-Red Sacrifice Golden Matchups

## Status

- implemented

## Goal

- prove that the current curated subset can express a black-red value matchup through discard, sacrifice-cost activation, creature removal, and creature recursion

## What this slice adds

- executable cucumber coverage for a sacrifice-cost outlet paid before resolution
- executable cucumber coverage for targeted discard choosing a specific card from hand
- executable cucumber coverage for removal into graveyard recursion back to hand and recast

## Scope

- one sacrifice-cost artifact line
- one discard line
- one removal-plus-recursion line

## Out of scope

- generalized aristocrats payoffs
- sacrifice of arbitrary permanents other than explicit source-sacrifice abilities
- token-sacrifice engines
- death-trigger payoff webs

## Notes

- this slice is intentionally narrower than a full black-red sacrifice deck
- it only proves the currently supported engine corridors in a matchup-shaped executable form
