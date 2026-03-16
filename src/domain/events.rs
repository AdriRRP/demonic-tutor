use crate::domain::ids::{CardInstanceId, GameId, PlayerId};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    GameStarted(GameStarted),
    OpeningHandDealt(OpeningHandDealt),
    LandPlayed(LandPlayed),
    TurnAdvanced(TurnAdvanced),
    CardDrawn(CardDrawn),
    MulliganTaken(MulliganTaken),
    LifeChanged(LifeChanged),
    TurnNumberChanged(TurnNumberChanged),
    PhaseChanged(PhaseChanged),
    LandTapped(LandTapped),
    ManaAdded(ManaAdded),
    SpellCast(SpellCast),
    CreatureEnteredBattlefield(CreatureEnteredBattlefield),
    AttackersDeclared(AttackersDeclared),
    BlockersDeclared(BlockersDeclared),
    CombatDamageResolved(CombatDamageResolved),
}

#[derive(Debug, Clone)]
pub struct GameStarted {
    pub game_id: GameId,
    pub players: Vec<PlayerId>,
}

impl GameStarted {
    #[must_use]
    pub const fn new(game_id: GameId, players: Vec<PlayerId>) -> Self {
        Self { game_id, players }
    }
}

#[derive(Debug, Clone)]
pub struct OpeningHandDealt {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub cards: Vec<CardInstanceId>,
}

impl OpeningHandDealt {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, cards: Vec<CardInstanceId>) -> Self {
        Self {
            game_id,
            player_id,
            cards,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LandPlayed {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl LandPlayed {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TurnAdvanced {
    pub game_id: GameId,
    pub new_active_player: PlayerId,
}

impl TurnAdvanced {
    #[must_use]
    pub const fn new(game_id: GameId, new_active_player: PlayerId) -> Self {
        Self {
            game_id,
            new_active_player,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardDrawn {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl CardDrawn {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MulliganTaken {
    pub game_id: GameId,
    pub player_id: PlayerId,
}

impl MulliganTaken {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId) -> Self {
        Self { game_id, player_id }
    }
}

#[derive(Debug, Clone)]
pub struct LifeChanged {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub from_life: u32,
    pub to_life: u32,
}

impl LifeChanged {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, from_life: u32, to_life: u32) -> Self {
        Self {
            game_id,
            player_id,
            from_life,
            to_life,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TurnNumberChanged {
    pub game_id: GameId,
    pub from_turn: u32,
    pub to_turn: u32,
}

impl TurnNumberChanged {
    #[must_use]
    pub const fn new(game_id: GameId, from_turn: u32, to_turn: u32) -> Self {
        Self {
            game_id,
            from_turn,
            to_turn,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhaseChanged {
    pub game_id: GameId,
    pub from_phase: crate::domain::game::Phase,
    pub to_phase: crate::domain::game::Phase,
}

impl PhaseChanged {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        from_phase: crate::domain::game::Phase,
        to_phase: crate::domain::game::Phase,
    ) -> Self {
        Self {
            game_id,
            from_phase,
            to_phase,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LandTapped {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl LandTapped {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManaAdded {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub amount: u32,
    pub new_mana_total: u32,
}

impl ManaAdded {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        amount: u32,
        new_mana_total: u32,
    ) -> Self {
        Self {
            game_id,
            player_id,
            amount,
            new_mana_total,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpellCast {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl SpellCast {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreatureEnteredBattlefield {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub power: u32,
    pub toughness: u32,
}

impl CreatureEnteredBattlefield {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        card_id: CardInstanceId,
        power: u32,
        toughness: u32,
    ) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
            power,
            toughness,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttackersDeclared {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub attackers: Vec<CardInstanceId>,
}

impl AttackersDeclared {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, attackers: Vec<CardInstanceId>) -> Self {
        Self {
            game_id,
            player_id,
            attackers,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockersDeclared {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub assignments: Vec<(CardInstanceId, CardInstanceId)>,
}

impl BlockersDeclared {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        assignments: Vec<(CardInstanceId, CardInstanceId)>,
    ) -> Self {
        Self {
            game_id,
            player_id,
            assignments,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CombatDamageResolved {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub damage_events: Vec<DamageEvent>,
}

#[derive(Debug, Clone)]
pub struct DamageEvent {
    pub source: CardInstanceId,
    pub target: DamageTarget,
    pub damage_amount: u32,
}

#[derive(Debug, Clone)]
pub enum DamageTarget {
    Creature(CardInstanceId),
    Player(PlayerId),
}

impl CombatDamageResolved {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        damage_events: Vec<DamageEvent>,
    ) -> Self {
        Self {
            game_id,
            player_id,
            damage_events,
        }
    }
}

impl From<GameStarted> for DomainEvent {
    fn from(event: GameStarted) -> Self {
        Self::GameStarted(event)
    }
}

impl From<OpeningHandDealt> for DomainEvent {
    fn from(event: OpeningHandDealt) -> Self {
        Self::OpeningHandDealt(event)
    }
}

impl From<LandPlayed> for DomainEvent {
    fn from(event: LandPlayed) -> Self {
        Self::LandPlayed(event)
    }
}

impl From<TurnAdvanced> for DomainEvent {
    fn from(event: TurnAdvanced) -> Self {
        Self::TurnAdvanced(event)
    }
}

impl From<CardDrawn> for DomainEvent {
    fn from(event: CardDrawn) -> Self {
        Self::CardDrawn(event)
    }
}

impl From<MulliganTaken> for DomainEvent {
    fn from(event: MulliganTaken) -> Self {
        Self::MulliganTaken(event)
    }
}

impl From<LifeChanged> for DomainEvent {
    fn from(event: LifeChanged) -> Self {
        Self::LifeChanged(event)
    }
}

impl From<TurnNumberChanged> for DomainEvent {
    fn from(event: TurnNumberChanged) -> Self {
        Self::TurnNumberChanged(event)
    }
}

impl From<PhaseChanged> for DomainEvent {
    fn from(event: PhaseChanged) -> Self {
        Self::PhaseChanged(event)
    }
}

impl From<LandTapped> for DomainEvent {
    fn from(event: LandTapped) -> Self {
        Self::LandTapped(event)
    }
}

impl From<ManaAdded> for DomainEvent {
    fn from(event: ManaAdded) -> Self {
        Self::ManaAdded(event)
    }
}

impl From<SpellCast> for DomainEvent {
    fn from(event: SpellCast) -> Self {
        Self::SpellCast(event)
    }
}

impl From<CreatureEnteredBattlefield> for DomainEvent {
    fn from(event: CreatureEnteredBattlefield) -> Self {
        Self::CreatureEnteredBattlefield(event)
    }
}

impl From<AttackersDeclared> for DomainEvent {
    fn from(event: AttackersDeclared) -> Self {
        Self::AttackersDeclared(event)
    }
}

impl From<BlockersDeclared> for DomainEvent {
    fn from(event: BlockersDeclared) -> Self {
        Self::BlockersDeclared(event)
    }
}

impl From<CombatDamageResolved> for DomainEvent {
    fn from(event: CombatDamageResolved) -> Self {
        Self::CombatDamageResolved(event)
    }
}
