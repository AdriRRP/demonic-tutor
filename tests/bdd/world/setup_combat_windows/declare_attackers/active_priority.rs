use super::super::super::{support, GameplayWorld};
use super::common::{attacker_card, prepare_priority_after_attackers_declared};
use demonictutor::Phase;

impl GameplayWorld {
    pub fn setup_priority_after_attackers_declared(&mut self) {
        prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers",
            vec![attacker_card()],
            Vec::new(),
        );
        self.reset_observations();
    }

    pub fn setup_priority_after_attackers_declared_with_instant(&mut self) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-instant",
            vec![
                attacker_card(),
                support::instant_card("bdd-window-instant", 0),
            ],
            Vec::new(),
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_priority_after_attackers_declared_with_two_instants(&mut self) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-two-instants",
            vec![
                attacker_card(),
                support::instant_card("bdd-window-instant-a", 0),
                support::instant_card("bdd-window-instant-b", 0),
            ],
            Vec::new(),
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::DeclareBlockers);
        assert_eq!(
            self.game()
                .priority()
                .expect("attackers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_priority_after_attackers_declared_with_own_turn_artifact(&mut self) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-own-turn-artifact",
            vec![
                attacker_card(),
                support::own_turn_priority_artifact_card("bdd-window-own-turn-artifact", 0),
            ],
            Vec::new(),
        );
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-own-turn-artifact"));
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::DeclareBlockers);
        assert_eq!(
            self.game()
                .priority()
                .expect("attackers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_priority_after_attackers_declared_with_controlled_attacking_spell(&mut self) {
        let attacker_id = prepare_priority_after_attackers_declared(
            self,
            "bdd-combat-priority-attackers-controlled-attacker",
            vec![
                attacker_card(),
                support::targeted_controlled_attacking_creature_damage_instant_card(
                    "bdd-rally-shot",
                    0,
                    2,
                ),
            ],
            Vec::new(),
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-rally-shot"));
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::DeclareBlockers);
        assert_eq!(
            self.game()
                .priority()
                .expect("attackers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }
}
