use super::super::support;
use super::super::GameplayWorld;
use demonictutor::{DiscardForCleanupCommand, GameId, Phase, StartGameCommand};

impl GameplayWorld {
    pub fn setup_started_game(&mut self, game_id: &str) {
        let service = support::create_service();
        let game = service
            .start_game(StartGameCommand::new(
                GameId::new(game_id),
                vec![
                    support::player_deck("player-1", "deck-1"),
                    support::player_deck("player-2", "deck-2"),
                ],
            ))
            .expect("game should start")
            .0;

        self.game = Some(game);
        self.reset_observations();
        self.reset_tracking();
    }

    pub(crate) fn satisfy_cleanup_for_setup(&mut self) {
        let active_player = self.game().active_player().clone();
        let active_player_hand_size = self
            .game()
            .players()
            .iter()
            .find(|player| player.id() == &active_player)
            .expect("active player should exist")
            .hand_size();

        if active_player_hand_size <= 7 {
            return;
        }

        let card_id = self
            .game()
            .players()
            .iter()
            .find(|player| player.id() == &active_player)
            .expect("active player should exist")
            .hand_card_at(0)
            .expect("active player hand card should exist")
            .id()
            .clone();
        self.game_mut()
            .discard_for_cleanup(DiscardForCleanupCommand::new(active_player, card_id))
            .expect("BDD setup cleanup discard should succeed");
    }

    pub fn setup_turn_state_satisfying_cleanup(
        &mut self,
        target_phase: Phase,
        target_player: &str,
        target_turn: u32,
    ) {
        self.reset_game_with_libraries(
            "bdd-turn-progression",
            support::filled_library(Vec::new(), 40),
            support::filled_library(Vec::new(), 40),
        );

        let target_player = Self::player_id(target_player);
        for _ in 0..64 {
            while self.game().phase() == &Phase::EndStep {
                let active_player = self.game().active_player().clone();
                let hand_size = self
                    .game()
                    .players()
                    .iter()
                    .find(|player| player.id() == &active_player)
                    .expect("active player should exist")
                    .hand_size();
                if hand_size <= 7 {
                    break;
                }
                self.satisfy_cleanup_for_setup();
            }

            if self.game().phase() == &target_phase
                && self.game().active_player() == &target_player
                && self.game().turn_number() == target_turn
            {
                self.reset_observations();
                return;
            }

            self.advance_turn();
        }

        panic!(
            "failed to reach target state: phase={target_phase:?}, player={target_player}, turn={target_turn}"
        );
    }

    pub fn setup_upkeep_with_empty_library(&mut self) {
        self.reset_game_with_libraries(
            "bdd-empty-library-draw",
            support::filled_library(Vec::new(), 7),
            support::filled_library(Vec::new(), 7),
        );

        let service = support::create_service();
        support::advance_n_raw(&service, self.game_mut(), 2);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::Upkeep);
        assert_eq!(self.player_library_size("Alice"), 0);
    }

    pub fn setup_first_main_with_library_size(&mut self, library_size: usize) {
        self.reset_game_with_libraries(
            "bdd-explicit-draw-effect",
            support::filled_library(Vec::new(), library_size + 8),
            support::filled_library(Vec::new(), 20),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::FirstMain);
        assert_eq!(self.player_library_size("Alice"), library_size);
    }
}
