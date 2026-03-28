# Slice Implemented - Add Limited Set Deck Construction Rules Baseline

## Outcome

The project now defines one canonical deckbuilding baseline for the first curated limited environment.

## What Landed

- one explicit baseline in `docs/domain/limited-set-deck-construction-baseline.md`
- clear separation between:
  - rules already enforced by the aggregate
  - product and content assumptions that are not yet engine-enforced
- explicit treatment of:
  - two-player best-of-one scope
  - `40`-card main decks as the canonical limited target
  - no sideboard in the first curated environment
  - ordered `PlayerLibrary` input and caller-owned shuffle policy
  - copy-count legality remaining outside aggregate enforcement today

## Notes

- this slice is intentionally a contract-and-coherence slice, not a new deck-validation subsystem
- if the project later needs hard enforcement for deck size, copy caps, or sideboards, that should land as a separate slice
