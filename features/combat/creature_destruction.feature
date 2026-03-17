# status: proposed
# rules: 704, 704.5g
# slices: creature-destruction.md

Feature: Creature destruction after lethal combat damage
  In order to make combat have lasting battlefield consequences
  As the play bounded context
  Creatures with lethal damage marked on them are destroyed automatically

  Scenario: A creature with lethal damage is moved to the graveyard
    Given a creature on the battlefield has damage marked on it equal to its toughness
    When state-based creature destruction is checked
    Then that creature leaves the battlefield
    And that creature enters its controller's graveyard
    And the game emits CreatureDestroyed

  Scenario: A creature with nonlethal damage remains on the battlefield
    Given a creature on the battlefield has damage marked on it less than its toughness
    When state-based creature destruction is checked
    Then that creature remains on the battlefield
    And no CreatureDestroyed event is emitted for that creature
