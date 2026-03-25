# Slice Implemented - Support Modal Choose One Spells

## Outcome

The engine now supports the first explicit `choose one` spell corridor.

## What Landed

- a narrow `ModalSpellMode` command and stack choice model
- one explicit supported spell profile:
  - `choose one: target player gains life`
  - `choose one: target player loses life`
- cast-time rejection when a required modal mode is missing
- stack storage of the selected mode through resolution
- public choice projection for modal spells so clients can surface the available modes

## Notes

- this is intentionally a bounded first corridor, not a generic modal prompt engine
- later slices can add more modal spell families without changing the public contract shape
