# Playable Subset V1

This document freezes the first public playable subset contract for DemonicTutor.

It exists so UI work, replay tooling, and curated-set authoring can rely on one explicit versioned boundary instead of an implicit moving target.

It does not claim full Magic support.

---

## Contract

`v1` means:

- the public gameplay surface exposes a stable `playable_subset_version`
- the current curated limited-set capability matrix is the only supported authoring contract for cards loaded into games
- authored cards outside that bounded matrix are rejected at load time instead of being interpreted loosely
- deterministic seeded setup, rematch, prompt ordering, replay event logs, and command envelopes are part of the public client contract

---

## Scope

`v1` is downstream from these canonical documents:

- [limited-set-capability-matrix.md](limited-set-capability-matrix.md)
- [limited-set-deck-construction-baseline.md](limited-set-deck-construction-baseline.md)
- [current-state.md](current-state.md)

If those documents grow beyond the currently frozen playable subset, the public version must also change.

---

## Operational Rule

For the current product shell:

- clients may treat `v1` as the supported playable baseline
- curated card pools must stay inside the frozen capability matrix
- any widening of gameplay or authoring support that changes what the client may safely assume requires a later subset version, not a silent expansion
