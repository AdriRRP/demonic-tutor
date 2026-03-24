use super::super::support;
use super::super::GameplayWorld;
use demonictutor::{DealOpeningHandsCommand, GameId, LibraryCard, StartGameCommand};

impl GameplayWorld {
    pub fn reset_observations(&mut self) {
        self.last_turn_progressed = None;
        self.last_game_ended = None;
        self.last_card_drawn = None;
        self.last_cards_drawn.clear();
        self.last_card_discarded = None;
        self.last_card_exiled = None;
        self.last_activated_ability_put_on_stack = None;
        self.last_spell_put_on_stack = None;
        self.last_spell_cast = None;
        self.last_priority_passed = None;
        self.last_stack_top_resolved = None;
        self.last_combat_damage = None;
        self.last_life_changed = None;
        self.last_creature_died.clear();
        self.last_error = None;
        self.pre_advance_hand_size = None;
        self.post_advance_hand_size = None;
    }

    pub fn reset_tracking(&mut self) {
        self.tracked_card_id = None;
        self.tracked_response_card_id = None;
        self.tracked_second_response_card_id = None;
        self.tracked_attacker_id = None;
        self.tracked_blocker_id = None;
        self.blocker_assignments.clear();
    }

    pub(crate) fn reset_game_with_libraries(
        &mut self,
        game_id: &str,
        alice_cards: Vec<LibraryCard>,
        bob_cards: Vec<LibraryCard>,
    ) {
        let service = support::create_service();
        let mut game = service
            .start_game(StartGameCommand::new(
                GameId::new(game_id),
                vec![
                    support::player_deck("player-1", "deck-1"),
                    support::player_deck("player-2", "deck-2"),
                ],
            ))
            .expect("game should start")
            .0;

        service
            .deal_opening_hands(
                &mut game,
                &DealOpeningHandsCommand::new(vec![
                    support::player_library("player-1", alice_cards),
                    support::player_library("player-2", bob_cards),
                ]),
            )
            .expect("opening hands should be dealt");

        self.game = Some(game);
        self.reset_observations();
        self.reset_tracking();
    }
}
