# Expose Opening-Hand Setup State For Browser Duels

## Goal

Keep browser duels in `Setup` long enough to support truthful opening-hand decisions instead of auto-skipping straight to `FirstMain`.

## Why This Slice Existed Now

The remote two-device flow already had host-authoritative transport, seat scoping, and reconnection, but it still booted the browser arena by forcibly advancing out of `Setup`. That made any faithful remote mulligan or "you go first" experience impossible. The next smallest useful step was to let the wasm adapter stay in `Setup`, expose browser-readable pregame state, and relay opening-hand commands through the same host-authoritative session layer.

## Supported Behavior

- the wasm adapter now starts browser duels in `Setup` instead of auto-advancing immediately to `FirstMain`
- the adapter serializes browser-side pregame state, including who starts and whose opening-hand decision is currently active
- browser sessions now support `mulligan` and `keep opening hand` commands across local, same-origin peer, and remote `WebRTC` paths
- viewer state now exposes whether the local player has already used the simplified mulligan
- the public `Setup` surface no longer advertises a misleading generic `AdvanceTurn` action while opening-hand decisions are still pending

## Out Of Scope

- a domain-level "keep opening hand" command
- full London Mulligan support
- a polished pregame UI
- rule changes to the core aggregate's simplified mulligan semantics

## Rules Support Statement

This slice does not widen core Magic rules support.

It only restores truthful browser access to the already-supported `Setup` phase and the current simplified one-mulligan opening-hand flow.

## Historical Note

Later browser setup slices extended this restored `Setup` flow with repeated London-style mulligans and explicit opening-hand bottom selection. This document remains the history of exposing truthful `Setup`, not the full current mulligan capability.
