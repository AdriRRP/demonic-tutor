# Slice Name

Broadcast Authoritative Public State Back To The Peer

## Goal

Keep both remote browsers visually converged by sending authoritative snapshots, replay entries, and command feedback from the host to the peer after each accepted or rejected command.

## Why This Slice Exists Now

Command relay is not enough on its own. The peer still needs canonical state updates so the remote duel reads as one shared match instead of two unsynchronized clients.

## Supported Behavior

- host sends authoritative public snapshots to the peer after command execution
- host sends replay/event-log updates and last-command feedback
- peer renders directly from that authoritative state
- both clients converge after successful and rejected commands

## Invariants / Legality Rules

- the peer must not reconstruct state locally
- the host remains the source of truth for snapshot order
- transport envelopes must stay within the existing public gameplay contract

## Out of Scope

- partial diff protocols
- reconnection recovery
- snapshot hashing or anti-desync checks

## Domain Impact

### Aggregate Impact

- none

### Events

- none

## Ownership Check

This behavior belongs to the browser transport and interface-adapter boundaries because it is about distributing already-projected public state.

## Documentation Impact

- `docs/architecture/web-client.md`
- `docs/domain/current-state.md`
- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- peer converges to the same authoritative public snapshot as the host
- rejected commands still produce synchronized public feedback
- replay entries remain in the same sequence on both clients

## Rules Support Statement

This slice does not change gameplay semantics.

It only distributes the already-supported public game state to a remote peer.
