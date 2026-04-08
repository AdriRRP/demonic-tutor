# Auto-Apply Live QR Pairing Signals In The Browser Modal

## Goal

Reduce the number of manual confirmation steps after camera scanning by letting recognized pairing payloads continue the host or peer flow immediately when the role-specific action is unambiguous.

## Why This Slice Exists Now

Live camera scanning removes the friction of moving the signal between devices, but the user still has to perform the same next click after the payload is already recognized. The next smallest useful step is to let the modal continue the obvious role-specific action automatically while staying honest about failures.

## Supported Behavior

- scanning a host offer on the peer side can immediately create the answer
- scanning a peer answer on the host side can immediately apply the answer
- the modal shows progress and success or failure through the existing pairing status surface
- users can still fall back to paste or image import when they prefer explicit manual control

## Invariants / Legality Rules

- only the role-specific next step is auto-applied
- auto-apply still uses the same pairing controller operations as manual buttons
- scanning never bypasses pairing validation

## Out Of Scope

- auto-retrying failed transport
- background pairing without a visible modal
- changes to gameplay authority or remote seat binding

## Domain Impact

### Aggregate Impact

- none

### Commands

- none

### Events

- none

### Errors

- browser-local pairing and scanning errors only

## Ownership Check

This behavior belongs to the browser pairing modal and transport orchestration.

It is still outside the gameplay domain.

## Documentation Impact

- `docs/architecture/web-client.md`
- `apps/web/README.md`
- this slice document

## Test Impact

- peer scan can generate an answer without an extra button click
- host scan can apply an answer without an extra button click
- scan failures surface status honestly without mutating connected state

## Rules Reference

- none

## Rules Support Statement

This slice does not add or change Magic rules support.

It only streamlines the browser pairing UX for the existing remote duel foundation.
