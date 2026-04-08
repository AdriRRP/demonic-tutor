# Streamline Browser Remote Pairing With Clipboard-First UX

## Goal

Reduce friction in the two-device remote-pairing flow so players can complete manual `WebRTC` signaling without feeling like they are operating a debug console.

## Why This Slice Existed Now

After remote pairing, authoritative transport, seat scoping, resilience, and shared battlefield presentation were already live, the remaining friction sat in the pairing modal itself. The transport worked, but the browser flow still looked like raw signaling payload plumbing. The next smallest valuable step was to turn that into a clipboard-first product flow with clearer steps, lighter cognitive load, and stronger role guidance.

## Supported Behavior

- the remote pairing modal now frames host and peer as product flows instead of generic textareas
- each side shows explicit step lists so players know what to do next without rereading prose
- host and peer signal outputs now surface a small summary card that identifies payload type and size
- input fields now support direct clipboard paste actions for quicker two-device handoff
- the active pairing role is highlighted visually inside the modal so the current browser clearly reads as host or peer
- transport status remains honest and visible while the modal is open

## Out Of Scope

- QR code pairing
- camera-based scanning
- signaling servers or backend room services
- secure hostile-client multiplayer guarantees

## Rules Support Statement

This slice does not add new Magic rules.

It only improves the browser-side pairing UX for the already-supported manual `WebRTC` transport flow.
