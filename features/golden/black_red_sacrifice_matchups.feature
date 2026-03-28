# implemented
# This feature is executable through cucumber-rs.

Feature: Black-red sacrifice golden matchups
  Scenario: A sacrifice outlet can be cashed in for value through the stack
    Given Alice has a sacrifice outlet artifact on the battlefield in first main
    When Alice activates the tracked ability
    Then the activated ability is on the stack under Alice's control
    And the tracked sacrifice artifact is in Alice's graveyard
    When Alice passes priority
    And Bob passes priority
    Then Alice has 22 life

  Scenario: Discard strips a creature card from the opponent's hand
    Given Bob is in first main with a discard spell while Alice holds a creature card
    When Bob casts the discard spell targeting Alice and choosing her tracked creature card
    And Bob passes priority
    And Alice passes priority
    Then Alice's tracked creature card is in her graveyard

  Scenario: Removal trades off a creature and recursion buys it back
    Given Bob is in first main with removal while Alice has a creature on the battlefield and recursion in hand
    When Bob casts the destroy-creature instant spell targeting Alice's creature
    And Bob passes priority
    And Alice passes priority
    Then Alice's creature dies
    When Alice reaches first main with the recursion spell available
    And Alice casts the recursion spell targeting her graveyard creature
    And Alice passes priority
    And Bob passes priority
    Then Alice's tracked creature card returns to her hand
    When Alice casts the recovered creature spell
    And Alice passes priority
    And Bob passes priority
    Then Alice's recovered creature enters the battlefield
