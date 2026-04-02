# Bridge Local Duel Rooms Across Two Browser Windows

## Goal

Allow two browser windows of the same web client to share one duel session without introducing a backend.

## Why This Slice Existed Now

The duel arena had already become a credible play surface, but it was still locked to hot-seat on a single screen. The next smallest useful step toward real multiplayer was not remote networking; it was letting two same-origin browser windows coordinate locally while keeping Rust authoritative over gameplay.

## Supported Behavior

- loading the web client now writes a local duel room id into the URL when one is missing
- opening the same room URL in a second browser window joins the existing duel instead of creating a second independent game
- one window acts as the host-authoritative owner of the wasm-backed `GameService` and `Game`
- the second window sends public commands through a same-origin `BroadcastChannel`
- the host window executes those commands through the existing public gameplay contract and broadcasts the updated public state back to the peer window
- the original hot-seat behavior remains available while no second window is connected
- once a second window joins, each window automatically prefers one seat locally instead of relying on pass-the-device handoff

## Out Of Scope

- internet or LAN multiplayer across separate devices
- hidden-information security against same-origin inspection
- host migration when the host window is closed
- target-line rendering, animation transport, or deeper multiplayer UX
- WebRTC signaling or any backend-assisted discovery

## Rules Support Statement

This slice does not add new Magic rules.

It adds a browser-only local transport over the existing public gameplay contract.

## Constraints And Honesty Notes

- this is a same-origin local room, not secure remote multiplayer
- both windows can still inspect the same-origin client bundle and payloads, so private hands are a UI affordance rather than a cryptographic boundary
- the host window must remain open because the authoritative Rust runtime still lives inside that browser instance
