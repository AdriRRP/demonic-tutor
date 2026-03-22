Feature: Pump spell changes combat outcome

  Scenario: A post-blockers pump lets the attacker survive combat
    Given Alice has priority after blockers are declared with a pump-creature instant spell in hand
    When Alice casts the pump-creature instant spell targeting her attacker
    Then the spell is on the stack targeting Alice's attacker
    When Alice passes priority
    And Bob passes priority
    Then the game emits StackTopResolved
    And Alice's attacker gets +2/+2 until end of turn
    When Alice passes priority
    And Bob passes priority
    When combat damage resolves
    Then Bob's blocker dies in combat
    And Alice's attacker survives combat
