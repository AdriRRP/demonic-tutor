use super::super::super::{support, GameplayWorld};
use super::common::{attacker_card, prepare_priority_after_combat_damage};
use demonictutor::Phase;

impl GameplayWorld {
    pub fn setup_priority_after_combat_damage_with_instant(&mut self) {
        let attacker_id = prepare_priority_after_combat_damage(
            self,
            "bdd-post-combat-damage-instant",
            vec![
                attacker_card(),
                support::instant_card("bdd-window-instant", 0),
            ],
            Vec::new(),
        );
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_after_combat_damage_with_two_instants(&mut self) {
        let attacker_id = prepare_priority_after_combat_damage(
            self,
            "bdd-post-combat-damage-two-instants",
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
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }
}
