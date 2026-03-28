# implemented
# This feature is executable through cucumber-rs.

Feature: Green-white counters golden matchups
  Scenario: Distributed counters turn a pair of creatures into a larger attack
    Given Alice is in first main with two creatures ready to grow and a distributed counter spell
    When Alice casts the distributed counter spell targeting both tracked creatures
    And Alice passes priority
    And Bob passes priority
    Then both tracked creatures are 2/2
    When Alice attacks with both tracked creatures
    And both players pass through blockers without declaring blockers
    And combat damage resolves
    Then Bob loses 4 life from the team attack

  Scenario: Token setup plus anthem turns a small board into a meaningful swing
    Given Alice is in first main with a token spell and an anthem enchantment in hand
    When Alice casts the token spell
    And Alice passes priority
    And Bob passes priority
    Then Alice creates two tracked creature tokens
    When Alice reaches next first main with the anthem still in hand
    And Alice casts the anthem enchantment
    And Alice passes priority
    And Bob passes priority
    Then both tracked tokens are 2/2
    When Alice attacks with both tracked creatures
    And both players pass through blockers without declaring blockers
    And combat damage resolves
    Then Bob loses 4 life from the team attack
