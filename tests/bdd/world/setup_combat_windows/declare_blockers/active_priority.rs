//! BDD coverage for setup combat windows declare blockers active priority.

use {
    super::super::super::{support, GameplayWorld},
    super::common::{attacker_card, blocker_card, prepare_priority_after_blockers_declared},
    demonictutor::Phase,
};

impl GameplayWorld {
    pub fn setup_priority_after_blockers_declared(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers",
            vec![attacker_card()],
            vec![blocker_card()],
        );
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_instant(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-instant",
            vec![
                attacker_card(),
                support::instant_card("bdd-window-instant", 0),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_flash_creature(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-flash",
            vec![
                attacker_card(),
                support::flash_creature_card("bdd-flash-creature", 0, 2, 1),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-flash-creature"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_flash_artifact(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-flash-artifact",
            vec![
                attacker_card(),
                support::flash_artifact_card("bdd-window-flash-artifact", 0),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-flash-artifact"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_flash_enchantment(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-flash-enchantment",
            vec![
                attacker_card(),
                support::flash_enchantment_card("bdd-window-flash-enchantment", 0),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-flash-enchantment"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_flash_planeswalker(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-flash-planeswalker",
            vec![
                attacker_card(),
                support::flash_planeswalker_card("bdd-window-flash-planeswalker", 0),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-flash-planeswalker"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_own_turn_artifact(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-own-turn-artifact",
            vec![
                attacker_card(),
                support::own_turn_priority_artifact_card("bdd-window-own-turn-artifact", 0),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-own-turn-artifact"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_own_turn_enchantment(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-own-turn-enchantment",
            vec![
                attacker_card(),
                support::own_turn_priority_enchantment_card("bdd-window-own-turn-enchantment", 0),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-own-turn-enchantment"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_blocking_creature_spell(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-targeted-blocker",
            vec![
                attacker_card(),
                support::targeted_blocking_creature_damage_instant_card("bdd-hold-the-line", 0, 2),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-hold-the-line"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_nonlethal_blocking_creature_spell(
        &mut self,
    ) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-targeted-blocker-nonlethal",
            vec![
                attacker_card(),
                support::targeted_blocking_creature_damage_instant_card("bdd-hold-the-line", 0, 1),
            ],
            vec![support::creature_card("bdd-blocker-priority", 0, 2, 3)],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-hold-the-line"));
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_two_instants(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-two-instants",
            vec![
                attacker_card(),
                support::instant_card("bdd-window-instant-a", 0),
                support::instant_card("bdd-window-instant-b", 0),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::CombatDamage);
        assert_eq!(
            self.game()
                .priority()
                .expect("blockers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_priority_after_blockers_declared_with_pump_creature_spell(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-pump-creature",
            vec![
                attacker_card(),
                support::targeted_pump_creature_instant_card("bdd-giant-growth-lite", 0, 2, 2),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-giant-growth-lite"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::CombatDamage);
        assert_eq!(
            self.game()
                .priority()
                .expect("blockers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_priority_after_blockers_declared_with_opponents_blocking_spell(&mut self) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-opponent-blocker",
            vec![
                attacker_card(),
                support::targeted_opponents_blocking_creature_damage_instant_card(
                    "bdd-punish-shield",
                    0,
                    2,
                ),
            ],
            vec![blocker_card()],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-punish-shield"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::CombatDamage);
        assert_eq!(
            self.game()
                .priority()
                .expect("blockers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_priority_after_blockers_declared_with_nonlethal_opponents_blocking_spell(
        &mut self,
    ) {
        prepare_priority_after_blockers_declared(
            self,
            "bdd-combat-priority-blockers-opponent-blocker-nonlethal",
            vec![
                attacker_card(),
                support::targeted_opponents_blocking_creature_damage_instant_card(
                    "bdd-punish-shield",
                    0,
                    1,
                ),
            ],
            vec![support::creature_card("bdd-blocker-priority", 0, 2, 3)],
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-punish-shield"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::CombatDamage);
        assert_eq!(
            self.game()
                .priority()
                .expect("blockers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }
}
