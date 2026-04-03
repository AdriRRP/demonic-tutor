# Render Hidden Opponent Hand Fan In Browser Arena

## Goal

Show the opponent's hand as a real fan of face-down cards instead of reducing it to a numeric counter, while keeping hidden information intact and avoiding extra image downloads.

## Why This Slice Existed Now

Once the duel HUD and zone piles were already reading as graphical tabletop objects, the opponent hand remained the most obviously abstract part of the arena. The next smallest valuable step was to give it the same physical card language as the player's own hand, but with generated backs and no inspection or zoom behavior.

## Supported Behavior

- the opponent seat now renders a compact fan of face-down cards near the top edge of its battlefield half
- the fan uses the same CSS-generated card back primitive as the library pile, keeping hidden information shielded
- the number of visible reversed cards follows the viewer-scoped `hand_count`
- the opponent hand stays non-interactive: no zoom, no inspect detail, and no direct actions
- the duel layout was tightened so the top seat, zone rail, and battlefield fit more cleanly inside the viewport

## Out Of Scope

- revealing real opponent card identities after game end
- animating draws, discards, or hand-to-stack motion
- syncing free battlefield placement between windows
- changing any gameplay or hidden-information rules

## Rules Support Statement

This slice does not add new Magic rules.

It changes only how already-supported hidden hand counts are visualized in the browser arena.
