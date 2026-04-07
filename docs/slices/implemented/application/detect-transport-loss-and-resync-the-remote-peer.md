# Detect Transport Loss And Resync The Remote Peer

## Goal

Let remote sessions survive transient transport interruptions by detecting connection loss honestly and resynchronizing the peer from the host's authoritative public snapshot.

## Why This Slice Existed Now

Once remote play was real, temporary disconnects became the first practical failure mode. The client needed an explicit degraded transport state and a clean way back to convergence without pretending the peer could rebuild state locally.

## Supported Behavior

- the `WebRTC` pairing state now exposes `reconnecting` as a first-class transport phase
- the cockpit surfaces that degraded state through the existing remote-pairing HUD affordance
- the remote peer asks the host for a fresh authoritative snapshot whenever the transport reconnects
- the host proactively sends a fresh authoritative snapshot whenever the relay reattaches to the recovered transport
- the peer converges again from that authoritative public state instead of replaying guessed local state

## Out Of Scope

- host migration
- offline continuation from the peer
- same-origin local-room resilience changes
- cryptographic or anti-desync verification

## Rules Support Statement

This slice does not widen Magic rules support.

It only hardens the browser transport around the already-supported public game contract.

## Constraints And Honesty Notes

- the host remains the only gameplay authority
- reconnect uses a full public snapshot, not partial client-side reconstruction
- transport recovery is still bounded by the current manual pairing model and current browser connection semantics
