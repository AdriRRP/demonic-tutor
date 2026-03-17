# status: implemented
# rules: 511
# slices: end-of-combat-step.md

Feature: End of combat progresses into second main

  Scenario: Closing the empty end-of-combat window advances to SecondMain
    Given a two-player game is in EndOfCombat
    And Alice is the active player
    And Alice has priority
    When Alice passes priority
    And Bob passes priority
    And the game advances the turn
    Then the phase becomes SecondMain
