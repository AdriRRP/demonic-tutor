# status: implemented
# rules: 117, 500, 504
# slices: draw-priority-window.md

Feature: Draw opens a priority window after the automatic draw

  Scenario: Advancing from Upkeep to Draw opens priority for the active player after the turn draw
    Given a two-player game is in Upkeep
    And Alice is the active player
    And Alice has at least one card in her library
    And the stack is empty
    When the game advances the turn
    Then Alice draws one card
    And the phase becomes Draw
    And Alice has priority
    And the game emits CardDrawn with draw kind TurnStep
    And the stack is empty

  Scenario: Two consecutive passes close an empty Draw priority window
    Given Alice is the active player in Draw
    And Alice has priority
    And the stack is empty
    When Alice passes priority
    And Bob passes priority
    Then no priority window is open
