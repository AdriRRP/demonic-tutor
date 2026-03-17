# status: implemented
# rules: 117, 500, 503
# slices: upkeep-priority-window.md

Feature: Upkeep opens a priority window

  Scenario: Advancing from Untap to Upkeep opens priority for the active player
    Given a two-player game is in Untap
    And Alice is the active player
    And the stack is empty
    When the game advances the turn
    Then the phase becomes Upkeep
    And Alice has priority
    And the stack is empty

  Scenario: Two consecutive passes close an empty Upkeep priority window
    Given Alice is the active player in Upkeep
    And Alice has priority
    And the stack is empty
    When Alice passes priority
    And Bob passes priority
    Then no priority window is open
