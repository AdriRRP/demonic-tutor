mod combat;
mod helpers;
mod invariants;
mod lifecycle;
pub mod model;
mod resource_actions;
pub mod rules;
mod stack;
mod turn_flow;

use crate::domain::play::{
    events::GameEndReason,
    ids::{GameId, PlayerId},
    phase::Phase,
};

pub use model::Player;
pub use model::{
    ActivatedAbilityOnStack, PlayerCardZone, PriorityState, SpellOnStack, SpellTarget, StackObject,
    StackObjectKind, StackZone, TerminalState,
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

#[derive(Debug)]
pub struct Game {
    id: GameId,
    active_player: PlayerId,
    phase: Phase,
    turn_number: u32,
    players: Vec<Player>,
    stack: StackZone,
    priority: Option<PriorityState>,
    terminal_state: TerminalState,
}

impl Game {
    #[must_use]
    pub const fn new(
        id: GameId,
        active_player: PlayerId,
        phase: Phase,
        turn_number: u32,
        players: Vec<Player>,
        terminal_state: TerminalState,
    ) -> Self {
        Self {
            id,
            active_player,
            phase,
            turn_number,
            players,
            stack: StackZone::empty(),
            priority: None,
            terminal_state,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &GameId {
        &self.id
    }

    #[must_use]
    pub const fn active_player(&self) -> &PlayerId {
        &self.active_player
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
}
