use super::support;
use super::GameplayWorld;
use demonictutor::{
    AdjustPlayerLifeEffectCommand, AdvanceTurnCommand, AdvanceTurnOutcome, CastSpellCommand,
    DealOpeningHandsCommand, DeclareBlockersCommand, DiscardForCleanupCommand,
    DrawCardsEffectCommand, ExileCardCommand, GameId, LibraryCard, PassPriorityCommand,
    PlayLandCommand, PlayerId, ResolveCombatDamageCommand, SpellTarget, StartGameCommand,
    TapLandCommand,
};

impl GameplayWorld {
    pub fn reset_observations(&mut self) {
        self.last_turn_progressed = None;
        self.last_game_ended = None;
        self.last_card_drawn = None;
        self.last_cards_drawn.clear();
        self.last_card_discarded = None;
        self.last_card_exiled = None;
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
        let service = support::create_service();

        // If priority is open and stack is empty, we must close it first by passing priority for all players
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

        self.pre_advance_hand_size = Some(self.player("Alice").hand().cards().len());

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

        self.post_advance_hand_size = Some(self.player("Alice").hand().cards().len());
    }

    pub fn cast_tracked_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(alias), card_id),
            )
            .expect("casting tracked spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn try_cast_tracked_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let res = service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn cast_tracked_targeted_player_spell(&mut self, caster_alias: &str, target_alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(caster_alias), card_id)
                    .with_target(SpellTarget::Player(Self::player_id(target_alias))),
            )
            .expect("casting targeted player spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn try_cast_tracked_targeted_player_spell(
        &mut self,
        caster_alias: &str,
        target_alias: &str,
    ) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let res = service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(caster_alias), card_id).with_target(
                SpellTarget::Player(match target_alias {
                    "Alice" | "Bob" => Self::player_id(target_alias),
                    raw_player_id => PlayerId::new(raw_player_id),
                }),
            ),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn cast_tracked_targeted_creature_spell(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let target_card_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked target creature should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(caster_alias), card_id)
                    .with_target(SpellTarget::Creature(target_card_id)),
            )
            .expect("casting targeted creature spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn cast_tracked_response_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(alias), card_id),
            )
            .expect("casting response spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn try_advance_turn(&mut self) {
        let service = support::create_service();

        // If priority is open and stack is empty, we must close it first by passing priority for all players
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

        self.pre_advance_hand_size = Some(self.player("Alice").hand().cards().len());

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
            self.post_advance_hand_size = Some(self.player("Alice").hand().cards().len());
        }
    }

    pub fn try_cast_tracked_response_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let service = support::create_service();
        let res = service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn cast_tracked_second_response_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_second_response_card_id
            .clone()
            .expect("tracked second response card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(alias), card_id),
            )
            .expect("casting second response spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
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

    pub fn resolve_combat_damage(&mut self, alias: &str) {
        let service = support::create_service();
        let outcome = service
            .resolve_combat_damage(
                self.game_mut(),
                ResolveCombatDamageCommand::new(Self::player_id(alias)),
            )
            .expect("resolving combat damage should succeed");

        self.last_combat_damage = Some(outcome.combat_damage_resolved);
        self.last_life_changed = outcome.life_changed;
        self.last_creature_died = outcome.creatures_died;
        self.last_game_ended = outcome.game_ended;
    }

    pub fn try_declare_multiple_blockers_on_one_attacker(&mut self, alias: &str) {
        let attacker_id = self
            .tracked_attacker_id
            .clone()
            .expect("tracked attacker should exist");
        let blocker_1_id = self.player(alias).battlefield().cards()[0].id().clone();
        let blocker_2_id = self.player(alias).battlefield().cards()[1].id().clone();

        let service = support::create_service();
        let res = service.declare_blockers(
            self.game_mut(),
            DeclareBlockersCommand::new(
                Self::player_id(alias),
                vec![
                    (blocker_1_id, attacker_id.clone()),
                    (blocker_2_id, attacker_id),
                ],
            ),
        );

        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
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

    pub fn declare_blocker_against(
        &mut self,
        blocker_alias: &str,
        blocker_id: &demonictutor::CardInstanceId,
        attacker_id: &demonictutor::CardInstanceId,
    ) {
        let service = support::create_service();
        service
            .declare_blockers(
                self.game_mut(),
                DeclareBlockersCommand::new(
                    Self::player_id(blocker_alias),
                    vec![(blocker_id.clone(), attacker_id.clone())],
                ),
            )
            .expect("blocking should succeed");
    }

    pub fn try_declare_blocker_against(
        &mut self,
        blocker_alias: &str,
        blocker_id: &demonictutor::CardInstanceId,
        attacker_id: &demonictutor::CardInstanceId,
    ) {
        let service = support::create_service();
        let res = service.declare_blockers(
            self.game_mut(),
            DeclareBlockersCommand::new(
                Self::player_id(blocker_alias),
                vec![(blocker_id.clone(), attacker_id.clone())],
            ),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn exile_tracked_card(&mut self, alias: &str, from_battlefield: bool) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let event = service
            .exile_card(
                self.game_mut(),
                &ExileCardCommand::new(Self::player_id(alias), card_id, from_battlefield),
            )
            .expect("exiling tracked card should succeed");
        self.last_card_exiled = Some(event);
    }
}
