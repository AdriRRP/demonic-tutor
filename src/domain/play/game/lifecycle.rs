//! Supports play game lifecycle.

use {
    super::{invariants, rules, Game},
    crate::domain::play::{
        commands::{DealOpeningHandsCommand, MulliganCommand, StartGameCommand},
        errors::DomainError,
        events::{GameStarted, MulliganTaken, OpeningHandDealt},
    },
};

impl Game {
    /// Starts a new game.
    ///
    /// # Errors
    /// See [`rules::lifecycle::start`].
    pub fn start(cmd: StartGameCommand) -> Result<(Self, GameStarted), DomainError> {
        rules::lifecycle::start(cmd)
    }

    /// Deals opening hands to players.
    ///
    /// # Errors
    /// See [`rules::lifecycle::deal_opening_hands`].
    pub fn deal_opening_hands(
        &mut self,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        rules::lifecycle::deal_opening_hands(&mut self.players, cmd, &self.id)
    }

    /// Performs a mulligan.
    ///
    /// # Errors
    /// See [`rules::lifecycle::mulligan`].
    pub fn mulligan(&mut self, cmd: MulliganCommand) -> Result<MulliganTaken, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        rules::lifecycle::mulligan(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }
}
