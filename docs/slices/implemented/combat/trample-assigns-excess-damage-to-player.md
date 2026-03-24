# Implemented Slice — Trample Assigns Excess Damage To Player

## Summary

Support `Trample` so a blocked attacker assigns lethal damage to its single blocker and the remaining combat damage to the defending player.

## Supported Behavior

- a blocked attacker with `Trample` deals lethal combat damage to its blocker first
- any remaining damage is dealt to the defending player
- the current support applies only inside the one-blocker combat model

## Invariants

- the blocker must be assigned lethal damage before excess reaches the player
- this slice does not support multiple blockers, prevention effects, or deathtouch interactions

## Implementation Notes

- the combat-damage assignment path now reads the blocker's current toughness state and marked damage
- the blocker still deals its combat damage back normally in the same supported combat-damage step

## Tests

- unit coverage for excess-damage assignment to the defending player
- executable BDD coverage in `features/combat/trample_assigns_excess_damage_to_player.feature`
