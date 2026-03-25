# Slice Implemented - Support Plus One Plus One Counters On Creatures

## Outcome

Creature runtime now tracks explicit `+1/+1` counters as permanent state.

## What Changed

- creature runtime stores an integer count of `+1/+1` counters
- current power and toughness now include those counters
- lethal-damage and zero-toughness checks use the updated stats

## Supported Behavior

- `+1/+1` counters survive normal turn progression and damage cleanup
- combat and SBA observe the counter-adjusted stats

## Notes

- this slice introduces the runtime state only
- explicit effects that place counters build on top of this foundation
