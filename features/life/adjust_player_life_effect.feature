# status: implemented
# rules: 118.1, 118.2
# slices: adjust-player-life-effect.md, player-life.md, lose-on-zero-life.md

Feature: Explicit life effects can target any player
  In order to model effect-style life changes honestly
  As the play bounded context
  The runtime lets a caster choose which player gains or loses life

  Scenario: A player makes another player lose life
    Given Bob has 20 life
    When Alice makes Bob lose 3 life
    Then Bob has 17 life

  Scenario: A player makes another player gain life
    Given Bob has 20 life
    When Alice makes Bob gain 2 life
    Then Bob has 22 life

  Scenario: A targeted life loss can end the game
    Given Bob has 1 life
    When Alice makes Bob lose 1 life
    Then Bob has 0 life
    And the game emits GameEnded with reason ZeroLife
    And Bob loses the game
    And Alice wins the game
