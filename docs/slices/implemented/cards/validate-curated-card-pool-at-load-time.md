# Slice Implemented - Validate Curated Card Pool At Load Time

## Outcome

The engine now rejects curated-set library loads whose authored `LibraryCard` definitions exceed the supported limited-set profile catalog.

## What Landed

- curated-set validation now runs at the effective card-load boundary for authored `PlayerLibrary` input during opening-hand setup
- `deal_opening_hands` rejects the first library card whose expressed behavior is outside `SupportedLimitedSetCardProfile`
- the rejection is explicit in domain space through `GameError::UnsupportedCuratedCardProfile`
- regression coverage fixes the boundary with an authored creature that combines supported families in an unsupported way

## Notes

- the enforcement point is `PlayerLibrary` ingestion rather than `StartGameCommand`, because that is where authored card pools actually enter the aggregate today
- this slice intentionally validates against the current catalog only; it does not yet publish a designer-facing capability matrix
