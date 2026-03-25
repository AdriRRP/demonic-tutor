//! Supports domain play game.

mod combat;
mod helpers;
mod invariants;
mod lifecycle;
pub mod model;
mod resource_actions;
pub mod rules;
mod stack;
mod targets;
mod turn_flow;

use crate::domain::play::{
    errors::{DomainError, GameError},
    events::GameEndReason,
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

pub use model::Player;
pub use model::{
    ActivatedAbilityOnStack, AggregateCardLocationIndex, PlayerCardZone, PrepareHandSpellCastError,
    PreparedHandSpellCast, PriorityState, SpellOnStack, StackObject, StackObjectKind, StackZone,
    TerminalState,
};
pub use rules::{
    combat::ResolveCombatDamageOutcome,
    resource_actions::AdjustPlayerLifeEffectOutcome,
    stack_priority::{
        ActivateAbilityOutcome, CastSpellOutcome, PassPriorityOutcome, StackPriorityContext,
    },
    turn_flow::TurnProgressionContext,
    turn_flow::{AdvanceTurnOutcome, DrawCardsEffectOutcome},
};
pub use targets::SpellTarget;

#[derive(Debug)]
pub struct Game {
    id: GameId,
    player_ids: Vec<PlayerId>,
    active_player_index: usize,
    phase: Phase,
    turn_number: u32,
    players: Vec<Player>,
    card_locations: AggregateCardLocationIndex,
    stack: StackZone,
    priority: Option<PriorityState>,
    terminal_state: TerminalState,
}

impl Game {
    /// # Errors
    ///
    /// Returns [`DomainError::Game`] with [`GameError::PlayerNotFound`] if
    /// `active_player` is not present in `players`.
    pub fn new(
        id: GameId,
        active_player: &PlayerId,
        phase: Phase,
        turn_number: u32,
        players: Vec<Player>,
        terminal_state: TerminalState,
    ) -> Result<Self, DomainError> {
        let player_ids: Vec<_> = players.iter().map(|player| player.id().clone()).collect();
        let active_player_index = player_ids
            .iter()
            .position(|player_id| player_id == active_player)
            .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(active_player.clone())))?;
        let card_locations = AggregateCardLocationIndex::from_players(&players);
        Ok(Self {
            id,
            player_ids,
            active_player_index,
            phase,
            turn_number,
            players,
            card_locations,
            stack: StackZone::empty(),
            priority: None,
            terminal_state,
        })
    }

    #[must_use]
    pub const fn id(&self) -> &GameId {
        &self.id
    }

    #[must_use]
    pub fn active_player(&self) -> &PlayerId {
        &self.player_ids[self.active_player_index]
    }

    #[must_use]
    pub const fn phase(&self) -> &Phase {
        &self.phase
    }

    #[must_use]
    pub const fn turn_number(&self) -> u32 {
        self.turn_number
    }

    #[must_use]
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    #[must_use]
    pub const fn stack(&self) -> &StackZone {
        &self.stack
    }

    #[must_use]
    pub const fn priority(&self) -> Option<&PriorityState> {
        self.priority.as_ref()
    }

    #[must_use]
    pub const fn has_open_priority_window(&self) -> bool {
        self.priority.is_some()
    }

    #[must_use]
    pub const fn is_over(&self) -> bool {
        self.terminal_state.is_over()
    }

    #[must_use]
    pub const fn winner(&self) -> Option<&PlayerId> {
        self.terminal_state.winner()
    }

    #[must_use]
    pub const fn loser(&self) -> Option<&PlayerId> {
        self.terminal_state.loser()
    }

    #[must_use]
    pub const fn end_reason(&self) -> Option<GameEndReason> {
        self.terminal_state.end_reason()
    }

    fn sync_card_location_from_player(&mut self, owner_index: usize, card_id: &CardInstanceId) {
        let Some(player) = self.players.get(owner_index) else {
            return;
        };

        let Some(handle) = player.resolve_public_card_handle(card_id) else {
            self.card_locations.remove(card_id);
            return;
        };
        let Some(zone) = player.card_zone(card_id) else {
            self.card_locations.remove(card_id);
            return;
        };

        self.card_locations
            .upsert(card_id.clone(), owner_index, handle, zone);
    }
}
