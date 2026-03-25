//! Supports play game turn flow.

use {
    super::{invariants, rules, Game, TurnProgressionContext},
    crate::domain::play::{
        commands::{
            AdvanceTurnCommand, DiscardForCleanupCommand, DrawCardsEffectCommand, ExileCardCommand,
        },
        errors::DomainError,
        events::{CardDiscarded, CardExiled},
    },
};

impl Game {
    /// Advances the turn to the next phase and player.
    ///
    /// # Errors
    /// See [`rules::turn_flow::advance_turn`].
    pub fn advance_turn(
        &mut self,
        cmd: AdvanceTurnCommand,
    ) -> Result<rules::turn_flow::AdvanceTurnOutcome, DomainError> {
        invariants::require_no_open_priority_window(self.priority())?;
        let result = rules::turn_flow::advance_turn(
            TurnProgressionContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                player_ids: &self.player_ids,
                active_player_index: &mut self.active_player_index,
                phase: &mut self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                turn_number: &mut self.turn_number,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(rules::turn_flow::AdvanceTurnOutcome::Progressed {
            card_drawn: Some(card_drawn),
            ..
        }) = &result
        {
            self.sync_card_location_from_player(self.active_player_index, &card_drawn.card_id);
        }
        result
    }

    /// Resolves an explicit draw effect.
    ///
    /// # Errors
    /// See [`rules::turn_flow::draw_cards_effect`].
    pub fn draw_cards_effect(
        &mut self,
        cmd: &DrawCardsEffectCommand,
    ) -> Result<rules::turn_flow::DrawCardsEffectOutcome, DomainError> {
        let active_player_index = self.active_player_index;
        invariants::require_empty_stack_priority_action_window(
            self.priority(),
            self.stack.is_empty(),
            self.active_player(),
        )?;
        let target_player_index =
            super::helpers::find_player_index(&self.players, &cmd.target_player_id)?;
        let result = rules::turn_flow::draw_cards_effect(
            &self.id,
            &mut self.players,
            active_player_index,
            target_player_index,
            &self.phase,
            &mut self.terminal_state,
            cmd,
        );
        if let Ok(outcome) = &result {
            for card_drawn in &outcome.cards_drawn {
                self.sync_card_location_from_player(target_player_index, &card_drawn.card_id);
            }
        }
        result
    }

    /// Discards one card from hand during cleanup-related turn flow.
    ///
    /// # Errors
    /// See [`rules::turn_flow::discard_for_cleanup`].
    pub fn discard_for_cleanup(
        &mut self,
        cmd: DiscardForCleanupCommand,
    ) -> Result<CardDiscarded, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player_index = self.active_player_index;
        let result = rules::turn_flow::discard_for_cleanup(
            &self.id,
            &mut self.players,
            active_player_index,
            &self.phase,
            cmd,
        );
        if let Ok(event) = &result {
            self.sync_card_location_from_player(active_player_index, &event.card_id);
        }
        result
    }

    /// Exiles a card from battlefield or graveyard.
    ///
    /// # Errors
    /// See [`rules::zones::exile_card_from_battlefield`] and [`rules::zones::exile_card_from_graveyard`].
    pub fn exile_card(&mut self, cmd: &ExileCardCommand) -> Result<CardExiled, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let indexed_location = self
            .card_locations
            .location(&cmd.card_id)
            .filter(|location| {
                let expected_zone = if cmd.from_battlefield {
                    crate::domain::play::game::PlayerCardZone::Battlefield
                } else {
                    crate::domain::play::game::PlayerCardZone::Graveyard
                };
                location.zone() == expected_zone
                    && self.players[location.owner_index()].id() == &cmd.player_id
            });
        let result = if let Some(location) = indexed_location {
            if cmd.from_battlefield {
                rules::zones::exile_card_from_battlefield_handle_by_index(
                    &self.id,
                    &mut self.players,
                    location.owner_index(),
                    location.handle(),
                )
            } else {
                rules::zones::exile_card_from_graveyard_handle_by_index(
                    &self.id,
                    &mut self.players,
                    location.owner_index(),
                    location.handle(),
                )
            }
        } else if cmd.from_battlefield {
            rules::zones::exile_card_from_battlefield(
                &self.id,
                &mut self.players,
                &cmd.player_id,
                &cmd.card_id,
            )
        } else {
            rules::zones::exile_card_from_graveyard(
                &self.id,
                &mut self.players,
                &cmd.player_id,
                &cmd.card_id,
            )
        };
        if let Ok(event) = &result {
            let owner_index = super::helpers::find_player_index(&self.players, &event.player_id)?;
            self.sync_card_location_from_player(owner_index, &event.card_id);
        }
        result
    }
}
