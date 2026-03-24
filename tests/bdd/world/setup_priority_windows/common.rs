//! BDD coverage for world setup priority windows common.

use {super::super::support, super::super::GameplayWorld, demonictutor::Phase};

impl GameplayWorld {
    pub(super) fn advance_to_turn_one_priority_window(&mut self, phase: Phase) {
        let service = support::create_service();
        for _ in 0..64 {
            while self.game().phase() == &Phase::EndStep && phase != Phase::EndStep {
                self.satisfy_cleanup_for_setup();
                if self.player_hand_size("Alice") <= 7 {
                    break;
                }
            }

            if self.game().phase() == &phase
                && self.game().active_player() == &Self::player_id("Alice")
                && self.game().turn_number() == 1
            {
                break;
            }

            support::advance_turn_raw(&service, self.game_mut());
        }
    }

    pub(super) fn assert_priority_window(&self, phase: Phase, holder_alias: &str) {
        assert_eq!(self.game().phase(), &phase);
        assert_eq!(
            self.game()
                .priority()
                .expect("target phase should have an open priority window")
                .current_holder(),
            &Self::player_id(holder_alias)
        );
        assert!(self.game().stack().is_empty());
    }
}
