//! Supports application-local rollback snapshots for persisted command failures.

use crate::domain::play::{
    errors::{DomainError, GameError},
    game::{
        AggregateCardLocationIndex, Game, PendingDecision, Player, PriorityState, StackZone,
        TerminalState,
    },
    ids::PlayerId,
    phase::Phase,
};

#[derive(Debug, Clone, Default)]
enum SnapshotField<T> {
    #[default]
    Skipped,
    Captured(T),
}

#[derive(Debug, Clone, Default)]
pub(super) struct GameRollback {
    active_player_index: SnapshotField<usize>,
    phase: SnapshotField<Phase>,
    turn_number: SnapshotField<u32>,
    players: Vec<(usize, Player)>,
    card_locations: SnapshotField<AggregateCardLocationIndex>,
    stack: SnapshotField<StackZone>,
    priority: SnapshotField<Option<PriorityState>>,
    pending_decision: SnapshotField<Option<PendingDecision>>,
    terminal_state: SnapshotField<TerminalState>,
}

impl GameRollback {
    pub(super) fn capture_player(
        self,
        game: &Game,
        player_id: &PlayerId,
    ) -> Result<Self, DomainError> {
        let index = game.player_index(player_id)?;
        self.capture_player_index(game, index)
    }

    pub(super) fn capture_player_index(
        mut self,
        game: &Game,
        index: usize,
    ) -> Result<Self, DomainError> {
        if self.players.iter().any(|(existing, _)| *existing == index) {
            return Ok(self);
        }

        let player = game.cloned_player(index).ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "missing player snapshot at index {index}"
            )))
        })?;
        self.players.push((index, player));
        Ok(self)
    }

    pub(super) fn capture_all_players(mut self, game: &Game) -> Result<Self, DomainError> {
        self.players = game
            .players()
            .iter()
            .enumerate()
            .map(|(index, _)| {
                game.cloned_player(index)
                    .map(|player| (index, player))
                    .ok_or_else(|| {
                        DomainError::Game(GameError::InternalInvariantViolation(format!(
                            "missing player snapshot at index {index}"
                        )))
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self)
    }

    #[must_use]
    pub(super) const fn capture_active_player_index(mut self, game: &Game) -> Self {
        self.active_player_index = SnapshotField::Captured(game.active_player_index_value());
        self
    }

    #[must_use]
    pub(super) const fn capture_phase(mut self, game: &Game) -> Self {
        self.phase = SnapshotField::Captured(game.phase_value());
        self
    }

    #[must_use]
    pub(super) const fn capture_turn_number(mut self, game: &Game) -> Self {
        self.turn_number = SnapshotField::Captured(game.turn_number_value());
        self
    }

    #[must_use]
    pub(super) fn capture_card_locations(mut self, game: &Game) -> Self {
        self.card_locations = SnapshotField::Captured(game.cloned_card_locations());
        self
    }

    #[must_use]
    pub(super) fn capture_stack(mut self, game: &Game) -> Self {
        self.stack = SnapshotField::Captured(game.cloned_stack());
        self
    }

    #[must_use]
    pub(super) fn capture_priority(mut self, game: &Game) -> Self {
        self.priority = SnapshotField::Captured(game.cloned_priority());
        self
    }

    #[must_use]
    pub(super) fn capture_pending_decision(mut self, game: &Game) -> Self {
        self.pending_decision = SnapshotField::Captured(game.cloned_pending_decision());
        self
    }

    #[must_use]
    pub(super) fn capture_terminal_state(mut self, game: &Game) -> Self {
        self.terminal_state = SnapshotField::Captured(game.cloned_terminal_state());
        self
    }

    pub(super) fn restore(self, game: &mut Game) -> Result<(), DomainError> {
        if let SnapshotField::Captured(active_player_index) = self.active_player_index {
            game.replace_active_player_index(active_player_index);
        }
        if let SnapshotField::Captured(phase) = self.phase {
            game.replace_phase(phase);
        }
        if let SnapshotField::Captured(turn_number) = self.turn_number {
            game.replace_turn_number(turn_number);
        }
        for (index, player) in self.players {
            game.replace_player(index, player).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "missing player slot {index} during rollback restore"
                )))
            })?;
        }
        if let SnapshotField::Captured(card_locations) = self.card_locations {
            game.replace_card_locations(card_locations);
        }
        if let SnapshotField::Captured(stack) = self.stack {
            game.replace_stack(stack);
        }
        if let SnapshotField::Captured(priority) = self.priority {
            game.replace_priority(priority);
        }
        if let SnapshotField::Captured(pending_decision) = self.pending_decision {
            game.replace_pending_decision(pending_decision);
        }
        if let SnapshotField::Captured(terminal_state) = self.terminal_state {
            game.replace_terminal_state(terminal_state);
        }

        Ok(())
    }
}
