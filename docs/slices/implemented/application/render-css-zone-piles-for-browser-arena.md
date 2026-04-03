# Render CSS Zone Piles For Browser Arena

## Goal

Replace the old numeric zone anchors with physical-looking library, graveyard, and exile piles generated entirely in CSS, so public zones read like tabletop objects instead of status widgets.

## Why This Slice Existed Now

Once the duel HUD became compact and more graphical, the next biggest source of "dashboard" feel was the right-hand rail of zone counters. The smallest valuable follow-up was to turn those counters into piles of cards with a reusable card back primitive and compact face-up tops for visible public zones.

## Supported Behavior

- the right rail now renders library, graveyard, and exile as compact card piles instead of text counters
- library is shown as a face-down pile using a generated card back built entirely from CSS gradients and shapes
- graveyard and exile show a compact face-up top card when one exists, falling back to an alternate pile back when empty
- each pile keeps its count visible through a compact glyph-and-count capsule
- clicking any pile opens the focused zone browser for that zone without adding new permanent panels to the battlefield
- the generated card back primitive is reusable for future hidden-information surfaces such as the opponent hand fan

## Out Of Scope

- syncing freeform battlefield positions across windows
- replacing the opponent hand with hidden card backs
- target lines, cast motion, or deeper animation polish
- adding new public zone semantics or changing replay behavior

## Rules Support Statement

This slice does not add new Magic rules.

It changes only the browser presentation of already-supported public zones.
