# Slice Implemented - Make Tokens Cease To Exist Off Battlefield

## Outcome

Supported tokens now leave the battlefield truthfully and then disappear instead of persisting in hand, graveyard, or exile.

## What Changed

- battlefield zone transitions now short-circuit token movement into off-battlefield zones
- bounced, destroyed, or exiled tokens leave the battlefield and then cease to exist in runtime state

## Supported Behavior

- tokens that die do not remain as graveyard cards
- bounced tokens do not appear in hand
- exiled tokens do not persist as reusable exile objects

## Notes

- public move events may still refer to the token that just left the battlefield
- this keeps token lifecycle honest without implying broader token-copy semantics
