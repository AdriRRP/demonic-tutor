# Slice Implemented - Define Supported Limited Set Card Profile Catalog

## Outcome

The engine now exposes one explicit authoring catalog for the first curated limited set, derived directly from `LibraryCard` definitions and the existing profile-based card model.

## What Landed

- one canonical `SupportedLimitedSetCardProfile` descriptor for authored cards
- classification for the currently supported curated-set families:
  - monocolor mana lands
  - creatures with supported keyword, activated, and triggered profiles
  - instant and sorcery spell cards through the existing explicit resolution profiles
  - artifacts through the current supported activated and triggered profiles
  - enchantments through the current supported Aura and controller-static profiles
  - planeswalkers through the current supported loyalty-ability subset
- rejection of cross-family authored shapes that exceed the current catalog, such as creatures carrying spell-resolution profiles
- `LibraryCard` now exposes the derived curated-set profile directly, so later validation can reuse the same source of truth

## Notes

- this slice intentionally defines the catalog without rejecting card-pool loads yet; that enforcement belongs to the next slice
- the catalog is compositional around the existing explicit runtime profiles rather than a separate text-driven card taxonomy
