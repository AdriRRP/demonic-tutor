# Implemented Slice — Stable Capability Matrix Sync

## Summary

Compress the stable supported engine subset into compact canonical matrices for mana, casting, targeting, and combat.

## Supported Behavior

- `docs/domain/current-state.md` now exposes stable capability matrices for:
  - mana
  - casting and stack behavior
  - targeting
  - combat
- the matrices summarize the exercised supported subset without implying broader Magic support
- the stable-v1 planning wave is now documentation-complete

## Invariants

- the matrices stay narrower than the full Magic rules space
- the matrices describe only behavior already supported by code and executable coverage
- canonical truth stays compact enough for both humans and agents to navigate quickly

## Implementation Notes

- this slice is documentation-only
- it closes the stable-v1 planning wave by replacing linear restatement with capability-oriented summaries

## Tests

- no runtime change
- repository validation remains green through the existing full check suite
