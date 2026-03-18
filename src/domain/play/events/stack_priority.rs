use crate::domain::play::{
    cards::CardType,
    game::SpellTarget,
    ids::{CardInstanceId, GameId, PlayerId, StackObjectId},
};

#[derive(Debug, Clone)]
pub struct SpellPutOnStack {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub card_type: CardType,
    pub mana_cost_paid: u32,
    pub stack_object_id: StackObjectId,
    pub target: Option<SpellTarget>,
}

impl SpellPutOnStack {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        card_id: CardInstanceId,
        card_type: CardType,
        mana_cost_paid: u32,
        stack_object_id: StackObjectId,
        target: Option<SpellTarget>,
    ) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
            card_type,
            mana_cost_paid,
            stack_object_id,
            target,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PriorityPassed {
    pub game_id: GameId,
    pub player_id: PlayerId,
}

impl PriorityPassed {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId) -> Self {
        Self { game_id, player_id }
    }
}

#[derive(Debug, Clone)]
pub struct StackTopResolved {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub stack_object_id: StackObjectId,
    pub source_card_id: CardInstanceId,
}

impl StackTopResolved {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        stack_object_id: StackObjectId,
        source_card_id: CardInstanceId,
    ) -> Self {
        Self {
            game_id,
            player_id,
            stack_object_id,
            source_card_id,
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
