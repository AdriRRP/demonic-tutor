# implemented
# This feature is executable through cucumber-rs.

Feature: White-blue tempo golden matchups
  Scenario: Bounce removes the flying blocker before blocks and the attacker connects
    Given Alice has a flying attacker and a bounce spell after attackers while Bob has a flying blocker
    When Alice casts the bounce spell targeting Bob's blocker
    And Alice passes priority
    And Bob passes priority
    Then Bob's blocker returns to his hand
    When both players pass through blockers without declaring blockers
    And combat damage resolves
    Then Bob loses 2 life from the flying attack

  Scenario: A flying combat trick wins the air combat exchange
    Given Alice has a flying attacker and a pump spell after blockers while Bob has blocked with a flying creature
    When Alice casts the pump-creature instant spell targeting her attacker
    And Alice passes priority
    And Bob passes priority
    And Alice passes priority
    And Bob passes priority
    And combat damage resolves
    Then Bob's blocker dies in combat
    And Alice's attacker survives combat
