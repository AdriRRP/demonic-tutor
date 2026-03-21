use super::super::super::{support, GameplayWorld};
use super::common::{attacker_card, blocker_card, prepare_priority_after_blockers_declared};
use demonictutor::Phase;

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
}
