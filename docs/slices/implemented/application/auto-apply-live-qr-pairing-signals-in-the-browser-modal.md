# Auto-Apply Live QR Pairing Signals In The Browser Modal

## Goal

Reduce the remaining friction in live `QR` pairing by letting the browser continue the obvious host or peer step automatically after a live scan succeeds.

## Why This Slice Existed Now

Live camera scanning already removed the need to copy or import a file, but the user still had to confirm the next host or peer action manually after the payload was recognized. The next smallest useful step was to add role-aware `scan + continue` actions that keep the existing pairing semantics while removing one more mechanical click.

## Supported Behavior

- the host can scan a peer answer and immediately apply it
- the peer can scan a host offer and immediately generate the answer
- the scanner stays available in a non-destructive fill-only mode when players prefer to review the payload first
- the live scanner now explains when it will continue automatically after recognition

## Out Of Scope

- background pairing outside the modal
- retry orchestration after failed connection attempts
- backend signaling
- changes to gameplay transport or seat ownership

## Rules Support Statement

This slice does not add or change Magic rules support.

It only streamlines the browser-side `QR` pairing UX for the already-supported remote duel flow.
