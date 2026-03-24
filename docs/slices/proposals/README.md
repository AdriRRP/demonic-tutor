# Slice Proposals

This directory contains the proposal backlog that still remains open after the latest runtime audit passes.

The backlog is intentionally organized by active legacy wave when proposals are still pending.

## Wave 1 — Engine Audit V5

Directory: `engine-audit-v5/`

1. `engine-audit-v5/introduce-internal-handle-first-card-identity.md`
2. `engine-audit-v5/make-player-card-location-a-primary-index.md`
3. `engine-audit-v5/thin-stack-spell-payloads-further.md`
4. `engine-audit-v5/commit-spells-to-stack-through-one-internal-object.md`
5. `engine-audit-v5/rework-ordered-zone-visible-indexing.md`
6. `engine-audit-v5/move-public-string-ids-to-edge-materialization.md`

These proposals capture older runtime-identity and zone-indexing cuts that remain uncurated as explicit backlog items.

## Wave 2 — Engine Audit V7

Directory: `engine-audit-v7/`

1. `engine-audit-v7/materialize-public-string-ids-only-at-true-boundaries.md`

This proposal tracks the remaining long-tail cleanup toward edge-only public string ids.

## Wave 3 — Engine Audit V8

Directory: `engine-audit-v8/`

1. `engine-audit-v8/make-player-card-handles-the-canonical-runtime-identity.md`
2. `engine-audit-v8/thin-stack-spell-payloads-beyond-definition-records.md`
3. `engine-audit-v8/replace-public-id-based-stack-references-with-internal-ones.md`
4. `engine-audit-v8/move-combat-runtime-links-to-internal-card-references.md`
5. `engine-audit-v8/make-aggregate-card-location-index-incremental.md`
6. `engine-audit-v8/finish-dual-layer-identity-with-edge-only-string-materialization.md`

These proposals preserve the pre-v9 wording of the same excellence direction and remain as legacy backlog until explicitly pruned.

## Wave 4 — Engine Audit V9

Directory: `engine-audit-v9/`

No active proposals remain in this wave.

These proposals focused on canonical handle-first identity, minimal in-flight payloads, and incremental aggregate indexing, and are now implemented.
