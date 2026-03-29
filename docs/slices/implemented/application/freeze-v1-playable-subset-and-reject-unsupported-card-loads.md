# Freeze V1 Playable Subset And Reject Unsupported Card Loads

## Summary

Freeze the first honest playable client contract as `v1` and make that boundary explicit in the public surface and canonical documentation.

## Why

The engine already had:

- curated-card validation through the limited-set authoring catalog
- deterministic public runtime helpers for seeded setup, rematch, prompts, and replay

What was still missing was one explicit, versioned statement saying: this is the playable subset a UI may safely target today.

## Implemented

- `PublicGameView` now exposes `playable_subset_version = V1`
- canonical docs now describe `v1` as the frozen playable subset contract
- this freeze provided the contract boundary that allowed `0.8.0 wave 4` to close honestly before the final `0.8.0` release curation

## Notes

Unsupported card-load rejection was already active through the curated-set catalog and load-time validation work from the previous authoring wave.
This slice freezes that bounded scope as a client-facing contract instead of widening support.
