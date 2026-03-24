# status: implemented
# rules: 510, 702.7, 704
# slices: first-strike-changes-combat-damage-order.md

Feature: First strike changes combat damage order
  Scenario: A first-strike attacker kills its blocker before normal retaliation
    Given Alice attacks with a first-strike creature and Bob blocks with an equal creature
    When combat damage resolves
    Then Bob's blocker dies in combat
    And Alice's attacker survives combat
    And Alice's attacker has no combat damage marked on it
