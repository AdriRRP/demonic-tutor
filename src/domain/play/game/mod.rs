mod helpers;
mod invariants;
pub mod model;
pub mod rules;

use crate::domain::play::{
    commands::{
        AdjustPlayerLifeEffectCommand, AdvanceTurnCommand, CastSpellCommand,
        DealOpeningHandsCommand, DeclareAttackersCommand, DeclareBlockersCommand,
        DiscardForCleanupCommand, DrawCardsEffectCommand, ExileCardCommand, MulliganCommand,
        PassPriorityCommand, PlayLandCommand, ResolveCombatDamageCommand, StartGameCommand,
        TapLandCommand,
    },
    errors::DomainError,
    events::{
        AttackersDeclared, BlockersDeclared, CardDiscarded, CardExiled, GameEndReason, GameStarted,
        LandPlayed, LandTapped, ManaAdded, MulliganTaken, OpeningHandDealt,
    },
    ids::{GameId, PlayerId},
    phase::Phase,
};

pub use model::Player;
pub use model::{
    PriorityState, SpellOnStack, SpellTarget, StackObject, StackObjectKind, StackZone,
    TerminalState,
};
pub use rules::{
    combat::ResolveCombatDamageOutcome,
    resource_actions::AdjustPlayerLifeEffectOutcome,
    stack_priority::{CastSpellOutcome, PassPriorityOutcome, StackPriorityContext},
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

    /// Plays a land from hand to battlefield.
    ///
    /// # Errors
    /// See [`rules::resource_actions::play_land`].
    pub fn play_land(&mut self, cmd: PlayLandCommand) -> Result<LandPlayed, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_priority_with_pending_stack(self.priority(), self.stack.is_empty())?;
        rules::resource_actions::play_land(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

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

    /// Resolves an explicit life effect from a caster onto a target player.
    ///
    /// # Errors
    /// See [`rules::resource_actions::adjust_player_life_effect`].
    pub fn adjust_player_life_effect(
        &mut self,
        cmd: AdjustPlayerLifeEffectCommand,
    ) -> Result<AdjustPlayerLifeEffectOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_empty_stack_priority_action_window(
            self.priority(),
            self.stack.is_empty(),
            &self.active_player,
        )?;
        rules::resource_actions::adjust_player_life_effect(
            &self.id,
            &mut self.players,
            &mut self.terminal_state,
            cmd,
        )
    }

    /// Taps a land to produce mana.
    ///
    /// # Errors
    /// See [`rules::resource_actions::tap_land`].
    pub fn tap_land(
        &mut self,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_priority_with_pending_stack(self.priority(), self.stack.is_empty())?;
        rules::resource_actions::tap_land(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

    /// Casts a spell.
    ///
    /// # Errors
    /// See [`rules::stack_priority::cast_spell`].
    pub fn cast_spell(&mut self, cmd: CastSpellCommand) -> Result<CastSpellOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        rules::stack_priority::cast_spell(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                active_player: &self.active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        )
    }

    /// Passes priority in an open priority window.
    ///
    /// # Errors
    /// See [`rules::stack_priority::pass_priority`].
    pub fn pass_priority(
        &mut self,
        cmd: PassPriorityCommand,
    ) -> Result<PassPriorityOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        rules::stack_priority::pass_priority(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                active_player: &self.active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        )
    }

    /// Declares attackers in combat.
    ///
    /// # Errors
    /// See [`rules::combat::declare_attackers`].
    pub fn declare_attackers(
        &mut self,
        cmd: DeclareAttackersCommand,
    ) -> Result<AttackersDeclared, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let event = rules::combat::declare_attackers(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )?;
        self.phase = Phase::DeclareBlockers;
        self.priority = Some(PriorityState::new(self.active_player.clone()));
        Ok(event)
    }

    /// Declares blockers in combat.
    ///
    /// # Errors
    /// See [`rules::combat::declare_blockers`].
    pub fn declare_blockers(
        &mut self,
        cmd: DeclareBlockersCommand,
    ) -> Result<BlockersDeclared, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let event = rules::combat::declare_blockers(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )?;
        self.phase = Phase::CombatDamage;
        self.priority = Some(PriorityState::new(self.active_player.clone()));
        Ok(event)
    }

    /// Resolves combat damage.
    ///
    /// # Errors
    /// See [`rules::combat::resolve_combat_damage`].
    pub fn resolve_combat_damage(
        &mut self,
        cmd: ResolveCombatDamageCommand,
    ) -> Result<ResolveCombatDamageOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let outcome = rules::combat::resolve_combat_damage(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            &mut self.terminal_state,
            cmd,
        )?;

        self.phase = Phase::EndOfCombat;
        self.priority = if self.is_over() {
            None
        } else {
            Some(PriorityState::new(self.active_player.clone()))
        };

        Ok(outcome)
    }

    /// Resets all blocker states.
    pub fn reset_blockers(&mut self) {
        for player in &mut self.players {
            for card in player.battlefield_mut().iter_mut() {
                card.set_blocking(false);
            }
        }
    }
}
