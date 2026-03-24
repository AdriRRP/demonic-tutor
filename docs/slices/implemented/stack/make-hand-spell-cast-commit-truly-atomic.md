# Implemented Slice — Make Hand Spell Cast Commit Truly Atomic

## Summary

Tighten the player-owned hand-to-stack commit so failed spell preparation no longer mutates mana before ownership extraction is known to succeed.

## Supported Behavior

- spell preparation still returns a payload ready for the stack corridor
- failed preparation leaves both mana and hand ownership unchanged
- outward casting behavior remains the same

## Invariants

- failed spell preparation must not mutate the mana pool
- failed spell preparation must not mutate hand ownership
- this slice does not expand supported Magic rules

## Implementation Notes

- `Player::prepare_hand_spell_cast` now validates a stable hand handle first
- mana payment is simulated on a cloned pool and only committed after card extraction succeeds
- internal hand removal and arena extraction now share the same stable handle

## Tests

- full repository validation remains green after the atomicity refactor
