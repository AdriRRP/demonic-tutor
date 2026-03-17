# status: implemented
# rules: 514.1, 514.1a
# slices: cleanup-hand-size-discard.md

Feature: Discard down to maximum hand size when the turn ends
  In order to keep end-of-turn cleanup semantically honest
  As the play bounded context
  The active player must discard to the maximum hand size before the turn can advance

  Scenario: The turn cannot advance while cleanup discard is still required
    Given Alice is the active player in EndStep and has eight cards in hand
    When Alice tries to advance the turn
    Then the action is rejected because cleanup discard is still required
    And Alice still has eight cards in hand

  Scenario: Discarding for cleanup moves a card from hand to graveyard
    Given Alice is the active player in EndStep and has eight cards in hand
    When Alice discards one card for cleanup
    Then the discarded card leaves Alice's hand
    And the discarded card enters Alice's graveyard
    And Alice has seven cards in hand
    And the game emits CardDiscarded with discard kind CleanupHandSize
