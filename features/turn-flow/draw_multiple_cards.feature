# status: implemented
# rules: 121.1, 121.2, 121.4
# slices: draw-card.md, lose-on-empty-draw.md

Feature: Explicit draw effects can draw multiple cards

  Scenario: An explicit effect draws two cards
    Given Alice is the active player in FirstMain with at least two cards in her library for an explicit draw effect
    When Alice draws 2 cards through an explicit draw effect
    Then Alice draws 2 cards from the explicit effect
    And the game emits 2 CardDrawn events with draw kind ExplicitEffect

  Scenario: An explicit effect ends the game if it tries to draw past an empty library
    Given Alice is the active player in FirstMain with only one card in her library for an explicit draw effect
    When Alice draws 2 cards through an explicit draw effect
    Then Alice draws 1 cards from the explicit effect
    And the game emits GameEnded with reason EmptyLibraryDraw
    And Alice loses the game
    And Bob wins the game
