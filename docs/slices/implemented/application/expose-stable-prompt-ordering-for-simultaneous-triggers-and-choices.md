# Expose Stable Prompt Ordering For Simultaneous Triggers And Choices

## Status

Implemented

## Goal

Make the current public client contract deterministic when the supported subset surfaces more than one prompt at once.

## Scope

- sort same-controller battlefield step triggers in one explicit deterministic order
- keep simultaneous trigger batches surfaced in a stable order for the currently supported upkeep, end-step, ETB, and dies corridors
- sort public choice requests and target candidate lists so clients do not depend on incidental storage iteration

## Out Of Scope

- player-chosen ordering for simultaneous triggers
- a general priority-time trigger ordering UI
- broader Magic trigger-order support beyond the current explicit subset

## Notes

- the current supported subset now uses one deterministic fallback order for simultaneous triggers instead of implying full Magic trigger-order choice
- public prompt ordering is stabilized for replayable UI behavior, not widened into a new gameplay mechanic
