# Slice Implementation - Support Stat Boost Auras

## Outcome

Implemented.

## What landed

- one explicit attached stat-bonus profile for creature Auras
- enchanted creatures now project `+N/+N` while the supported Aura stays attached
- the runtime removes that bonus when the Aura leaves the battlefield
- the public snapshot reflects the boosted creature stats without adding a separate static-layer model

## Notes

- this remains a bounded attached-static-effect subset
- it does not introduce general layers, timestamps, or mixed keyword-plus-stat Aura composition
