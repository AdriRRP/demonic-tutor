# Slice: Rename Aggregate Location Owner Index To Player Index

Status: proposed

## Summary

Rename aggregate location and battlefield-ref indices away from `owner_*` wording when they actually represent the current runtime carrier/player arena rather than persistent card ownership.

## Scope

- rename aggregate location index accessors and related battlefield lookup helpers
- align stack/runtime references that use the same player-arena concept
- preserve persistent `owner_id` on `CardInstance` as the only ownership source of truth

## Out of scope

- adding full control-changing rules support
- widening owner/controller semantics beyond the currently supported corridors
- changing public ids or domain events

## Why now

The runtime already preserves persistent owner identity separately. Keeping `owner_index` for carrier-based lookups invites semantic mistakes in future control and zone work.
