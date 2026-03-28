# Release 0.8.0 - Wave 2 - Limited Set Authoring Contract

Goal:

- freeze what a card designer may and may not express in the first playable curated set

Slice count:

- `4`

Slices:

- `define-supported-limited-set-card-profile-catalog`
  - declare the exact supported card-profile families allowed in the first playable set
- `validate-curated-card-pool-at-load-time`
  - reject cards whose declared behavior exceeds the supported profile catalog
- `publish-card-capability-matrix-for-set-design`
  - provide one canonical matrix that maps supported profiles to rules support
- `add-limited-set-deck-construction-rules-baseline`
  - formalize the exact deckbuilding assumptions for the first curated environment

Why this wave has high return:

- it stops the UI and content layers from drifting into unsupported card promises
