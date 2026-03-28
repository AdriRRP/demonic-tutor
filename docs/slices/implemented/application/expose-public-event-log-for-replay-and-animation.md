# Slice: Expose Public Event Log For Replay And Animation

Status: implemented

## Summary

The public application layer now exposes one deterministic replay log built from the persisted domain-event stream for a game.

## What this slice adds

- a public `PublicEventLogEntry` contract with explicit monotonic `sequence`
- a pure `public_event_log(...)` projection helper for turning persisted `DomainEvent` streams into UI-facing replay entries
- a `GameService::public_event_log(&GameId)` query that reads the stored stream and returns that ordered public log

## Why this matters

- UI clients can now load one stable timeline for animation and replay without stitching together only the last command's `emitted_events`
- debugging and playback can rely on persisted event order instead of hidden local state
- the write path stays thin because the replay log remains a separate read query rather than extra synchronous reconstruction work after every command

## Boundaries kept explicit

- this slice does not freeze event ordering for simultaneous prompt families beyond the current persisted order
- this slice does not introduce textual replay formatting as the canonical UI contract
