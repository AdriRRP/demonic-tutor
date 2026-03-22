# status: implemented
# rules: 114.1, 114.4, 601.2c
# slices: reject-controlled-creature-for-opponents-creature-spell.md

Feature: Reject a controlled creature for an opponents-creature spell
  Scenario: Alice cannot target her own creature with an opponents-creature instant
    Given Alice is the active player in FirstMain with an opponents-creature instant spell and only her creature on the battlefield
    When Alice tries to cast the opponents-creature instant spell targeting her creature
    Then casting fails because the spell requires an opponents-creature target
