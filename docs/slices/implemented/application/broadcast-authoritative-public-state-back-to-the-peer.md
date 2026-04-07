# Broadcast Authoritative Public State Back To The Peer

## Goal

Keep both remote browsers visually converged by sending authoritative snapshots, replay entries, and command feedback from the host to the peer after each accepted or rejected command and after host-side local changes.

## Why This Slice Existed Now

Command relay alone was not enough. The peer still needed canonical state updates so the remote duel would read as one shared match instead of two unsynchronized clients.

## Supported Behavior

- the host sends authoritative public snapshots to the peer after local host-side state changes
- the host still returns the authoritative command result envelope after peer-issued commands
- rejected remote commands now carry the current authoritative public snapshot alongside the error
- the peer renders directly from that authoritative state instead of reconstructing anything locally
- both clients converge after successful commands, rejected commands, and local host-side actions

## Out Of Scope

- partial diff protocols
- reconnection recovery
- snapshot hashing
- anti-desync verification

## Rules Support Statement

This slice does not change gameplay semantics.

It only distributes the already-supported public game state to a remote peer.

## Constraints And Honesty Notes

- the peer remains a pure consumer of authoritative public state
- the host stays the only gameplay authority
- the transport still uses the existing public contract rather than a remote-only protocol
- resilience after transport loss remains a later wave
