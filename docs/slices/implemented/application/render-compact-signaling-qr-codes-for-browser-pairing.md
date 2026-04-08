# Render Compact Signaling QR Codes For Browser Pairing

## Goal

Reduce friction in the browser-to-browser pairing flow by rendering the local `WebRTC` signaling payload as a scan-friendly `QR` code inside the existing remote-pairing modal.

## Why This Slice Existed Now

Clipboard-first pairing already made the manual transport flow usable, but it still assumed that both devices could conveniently exchange raw text. The next smallest useful step was to compact the signal envelope and render it visually so a second device could acquire it without retyping or relying only on clipboard handoff.

## Supported Behavior

- host offers and peer answers are now serialized into a compact versioned signaling envelope
- the remote-pairing modal renders the local signaling payload as a generated `QR` code when one exists
- the `QR` sits alongside the existing text output and copy affordances instead of replacing them
- the flow stays honest: players can still fall back to copy/paste if `QR` transfer is inconvenient

## Out Of Scope

- camera-based `QR` scanning
- saved pairings or invite history
- signaling servers
- secure hostile-client multiplayer guarantees

## Rules Support Statement

This slice does not add or change gameplay rules.

## Constraints And Honesty Notes

- `QR` generation stays fully browser-side and asset-light
- the compact signaling envelope remains versioned so the parser can continue accepting earlier payloads during rollout
- the modal still presents the raw payload because pairing is not yet reduced to a one-tap consumer flow
