//! Supports compact token-creation spell-resolution effects.

use super::shared::{
    review_state_based_actions_after_effect, EffectOutcomeSeed, ResolutionContext,
    SpellResolutionSideEffects,
};
use crate::domain::play::{
    cards::CardInstance,
    errors::{DomainError, GameError},
    ids::{CardDefinitionId, CardInstanceId},
};

pub(super) fn resolve_create_vanilla_creature_token_effect(
    context: &mut ResolutionContext<'_>,
    power: u32,
    toughness: u32,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let token_number = context.stack.next_object_number();
    let token_id = CardInstanceId::new(format!("{}-token-{}", context.game_id, token_number));
    let definition_id = CardDefinitionId::new(format!("token-{power}-{toughness}"));
    let token =
        CardInstance::new_vanilla_creature_token(token_id.clone(), definition_id, power, toughness);
    context.players[context.controller_index]
        .receive_battlefield_card(token)
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "failed to place created token onto the battlefield".to_string(),
            ))
        })?;

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: vec![token_id],
        },
    )
}
