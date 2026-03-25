# Slice Proposal - Support Explicit Counter Placement Effects

## Goal

Allow supported spells and abilities to place `+1/+1` counters onto a target creature or onto the source permanent itself.

## Why This Slice

Once counters exist in runtime, the next usable step is letting cards intentionally place them through a narrow explicit effect corridor.

## Scope

- explicit `put one +1/+1 counter on target creature`
- explicit `put one +1/+1 counter on this permanent`
- shared legality and resolution behavior for the supported target forms

## Out of Scope

- modal counter-placement effects
- distributing counters across multiple targets
- counters on players or other permanent types

## Notes

- build on the existing targeted-spell and activated-ability infrastructure
- keep count amounts small and explicit at first
