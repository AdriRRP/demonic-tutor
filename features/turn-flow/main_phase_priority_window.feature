# status: implemented
# rules: 117, 500
# slices: main-phase-priority-window.md

Feature: Main phases open a priority window

  Scenario: Advancing from Draw to FirstMain opens priority for the active player
    Given a two-player game is in Draw
    And Alice is the active player
    And the stack is empty
    When the game advances the turn
    Then the phase becomes FirstMain
    And Alice has priority
    And the stack is empty

  Scenario: Two consecutive passes close an empty FirstMain priority window
    Given Alice is the active player in FirstMain
    And Alice has priority
    And the stack is empty
    When Alice passes priority
    And Bob passes priority
    Then no priority window is open
