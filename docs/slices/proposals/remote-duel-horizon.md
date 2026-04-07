# Remote Duel Horizon

This document records the current post-`0.8.0` proposal horizon for first honest two-device play.

It does not claim secure internet-scale multiplayer.

It defines the minimum wave plan needed to move from:

- same-origin local duel rooms across two browser windows

to:

- two-device remote play with one authoritative browser host and one remote peer

## Target Horizon

The proposed horizon is:

- `1` release horizon
- `4` waves
- `8` slices

The target milestone is:

- first honest remote best-of-one across two devices, still constrained and host-authoritative

## Architectural Direction

This horizon assumes:

- browser-to-browser transport uses `WebRTC DataChannel`
- signaling starts manually through copy-paste or QR-style exchange
- one browser instance remains the authoritative wasm host
- the remote peer sends public commands and renders public snapshots
- gameplay rules remain in Rust
- transport state remains in the browser client and interface adapter layers

## Technical Seams

The likely implementation seams are:

- `apps/web/src/lib/session.ts`
  current local-room orchestration; likely split into local and remote transports
- `apps/web/src/lib/runtime.ts`
  host-side command execution boundary that the remote transport will call into
- `apps/web/src/lib/types.ts`
  transport-safe envelopes for commands, snapshots, and transport state
- `apps/web/src/components/*`
  pairing and connection UX
- `src/interfaces/web/wasm.rs`
  thin authoritative command/snapshot boundary for the host browser

The `Game` aggregate should not learn about `WebRTC`, peers, rooms, or browser authority.

## Wave Plan

### Wave 1 — Remote Pairing

Goal:

- establish a real browser-to-browser connection without introducing a backend game service

Slices:

- [pair-two-browsers-with-manual-webrtc-signaling.md](wave-1-remote-pairing/pair-two-browsers-with-manual-webrtc-signaling.md)
- [present-remote-pairing-ui-for-browser-duels.md](wave-1-remote-pairing/present-remote-pairing-ui-for-browser-duels.md)

### Wave 2 — Authoritative Transport

Goal:

- make the host-owned runtime playable from the peer over the remote channel

Slices:

- [relay-public-commands-to-the-authoritative-host-over-webrtc.md](wave-2-authoritative-transport/relay-public-commands-to-the-authoritative-host-over-webrtc.md)
- [broadcast-authoritative-public-state-back-to-the-peer.md](wave-2-authoritative-transport/broadcast-authoritative-public-state-back-to-the-peer.md)

### Wave 3 — Remote Seat Views

Goal:

- turn the remote session into a true one-seat-per-device duel instead of a hot-seat derivative

Slices:

- [bind-each-browser-instance-to-one-remote-seat.md](wave-3-remote-seat-views/bind-each-browser-instance-to-one-remote-seat.md)
- [scope-private-hands-and-prompts-to-the-local-remote-viewer.md](wave-3-remote-seat-views/scope-private-hands-and-prompts-to-the-local-remote-viewer.md)

### Wave 4 — Resilience

Goal:

- make remote sessions robust enough for real playtesting and honest failure handling

Slices:

- [detect-transport-loss-and-resync-the-remote-peer.md](wave-4-resilience/detect-transport-loss-and-resync-the-remote-peer.md)
- [end-remote-duels-honestly-when-the-host-disappears.md](wave-4-resilience/end-remote-duels-honestly-when-the-host-disappears.md)

## Out Of Scope For This Horizon

- matchmaking
- permanent backend room services
- automatic host migration
- lockstep deterministic dual execution
- anti-cheat or secure hidden-information guarantees against hostile clients
- spectator mode
- broad TURN/STUN infrastructure beyond what later slices explicitly justify
