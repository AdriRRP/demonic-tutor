# Slice Name

Scope Private Hands And Prompts To The Local Remote Viewer

## Goal

Ensure each remote browser sees only its own private hand and only the prompts that belong to its viewer-scoped public state.

## Why This Slice Exists Now

Remote play stops feeling honest as soon as seat ownership exists but private information and prompts still behave like a local debug surface. This slice is the minimum needed to make the duel read as two real viewpoints.

## Supported Behavior

- each browser receives only its own hand in clear
- the opponent hand remains hidden/back-faced
- prompts and choice requests only appear for the local viewer when they belong to that seat
- shared public surfaces such as stack, battlefield, graveyard, and exile remain visible to both clients

## Invariants / Legality Rules

- private information comes from the host-owned viewer projection, not CSS hiding
- the peer must not receive the remote hand in a readable transport payload
- prompts remain derived from the public application contract

## Out of Scope

- hostile-client security guarantees
- encrypted secure hidden information
- spectators

## Domain Impact

### Aggregate Impact

- none

### Commands

- none

## Ownership Check

This behavior belongs to the public application projection plus the browser viewer/session layers, because it is about viewer scoping rather than gameplay legality.

## Documentation Impact

- `docs/architecture/web-client.md`
- `docs/domain/current-state.md`
- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- each browser receives only its own hand in clear
- opponent hand remains hidden in the remote payload
- choice prompts appear only for the local relevant seat

## Rules Support Statement

This slice does not change gameplay rules support.

It makes the existing viewer-scoped public contract honest in remote play.
