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
