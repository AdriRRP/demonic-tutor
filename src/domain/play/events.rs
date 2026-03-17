use crate::domain::play::{
    cards::CardType,
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

macro_rules! impl_domain_event_from {
    ($event_type:ident, $variant:ident) => {
        impl From<$event_type> for DomainEvent {
            fn from(event: $event_type) -> Self {
                Self::$variant(event)
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum DomainEvent {
    GameStarted(GameStarted),
    OpeningHandDealt(OpeningHandDealt),
    GameEnded(GameEnded),
    LandPlayed(LandPlayed),
    TurnProgressed(TurnProgressed),
    CardDrawn(CardDrawn),
    CardDiscarded(CardDiscarded),
    MulliganTaken(MulliganTaken),
    LifeChanged(LifeChanged),
    LandTapped(LandTapped),
    ManaAdded(ManaAdded),
    SpellCast(SpellCast),
    AttackersDeclared(AttackersDeclared),
    BlockersDeclared(BlockersDeclared),
    CombatDamageResolved(CombatDamageResolved),
    CreatureDied(CreatureDied),
}

// Game lifecycle events

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEndReason {
    EmptyLibraryDraw,
    ZeroLife,
}

#[derive(Debug, Clone)]
pub struct GameEnded {
    pub game_id: GameId,
    pub winner_id: PlayerId,
    pub loser_id: PlayerId,
    pub reason: GameEndReason,
}

impl GameEnded {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        winner_id: PlayerId,
        loser_id: PlayerId,
        reason: GameEndReason,
    ) -> Self {
        Self {
            game_id,
            winner_id,
            loser_id,
            reason,
        }
    }
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

// Turn flow events

#[derive(Debug, Clone)]
pub struct TurnProgressed {
    pub game_id: GameId,
    pub active_player: PlayerId,
    pub from_turn: u32,
    pub to_turn: u32,
    pub from_phase: Phase,
    pub to_phase: Phase,
}

impl TurnProgressed {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        active_player: PlayerId,
        from_turn: u32,
        to_turn: u32,
        from_phase: Phase,
        to_phase: Phase,
    ) -> Self {
        Self {
            game_id,
            active_player,
            from_turn,
            to_turn,
            from_phase,
            to_phase,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawKind {
    TurnStep,
    ExplicitEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscardKind {
    CleanupHandSize,
}

#[derive(Debug, Clone)]
pub struct CardDrawn {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub draw_kind: DrawKind,
}

#[derive(Debug, Clone)]
pub struct CardDiscarded {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub discard_kind: DiscardKind,
}

impl CardDiscarded {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        card_id: CardInstanceId,
        discard_kind: DiscardKind,
    ) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
            discard_kind,
        }
    }
}

impl CardDrawn {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        card_id: CardInstanceId,
        draw_kind: DrawKind,
    ) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
            draw_kind,
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

// Resource and battlefield events

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
pub enum SpellCastOutcome {
    EnteredBattlefield,
    ResolvedToGraveyard,
}

#[derive(Debug, Clone)]
pub struct SpellCast {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub card_type: CardType,
    pub mana_cost_paid: u32,
    pub outcome: SpellCastOutcome,
}

impl SpellCast {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        card_id: CardInstanceId,
        card_type: CardType,
        mana_cost_paid: u32,
        outcome: SpellCastOutcome,
    ) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
            card_type,
            mana_cost_paid,
            outcome,
        }
    }
}

// Combat events

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
pub struct CreatureDied {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl CreatureDied {
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

impl_domain_event_from!(GameStarted, GameStarted);
impl_domain_event_from!(OpeningHandDealt, OpeningHandDealt);
impl_domain_event_from!(GameEnded, GameEnded);
impl_domain_event_from!(LandPlayed, LandPlayed);
impl_domain_event_from!(TurnProgressed, TurnProgressed);
impl_domain_event_from!(CardDrawn, CardDrawn);
impl_domain_event_from!(CardDiscarded, CardDiscarded);
impl_domain_event_from!(MulliganTaken, MulliganTaken);
impl_domain_event_from!(LifeChanged, LifeChanged);
impl_domain_event_from!(LandTapped, LandTapped);
impl_domain_event_from!(ManaAdded, ManaAdded);
impl_domain_event_from!(SpellCast, SpellCast);
impl_domain_event_from!(AttackersDeclared, AttackersDeclared);
impl_domain_event_from!(BlockersDeclared, BlockersDeclared);
impl_domain_event_from!(CombatDamageResolved, CombatDamageResolved);
impl_domain_event_from!(CreatureDied, CreatureDied);
