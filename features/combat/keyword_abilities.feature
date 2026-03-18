# status: implemented
# rules: 702.2, 702.2b, 702.2c, 702.2d
# slices: keyword-abilities.md

Feature: Creatures with keyword abilities modify combat interactions
  In order to model Magic's keyword ability system
  As the play bounded context
  Creatures with Flying and Reach affect which blockers are legal

  Scenario: A creature with flying can be blocked by a creature with flying
    Given Alice attacks with a flying creature
    And Bob controls a creature with flying
    When Bob declares that creature as a blocker against the flying attacker
    Then the blocker assignment is accepted

  Scenario: A creature with flying can be blocked by a creature with reach
    Given Alice attacks with a flying creature
    And Bob controls a creature with reach
    When Bob declares that creature as a blocker against the flying attacker
    Then the blocker assignment is accepted

  Scenario: A creature with flying cannot be blocked by a creature without flying or reach
    Given Alice attacks with a flying creature
    And Bob controls a creature without flying or reach
    When Bob tries to declare that creature as a blocker against the flying attacker
    Then the action is rejected because the blocker cannot block flying creatures

  Scenario: A creature without flying can block a non-flying attacker
    Given Alice attacks with a non-flying creature
    And Bob controls a non-flying creature
    When Bob declares that creature as a blocker
    Then the blocker assignment is accepted

  Scenario: A creature with both flying and reach can block flying attackers
    Given Alice attacks with a flying creature
    And Bob controls a creature with both flying and reach
    When Bob declares that creature as a blocker against the flying attacker
    Then the blocker assignment is accepted

  Scenario: An unblocked flying attacker deals damage to the defending player
    Given Alice attacks with a flying creature that has 3 power
    And Bob has no creatures that can block flying
    And the flying attacker is unblocked
    When combat damage resolves
    Then Bob loses 3 life
