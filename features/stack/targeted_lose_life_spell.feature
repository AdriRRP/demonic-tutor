Feature: Targeted lose life spell

  Scenario: Alice casts and resolves a targeted lose-life instant at Bob
    Given Alice is the active player in FirstMain with a targeted lose-life instant spell in hand
    When Alice casts the targeted lose-life instant spell targeting Bob
    Then the spell is on the stack targeting Bob
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Bob loses 3 life from the spell
