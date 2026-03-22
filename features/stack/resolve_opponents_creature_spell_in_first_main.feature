# status: implemented
# rules: 114.1, 114.4, 601.2c, 608.2b
# slices: resolve-opponents-creature-spell-in-first-main.md

Feature: Resolve an opponents-creature spell outside combat
  Scenario: Alice casts and resolves an opponents-creature instant at Bob's creature in FirstMain
    Given Alice is the active player in FirstMain with an opponents-creature instant spell and Bob's creature on the battlefield
    When Alice casts the opponents-creature instant spell targeting Bob's creature
    Then the spell is on the stack targeting Bob's creature
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Bob's creature dies
