# status: implemented
# rules: 509.1, 509.2, 509.3
# slices: declare-blockers.md, combat-damage.md

Feature: Combat currently supports at most one blocker per attacker

  Scenario: A defending player cannot assign two blockers to the same attacker
    Given Alice attacks with a creature and Bob has two creatures that could block the same attacker
    When Bob tries to assign both blockers to that attacker
    Then the action is rejected because multiple blockers per attacker are not yet supported
