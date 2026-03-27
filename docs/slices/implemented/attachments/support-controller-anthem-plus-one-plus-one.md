# Slice Implementation - Support Controller Anthem Plus One Plus One

## Outcome

Implemented.

## What landed

- one explicit controller-scoped anthem profile for permanents on the battlefield
- controlled creatures get `+1/+1` while the supported anthem remains on the battlefield
- the bonus applies both to creatures already on the battlefield and to creatures that enter later
- the bonus is removed automatically when the anthem leaves the battlefield

## Notes

- this remains the first bounded non-attachment static battlefield effect
- it does not introduce general continuous-effect layers or keyword-granting anthems
