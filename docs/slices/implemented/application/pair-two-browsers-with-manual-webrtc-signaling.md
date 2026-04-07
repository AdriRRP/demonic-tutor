# Pair Two Browsers With Manual WebRTC Signaling

## Goal

Enable two browser instances on different devices to establish a direct remote duel connection through manual offer/answer exchange, without introducing a backend gameplay service.

## Why This Slice Existed Now

Remote play could not begin until two devices could form a transport link. Manual signaling was the smallest honest step because it proves the transport and authority model before any server-side signaling or room infrastructure exists.

## Supported Behavior

- start a remote duel pairing flow from a host browser
- generate a manual `offer` payload from the host
- import that `offer` into a peer browser
- generate a manual `answer` payload from the peer
- import that `answer` back into the host
- establish one `WebRTC DataChannel` session once the exchange is complete
- surface connected and failed transport states back into the browser UI

## Out Of Scope

- matchmaking
- backend signaling services
- host migration
- gameplay command relay
- TURN-specific reliability work

## Rules Support Statement

This slice does not widen Magic rules support.

It only establishes browser-to-browser transport setup for future remote duel slices.

## Constraints And Honesty Notes

- pairing state remains browser-local presentation state
- manual signaling does not execute gameplay commands
- the domain model stays unaware of `WebRTC`, peers, or room negotiation
- the connected channel is transport-only until the authoritative command relay slices land
