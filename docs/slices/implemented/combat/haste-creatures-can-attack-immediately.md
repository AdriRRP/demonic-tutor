# Implemented Slice — Haste Creatures Can Attack Immediately

## Summary

Support `Haste` so a creature may attack on the turn it entered the battlefield under its controller's control.

## Supported Behavior

- a creature with `Haste` may be declared as an attacker on the turn it entered
- non-haste creatures keep the current summoning-sickness restriction

## Invariants

- `Haste` changes attack legality only for the currently supported declare-attackers model
- this slice does not imply broader keyword support beyond the explicit `Haste` exception

## Implementation Notes

- attack declaration now checks summoning sickness together with the creature's keyword set
- the current runtime still models only the supported attack-legality corridor, not temporary haste-granting effects

## Tests

- unit coverage for immediate attack legality
- executable BDD coverage in `features/combat/haste_creatures_can_attack_immediately.feature`
