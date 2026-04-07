# Present Remote Pairing UI For Browser Duels

## Goal

Provide a dedicated remote-pairing experience in the browser client so players can create or join a duel room without falling back to debug-style transport controls.

## Why This Slice Existed Now

Manual signaling is only useful if players can discover and complete it without developer tooling. This slice turns the transport handshake into a product-facing flow instead of a hidden technical step.

## Supported Behavior

- open a pairing modal from the duel cockpit
- present clear `Host` and `Join` flows side by side
- copy the local signaling payload directly from the modal
- paste the remote signaling payload back into the relevant flow
- see clear transport states such as `idle`, `offer ready`, `answer ready`, `connecting`, `connected`, and `failed`
- reset the pairing flow safely without disturbing the current duel table

## Out Of Scope

- QR flows
- saved rooms
- reconnect logic
- spectator UX
- implying that remote gameplay is already synchronized

## Rules Support Statement

This slice does not change gameplay rules support.

## Constraints And Honesty Notes

- transport errors are surfaced honestly in the modal
- pairing affordances stay separate from live gameplay controls until the remote authoritative transport slices land
- the current UI makes the transport-only status explicit so the feature does not overclaim remote play support
