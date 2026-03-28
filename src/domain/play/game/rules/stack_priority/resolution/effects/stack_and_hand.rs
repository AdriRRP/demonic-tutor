//! Supports stack interaction, bounce, and hand-disruption spell-resolution effects.

use super::shared::{
    resolve_target_legality_for_effect, review_state_based_actions,
    review_state_based_actions_after_effect, EffectOutcomeSeed, ResolutionContext,
    SpellResolutionSideEffects,
};
use crate::domain::play::{
    errors::{DomainError, GameError},
    events::{CardDiscarded, CardMovedZone, DiscardKind, ZoneType},
    game::{helpers, model::StackObjectKind, rules::zones, SpellTarget},
    ids::{CardInstanceId, GameId},
};

pub(super) fn return_permanent_to_owners_hand(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardInstanceId> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    zones::move_battlefield_handle_to_owner_hand_by_index(
        players,
        card_locations,
        location.player_index(),
        location.handle(),
    )
    .ok()?;
    Some(target_id.clone())
}

fn discard_chosen_hand_card(
    game_id: &GameId,
    players: &mut [crate::domain::play::game::Player],
    target_player_index: usize,
    choice: crate::domain::play::game::model::StackSpellChoice,
) -> Option<(CardDiscarded, CardInstanceId)> {
    let crate::domain::play::game::model::StackSpellChoice::HandCard(card_ref) = choice else {
        return None;
    };
    (card_ref.player_index() == target_player_index).then_some(())?;
    let player = players.get_mut(target_player_index)?;
    player.move_hand_handle_to_graveyard(card_ref.handle())?;
    let card_id = player.card_by_handle(card_ref.handle())?.id().clone();
    let event = CardDiscarded::new(
        game_id.clone(),
        player.id().clone(),
        card_id.clone(),
        DiscardKind::SpellEffect,
    );
    Some((event, card_id))
}

pub(super) fn resolve_return_target_permanent_to_hand_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "bounce spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let moved_cards = match target {
        SpellTarget::Permanent(card_id) => {
            return_permanent_to_owners_hand(context.players, context.card_locations, &card_id)
                .into_iter()
                .collect()
        }
        SpellTarget::Player(_)
        | SpellTarget::Creature(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => Vec::new(),
    };
    let zone_changes = moved_cards
        .iter()
        .filter_map(|card_id| {
            let owner_index = context
                .players
                .iter()
                .position(|player| player.owns_card(card_id))?;
            Some(CardMovedZone::new(
                context.game_id.clone(),
                context.players[owner_index].id().clone(),
                card_id.clone(),
                ZoneType::Battlefield,
                ZoneType::Hand,
            ))
        })
        .collect();

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            zone_changes,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

pub(super) fn resolve_counter_target_spell_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "counter spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let mut moved_cards = Vec::new();
    let mut zone_changes = Vec::new();
    if let SpellTarget::StackObject(stack_object_id) = target {
        let object_number = stack_object_id.object_number().ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "counter target lost stack object number for {stack_object_id}"
            )))
        })?;

        if let Some(countered_object) = context.stack.remove_by_number(object_number) {
            let countered_controller_index = countered_object.controller_index();
            let StackObjectKind::Spell(countered_spell) = countered_object.into_kind() else {
                return Err(DomainError::Game(GameError::InternalInvariantViolation(
                    "counter target must remove a spell stack object".to_string(),
                )));
            };

            let payload = countered_spell.into_payload();
            let countered_card_id = payload.id().clone();
            let player = context
                .players
                .get_mut(countered_controller_index)
                .ok_or_else(|| {
                    DomainError::Game(GameError::InternalInvariantViolation(format!(
                        "missing countered spell controller at player index {countered_controller_index}"
                    )))
            })?;
            player.receive_graveyard_card(payload.into_card_instance());
            moved_cards.push(countered_card_id.clone());
            zone_changes.push(CardMovedZone::new(
                context.game_id.clone(),
                player.id().clone(),
                countered_card_id,
                ZoneType::Stack,
                ZoneType::Graveyard,
            ));
        }
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            zone_changes,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

pub(super) fn resolve_target_player_discards_chosen_card_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "discard spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let (card_discarded, moved_cards) = match (target, context.choice) {
        (SpellTarget::Player(player_id), Some(choice)) => {
            let target_player_index = helpers::find_player_index(context.players, &player_id)?;
            match discard_chosen_hand_card(
                context.game_id,
                context.players,
                target_player_index,
                choice,
            ) {
                Some((event, card_id)) => (Some(event), vec![card_id]),
                None => (None, Vec::new()),
            }
        }
        _ => (None, Vec::new()),
    };
    let zone_changes = card_discarded
        .as_ref()
        .map(|event| {
            vec![CardMovedZone::new(
                context.game_id.clone(),
                event.player_id.clone(),
                event.card_id.clone(),
                ZoneType::Hand,
                ZoneType::Graveyard,
            )]
        })
        .unwrap_or_default();

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded,
            zone_changes,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}
