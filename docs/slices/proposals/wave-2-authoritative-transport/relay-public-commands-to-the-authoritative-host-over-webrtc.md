# Slice Name

Relay Public Commands To The Authoritative Host Over WebRTC

## Goal

Allow the remote peer to issue the same public gameplay commands through the host browser, with the host remaining the only runtime that executes rules.

## Why This Slice Exists Now

Once the transport exists, the smallest honest multiplayer behavior is not full state sync. It is command relay into the already-authoritative host runtime.

## Supported Behavior

- peer sends public commands over the `WebRTC DataChannel`
- host validates the caller seat and executes the command against the wasm-backed runtime
- host returns the public command result envelope
- peer can trigger legal actions such as pass, play land, tap mana, and cast from its own seat

## Invariants / Legality Rules

- only the host executes gameplay commands
- the peer never computes legality locally
- command payloads remain grounded in the existing public gameplay contract

## Out of Scope

- lockstep simulation
- dual execution
- host migration
- reconnect/resync

## Domain Impact

### Aggregate Impact

- none

### Commands

- no new domain commands
- remote transport reuses the existing public command set

### Events

- none

## Ownership Check

This behavior belongs to browser transport orchestration plus the wasm interface boundary because it is about where commands are executed, not what they mean.

## Documentation Impact

- `docs/architecture/web-client.md`
- `docs/slices/proposals/remote-duel-horizon.md`
- this slice document

## Test Impact

- peer can issue `PassPriority`
- peer can issue `PlayLand`
- illegal peer commands are rejected by the host and reported back honestly

## Rules Support Statement

This slice does not widen Magic rules support.

It only transports existing public commands to the already-supported authoritative runtime.
