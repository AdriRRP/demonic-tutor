//! Supports battlefield removal spell-resolution effects.

use super::shared::{
    resolve_target_legality_for_effect, review_state_based_actions,
    review_state_based_actions_after_effect, EffectOutcomeSeed, ResolutionContext,
    SpellResolutionSideEffects,
};
use crate::domain::play::{
    errors::DomainError,
    events::{CardMovedZone, CreatureDied, ZoneType},
    game::{helpers, rules::zones, SpellTarget},
    ids::{CardInstanceId, GameId},
};

fn destroy_creature(
    game_id: &GameId,
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<(CreatureDied, CardMovedZone)> {
    let target = helpers::battlefield_card_location(players, card_locations, target_id)?;
    if target.card().has_indestructible() {
        return None;
    }
    let handle = card_locations.location(target_id)?.handle();
    let zone_change = zones::move_battlefield_handle_to_owner_graveyard_by_index(
        game_id,
        players,
        card_locations,
        target.player_index(),
        handle,
    )
    .ok()?;
    Some((
        CreatureDied::new(
            game_id.clone(),
            zone_change.zone_owner_id.clone(),
            target_id.clone(),
        ),
        zone_change,
    ))
}

fn destroy_noncreature_permanent(
    game_id: &GameId,
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<(CardInstanceId, CardMovedZone)> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    let zone_change = zones::move_battlefield_handle_to_owner_graveyard_by_index(
        game_id,
        players,
        card_locations,
        location.player_index(),
        location.handle(),
    )
    .ok()?;
    Some((target_id.clone(), zone_change))
}

fn exile_creature_from_battlefield(
    game_id: &GameId,
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardMovedZone> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    let exiled = zones::exile_card_from_battlefield_handle_by_index(
        game_id,
        players,
        card_locations,
        location.player_index(),
        location.handle(),
    )
    .ok()?;
    Some(CardMovedZone::new(
        exiled.game_id,
        exiled.zone_owner_id,
        exiled.card_id,
        exiled.origin_zone,
        ZoneType::Exile,
    ))
}

pub(super) fn resolve_destroy_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "destroy spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let mut creatures_died = Vec::new();
    let mut zone_changes = Vec::new();
    if let SpellTarget::Creature(card_id) = target {
        if let Some((creature_died, zone_change)) = destroy_creature(
            context.game_id,
            context.players,
            context.card_locations,
            &card_id,
        ) {
            creatures_died.push(creature_died);
            zone_changes.push(zone_change);
        }
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_discarded: None,
            zone_changes,
            life_changed: None,
            creatures_died,
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_destroy_target_artifact_or_enchantment_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "artifact or enchantment destruction spell resolved without a targeting profile",
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
    match target {
        SpellTarget::Permanent(card_id) => {
            if let Some((moved_card, zone_change)) =
                destroy_noncreature_permanent(
                    context.game_id,
                    context.players,
                    context.card_locations,
                    &card_id,
                )
            {
                moved_cards.push(moved_card);
                zone_changes.push(zone_change);
            }
        }
        SpellTarget::Player(_)
        | SpellTarget::Creature(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => {}
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_discarded: None,
            zone_changes,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

pub(super) fn resolve_exile_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        &crate::domain::play::game::model::StackZone::empty(),
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "exile spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let zone_changes = match target {
        SpellTarget::Creature(card_id) => exile_creature_from_battlefield(
            context.game_id,
            context.players,
            context.card_locations,
            &card_id,
        )
        .into_iter()
        .collect(),
        SpellTarget::Player(_)
        | SpellTarget::Permanent(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => Vec::new(),
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_discarded: None,
            zone_changes,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}
