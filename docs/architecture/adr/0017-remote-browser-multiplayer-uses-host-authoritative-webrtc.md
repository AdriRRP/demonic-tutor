# ADR 0017 — Remote browser multiplayer uses host-authoritative WebRTC with manual signaling first

## Status
Accepted

## Context

The repository already supports a same-origin local duel room across two browser windows.

The next multiplayer step is two-device play without introducing a backend gameplay service or moving rules into TypeScript.

That means the project needs:

- a browser-to-browser transport
- a way to connect two browsers without a game server
- a clear authority model that keeps one canonical runtime

The project also wants to stay:

- client-first
- statically deployable
- operationally light

## Decision

Remote browser multiplayer will use `WebRTC DataChannel`.

The first remote pairing slices will use manual signaling between players rather than a dedicated signaling service.

One browser instance remains the authoritative host and owns the wasm-backed runtime.

The remote peer sends public commands to that host and renders the authoritative public snapshots, command feedback, and replay data it receives back.

The domain model and `Game` aggregate remain unaware of transport, rooms, peers, and signaling.

## Consequences

### Positive

- keeps one authoritative runtime
- avoids duplicating gameplay rules in TypeScript
- preserves static deployment and low operational complexity for the first remote slices
- lets multiplayer grow through browser and interface-adapter code rather than domain intrusion
- makes later signaling-service or TURN decisions additive instead of foundational

### Negative

- manual signaling is awkward compared with a real pairing service
- some real-world networks may still need later TURN/STUN work
- the host browser becomes a single point of authority and failure
- hidden information is only as strong as the trusted-host model, not hostile-client safe

## Notes

This ADR chooses the first honest remote architecture.

It does not claim:

- full internet-scale reliability
- secure hostile-client multiplayer
- automatic host migration
- lockstep simulation across peers
