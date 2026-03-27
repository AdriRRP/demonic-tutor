# Slice: Unify Pending Stack Decisions

Status: implemented

## Summary

The aggregate now models pending stack resolution choices through a single closed `PendingDecision` enum instead of parallel optional fields.

## What changed

- `Game` holds `pending_decision: Option<PendingDecision>`
- optional effect, hand-choice, and scry decisions now share one runtime corridor
- stack-priority handlers and the public contract pattern-match over one canonical pending-decision shape

## Why it matters

- lowers cognitive load in the aggregate and stack corridor
- makes future choice families additive instead of structural copy-paste
- keeps pending-decision semantics explicit without introducing a generic rule engine
