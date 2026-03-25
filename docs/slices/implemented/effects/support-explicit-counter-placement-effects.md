# Slice Implemented - Support Explicit Counter Placement Effects

## Outcome

The supported subset can now place `+1/+1` counters through explicit spell and activated-ability corridors.

## What Changed

- added one explicit spell profile for `put one +1/+1 counter on target creature`
- added one explicit activated-ability effect for `put one +1/+1 counter on this source creature`
- creature spell payloads now preserve activated abilities when the creature resolves onto the battlefield

## Supported Behavior

- targeted counter-placement spells revalidate the battlefield creature target on resolution
- supported activated abilities can grow their own source creature

## Notes

- this slice keeps amounts fixed at one counter
- broader modal or distributed counter placement remains out of scope
