# Relay Public Commands To The Authoritative Host Over WebRTC

## Goal

Allow the remote peer to issue the same public gameplay commands through the host browser, with the host remaining the only runtime that executes rules.

## Why This Slice Existed Now

Once the transport existed, the smallest honest multiplayer behavior was not full state sync. It was command relay into the already-authoritative host runtime.

## Supported Behavior

- the remote peer can switch from its local placeholder session to a remote WebRTC-backed peer session once pairing completes
- the remote peer sends public commands over the `WebRTC DataChannel`
- the host validates and executes those commands against the wasm-backed runtime it already owns
- the host returns the public command result envelope over the same channel
- the peer can trigger legal actions such as `PassPriority`, `PlayLand`, `TapManaSource`, and other already-supported public commands from its own seat

## Out Of Scope

- passive broadcast of every host-side state change
- rejected-command convergence beyond the returned error
- lockstep simulation
- dual execution
- host migration
- reconnect or resync

## Rules Support Statement

This slice does not widen Magic rules support.

It only transports existing public commands to the already-supported authoritative runtime.

## Constraints And Honesty Notes

- only the host executes gameplay commands
- the peer never computes legality locally
- command payloads stay grounded in the existing public gameplay contract
- the peer currently converges from explicit command responses and the initial state handshake, not yet from passive host-side state broadcast
