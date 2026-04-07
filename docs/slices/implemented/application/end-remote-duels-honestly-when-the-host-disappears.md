# End Remote Duels Honestly When The Host Disappears

## Goal

Handle host loss explicitly and honestly so remote players are not left with a misleading or zombie session when the authoritative browser vanishes.

## Why This Slice Existed Now

Host-authoritative multiplayer is the right first shape, but it creates one unavoidable truth: if the host disappears, the peer cannot keep playing. The client needed to say that plainly and give the user a clean next step.

## Supported Behavior

- a peer session becomes read-only when the remote host becomes unreachable and the transport enters a terminal failed state
- the last rendered authoritative remote state remains visible instead of disappearing or pretending it can advance
- the client presents a clear modal explaining that the authoritative host is gone
- the user can reopen remote pairing or reset back to a local duel from that ended state

## Out Of Scope

- automatic host migration
- local continuation from the abandoned peer
- spectator takeover
- hiding the fact that authority was lost

## Rules Support Statement

This slice does not change gameplay rules support.

It only makes the host-authoritative browser session fail honestly when authority disappears.

## Constraints And Honesty Notes

- no implicit authority transfer occurs
- the peer keeps the abandoned table as read-only presentation only
- continuing the match requires a new host-backed session, not local guesswork
