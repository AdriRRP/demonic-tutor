# status: implemented
# rules: 202.1, 202.1a, 601.1, 601.2
# slices: cast-spell.md, pay-mana-cost.md, remove-summoning-sickness.md

Feature: Cast a creature spell
  In order to model canonical spell casting semantics
  As the play bounded context
  Creatures are cast as spells rather than played through a separate action

  Scenario: Casting a creature spell uses the stack before it resolves
    Given Alice is the active player in FirstMain
    And Alice has a creature card in hand with valid power and toughness
    And Alice has enough mana to pay its cost
    When Alice casts the creature spell
    Then the card leaves Alice's hand
    And the spell is on the stack under Alice's control
    And the spell has not resolved yet
    And Alice has priority
    And the game emits SpellPutOnStack
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the card enters Alice's battlefield
    And the card has summoning sickness
    And the game emits SpellCast with outcome EnteredBattlefield

  Scenario: A land cannot be cast as a spell
    Given Alice is the active player in FirstMain
    And Alice has a land card in hand
    When Alice tries to cast the card as a spell
    Then the action is rejected
    And the land remains in Alice's hand
