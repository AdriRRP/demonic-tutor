# Release 0.7.0 - Wave 1 - UI Gameplay Contract

Goal:

- make the domain usable from a real frontend without pushing gameplay logic into UI code

Slice count:

- `4`

Slices:

- `expose-canonical-game-snapshot-projection`
  - publish a stable read model for phase, priority, players, battlefield, stack, hand counts, graveyards, exile, and visible prompts
- `expose-legal-actions-for-the-current-priority-holder`
  - surface only the actions the current actor may actually take in the supported subset
- `expose-pending-choice-requests-in-the-public-game-view`
  - make discard, modal, target, and future optional-choice prompts explicit to the application layer
- `add-deterministic-command-result-envelope-for-ui-clients`
  - standardize success, illegality, emitted public events, and next-prompt shape so UI flow is predictable

Why this wave has high return:

- it is the earliest point where UI can start without re-deriving rules from raw internals
