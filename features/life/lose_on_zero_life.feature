# status: implemented
# rules: 104.3b, 704.5a
# slices: adjust-player-life-effect.md, lose-on-zero-life.md, player-life.md

Feature: A player loses when their life total reaches zero
  In order to keep life semantics truthful
  As the play bounded context
  A player who reaches zero life loses the game immediately

  Scenario: A player loses when an effect reduces their life total to zero
    Given Alice has 1 life
    When Alice loses 1 life
    Then Alice has 0 life
    And the game emits GameEnded with reason ZeroLife
    And Alice loses the game
    And Bob wins the game
