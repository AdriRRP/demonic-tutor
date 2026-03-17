pub mod combat;
pub mod creatures;
pub mod draw;
pub mod lands;
pub mod life;
pub mod mana;
pub mod mulligan;
pub mod opening_hands;
pub mod phase_behavior;
pub mod spells;
pub mod start_game;
pub mod turns;

use crate::{
    application::Command,
    domain::{
        commands::{
            AdvanceTurnCommand, CastSpellCommand, DealOpeningHandsCommand, DeclareAttackersCommand,
            DeclareBlockersCommand, DrawCardCommand, MulliganCommand, PlayCreatureCommand,
            PlayLandCommand, ResolveCombatDamageCommand, StartGameCommand, TapLandCommand,
        },
        errors::{DomainError, GameError, PhaseError},
        events::{
            AttackersDeclared, BlockersDeclared, CardDrawn, CombatDamageResolved,
            CreatureEnteredBattlefield, DomainEvent, GameStarted, LandPlayed, LandTapped,
            LifeChanged, ManaAdded, MulliganTaken, OpeningHandDealt, PhaseChanged, SpellCast,
            TurnAdvanced, TurnNumberChanged,
        },
        ids::{GameId, PlayerId},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Setup,
    Untap,
    Upkeep,
    Draw,
    FirstMain,
    Combat,
    SecondMain,
    EndStep,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Setup => write!(f, "Setup"),
            Self::Untap => write!(f, "Untap"),
            Self::Upkeep => write!(f, "Upkeep"),
            Self::Draw => write!(f, "Draw"),
            Self::FirstMain => write!(f, "FirstMain"),
            Self::Combat => write!(f, "Combat"),
            Self::SecondMain => write!(f, "SecondMain"),
            Self::EndStep => write!(f, "EndStep"),
        }
    }
}

mod player;

pub use player::Player;

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

    /// Executes a command using the `Command` pattern.
    ///
    /// # Errors
    /// Returns a `DomainError` if the command violates domain rules or invariants.
    pub fn execute_command<C: Command>(
        &mut self,
        command: &C,
    ) -> Result<Vec<DomainEvent>, DomainError> {
        command.execute(self)
    }

    /// Starts a new game.
    ///
    /// # Errors
    /// See [`start_game::start`].
    pub fn start(cmd: StartGameCommand) -> Result<(Self, GameStarted), DomainError> {
        start_game::start(cmd)
    }

    /// Deals opening hands to players.
    ///
    /// # Errors
    /// See [`opening_hands::deal_opening_hands`].
    pub fn deal_opening_hands(
        &mut self,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        opening_hands::deal_opening_hands(&mut self.players, cmd, &self.id)
    }

    /// Performs a mulligan.
    ///
    /// # Errors
    /// See [`mulligan::mulligan`].
    pub fn mulligan(&mut self, cmd: MulliganCommand) -> Result<MulliganTaken, DomainError> {
        mulligan::mulligan(
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
    /// See [`lands::play_land`].
    pub fn play_land(&mut self, cmd: PlayLandCommand) -> Result<LandPlayed, DomainError> {
        lands::play_land(
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
    /// See [`turns::advance_turn`].
    pub fn advance_turn(
        &mut self,
        cmd: AdvanceTurnCommand,
    ) -> Result<
        (
            TurnAdvanced,
            TurnNumberChanged,
            PhaseChanged,
            Option<CardDrawn>,
        ),
        DomainError,
    > {
        turns::advance_turn(
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
    /// See [`draw::draw_card`].
    pub fn draw_card(&mut self, cmd: DrawCardCommand) -> Result<CardDrawn, DomainError> {
        draw::draw_card(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

    /// Sets a player's life total.
    ///
    /// # Errors
    /// See [`life::set_life`].
    pub fn set_life(
        &mut self,
        cmd: crate::domain::commands::SetLifeCommand,
    ) -> Result<LifeChanged, DomainError> {
        life::set_life(&self.id, &mut self.players, cmd)
    }

    /// Taps a land to produce mana.
    ///
    /// # Errors
    /// See [`mana::tap_land`].
    pub fn tap_land(
        &mut self,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        mana::tap_land(&self.id, &mut self.players, cmd)
    }

    /// Casts a non-creature spell.
    ///
    /// # Errors
    /// See [`spells::cast_spell`].
    pub fn cast_spell(&mut self, cmd: CastSpellCommand) -> Result<SpellCast, DomainError> {
        spells::cast_spell(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }

    /// Plays a creature from hand to battlefield.
    ///
    /// # Errors
    /// See [`creatures::play_creature`].
    pub fn play_creature(
        &mut self,
        cmd: PlayCreatureCommand,
    ) -> Result<CreatureEnteredBattlefield, DomainError> {
        creatures::play_creature(
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
    /// See [`combat::declare_attackers`].
    pub fn declare_attackers(
        &mut self,
        cmd: DeclareAttackersCommand,
    ) -> Result<AttackersDeclared, DomainError> {
        combat::declare_attackers(
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
    /// See [`combat::declare_blockers`].
    pub fn declare_blockers(
        &mut self,
        cmd: DeclareBlockersCommand,
    ) -> Result<BlockersDeclared, DomainError> {
        combat::declare_blockers(
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
    /// See [`combat::resolve_combat_damage`].
    pub fn resolve_combat_damage(
        &mut self,
        cmd: ResolveCombatDamageCommand,
    ) -> Result<CombatDamageResolved, DomainError> {
        combat::resolve_combat_damage(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
    }
}
