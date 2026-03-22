# status: implemented
# rules: 106.4, 500.4, 605.1, 605.3a
# slices: mana-pool-clears-on-phase-advance.md

Feature: Mana pools clear when the game advances to the next phase
  Scenario: Mana produced in Upkeep is lost when the game advances to Draw
    Given Alice is the active player in Upkeep with a land on the battlefield
    And Alice has priority
    When Alice taps her land for mana
    Then Alice has 1 mana
    When Alice passes priority
    And Bob passes priority
    And the game advances the turn
    Then the phase becomes Draw
    And Alice has 0 mana
