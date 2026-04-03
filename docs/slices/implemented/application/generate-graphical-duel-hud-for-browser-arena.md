# Generate Graphical Duel HUD For Browser Arena

## Goal

Replace the old text-heavy duel cockpit with a graphical HUD that reads like a card game interface: a generated phase track, compact turn marker, icon-led seat stats, and visual active/priority sigils.

## Why This Slice Existed Now

The browser arena had already become playable, but the top of the screen still looked like a dashboard. The next smallest valuable step was to turn that chrome into a proper game HUD before adding deeper object-level visuals such as card piles and hidden opponent hands.

## Supported Behavior

- the cockpit now renders a generated phase track with one highlighted current phase and completed earlier phases
- turn number is shown through a compact rune-like counter instead of a labeled status box
- active player and priority holder are shown through seat sigils rather than repeated text labels
- seat life, hand count, and mana pool are shown as icon-led stat pips
- cockpit controls for hand, zones, log, room link, and reset now use generated glyph buttons instead of text-first buttons
- the battlefield header and empty battlefield state are reduced to compact graphic affordances instead of redundant labels

## Out Of Scope

- card-back rendering for library, graveyard, exile, or hidden opponent hands
- free battlefield positioning or drag layout persistence
- target lines, cast animations, or motion polish beyond the HUD transition states
- removing every remaining textual affordance from secondary modals like replay or zone browsers

## Rules Support Statement

This slice does not add new Magic rules.

It changes only how already-supported state is visualized in the browser arena.
