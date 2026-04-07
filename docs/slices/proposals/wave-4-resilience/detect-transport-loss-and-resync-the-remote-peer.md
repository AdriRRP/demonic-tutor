# Slice Name

Detect Transport Loss And Resync The Remote Peer

## Goal

Let remote sessions survive transient transport interruptions by detecting connection loss honestly and resynchronizing the peer from the host's authoritative public snapshot.

## Why This Slice Exists Now

Once remote play is possible, temporary disconnects become the first real operational failure. A playtesting client needs a clear degraded state and a clean way back to convergence.

## Supported Behavior

- detect `WebRTC` channel loss or timeout
- surface disconnected and reconnecting transport states
- allow the host to resend a full authoritative snapshot when the peer reconnects
- restore peer rendering from that authoritative snapshot

## Invariants / Legality Rules

- the host remains the sole authority
- the peer never resyncs by replaying guessed local state
- resync uses public snapshots and public replay data only

## Out of Scope

- host migration
- offline-first local continuation
- conflict resolution between diverged peers

## Domain Impact

### Aggregate Impact

- none

### Events

- none

## Ownership Check

This behavior belongs to the transport/session layer because it is about recovering browser synchronization around the existing authoritative runtime.

## Documentation Impact

- `docs/architecture/web-client.md`
- `docs/architecture/system-overview.md`
- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- peer enters a disconnected state when the channel drops
- host can resend a full authoritative snapshot
- peer converges again after reconnection

## Rules Support Statement

This slice does not widen Magic rules support.
