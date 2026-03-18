use crate::domain::play::{
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

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
