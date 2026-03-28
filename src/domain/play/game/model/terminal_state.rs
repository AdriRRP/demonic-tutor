//! Supports game model terminal state.

use crate::domain::play::{events::GameEndReason, ids::PlayerId};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TerminalState {
    winner: Option<PlayerId>,
    loser: Option<PlayerId>,
    end_reason: Option<GameEndReason>,
}

impl TerminalState {
    #[must_use]
    pub const fn active() -> Self {
        Self {
            winner: None,
            loser: None,
            end_reason: None,
        }
    }

    #[must_use]
    pub const fn is_over(&self) -> bool {
        self.end_reason.is_some()
    }

    #[must_use]
    pub const fn winner(&self) -> Option<&PlayerId> {
        self.winner.as_ref()
    }

    #[must_use]
    pub const fn loser(&self) -> Option<&PlayerId> {
        self.loser.as_ref()
    }

    #[must_use]
    pub const fn end_reason(&self) -> Option<GameEndReason> {
        self.end_reason
    }

    pub fn end(&mut self, winner: PlayerId, loser: PlayerId, reason: GameEndReason) {
        self.winner = Some(winner);
        self.loser = Some(loser);
        self.end_reason = Some(reason);
    }

    pub fn end_draw(&mut self, reason: GameEndReason) {
        self.winner = None;
        self.loser = None;
        self.end_reason = Some(reason);
    }
}
