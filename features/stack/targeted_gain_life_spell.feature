Feature: Targeted gain life spell

  Scenario: Alice casts and resolves a targeted gain-life instant at Bob
    Given Alice is the active player in FirstMain with a targeted gain-life instant spell in hand
    When Alice casts the targeted gain-life instant spell targeting Bob
    Then the spell is on the stack targeting Bob
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Bob gains 3 life
