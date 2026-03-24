//! Supports stack priority resolution events.

use crate::domain::play::{
    cards::CardType,
    events::{SpellCast, SpellCastOutcome, StackTopResolved},
    ids::{CardInstanceId, GameId, PlayerId, StackObjectId},
};

pub(super) fn build_resolution_events(
    game_id: &GameId,
    controller_id: &PlayerId,
    stack_object_number: u32,
    source_card_id: &CardInstanceId,
    card_type: CardType,
    mana_cost_paid: u32,
    outcome: SpellCastOutcome,
) -> (StackTopResolved, SpellCast) {
    let stack_object_id = StackObjectId::for_stack_object(game_id, stack_object_number);
    let spell_cast = SpellCast::new(
        game_id.clone(),
        controller_id.clone(),
        source_card_id.clone(),
        card_type,
        mana_cost_paid,
        outcome,
    );
    let stack_top_resolved = StackTopResolved::new(
        game_id.clone(),
        controller_id.clone(),
        stack_object_id,
        source_card_id.clone(),
    );

    (stack_top_resolved, spell_cast)
}
