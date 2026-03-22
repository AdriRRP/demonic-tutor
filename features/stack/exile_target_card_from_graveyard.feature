# status: implemented
# rules: 114, 406, 608.2b
# slices: exile-target-card-from-graveyard.md

Feature: Exile target card from graveyard
  Scenario: Alice casts and resolves an exile-graveyard-card instant at Bob's graveyard card
    Given Alice is the active player in FirstMain with an exile-graveyard-card instant spell and Bob's card in the graveyard
    When Alice casts the exile-graveyard-card instant spell targeting Bob's graveyard card
    Then the spell is on the stack targeting Bob's graveyard card
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And the game emits CardExiled
    And Bob's graveyard card is in exile
