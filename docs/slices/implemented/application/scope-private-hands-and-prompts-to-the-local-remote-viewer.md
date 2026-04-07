# Scope Private Hands And Prompts To The Local Remote Viewer

## Goal

Ensure each remote browser sees only its own hand in clear and only the prompts that belong to its viewer-scoped public state.

## Why This Slice Existed Now

Seat ownership alone was not enough. The remote duel still felt dishonest while the peer transport kept carrying both viewers' private hands and prompt surfaces.

## Supported Behavior

- host-to-peer `WebRTC` state sync now keeps the peer's own viewer in clear
- the opposing viewer reaches the peer with an empty hand, no legal actions, and no choice prompts
- shared public surfaces such as battlefield, stack, event log, graveyard, exile, and hand counts remain visible
- rejected remote commands still resync the peer with the same viewer-scoped state shape

## Out Of Scope

- cryptographic hidden-information guarantees
- hostile-host security
- same-origin local two-window privacy
- spectators

## Rules Support Statement

This slice does not widen Magic rules support.

It makes the existing viewer-scoped public contract honest for the current remote `WebRTC` path.

## Constraints And Honesty Notes

- private information is scoped at the host-owned projection boundary, not hidden with CSS
- the authoritative host still owns the full runtime and full state
- the same-origin local two-window room remains a trusted local setup and is intentionally not treated as hidden-information-safe
