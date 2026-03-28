//! Supports domain play game.

mod combat;
mod helpers;
mod invariants;
mod lifecycle;
pub mod model;
mod queries;
mod resource_actions;
pub mod rules;
mod stack;
mod targets;
mod turn_flow;

use crate::domain::play::{
    errors::{DomainError, GameError},
    events::{CardDiscarded, CardDrawn, CardMovedZone, GameEndReason, LandPlayed, ZoneType},
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

pub use model::Player;
pub use model::{
    ActivatedAbilityOnStack, AggregateCardLocationIndex, PlayerCardZone, PrepareHandSpellCastError,
    PreparedHandSpellCast, PriorityState, SpellOnStack, StackObject, StackObjectKind, StackZone,
    TerminalState, TriggeredAbilityOnStack,
};
pub use queries::LegalBlockerOption;
pub use rules::{
    combat::{DeclareAttackersOutcome, ResolveCombatDamageOutcome},
    resource_actions::AdjustPlayerLifeEffectOutcome,
    stack_priority::{
        ActivateAbilityOutcome, CastSpellOutcome, PassPriorityOutcome,
        ResolveOptionalEffectOutcome, ResolvePendingHandChoiceOutcome, ResolvePendingScryOutcome,
        ResolvePendingSurveilOutcome, StackPriorityContext,
    },
    turn_flow::TurnProgressionContext,
    turn_flow::{AdvanceTurnOutcome, DrawCardsEffectOutcome},
};
pub use targets::SpellTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingHandChoiceKind {
    Loot { draw_count: u32 },
    Rummage { draw_count: u32 },
}

#[derive(Debug, Clone)]
pub enum PendingDecision {
    OptionalEffect {
        controller_index: usize,
        stack_object_number: u32,
    },
    HandChoice {
        controller_index: usize,
        stack_object_number: u32,
        kind: PendingHandChoiceKind,
    },
    Scry {
        controller_index: usize,
        stack_object_number: u32,
        amount: u32,
    },
    Surveil {
        controller_index: usize,
        stack_object_number: u32,
        amount: u32,
    },
}

impl PendingDecision {
    #[must_use]
    pub const fn optional_effect(controller_index: usize, stack_object_number: u32) -> Self {
        Self::OptionalEffect {
            controller_index,
            stack_object_number,
        }
    }

    #[must_use]
    pub const fn hand_choice(
        controller_index: usize,
        stack_object_number: u32,
        kind: PendingHandChoiceKind,
    ) -> Self {
        Self::HandChoice {
            controller_index,
            stack_object_number,
            kind,
        }
    }

    #[must_use]
    pub const fn scry(controller_index: usize, stack_object_number: u32, amount: u32) -> Self {
        Self::Scry {
            controller_index,
            stack_object_number,
            amount,
        }
    }

    #[must_use]
    pub const fn surveil(controller_index: usize, stack_object_number: u32, amount: u32) -> Self {
        Self::Surveil {
            controller_index,
            stack_object_number,
            amount,
        }
    }

    #[must_use]
    pub const fn controller_index(&self) -> usize {
        match self {
            Self::OptionalEffect {
                controller_index, ..
            }
            | Self::HandChoice {
                controller_index, ..
            }
            | Self::Scry {
                controller_index, ..
            }
            | Self::Surveil {
                controller_index, ..
            } => *controller_index,
        }
    }

    #[must_use]
    pub const fn stack_object_number(&self) -> u32 {
        match self {
            Self::OptionalEffect {
                stack_object_number,
                ..
            }
            | Self::HandChoice {
                stack_object_number,
                ..
            }
            | Self::Scry {
                stack_object_number,
                ..
            }
            | Self::Surveil {
                stack_object_number,
                ..
            } => *stack_object_number,
        }
    }
}

#[derive(Debug, Clone)]
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
    pending_decision: Option<PendingDecision>,
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
            pending_decision: None,
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
    pub const fn pending_decision(&self) -> Option<&PendingDecision> {
        self.pending_decision.as_ref()
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

    pub(crate) fn player_index(&self, player_id: &PlayerId) -> Result<usize, DomainError> {
        helpers::find_player_index(&self.players, player_id)
    }

    #[must_use]
    pub(crate) fn cloned_player(&self, index: usize) -> Option<Player> {
        self.players.get(index).cloned()
    }

    pub(crate) fn replace_player(&mut self, index: usize, player: Player) -> Option<()> {
        let slot = self.players.get_mut(index)?;
        *slot = player;
        Some(())
    }

    #[must_use]
    pub(crate) const fn active_player_index_value(&self) -> usize {
        self.active_player_index
    }

    pub(crate) const fn replace_active_player_index(&mut self, active_player_index: usize) {
        self.active_player_index = active_player_index;
    }

    #[must_use]
    pub(crate) const fn phase_value(&self) -> Phase {
        self.phase
    }

    pub(crate) const fn replace_phase(&mut self, phase: Phase) {
        self.phase = phase;
    }

    #[must_use]
    pub(crate) const fn turn_number_value(&self) -> u32 {
        self.turn_number
    }

    pub(crate) const fn replace_turn_number(&mut self, turn_number: u32) {
        self.turn_number = turn_number;
    }

    #[must_use]
    pub(crate) fn cloned_card_locations(&self) -> AggregateCardLocationIndex {
        self.card_locations.clone()
    }

    pub(crate) fn replace_card_locations(&mut self, card_locations: AggregateCardLocationIndex) {
        self.card_locations = card_locations;
    }

    #[must_use]
    pub(crate) fn cloned_stack(&self) -> StackZone {
        self.stack.clone()
    }

    pub(crate) fn replace_stack(&mut self, stack: StackZone) {
        self.stack = stack;
    }

    #[must_use]
    pub(crate) fn cloned_priority(&self) -> Option<PriorityState> {
        self.priority.clone()
    }

    pub(crate) fn replace_priority(&mut self, priority: Option<PriorityState>) {
        self.priority = priority;
    }

    #[must_use]
    pub(crate) fn cloned_pending_decision(&self) -> Option<PendingDecision> {
        self.pending_decision.clone()
    }

    pub(crate) const fn replace_pending_decision(
        &mut self,
        pending_decision: Option<PendingDecision>,
    ) {
        self.pending_decision = pending_decision;
    }

    #[must_use]
    pub(crate) fn cloned_terminal_state(&self) -> TerminalState {
        self.terminal_state.clone()
    }

    pub(crate) fn replace_terminal_state(&mut self, terminal_state: TerminalState) {
        self.terminal_state = terminal_state;
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

    fn sync_zone_changes(&mut self, zone_changes: &[CardMovedZone]) -> Result<(), DomainError> {
        for zone_change in zone_changes {
            self.sync_card_location_from_zone_change(zone_change)?;
        }

        Ok(())
    }

    fn rebuild_card_locations_from_players(&mut self) {
        self.card_locations = AggregateCardLocationIndex::from_players(&self.players);
    }

    pub(crate) fn zone_change_for_card_drawn(event: &CardDrawn) -> CardMovedZone {
        CardMovedZone::new(
            event.game_id.clone(),
            event.player_id.clone(),
            event.card_id.clone(),
            ZoneType::Library,
            ZoneType::Hand,
        )
    }

    pub(crate) fn zone_change_for_card_discarded(event: &CardDiscarded) -> CardMovedZone {
        CardMovedZone::new(
            event.game_id.clone(),
            event.player_id.clone(),
            event.card_id.clone(),
            ZoneType::Hand,
            ZoneType::Graveyard,
        )
    }

    pub(crate) fn zone_change_for_land_played(event: &LandPlayed) -> CardMovedZone {
        CardMovedZone::new(
            event.game_id.clone(),
            event.player_id.clone(),
            event.card_id.clone(),
            ZoneType::Hand,
            ZoneType::Battlefield,
        )
    }

    fn sync_card_location_from_zone_change(
        &mut self,
        zone_change: &CardMovedZone,
    ) -> Result<(), DomainError> {
        match zone_change.destination_zone {
            ZoneType::Library
            | ZoneType::Hand
            | ZoneType::Battlefield
            | ZoneType::Graveyard
            | ZoneType::Exile => {
                let owner_index =
                    helpers::find_player_index(&self.players, &zone_change.zone_owner_id)?;
                self.sync_card_location_from_player(owner_index, &zone_change.card_id);
            }
            ZoneType::Stack | ZoneType::Created => {
                self.card_locations.remove(&zone_change.card_id);
            }
        }

        Ok(())
    }
}
