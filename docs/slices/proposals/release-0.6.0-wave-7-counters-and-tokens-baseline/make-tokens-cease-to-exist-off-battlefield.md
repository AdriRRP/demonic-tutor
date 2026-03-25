# Slice Proposal - Make Tokens Cease To Exist Off Battlefield

## Goal

Model the supported token lifecycle honestly by ensuring created tokens cease to exist once they leave the battlefield.

## Why This Slice

Token creation without token cleanup produces long-lived false state in graveyard, hand, exile, or library and makes later recursion rules misleading.

## Scope

- tokens that die do not persist as graveyard cards
- tokens that are bounced or exiled leave the battlefield and then cease to exist
- public events remain truthful about the battlefield move before disappearance when appropriate

## Out of Scope

- replacement effects around token movement
- copying token information into non-battlefield zones

## Notes

- this slice should follow token creation quickly
- the engine should not imply reusable off-battlefield token identity
