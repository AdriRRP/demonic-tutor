# Add Deterministic Command Result Envelope For UI Clients

## Goal

Return one stable application-level result envelope for supported public commands so client code can handle success and rejection without bespoke per-command logic.

## Why This Slice Existed Now

The engine already returned domain outcomes, but the UI roadmap needs a uniform contract containing emitted events plus the next renderable state.

## Supported Behavior

- execute supported public gameplay commands through one application-level envelope
- return applied versus rejected status
- return emitted public domain events
- return the next snapshot, legal actions, and choice requests after the command attempt

## Out Of Scope

- transport-level HTTP concerns
- persistence-independent request ids
- replay cursors

## Rules Support Statement

This slice does not add new Magic mechanics.

It standardizes how the current supported subset is surfaced to clients.
