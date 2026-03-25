# Slice Proposal - Support Modal Choose One Spells

## Goal

Support the first explicit `choose one` spell corridor so a card can present multiple supported modes and the caster selects exactly one mode during cast.

## Why

- unlocks a broad family of limited-friendly cards without needing a general text parser
- forces the public contract to expose bounded modal choice in a UI-safe way
- builds directly on the public `choice_requests` and command envelope added in `0.7.0 wave 1`

## In Scope

- one explicit modal spell profile with:
  - `choose exactly one mode`
  - a small fixed set of supported modes
- cast-time validation requires one legal selected mode
- the selected mode is stored on stack and drives resolution
- public choice projection surfaces modal options for supported spells
- one or two executable cards/tests proving the corridor

## Out of Scope

- `choose two` or `choose one or more`
- mode-dependent multi-target combinations
- entwine, kicker, replicate, fuse, split cards
- generic prompt engines for arbitrary nested choices

## Acceptance

- casting a supported modal spell without a mode is rejected
- casting with an unsupported or illegal mode is rejected
- resolution applies only the selected mode
- public game contract exposes the pending modal choice for the spell

## Notes

- prefer introducing a narrow explicit mode enum in the command/stack/runtime model
- do not encode modal choice as stringly-typed UI metadata
