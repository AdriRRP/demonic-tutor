# status: implemented
# rules: 704.5f
# slices: zero-toughness-creature-dies.md, cast-spell.md

Feature: A creature with zero toughness dies after entering the battlefield
  In order to keep creature state-based behavior truthful
  As the play bounded context
  A creature spell with zero toughness dies immediately after it enters the battlefield

  Scenario: Casting a creature spell with zero toughness
    Given Alice is the active player in FirstMain
    And Alice has a creature card in hand with zero toughness
    And Alice has enough mana to pay its cost
    When Alice casts the zero-toughness creature spell
    Then the card leaves Alice's hand
    And the spell is on the stack under Alice's control
    And the spell has not resolved yet
    And Bob has priority
    And the game emits SpellPutOnStack
    When Bob passes priority
    And Alice passes priority
    Then the game emits StackTopResolved
    And the card is not on Alice's battlefield
    And the card enters Alice's graveyard
    And the game emits SpellCast with outcome EnteredBattlefield
    And the game emits CreatureDied
