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
    events::{
        CardDiscarded, CardDrawn, CardMovedZone, CreatureDied, GameEndReason, LandPlayed, ZoneType,
    },
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

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct GameCheckpointSpec(u16);

impl GameCheckpointSpec {
    const ACTIVE_PLAYER_INDEX: u16 = 1 << 0;
    const PHASE: u16 = 1 << 1;
    const TURN_NUMBER: u16 = 1 << 2;
    const PLAYERS: u16 = 1 << 3;
    const CARD_LOCATIONS: u16 = 1 << 4;
    const STACK: u16 = 1 << 5;
    const PRIORITY: u16 = 1 << 6;
    const PENDING_DECISION: u16 = 1 << 7;
    const TERMINAL_STATE: u16 = 1 << 8;

    pub(crate) const DEAL_OPENING_HANDS: Self = Self(Self::PLAYERS);

    pub(crate) const MULLIGAN: Self = Self::DEAL_OPENING_HANDS;

    pub(crate) const CONCEDE: Self = Self(Self::TERMINAL_STATE);

    pub(crate) const PLAY_LAND: Self = Self(Self::PLAYERS | Self::CARD_LOCATIONS);

    pub(crate) const EXILE_CARD: Self = Self::PLAY_LAND;
    pub(crate) const DRAW_CARDS_EFFECT: Self =
        Self(Self::PLAYERS | Self::CARD_LOCATIONS | Self::TERMINAL_STATE);
    pub(crate) const DISCARD_FOR_CLEANUP: Self = Self::PLAY_LAND;

    pub(crate) const ADJUST_PLAYER_LIFE_EFFECT: Self =
        Self(Self::PLAYERS | Self::CARD_LOCATIONS | Self::TERMINAL_STATE);

    pub(crate) const TAP_LAND: Self = Self(Self::PLAYERS);

    pub(crate) const DECLARE_ATTACKERS: Self =
        Self(Self::PHASE | Self::PLAYERS | Self::STACK | Self::PRIORITY);

    pub(crate) const DECLARE_BLOCKERS: Self = Self(Self::PHASE | Self::PLAYERS | Self::PRIORITY);

    pub(crate) const RESOLVE_COMBAT_DAMAGE: Self = Self(
        Self::PHASE
            | Self::PLAYERS
            | Self::CARD_LOCATIONS
            | Self::STACK
            | Self::PRIORITY
            | Self::TERMINAL_STATE,
    );

    pub(crate) const ADVANCE_TURN: Self = Self(
        Self::ACTIVE_PLAYER_INDEX
            | Self::PHASE
            | Self::TURN_NUMBER
            | Self::PLAYERS
            | Self::CARD_LOCATIONS
            | Self::STACK
            | Self::PRIORITY
            | Self::TERMINAL_STATE,
    );

    pub(crate) const STACK_PRIORITY: Self = Self(
        Self::PLAYERS
            | Self::CARD_LOCATIONS
            | Self::STACK
            | Self::PRIORITY
            | Self::PENDING_DECISION
            | Self::TERMINAL_STATE,
    );

    const fn captures(self, flag: u16) -> bool {
        self.0 & flag != 0
    }
}

#[derive(Debug, Clone, Default)]
enum SnapshotField<T> {
    #[default]
    Skipped,
    Captured(T),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct GameCheckpoint {
    active_player_index: SnapshotField<usize>,
    phase: SnapshotField<Phase>,
    turn_number: SnapshotField<u32>,
    players: SnapshotField<Vec<Player>>,
    card_locations: SnapshotField<AggregateCardLocationIndex>,
    stack: SnapshotField<StackZone>,
    priority: SnapshotField<Option<PriorityState>>,
    pending_decision: SnapshotField<Option<PendingDecision>>,
    terminal_state: SnapshotField<TerminalState>,
}

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

    pub(crate) fn zone_change_for_creature_died(event: &CreatureDied) -> CardMovedZone {
        CardMovedZone::new(
            event.game_id.clone(),
            event.player_id.clone(),
            event.card_id.clone(),
            ZoneType::Battlefield,
            ZoneType::Graveyard,
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

    pub(crate) fn checkpoint(&self, spec: GameCheckpointSpec) -> GameCheckpoint {
        GameCheckpoint {
            active_player_index: if spec.captures(GameCheckpointSpec::ACTIVE_PLAYER_INDEX) {
                SnapshotField::Captured(self.active_player_index)
            } else {
                SnapshotField::Skipped
            },
            phase: if spec.captures(GameCheckpointSpec::PHASE) {
                SnapshotField::Captured(self.phase)
            } else {
                SnapshotField::Skipped
            },
            turn_number: if spec.captures(GameCheckpointSpec::TURN_NUMBER) {
                SnapshotField::Captured(self.turn_number)
            } else {
                SnapshotField::Skipped
            },
            players: if spec.captures(GameCheckpointSpec::PLAYERS) {
                SnapshotField::Captured(self.players.clone())
            } else {
                SnapshotField::Skipped
            },
            card_locations: if spec.captures(GameCheckpointSpec::CARD_LOCATIONS) {
                SnapshotField::Captured(self.card_locations.clone())
            } else {
                SnapshotField::Skipped
            },
            stack: if spec.captures(GameCheckpointSpec::STACK) {
                SnapshotField::Captured(self.stack.clone())
            } else {
                SnapshotField::Skipped
            },
            priority: if spec.captures(GameCheckpointSpec::PRIORITY) {
                SnapshotField::Captured(self.priority.clone())
            } else {
                SnapshotField::Skipped
            },
            pending_decision: if spec.captures(GameCheckpointSpec::PENDING_DECISION) {
                SnapshotField::Captured(self.pending_decision.clone())
            } else {
                SnapshotField::Skipped
            },
            terminal_state: if spec.captures(GameCheckpointSpec::TERMINAL_STATE) {
                SnapshotField::Captured(self.terminal_state.clone())
            } else {
                SnapshotField::Skipped
            },
        }
    }

    pub(crate) fn restore_checkpoint(&mut self, checkpoint: GameCheckpoint) {
        if let SnapshotField::Captured(active_player_index) = checkpoint.active_player_index {
            self.active_player_index = active_player_index;
        }
        if let SnapshotField::Captured(phase) = checkpoint.phase {
            self.phase = phase;
        }
        if let SnapshotField::Captured(turn_number) = checkpoint.turn_number {
            self.turn_number = turn_number;
        }
        if let SnapshotField::Captured(players) = checkpoint.players {
            self.players = players;
        }
        if let SnapshotField::Captured(card_locations) = checkpoint.card_locations {
            self.card_locations = card_locations;
        }
        if let SnapshotField::Captured(stack) = checkpoint.stack {
            self.stack = stack;
        }
        if let SnapshotField::Captured(priority) = checkpoint.priority {
            self.priority = priority;
        }
        if let SnapshotField::Captured(pending_decision) = checkpoint.pending_decision {
            self.pending_decision = pending_decision;
        }
        if let SnapshotField::Captured(terminal_state) = checkpoint.terminal_state {
            self.terminal_state = terminal_state;
        }
    }
}
