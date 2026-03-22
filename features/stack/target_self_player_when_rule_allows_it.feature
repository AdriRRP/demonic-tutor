# status: implemented
# rules: 114.1, 114.4, 601.2c, 608.2b
# slices: target-self-player-when-rule-allows-it.md

Feature: Target self when the spell allows any player
  Scenario: Alice casts and resolves a targeted instant at herself
    Given Alice is the active player in FirstMain with a targeted instant spell in hand
    When Alice casts the targeted instant spell targeting herself
    Then the spell is on the stack targeting Alice
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Alice loses 2 life
