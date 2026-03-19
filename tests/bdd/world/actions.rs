use super::GameplayWorld;
use super::support;
use demonictutor::{
    AdjustPlayerLifeEffectCommand, AdvanceTurnCommand, AdvanceTurnOutcome,
    CastSpellCommand, DealOpeningHandsCommand, DiscardForCleanupCommand,
    DrawCardsEffectCommand, GameId, LibraryCard, PassPriorityCommand,
    PlayLandCommand, PlayerId, ResolveCombatDamageCommand,
    SpellTarget, StartGameCommand, TapLandCommand, Phase,
};

impl GameplayWorld {
    pub fn reset_observations(&mut self) {
        self.last_turn_progressed = None;
        self.last_game_ended = None;
        self.last_card_drawn = None;
        self.last_cards_drawn.clear();
        self.last_card_discarded = None;
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

    pub(super) fn reset_game_with_libraries(
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

    pub fn advance_turn(&mut self) {
        self.pre_advance_hand_size = Some(self.player_hand_size("Alice"));

        self.close_empty_priority_window_for_setup();

        match self.game_mut().advance_turn(AdvanceTurnCommand::new()) {
            Ok(AdvanceTurnOutcome::Progressed {
                turn_progressed,
                card_drawn,
            }) => {
                self.last_turn_progressed = Some(turn_progressed);
                self.last_game_ended = None;
                self.last_card_drawn = card_drawn;
                self.last_error = None;
            }
            Ok(AdvanceTurnOutcome::GameEnded(game_ended)) => {
                self.last_turn_progressed = None;
                self.last_game_ended = Some(game_ended);
                self.last_card_drawn = None;
                self.last_error = None;
            }
            Err(error) => {
                self.last_error = Some(error.to_string());
                self.last_turn_progressed = None;
                self.last_game_ended = None;
                self.last_card_drawn = None;
            }
        }

        self.post_advance_hand_size = Some(self.player_hand_size("Alice"));
    }

    fn close_empty_priority_window_for_setup(&mut self) {
        if !self.game().has_open_priority_window() {
            return;
        }

        assert!(
            self.game().stack().is_empty(),
            "BDD setup only auto-closes empty priority windows"
        );

        let first_holder = self.game().priority().map_or_else(
            || panic!("priority window should be open"),
            |p| p.current_holder().clone(),
        );
        self.pass_priority_by_id(first_holder);

        let second_holder = self.game().priority().map_or_else(
            || panic!("priority window should remain open after one pass"),
            |p| p.current_holder().clone(),
        );
        self.pass_priority_by_id(second_holder);
    }

    pub fn cast_tracked_spell(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked spell card should exist");

        match service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        ) {
            Ok(outcome) => {
                self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
                self.last_spell_cast = None;
                self.last_priority_passed = None;
                self.last_stack_top_resolved = None;
                self.last_creature_died.clear();
                self.last_error = None;
            }
            Err(error) => {
                self.last_spell_put_on_stack = None;
                self.last_spell_cast = None;
                self.last_priority_passed = None;
                self.last_stack_top_resolved = None;
                self.last_creature_died.clear();
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn cast_tracked_spell_targeting_player(&mut self, alias: &str, target_alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked spell card should exist");

        match service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id)
                .with_target(SpellTarget::Player(Self::player_id(target_alias))),
        ) {
            Ok(outcome) => {
                self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
                self.last_spell_cast = None;
                self.last_error = None;
            }
            Err(error) => {
                self.last_spell_put_on_stack = None;
                self.last_spell_cast = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn cast_tracked_spell_targeting_missing_player(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked spell card should exist");

        match service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id)
                .with_target(SpellTarget::Player(PlayerId::new("missing-player"))),
        ) {
            Ok(outcome) => {
                self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
                self.last_spell_cast = None;
                self.last_error = None;
            }
            Err(error) => {
                self.last_spell_put_on_stack = None;
                self.last_spell_cast = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn cast_tracked_spell_without_target(&mut self, alias: &str) {
        self.cast_tracked_spell(alias);
    }

    pub fn cast_tracked_spell_targeting_tracked_creature(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked spell card should exist");
        let target_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked creature target should exist");

        match service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id)
                .with_target(SpellTarget::Creature(target_id)),
        ) {
            Ok(outcome) => {
                self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
                self.last_spell_cast = None;
                self.last_error = None;
            }
            Err(error) => {
                self.last_spell_put_on_stack = None;
                self.last_spell_cast = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn cast_tracked_response_spell(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response spell card should exist");

        match service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        ) {
            Ok(outcome) => {
                self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
                self.last_spell_cast = None;
                self.last_error = None;
            }
            Err(error) => {
                self.last_error = Some(error.to_string());
                self.last_spell_put_on_stack = None;
                self.last_spell_cast = None;
            }
        }
    }

    pub fn cast_tracked_second_response_spell(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_second_response_card_id
            .clone()
            .expect("tracked second response spell card should exist");

        match service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        ) {
            Ok(outcome) => {
                self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
                self.last_spell_cast = None;
                self.last_error = None;
            }
            Err(error) => {
                self.last_error = Some(error.to_string());
                self.last_spell_put_on_stack = None;
                self.last_spell_cast = None;
            }
        }
    }

    pub fn pass_priority(&mut self, alias: &str) {
        self.pass_priority_by_id(Self::player_id(alias));
    }

    pub(super) fn pass_priority_by_id(&mut self, player_id: PlayerId) {
        let service = support::create_service();
        match service.pass_priority(self.game_mut(), PassPriorityCommand::new(player_id)) {
            Ok(outcome) => {
                self.last_priority_passed = Some(outcome.priority_passed);
                self.last_stack_top_resolved = outcome.stack_top_resolved;
                self.last_spell_cast = outcome.spell_cast;
                self.last_life_changed = outcome.life_changed;
                self.last_creature_died = outcome.creatures_died;
                self.last_game_ended = outcome.game_ended;
                self.last_error = None;
            }
            Err(error) => {
                self.last_priority_passed = None;
                self.last_stack_top_resolved = None;
                self.last_spell_cast = None;
                self.last_life_changed = None;
                self.last_creature_died.clear();
                self.last_game_ended = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn draw_cards_effect(&mut self, caster_alias: &str, target_alias: &str, count: u32) {
        let service = support::create_service();

        match service.draw_cards_effect(
            self.game_mut(),
            &DrawCardsEffectCommand::new(
                Self::player_id(caster_alias),
                Self::player_id(target_alias),
                count,
            ),
        ) {
            Ok(outcome) => {
                self.last_card_drawn = outcome.cards_drawn.last().cloned();
                self.last_cards_drawn = outcome.cards_drawn;
                self.last_game_ended = outcome.game_ended;
                self.last_error = None;
            }
            Err(error) => {
                self.last_card_drawn = None;
                self.last_cards_drawn.clear();
                self.last_game_ended = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn adjust_life(&mut self, alias: &str, delta: i32) {
        self.adjust_player_life_effect(alias, alias, delta);
    }

    pub fn adjust_player_life_effect(
        &mut self,
        caster_alias: &str,
        target_alias: &str,
        delta: i32,
    ) {
        let service = support::create_service();

        match service.adjust_player_life_effect(
            self.game_mut(),
            AdjustPlayerLifeEffectCommand::new(
                Self::player_id(caster_alias),
                Self::player_id(target_alias),
                delta,
            ),
        ) {
            Ok(outcome) => {
                self.last_game_ended = outcome.game_ended;
                self.last_error = None;
            }
            Err(error) => {
                self.last_game_ended = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn discard_tracked_card_for_cleanup(&mut self, alias: &str) {
        let service = support::create_service();
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked discard card should exist");

        match service.discard_for_cleanup(
            self.game_mut(),
            DiscardForCleanupCommand::new(Self::player_id(alias), card_id),
        ) {
            Ok(event) => {
                self.last_card_discarded = Some(event);
                self.last_error = None;
            }
            Err(error) => {
                self.last_card_discarded = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn resolve_combat_damage(&mut self) {
        let service = support::create_service();
        support::close_empty_priority_window(&service, self.game_mut());
        if self.game().phase() == &Phase::DeclareBlockers {
            support::advance_turn_raw(&service, self.game_mut());
        }
        match service.resolve_combat_damage(
            self.game_mut(),
            ResolveCombatDamageCommand::new(Self::player_id("Alice")),
        ) {
            Ok(outcome) => {
                self.last_combat_damage = Some(outcome.combat_damage_resolved);
                self.last_life_changed = outcome.life_changed;
                self.last_creature_died = outcome.creatures_died;
                self.last_game_ended = outcome.game_ended;
                self.last_error = None;
            }
            Err(error) => {
                self.last_combat_damage = None;
                self.last_life_changed = None;
                self.last_creature_died.clear();
                self.last_game_ended = None;
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn try_declare_multiple_blockers_on_one_attacker(&mut self) {
        let service = support::create_service();
        let assignments = self.blocker_assignments.clone();

        match service.declare_blockers(
            self.game_mut(),
            demonictutor::DeclareBlockersCommand::new(Self::player_id("Bob"), assignments),
        ) {
            Ok(_) => {
                self.last_error = None;
            }
            Err(error) => {
                self.last_error = Some(error.to_string());
            }
        }
    }

    pub fn ensure_tracked_land_provides_mana(&mut self) {
        let service = support::create_service();
        let land_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked land card should exist");

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
}
