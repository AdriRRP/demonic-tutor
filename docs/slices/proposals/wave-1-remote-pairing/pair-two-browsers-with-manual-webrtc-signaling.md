# Slice Name

Pair Two Browsers With Manual WebRTC Signaling

## Goal

Enable two browser instances on different devices to establish a direct remote duel connection through manual offer/answer exchange, without introducing a backend gameplay service.

## Why This Slice Exists Now

Remote play cannot begin until two devices can form a transport link. Manual signaling is the smallest honest step because it proves the transport and authority model before any server-side signaling or room infrastructure exists.

## Supported Behavior

- start a remote duel pairing flow from a host browser
- generate a manual `offer` payload from the host
- import that `offer` into a peer browser
- generate a manual `answer` payload from the peer
- import that `answer` back into the host
- establish one `WebRTC DataChannel` session once the exchange is complete

## Invariants / Legality Rules

- pairing state remains browser-local presentation state
- manual signaling does not execute gameplay commands
- the domain model must not learn about `WebRTC`, peers, or room negotiation

## Out of Scope

- matchmaking
- backend signaling services
- host migration
- replay or command transport
- TURN-specific reliability work

## Domain Impact

### Aggregate Impact

- none

### Commands

- none

### Events

- none

## Ownership Check

This behavior belongs to browser session orchestration in `apps/web` and the browser adapter boundary, not to the `Game` aggregate, because it is transport setup rather than gameplay.

## Documentation Impact

- `docs/architecture/adr/0017-remote-browser-multiplayer-uses-host-authoritative-webrtc.md`
- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- host can generate an `offer`
- peer can import an `offer` and produce an `answer`
- host can import an `answer` and observe a connected transport state
- invalid payloads fail honestly

## Rules Support Statement

This slice does not widen Magic rules support.

It only establishes browser-to-browser transport setup for future remote duel slices.
