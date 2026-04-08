# Present Viewer-Scoped Opening-Hand Overlay For Remote Duels

## Goal

Make the restored `Setup` phase usable in the browser by presenting an Arena-like opening-hand overlay that keeps each player on their own hand while only the current chooser can act.

## Why This Slice Existed Now

After the adapter stopped skipping `Setup`, the browser still needed an interaction model that did not regress into hot-seat behavior or opaque blocking modals. The next smallest valuable step was a viewer-scoped overlay that leaves the table visible, keeps each player's own opening hand in view, and only enables the active keep-or-mulligan decision on the correct device.

## Supported Behavior

- each browser instance now keeps its own opening hand visible during `Setup`
- the pregame UI announces whether the local player goes first or second
- only the current decision holder can press `Keep` or `Mulligan`
- waiting players still see their own hand while the overlay honestly explains that the other player is deciding
- the overlay completes pregame by advancing to the first real main-phase priority window only after both players have kept

## Out Of Scope

- detailed mulligan heuristics or recommendations
- polished opening coin-flip animation
- full production-grade Arena pregame parity
- rules support beyond the repository's existing simplified mulligan

## Rules Support Statement

This slice does not add new Magic rules.

It presents the repository's existing `Setup` and simplified mulligan support through a viewer-scoped remote-duel overlay.

## Historical Note

Later browser setup slices extended this overlay with repeated London-style mulligans, explicit bottom-card selection, and direct bottoming from the visible hand fan. This document remains the history of the first truthful viewer-scoped pregame overlay rather than the full current mulligan UX.
