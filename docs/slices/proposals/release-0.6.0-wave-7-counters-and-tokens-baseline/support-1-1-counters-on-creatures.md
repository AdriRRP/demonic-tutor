# Slice Proposal - Support Plus One Plus One Counters On Creatures

## Goal

Add the first explicit permanent counter type to creature runtime: `+1/+1` counters.

## Why This Slice

`+1/+1` counters are a compact, high-return capability that makes a large family of creatures, ETB effects, and activated abilities meaningfully playable.

## Scope

- creature runtime tracks an integer count of `+1/+1` counters
- counters modify current power and toughness in the supported subset
- counters survive damage cleanup and turn progression
- SBA and combat use the updated stats

## Out of Scope

- named counters beyond `+1/+1`
- counter movement between permanents
- proliferate or doubling effects

## Notes

- treat counters as explicit runtime state, not temporary pump
- keep the first counter model narrow and deterministic
