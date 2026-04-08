# Remote Duel Horizon

This document records the current post-`0.8.0` proposal horizon for first honest two-device play after the manual remote-pairing foundation landed.

It does not claim secure internet-scale multiplayer.

It defines the minimum wave plan needed to move from:

- same-origin local duel rooms across two browser windows

to:

- two-device remote play with one authoritative browser host and one remote peer

## Target Horizon

The remaining proposed horizon is:

- `1` release horizon
- `0` active proposal waves
- `0` active proposal slices

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

## Implemented Foundation

Wave 1 is now implemented through:

- [pair-two-browsers-with-manual-webrtc-signaling.md](../implemented/application/pair-two-browsers-with-manual-webrtc-signaling.md)
- [present-remote-pairing-ui-for-browser-duels.md](../implemented/application/present-remote-pairing-ui-for-browser-duels.md)

That foundation proves browser-to-browser transport setup without yet claiming remote gameplay relay.

Wave 2 is now fully implemented through:

- [relay-public-commands-to-the-authoritative-host-over-webrtc.md](../implemented/application/relay-public-commands-to-the-authoritative-host-over-webrtc.md)
- [broadcast-authoritative-public-state-back-to-the-peer.md](../implemented/application/broadcast-authoritative-public-state-back-to-the-peer.md)

That means the remote peer can now issue existing public gameplay commands through the authoritative host and stay converged with the host's authoritative public state without local rule reconstruction.

Wave 3 is now fully implemented through:

- [bind-each-browser-instance-to-one-remote-seat.md](../implemented/application/bind-each-browser-instance-to-one-remote-seat.md)
- [scope-private-hands-and-prompts-to-the-local-remote-viewer.md](../implemented/application/scope-private-hands-and-prompts-to-the-local-remote-viewer.md)

That means remote browser sessions are now bound to one seat each, and the current `WebRTC` peer path only receives its own hand and prompt surfaces in clear.

Wave 4 is now fully implemented through:

- [detect-transport-loss-and-resync-the-remote-peer.md](../implemented/application/detect-transport-loss-and-resync-the-remote-peer.md)
- [end-remote-duels-honestly-when-the-host-disappears.md](../implemented/application/end-remote-duels-honestly-when-the-host-disappears.md)

That means transient transport loss now surfaces as reconnecting state with authoritative resync, and terminal host loss now ends the remote duel honestly instead of leaving a zombie peer session.

Wave 5 is now fully implemented through:

- [scan-live-qr-pairing-signals-from-device-cameras.md](../implemented/application/scan-live-qr-pairing-signals-from-device-cameras.md)
- [auto-apply-live-qr-pairing-signals-in-the-browser-modal.md](../implemented/application/auto-apply-live-qr-pairing-signals-in-the-browser-modal.md)

That means remote pairing can now use live in-browser camera scanning and can optionally continue the obvious host or peer step immediately after a successful scan.

Wave 6 is now fully implemented through:

- [present-seat-aware-opening-hand-hero-states.md](../implemented/application/present-seat-aware-opening-hand-hero-states.md)
- [mark-opening-hand-bottom-picks-directly-on-cards.md](../implemented/application/mark-opening-hand-bottom-picks-directly-on-cards.md)

That means the truthful remote mulligan flow now presents stronger pregame hero states and lets the visible hand fan carry bottom-selection meaning directly through numbered markers on the chosen cards.

## Active Wave Plan

- no active proposal waves remain in this remote-duel horizon

## Out Of Scope For This Horizon

- matchmaking
- permanent backend room services
- automatic host migration
- lockstep deterministic dual execution
- anti-cheat or secure hidden-information guarantees against hostile clients
- spectator mode
- broad TURN/STUN infrastructure beyond what later slices explicitly justify
