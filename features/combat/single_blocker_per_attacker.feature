# status: implemented
# rules: 509.1, 509.2, 509.3
# slices: multiple-blockers-per-attacker.md, combat-damage-assignment-order.md

Feature: Combat supports multiple blockers per attacker

  Scenario: A defending player can assign two blockers to the same attacker
    Given Alice attacks with a creature and Bob has two creatures that could block the same attacker
    When Bob tries to assign both blockers to that attacker
    Then the declaration is accepted with both blockers
