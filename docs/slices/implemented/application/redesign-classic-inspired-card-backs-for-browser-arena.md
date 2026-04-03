# Redesign Classic-Inspired Card Backs For Browser Arena

## Goal

Refine the generated browser card-back primitive so hidden-information surfaces capture the essence of the classic Magic card back with a simpler, better-fitted composition that stays fully CSS-generated and lightweight.

## Why This Slice Existed Now

Once library, graveyard, exile, and the opponent hand were already using generated card backs, the remaining visual mismatch was the back design itself. The next smallest valuable step was to simplify that primitive so it reads closer to the original card back at gameplay sizes without introducing raster assets.

## Supported Behavior

- the shared browser card-back primitive now uses a simplified classic-inspired composition with a brown marbled frame, blue oval, warm center core, mana-like gems, and a textless deckmaster-style plaque
- the redesigned back remains fully CSS-generated, with no downloaded background images
- library keeps the default saturated classic-inspired back
- graveyard and exile keep their existing semantic distinction through cooler and more desaturated variants of the same back language
- the hidden opponent hand fan reuses the same redesigned back primitive without changing its gameplay visibility behavior

## Out Of Scope

- changing any gameplay rules or visibility semantics
- adding real card art to the back surface
- reintroducing logo text or decorative lettering on the card back
- replacing the front card frame or battlefield interaction model
- introducing remote hidden-information security guarantees

## Rules Support Statement

This slice does not add new Magic rules.

It only refines the browser presentation of already-supported hidden-information and public-zone surfaces.
