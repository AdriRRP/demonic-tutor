# Slice Implemented - Preserve Card Ownership Across Battlefield Control Transfers

## Outcome

The runtime now preserves a card's owner identity even when that card is temporarily controlled from another player's battlefield.

## Supported Behavior

- `CardInstance` carries persistent owner identity across zone transitions and spell payload round-trips
- a foreign-owned permanent on your battlefield still goes to its owner's graveyard when it dies
- a foreign-owned permanent on your battlefield still returns to its owner's hand when bounced
- a foreign-owned permanent on your battlefield still enters its owner's exile zone when exiled
- the current battlefield container still models current control for supported combat, targeting, and activation corridors

## Notes

- this slice closes the ownership corruption that appeared when a permanent entered one player's battlefield without being owned by that same player
- it does not yet attempt a broader owner-vs-controller overhaul for all future control-changing effects
