# status: implemented
# rules: 114.1, 114.4, 601.2c, 608.2b
# slices: target-opponent-player-spell-foundation.md, reject-self-target-for-opponent-player-spell.md

Feature: Target an opponent player with a spell
  Scenario: Alice casts and resolves an opponent-targeted instant at Bob
    Given Alice is the active player in FirstMain with an opponent-targeted instant spell in hand
    When Alice casts the opponent-targeted instant spell targeting Bob
    Then the spell is on the stack targeting Bob
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Bob loses 2 life

  Scenario: Alice cannot target herself with an opponent-targeted instant
    Given Alice is the active player in FirstMain with an opponent-targeted instant spell in hand
    When Alice tries to cast the opponent-targeted instant spell targeting herself
    Then casting fails because the spell requires an opponent target
