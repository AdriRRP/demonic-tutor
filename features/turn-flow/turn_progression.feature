# status: implemented
# rules: 500-507
# slices: advance-turn.md, full-turn-phases.md, turn-number.md

Feature: Turn progression
  In order to preserve playable turn structure
  As the play bounded context
  The game progresses through explicit phases and turns

  Scenario: End Step advances to the next player's Untap when cleanup is satisfied
    Given a two-player game is in EndStep
    And Alice is the active player
    And the current turn number is 3
    When the game advances the turn
    Then Bob becomes the active player
    And the turn number becomes 4
    And the phase becomes Untap
    And the game emits TurnProgressed

  Scenario: Draw phase produces the automatic turn draw
    Given a two-player game is in Draw
    And Alice is the active player
    And Alice has at least one card in her library
    When the game advances the turn
    Then Alice draws one card
    And the phase becomes FirstMain
    And the game emits CardDrawn with draw kind TurnStep
