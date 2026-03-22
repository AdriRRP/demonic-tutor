use super::super::support;
use super::super::GameplayWorld;
use demonictutor::{
    AdjustPlayerLifeEffectCommand, AdvanceTurnCommand, AdvanceTurnOutcome,
    DiscardForCleanupCommand, DrawCardsEffectCommand, PassPriorityCommand, PlayLandCommand,
    PlayerId, TapLandCommand,
};

impl GameplayWorld {
    pub fn advance_turn(&mut self) {
        let service = support::create_service();

        if self.game().has_open_priority_window() && self.game().stack().is_empty() {
            let active_player = self.game().active_player().clone();
            let other_player = self
                .game()
                .players()
                .iter()
                .find(|p| p.id() != &active_player)
                .unwrap()
                .id()
                .clone();

            let holder = self.game().priority().unwrap().current_holder().clone();
            if holder == active_player {
                self.pass_priority_by_id(active_player);
                self.pass_priority_by_id(other_player);
            } else {
                self.pass_priority_by_id(other_player);
                self.pass_priority_by_id(active_player);
            }
        }

        self.pre_advance_hand_size = Some(self.player("Alice").hand_size());

        let outcome = service
            .advance_turn(self.game_mut(), AdvanceTurnCommand::new())
            .expect("advancing turn should succeed");

        match outcome {
            AdvanceTurnOutcome::Progressed {
                turn_progressed,
                card_drawn,
            } => {
                self.last_turn_progressed = Some(turn_progressed);
                self.last_card_drawn = card_drawn;
            }
            AdvanceTurnOutcome::GameEnded(game_ended) => {
                self.last_game_ended = Some(game_ended);
            }
        }

        self.post_advance_hand_size = Some(self.player("Alice").hand_size());
    }

    pub fn try_advance_turn(&mut self) {
        let service = support::create_service();

        if self.game().has_open_priority_window() && self.game().stack().is_empty() {
            let active_player = self.game().active_player().clone();
            let other_player = self
                .game()
                .players()
                .iter()
                .find(|p| p.id() != &active_player)
                .unwrap()
                .id()
                .clone();

            let holder = self.game().priority().unwrap().current_holder().clone();
            if holder == active_player {
                self.pass_priority_by_id(active_player);
                self.pass_priority_by_id(other_player);
            } else {
                self.pass_priority_by_id(other_player);
                self.pass_priority_by_id(active_player);
            }
        }

        self.pre_advance_hand_size = Some(self.player("Alice").hand_size());

        let res = service.advance_turn(self.game_mut(), AdvanceTurnCommand::new());

        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        } else {
            match res.unwrap() {
                AdvanceTurnOutcome::Progressed {
                    turn_progressed,
                    card_drawn,
                } => {
                    self.last_turn_progressed = Some(turn_progressed);
                    self.last_card_drawn = card_drawn;
                }
                AdvanceTurnOutcome::GameEnded(game_ended) => {
                    self.last_game_ended = Some(game_ended);
                }
            }
            self.post_advance_hand_size = Some(self.player("Alice").hand_size());
        }
    }

    pub fn pass_priority(&mut self, alias: &str) {
        self.pass_priority_by_id(Self::player_id(alias));
    }

    pub fn pass_priority_by_id(&mut self, player_id: PlayerId) {
        let service = support::create_service();
        let outcome = service
            .pass_priority(self.game_mut(), PassPriorityCommand::new(player_id))
            .expect("passing priority should succeed");

        self.last_priority_passed = Some(outcome.priority_passed);
        self.last_stack_top_resolved = outcome.stack_top_resolved;
        self.last_spell_cast = outcome.spell_cast;
        self.last_life_changed = outcome.life_changed;
        self.last_creature_died = outcome.creatures_died;
        self.last_game_ended = outcome.game_ended;
    }

    pub fn draw_cards_effect(&mut self, target_alias: &str, count: u32) {
        let service = support::create_service();
        let active_player = self.game().active_player().clone();
        let outcome = service
            .draw_cards_effect(
                self.game_mut(),
                &DrawCardsEffectCommand::new(active_player, Self::player_id(target_alias), count),
            )
            .expect("drawing cards effect should succeed");

        self.last_cards_drawn = outcome.cards_drawn;
        self.last_game_ended = outcome.game_ended;
    }

    pub fn adjust_life(&mut self, alias: &str, delta: i32) {
        let service = support::create_service();
        let active_player = self.game().active_player().clone();
        let outcome = service
            .adjust_player_life_effect(
                self.game_mut(),
                AdjustPlayerLifeEffectCommand::new(active_player, Self::player_id(alias), delta),
            )
            .expect("adjusting life should succeed");

        self.last_life_changed = Some(outcome.life_changed);
        self.last_creature_died = outcome.creatures_died;
        self.last_game_ended = outcome.game_ended;
    }

    pub fn adjust_player_life_effect(
        &mut self,
        caster_alias: &str,
        target_alias: &str,
        delta: i32,
    ) {
        let service = support::create_service();
        let outcome = service
            .adjust_player_life_effect(
                self.game_mut(),
                AdjustPlayerLifeEffectCommand::new(
                    Self::player_id(caster_alias),
                    Self::player_id(target_alias),
                    delta,
                ),
            )
            .expect("adjusting life effect should succeed");

        self.last_life_changed = Some(outcome.life_changed);
        self.last_creature_died = outcome.creatures_died;
        self.last_game_ended = outcome.game_ended;
    }

    pub fn discard_tracked_card_for_cleanup(&mut self, alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let event = service
            .discard_for_cleanup(
                self.game_mut(),
                DiscardForCleanupCommand::new(Self::player_id(alias), card_id),
            )
            .expect("discarding for cleanup should succeed");

        self.last_card_discarded = Some(event);
    }

    pub fn ensure_tracked_land_provides_mana(&mut self) {
        let land_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked land should exist in tracked_blocker_id slot");
        let service = support::create_service();
        service
            .play_land(
                self.game_mut(),
                PlayLandCommand::new(Self::player_id("Alice"), land_id.clone()),
            )
            .expect("playing land should succeed");

        service
            .tap_land(
                self.game_mut(),
                TapLandCommand::new(Self::player_id("Alice"), land_id),
            )
            .expect("tapping land should succeed");
    }

    pub fn tap_tracked_response_land_for_mana(&mut self, alias: &str) {
        let land_id = self
            .tracked_second_response_card_id
            .clone()
            .expect("tracked response land should exist");
        let service = support::create_service();
        service
            .tap_land(
                self.game_mut(),
                TapLandCommand::new(Self::player_id(alias), land_id),
            )
            .expect("tapping response land should succeed");
    }

    pub fn tap_tracked_land_for_mana(&mut self, alias: &str) {
        let land_id = self
            .tracked_card_id
            .clone()
            .expect("tracked land should exist");
        let service = support::create_service();
        service
            .tap_land(
                self.game_mut(),
                TapLandCommand::new(Self::player_id(alias), land_id),
            )
            .expect("tapping tracked land should succeed");
    }
}
