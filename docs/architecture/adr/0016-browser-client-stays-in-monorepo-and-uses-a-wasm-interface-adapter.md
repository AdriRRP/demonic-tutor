# ADR 0016 — Browser client stays in monorepo and uses a wasm interface adapter

## Status
Accepted

## Context

The repository now includes a real browser-facing client under `apps/web`.

That client needs to consume the gameplay engine without duplicating rules in TypeScript, and the repository needs a stable place for browser-specific integration code so that `src/application/public_game/` can remain the public gameplay contract rather than growing UI-adapter concerns.

The project also benefits from evolving engine, client, replay, and wasm compatibility together while the product is still moving through tight vertical slices.

## Decision

The browser client stays in the monorepo under `apps/web/`.

Browser-specific Rust integration code lives under `src/interfaces/web/` as a dedicated wasm interface adapter.

The browser consumes the existing public gameplay contract from `src/application/public_game/` through that adapter.

Gameplay rules, legality, replay semantics, and public read-model projection remain owned by the Rust application and domain layers.

## Consequences

### Positive

- one authoritative gameplay runtime across tests, wasm, and browser UI
- faster end-to-end slices because engine and client evolve together
- clearer layering between public gameplay contract and browser adapter code
- easier synchronization of replay, fixtures, and viewer-scoped behavior
- a cleaner architectural home for future browser-specific adapters

### Negative

- the repository now carries both Rust and web-client workflows in one tree
- wasm compatibility must stay part of normal validation
- browser adapter code still adds an extra architectural surface that needs curation
- a future independently deployed client may still need packaging or repository changes

## Notes

This ADR establishes the current client integration strategy.

It does not imply that all future interfaces must use wasm, only that browser-facing adapters should live in the `interfaces` layer instead of leaking into application or domain code.
