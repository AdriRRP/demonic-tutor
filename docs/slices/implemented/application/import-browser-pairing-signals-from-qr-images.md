# Import Browser Pairing Signals From QR Images

## Goal

Let browsers acquire remote pairing payloads from `QR` images so two-device setup no longer depends entirely on clipboard handoff.

## Why This Slice Existed Now

Once the pairing modal could export compact signaling payloads as generated `QR` codes, the next smallest product step was to let another browser bring that payload back in without manually retyping or pasting it from a second app. Native browser barcode detection was enough to land that improvement honestly without introducing a backend or a heavy scanning stack.

## Supported Behavior

- the host answer field can now import a `QR` payload from an image or camera capture when the browser exposes native barcode detection
- the peer offer field can do the same for the host offer
- unsupported browsers keep the existing clipboard-first flow without fake affordances
- import errors are surfaced honestly inside the pairing modal

## Out Of Scope

- continuous camera scanning sessions
- fallback decoding through third-party hosted services
- saved pairing history
- secure hostile-client multiplayer guarantees

## Rules Support Statement

This slice does not add or change gameplay rules.

## Constraints And Honesty Notes

- `QR` import remains browser-local and uses native detection only when it exists
- the modal still exposes copy/paste because not every browser offers the same barcode capabilities
- the feature improves pairing ergonomics but does not change the authoritative transport model
