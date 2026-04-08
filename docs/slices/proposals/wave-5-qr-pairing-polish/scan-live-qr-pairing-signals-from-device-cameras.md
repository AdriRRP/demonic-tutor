# Scan Live QR Pairing Signals From Device Cameras

## Goal

Let the remote pairing modal scan offer and answer payloads from a live device camera preview instead of relying only on clipboard or still-image imports.

## Why This Slice Exists Now

The current pairing flow already supports generated `QR` export and image-based import, but it still treats `QR` handoff as a file workflow. The next smallest product step is to let two devices point their cameras at each other and complete the same manual transport flow without leaving the browser modal.

## Supported Behavior

- the remote pairing modal can open a live camera scanner for host answers and peer offers
- supported browsers show an inline preview while the scanner looks for a `QR` payload
- recognized payloads re-enter the existing pairing flow without manual typing
- users can close the scanner cleanly and return to clipboard or image import

## Invariants / Legality Rules

- scanning remains browser-only presentation behavior
- the detected payload still uses the existing signaling format
- scanning failure does not mutate pairing state

## Out Of Scope

- secure backend signaling
- saved pairing history
- gameplay command transport changes
- camera scanning for anything beyond pairing payloads

## Domain Impact

### Aggregate Impact

- none

### Commands

- none

### Events

- none

### Errors

- browser-local scanning errors only

## Ownership Check

This behavior belongs to the browser client.

It is transport UX on top of the already-implemented manual pairing model and does not affect the `Game` aggregate.

## Documentation Impact

- `docs/architecture/web-client.md`
- `apps/web/README.md`
- this slice document

## Test Impact

- opening and closing the live scanner
- importing a detected payload into the correct host or peer field
- handling unsupported camera/browser cases honestly

## Rules Reference

- none

## Rules Support Statement

This slice does not add or change Magic rules support.

It only improves browser pairing UX for the existing host-authoritative remote duel transport.
