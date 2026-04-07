# Slice Name

Present Remote Pairing UI For Browser Duels

## Goal

Provide a dedicated remote-pairing experience in the browser client so players can create or join a duel room without falling back to debug-style transport controls.

## Why This Slice Exists Now

Manual signaling is only useful if players can discover and complete it without developer tooling. This slice turns the transport handshake into a product-facing flow instead of a hidden technical step.

## Supported Behavior

- open a pairing modal or route from the duel client
- choose `host` or `join`
- copy the local signaling payload
- paste the remote signaling payload
- see clear transport states such as `idle`, `waiting`, `connecting`, `connected`, and `failed`
- cancel pairing and return to the current session safely

## Invariants / Legality Rules

- pairing UI must not imply that gameplay is already synchronized
- transport errors must be surfaced honestly
- pairing affordances must stay separate from live gameplay controls until the session is connected

## Out of Scope

- QR flows
- saved rooms
- reconnect logic
- spectator UX

## Domain Impact

### Aggregate Impact

- none

### Commands

- none

### Events

- none

## Ownership Check

This behavior belongs to the browser UI and local transport state because it is purely about presenting remote session setup.

## Documentation Impact

- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- host and join flows render the correct controls
- paste/import failures render honest errors
- the pairing UI transitions through the expected transport states

## Rules Support Statement

This slice does not change gameplay rules support.
