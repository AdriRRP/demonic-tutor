# status: implemented
# rules: 406, 406.1, 406.2, 406.3
# slices: exile-zone.md

Feature: Exile zone exists as a player-owned game zone
  In order to model Magic's exile zone mechanics
  As the play bounded context
  Cards can be moved to exile and examined by any player

  Scenario: A creature can be exiled from the battlefield
    Given Alice controls a creature on the battlefield
    When a spell or ability exiles that creature
    Then that creature is no longer on the battlefield
    And that creature enters Alice's exile zone
    And the game emits CardMovedZone to exile


  Scenario: A card is moved from the graveyard to exile
    Given a creature is in Bob's graveyard
    When a spell or ability exiles that creature from the graveyard
    Then that creature leaves the graveyard
    And that creature enters Bob's exile zone
    And the game emits CardMovedZone to exile

  Scenario: A card cannot exist in exile and another zone simultaneously
    Given a creature is in Alice's exile zone
    Then that creature is not in Alice's battlefield
    And that creature is not in Alice's graveyard
    And that creature is not in Alice's hand
    And that creature is not in Alice's library
