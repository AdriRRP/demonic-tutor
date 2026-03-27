# Slice Implementation - Support Pacifism Style Auras

## Outcome

Implemented.

## What landed

- one explicit attached combat-restriction profile for creature Auras
- enchanted creatures now cannot attack or block while the supported Aura remains attached
- declare attackers and declare blockers both reject the restricted creature
- the restriction is released automatically when the Aura leaves the battlefield

## Notes

- this remains a bounded `can't attack or block` subset
- it does not suppress activated abilities and does not introduce generic static-rule composition
