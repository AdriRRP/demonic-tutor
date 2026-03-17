mod invariants;
pub mod model;
pub mod rules;

use crate::domain::play::{
    commands::{
        AdjustLifeCommand, AdvanceTurnCommand, CastSpellCommand, DealOpeningHandsCommand,
        DeclareAttackersCommand, DeclareBlockersCommand, DrawCardCommand, MulliganCommand,
        PlayLandCommand, ResolveCombatDamageCommand, StartGameCommand, TapLandCommand,
    },
    errors::{DomainError, GameError},
    events::{
        AttackersDeclared, BlockersDeclared, CardDrawn, CombatDamageResolved, GameStarted,
        LandPlayed, LandTapped, LifeChanged, ManaAdded, MulliganTaken, OpeningHandDealt, SpellCast,
        TurnProgressed,
    },
    ids::{GameId, PlayerId},
    phase::Phase,
};

pub use model::Player;

#[derive(Debug)]
pub struct Game {
    id: GameId,
    active_player: PlayerId,
    phase: Phase,
    turn_number: u32,
    players: Vec<Player>,
}

impl Game {
    #[must_use]
    pub const fn new(
        id: GameId,
        active_player: PlayerId,
        phase: Phase,
        turn_number: u32,
        players: Vec<Player>,
    ) -> Self {
        Self {
            id,
            active_player,
            phase,
            turn_number,
            players,
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

    /// Gets a mutable reference to a player by their ID.
    ///
    /// # Errors
    /// Returns `DomainError::Game(GameError::PlayerNotFound)` if no player with the given ID exists.
    pub fn get_player_mut(&mut self, player_id: &PlayerId) -> Result<&mut Player, DomainError> {
        self.players
            .iter_mut()
            .find(|p| p.id() == player_id)
            .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(player_id.clone())))
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
    ) -> Result<(TurnProgressed, Option<CardDrawn>), DomainError> {
        rules::turn_flow::advance_turn(
            &self.id,
            &mut self.players,
            &mut self.active_player,
            &mut self.phase,
            &mut self.turn_number,
            cmd,
        )
    }

    /// Draws a card from library to hand.
    ///
    /// # Errors
    /// See [`rules::turn_flow::draw_card`].
    pub fn draw_card(&mut self, cmd: DrawCardCommand) -> Result<CardDrawn, DomainError> {
        rules::turn_flow::draw_card(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

    /// Adjusts a player's life total by a signed delta.
    ///
    /// # Errors
    /// See [`rules::resource_actions::adjust_life`].
    pub fn adjust_life(&mut self, cmd: AdjustLifeCommand) -> Result<LifeChanged, DomainError> {
        rules::resource_actions::adjust_life(&self.id, &mut self.players, cmd)
    }

    /// Taps a land to produce mana.
    ///
    /// # Errors
    /// See [`rules::resource_actions::tap_land`].
    pub fn tap_land(
        &mut self,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        rules::resource_actions::tap_land(&self.id, &mut self.players, cmd)
    }

    /// Casts a spell.
    ///
    /// # Errors
    /// See [`rules::resource_actions::cast_spell`].
    pub fn cast_spell(&mut self, cmd: CastSpellCommand) -> Result<SpellCast, DomainError> {
        rules::resource_actions::cast_spell(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
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
        rules::combat::declare_attackers(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

    /// Declares blockers in combat.
    ///
    /// # Errors
    /// See [`rules::combat::declare_blockers`].
    pub fn declare_blockers(
        &mut self,
        cmd: DeclareBlockersCommand,
    ) -> Result<BlockersDeclared, DomainError> {
        rules::combat::declare_blockers(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

    /// Resolves combat damage.
    ///
    /// # Errors
    /// See [`rules::combat::resolve_combat_damage`].
    pub fn resolve_combat_damage(
        &mut self,
        cmd: ResolveCombatDamageCommand,
    ) -> Result<CombatDamageResolved, DomainError> {
        rules::combat::resolve_combat_damage(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }
}
