# status: implemented
# rules: 114, 406, 608.2b
# slices: exile-target-creature-foundation.md

Feature: Exile target creature
  Scenario: Alice casts and resolves an exile-creature instant at Bob's creature
    Given Alice is the active player in FirstMain with an exile-creature instant spell and Bob's creature on the battlefield
    When Alice casts the exile-creature instant spell targeting Bob's creature
    Then the spell is on the stack targeting Bob's creature
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits CardMovedZone to exile
    And Bob's creature is in exile
