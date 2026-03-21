# status: implemented
# rules: 114.1, 114.4, 601.2c, 608.2b
# slices: target-controlled-creature-spell-outside-combat.md

Feature: Target a controlled creature outside combat
  Scenario: Alice casts and resolves a controlled-creature instant at her creature
    Given Alice is the active player in FirstMain with a controlled-creature instant spell and Alice's creature on the battlefield
    When Alice casts the controlled-creature instant spell targeting her creature
    Then the spell is on the stack targeting Alice's creature
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Alice's creature dies

  Scenario: Alice cannot target Bob's creature with a controlled-creature instant
    Given Alice is the active player in FirstMain with a controlled-creature instant spell and only Bob's creature on the battlefield
    When Alice tries to cast the controlled-creature instant spell targeting Bob's creature
    Then casting fails because the spell requires a controlled creature target
