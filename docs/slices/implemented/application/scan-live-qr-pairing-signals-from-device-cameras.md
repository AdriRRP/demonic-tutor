# Scan Live QR Pairing Signals From Device Cameras

## Goal

Let the browser pairing modal scan offer and answer payloads from a live camera preview so two devices can exchange signaling `QR` codes without leaving the duel UI.

## Why This Slice Existed Now

The remote pairing flow already supported generated `QR` export and image-based `QR` import, but that still felt like a file workflow instead of a live two-device pairing gesture. The next smallest useful step was to let the browser request camera access and detect the pairing payload directly from a live preview inside the existing modal.

## Supported Behavior

- the host can open a live camera scanner while waiting for the remote answer
- the peer can open a live camera scanner while importing the host offer
- supported browsers show an inline preview with a scan frame inside the pairing modal
- recognized payloads flow back into the existing host or peer text fields without retyping
- the scanner can be dismissed cleanly without disturbing the current pairing state

## Out Of Scope

- automatic application of the detected payload
- backend signaling
- persistent scan history
- pairing beyond the existing host-authoritative remote duel flow

## Rules Support Statement

This slice does not add or change Magic rules support.

It only improves the browser-side signaling UX for the existing remote duel transport.
