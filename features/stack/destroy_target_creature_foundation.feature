# status: implemented
# rules: 114.1, 114.4, 608.2b, 701.7
# slices: destroy-target-creature-foundation.md

Feature: Destroy target creature
  Scenario: Alice casts and resolves a destroy-creature instant at Bob's creature
    Given Alice is the active player in FirstMain with a destroy-creature instant spell and Bob's creature on the battlefield
    When Alice casts the destroy-creature instant spell targeting Bob's creature
    Then the spell is on the stack targeting Bob's creature
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Bob's creature dies
