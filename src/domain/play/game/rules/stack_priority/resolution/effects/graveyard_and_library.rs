//! Supports graveyard and library oriented spell-resolution effects.

use super::shared::{
    resolve_target_legality_for_effect, review_state_based_actions,
    review_state_based_actions_after_effect, EffectOutcomeSeed, ResolutionContext,
    SpellResolutionSideEffects,
};
use crate::domain::play::{
    errors::DomainError,
    game::{rules::zones, SpellTarget},
    ids::{CardInstanceId, GameId},
};

fn return_creature_card_from_graveyard_to_hand(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardInstanceId> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Graveyard).then_some(())?;
    let card = players[location.player_index()].card_by_handle(location.handle())?;
    card.card_type().is_creature().then_some(())?;
    players[location.player_index()].move_graveyard_handle_to_hand(location.handle())?;
    Some(target_id.clone())
}

fn return_instant_or_sorcery_card_from_graveyard_to_hand(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardInstanceId> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Graveyard).then_some(())?;
    let card = players[location.player_index()].card_by_handle(location.handle())?;
    matches!(
        card.card_type(),
        crate::domain::play::cards::CardType::Instant
            | crate::domain::play::cards::CardType::Sorcery
    )
    .then_some(())?;
    players[location.player_index()].move_graveyard_handle_to_hand(location.handle())?;
    Some(target_id.clone())
}

fn reanimate_creature_card_to_battlefield(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    controller_index: usize,
    target_id: &CardInstanceId,
) -> Option<CardInstanceId> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Graveyard).then_some(())?;
    let card = players[location.player_index()].card_by_handle(location.handle())?;
    card.card_type().is_creature().then_some(())?;
    if location.player_index() == controller_index {
        players[location.player_index()].move_graveyard_handle_to_battlefield(location.handle())?;
        return Some(target_id.clone());
    }

    let moved_card = players[location.player_index()].remove_graveyard_card(target_id)?;
    players[controller_index].receive_battlefield_card(moved_card)?;
    Some(target_id.clone())
}

fn exile_card_from_graveyard(
    game_id: &GameId,
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<crate::domain::play::events::CardExiled> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Graveyard).then_some(())?;
    zones::exile_card_from_graveyard_handle_by_index(
        game_id,
        players,
        location.player_index(),
        location.handle(),
    )
    .ok()
}

pub(super) fn resolve_return_target_creature_from_graveyard_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "graveyard recursion spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let moved_cards = match target {
        SpellTarget::GraveyardCard(card_id) => return_creature_card_from_graveyard_to_hand(
            context.players,
            context.card_locations,
            &card_id,
        )
        .into_iter()
        .collect(),
        _ => Vec::new(),
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

pub(super) fn resolve_reanimate_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "reanimation spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let moved_cards = match target {
        SpellTarget::GraveyardCard(card_id) => reanimate_creature_card_to_battlefield(
            context.players,
            context.card_locations,
            context.controller_index,
            &card_id,
        )
        .into_iter()
        .collect(),
        _ => Vec::new(),
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

pub(super) fn resolve_return_target_instant_or_sorcery_from_graveyard_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "graveyard spell recursion resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let moved_cards = match target {
        SpellTarget::GraveyardCard(card_id) => {
            return_instant_or_sorcery_card_from_graveyard_to_hand(
                context.players,
                context.card_locations,
                &card_id,
            )
            .into_iter()
            .collect()
        }
        _ => Vec::new(),
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

pub(super) fn resolve_exile_target_graveyard_card_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        &crate::domain::play::game::model::StackZone::empty(),
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "graveyard exile spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let card_exiled = match target {
        SpellTarget::GraveyardCard(card_id) => exile_card_from_graveyard(
            context.game_id,
            context.players,
            context.card_locations,
            &card_id,
        ),
        SpellTarget::Player(_)
        | SpellTarget::Creature(_)
        | SpellTarget::Permanent(_)
        | SpellTarget::StackObject(_) => None,
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_mill_effect(
    context: &mut ResolutionContext<'_>,
    amount: u32,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let target_player_index = match context.target {
        Some(crate::domain::play::game::SpellTarget::Player(player_id)) => {
            crate::domain::play::game::helpers::find_player_index(context.players, player_id)?
        }
        None | Some(_) => context.controller_index,
    };
    let moved_cards = context.players[target_player_index]
        .mill_cards_to_graveyard(amount as usize)
        .unwrap_or_default();

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}
