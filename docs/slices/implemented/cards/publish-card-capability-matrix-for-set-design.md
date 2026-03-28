# Slice Implemented - Publish Card Capability Matrix For Set Design

## Outcome

The project now exposes one canonical capability matrix for the first curated limited set, so card design can stay aligned with the authored-card catalog and the load-time validator.

## What Landed

- one canonical matrix in `docs/domain/limited-set-capability-matrix.md`
- explicit supported authored families for:
  - lands
  - creatures
  - instants and sorceries
  - artifacts
  - enchantments
  - planeswalkers
- one explicit list of supported casting permissions already exercised by the engine
- one explicit boundary for what the curated set still must not express

## Notes

- this matrix is descriptive, not speculative: it follows the current code and the enforced `SupportedLimitedSetCardProfile` catalog
- if a desired card shape is missing from the matrix, it should become a new slice rather than an informal exception
