# status: implemented
# rules: 121.4, 704.5b
# slices: lose-on-empty-draw.md, draw-card.md, advance-turn.md

Feature: A player loses when they draw from an empty library
  In order to keep draw semantics truthful
  As the play bounded context
  A player who must draw from an empty library loses the game immediately

  Scenario: A player loses when they must draw from an empty library during the draw step
    Given Alice is the active player in Upkeep
    And Alice has no cards in her library
    When the game advances the turn
    Then the game emits GameEnded with reason EmptyLibraryDraw
    And Alice loses the game
    And Bob wins the game
