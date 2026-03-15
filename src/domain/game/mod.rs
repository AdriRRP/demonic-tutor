pub mod combat;
pub mod creatures;
pub mod draw;
pub mod lands;
pub mod life;
pub mod mana;
pub mod mulligan;
pub mod opening_hands;
pub mod spells;
pub mod start_game;
pub mod turns;

use crate::domain::{
    commands::{
        AdvanceTurnCommand, CastSpellCommand, DealOpeningHandsCommand, DeclareAttackersCommand,
        DeclareBlockersCommand, DrawCardCommand, MulliganCommand, PlayCreatureCommand,
        PlayLandCommand, StartGameCommand, TapLandCommand,
    },
    errors::{DomainError, GameError, PhaseError},
    events::{
        AttackersDeclared, BlockersDeclared, CardDrawn, CreatureEnteredBattlefield, GameStarted,
        LandPlayed, LandTapped, LifeChanged, ManaAdded, MulliganTaken, OpeningHandDealt,
        PhaseChanged, SpellCast, TurnAdvanced, TurnNumberChanged,
    },
    ids::{DeckId, GameId, PlayerId},
    zones::{Battlefield, Hand, Library},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Setup,
    Main,
    Ending,
}

mod player {
    use super::{Battlefield, DeckId, Hand, Library, PlayerId};

    const DEFAULT_STARTING_LIFE: u32 = 20;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Player {
        id: PlayerId,
        deck_id: DeckId,
        library: Library,
        hand: Hand,
        battlefield: Battlefield,
        life: u32,
        mana: u32,
        lands_played_this_turn: usize,
        mulligan_used: bool,
    }

    impl Player {
        pub const fn new(id: PlayerId, deck_id: DeckId) -> Self {
            Self {
                id,
                deck_id,
                library: Library::new(Vec::new()),
                hand: Hand::new(),
                battlefield: Battlefield::new(),
                life: DEFAULT_STARTING_LIFE,
                mana: 0,
                lands_played_this_turn: 0,
                mulligan_used: false,
            }
        }

        #[must_use]
        pub const fn id(&self) -> &PlayerId {
            &self.id
        }

        #[must_use]
        pub const fn deck_id(&self) -> &DeckId {
            &self.deck_id
        }

        #[must_use]
        pub const fn hand(&self) -> &Hand {
            &self.hand
        }

        #[must_use]
        pub const fn library(&self) -> &Library {
            &self.library
        }

        #[must_use]
        pub const fn battlefield(&self) -> &Battlefield {
            &self.battlefield
        }

        #[must_use]
        pub const fn life(&self) -> u32 {
            self.life
        }

        pub const fn life_mut(&mut self) -> &mut u32 {
            &mut self.life
        }

        #[must_use]
        pub const fn mana(&self) -> u32 {
            self.mana
        }

        pub const fn mana_mut(&mut self) -> &mut u32 {
            &mut self.mana
        }

        #[must_use]
        pub const fn lands_played_this_turn(&self) -> usize {
            self.lands_played_this_turn
        }

        pub const fn library_mut(&mut self) -> &mut Library {
            &mut self.library
        }

        pub const fn hand_mut(&mut self) -> &mut Hand {
            &mut self.hand
        }

        pub const fn battlefield_mut(&mut self) -> &mut Battlefield {
            &mut self.battlefield
        }

        pub const fn lands_played_this_turn_mut(&mut self) -> &mut usize {
            &mut self.lands_played_this_turn
        }

        #[must_use]
        pub const fn mulligan_used(&self) -> bool {
            self.mulligan_used
        }

        pub const fn mulligan_used_mut(&mut self) -> &mut bool {
            &mut self.mulligan_used
        }
    }
}

use player::Player;

#[derive(Debug)]
pub struct Game {
    id: GameId,
    active_player: PlayerId,
    phase: Phase,
    turn_number: u32,
    players: Vec<Player>,
}

impl Game {
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

    pub fn id_from_player_id(player_id: &PlayerId) -> GameId {
        GameId(format!("game-from-{}", player_id.0))
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

    pub fn start(cmd: StartGameCommand) -> Result<(Self, GameStarted), DomainError> {
        start_game::start(cmd)
    }

    pub fn deal_opening_hands(
        &mut self,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        opening_hands::deal_opening_hands(&mut self.players, cmd, &self.id)
    }

    pub fn mulligan(&mut self, cmd: MulliganCommand) -> Result<MulliganTaken, DomainError> {
        mulligan::mulligan(&mut self.players, &self.active_player, &self.phase, cmd)
    }

    pub fn play_land(&mut self, cmd: PlayLandCommand) -> Result<LandPlayed, DomainError> {
        lands::play_land(&mut self.players, &self.active_player, &self.phase, cmd)
    }

    pub fn advance_turn(
        &mut self,
        cmd: AdvanceTurnCommand,
    ) -> Result<(TurnAdvanced, TurnNumberChanged, PhaseChanged), DomainError> {
        turns::advance_turn(
            &mut self.players,
            &mut self.active_player,
            &mut self.phase,
            &mut self.turn_number,
            cmd,
        )
    }

    pub fn draw_card(&mut self, cmd: DrawCardCommand) -> Result<CardDrawn, DomainError> {
        draw::draw_card(&mut self.players, &self.active_player, &self.phase, cmd)
    }

    pub fn set_life(
        &mut self,
        cmd: crate::domain::commands::SetLifeCommand,
    ) -> Result<LifeChanged, DomainError> {
        life::set_life(&mut self.players, cmd)
    }

    pub fn tap_land(
        &mut self,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        mana::tap_land(&mut self.players, cmd)
    }

    pub fn cast_spell(&mut self, cmd: CastSpellCommand) -> Result<SpellCast, DomainError> {
        spells::cast_spell(&mut self.players, &self.active_player, &self.phase, cmd)
    }

    pub fn play_creature(
        &mut self,
        cmd: PlayCreatureCommand,
    ) -> Result<CreatureEnteredBattlefield, DomainError> {
        creatures::play_creature(&mut self.players, &self.active_player, &self.phase, cmd)
    }

    pub fn declare_attackers(
        &mut self,
        cmd: DeclareAttackersCommand,
    ) -> Result<AttackersDeclared, DomainError> {
        combat::declare_attackers(&mut self.players, &self.active_player, &self.phase, cmd)
    }

    pub fn declare_blockers(
        &mut self,
        cmd: DeclareBlockersCommand,
    ) -> Result<BlockersDeclared, DomainError> {
        combat::declare_blockers(&mut self.players, &self.active_player, &self.phase, cmd)
    }
}
