# Slice Name

End Remote Duels Honestly When The Host Disappears

## Goal

Handle host loss explicitly and honestly so remote players are not left with a misleading or zombie session when the authoritative browser vanishes.

## Why This Slice Exists Now

The host-authoritative model is the right first multiplayer shape, but it creates a single obvious failure mode. The client needs to surface that truth cleanly before pretending to offer migration or seamless recovery.

## Supported Behavior

- detect when the authoritative host is no longer reachable
- end the peer session with a clear message
- offer a clean return path to pairing or local reset
- keep the abandoned remote view read-only instead of pretending the match can continue

## Invariants / Legality Rules

- no implicit host migration
- no local continuation from the peer
- transport failure must be communicated honestly

## Out of Scope

- automatic authority transfer
- seamless recovery after host crash
- spectator takeover

## Domain Impact

### Aggregate Impact

- none

### Commands

- none

## Ownership Check

This behavior belongs to browser session orchestration, because it is about ending a remote transport relationship rather than resolving gameplay.

## Documentation Impact

- `docs/architecture/web-client.md`
- `docs/architecture/system-overview.md`
- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- peer transitions to an ended remote-session state when the host disappears
- controls that require authority are disabled
- the user can leave the dead session cleanly

## Rules Support Statement

This slice does not change gameplay rules support.
