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
        rules::turn_flow::advance_turn(
            TurnProgressionContext {
                game_id: &self.id,
                players: &mut self.players,
                active_player: &mut self.active_player,
                phase: &mut self.phase,
                priority: &mut self.priority,
                turn_number: &mut self.turn_number,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        )
    }

    /// Resolves an explicit draw effect.
    ///
    /// # Errors
    /// See [`rules::turn_flow::draw_cards_effect`].
    pub fn draw_cards_effect(
        &mut self,
        cmd: &DrawCardsEffectCommand,
    ) -> Result<rules::turn_flow::DrawCardsEffectOutcome, DomainError> {
        invariants::require_empty_stack_priority_action_window(
            self.priority(),
            self.stack.is_empty(),
            &self.active_player,
        )?;
        rules::turn_flow::draw_cards_effect(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            &mut self.terminal_state,
            cmd,
        )
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
        rules::turn_flow::discard_for_cleanup(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

    /// Exiles a card from battlefield or graveyard.
    ///
    /// # Errors
    /// See [`rules::zones::exile_card_from_battlefield`] and [`rules::zones::exile_card_from_graveyard`].
    pub fn exile_card(&mut self, cmd: &ExileCardCommand) -> Result<CardExiled, DomainError> {
        invariants::require_game_active(self.is_over())?;
        if cmd.from_battlefield {
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
        }
    }
}
