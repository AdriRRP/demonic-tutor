# Proposal Slice — Split Target Discovery From Target Policy

## Summary

Separate target lookup and typed target discovery from rule-policy evaluation so target rules own legality predicates directly.

## Motivation

- keep the targeting corridor from regrowing into a procedural policy router
- make new target families cheaper to add semantically
- let rule objects consume typed inputs instead of mixed discovery logic

## Target Shape

- one step resolves candidate targets into typed locations or references
- rule objects answer legality for those typed targets
- the orchestration layer only translates results into domain errors and outcomes

## Invariants

- cast-time and resolution-time target legality stay aligned
- missing-target errors remain explicit
- this slice does not expand supported Magic rules
