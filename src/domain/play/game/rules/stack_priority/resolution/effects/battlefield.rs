//! Supports battlefield and life-oriented spell-resolution effects.

use super::{
    require_modal_choice,
    shared::{
        resolve_target_legality_for_effect, review_state_based_actions,
        review_state_based_actions_after_effect, EffectOutcomeSeed, ResolutionContext,
        SpellResolutionSideEffects,
    },
};
use crate::domain::play::{
    errors::DomainError,
    events::CreatureDied,
    game::{
        helpers,
        rules::{game_effects, zones},
        SpellTarget,
    },
    ids::{CardInstanceId, GameId},
};

fn apply_damage_to_creature(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
    damage: u32,
) {
    if let Some(card) = helpers::battlefield_card_mut(players, card_locations, target_id) {
        card.add_damage(damage);
    }
}

fn apply_temporary_pump_to_creature(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
    power: u32,
    toughness: u32,
) {
    if let Some(card) = helpers::battlefield_card_mut(players, card_locations, target_id) {
        card.apply_temporary_stat_bonus(power, toughness);
    }
}

fn destroy_creature(
    game_id: &GameId,
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CreatureDied> {
    let target = helpers::battlefield_card_location(players, card_locations, target_id)?;
    if target.card().has_indestructible() {
        return None;
    }
    let handle = card_locations.location(target_id)?.handle();
    let (owner_id, _) = zones::move_battlefield_handle_to_owner_graveyard_by_index(
        players,
        Some(card_locations),
        target.player_index(),
        handle,
    )
    .ok()?;
    Some(CreatureDied::new(
        game_id.clone(),
        owner_id,
        target_id.clone(),
    ))
}

fn destroy_noncreature_permanent(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardInstanceId> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    zones::move_battlefield_handle_to_owner_graveyard_by_index(
        players,
        Some(card_locations),
        location.player_index(),
        location.handle(),
    )
    .ok()?;
    Some(target_id.clone())
}

fn exile_creature_from_battlefield(
    game_id: &GameId,
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<crate::domain::play::events::CardExiled> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    zones::exile_card_from_battlefield_handle_by_index(
        game_id,
        players,
        location.player_index(),
        location.handle(),
    )
    .ok()
}

pub(super) fn resolve_targeted_player_life_effect(
    context: &mut ResolutionContext<'_>,
    life_delta: i32,
    missing_profile_message: &str,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        missing_profile_message,
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let life_changed = match target {
        SpellTarget::Player(player_id) => {
            let player_index = helpers::find_player_index(context.players, &player_id)?;
            Some(game_effects::adjust_player_life_by_index(
                context.game_id,
                context.players,
                player_index,
                life_delta,
            )?)
        }
        SpellTarget::Creature(_)
        | SpellTarget::Permanent(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => None,
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_choose_one_target_player_life_effect(
    context: &mut ResolutionContext<'_>,
    gain_amount: u32,
    lose_amount: u32,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let mode = require_modal_choice(context.choice)?;
    let life_delta = match mode {
        crate::domain::play::commands::ModalSpellMode::TargetPlayerGainLife => {
            gain_amount.cast_signed()
        }
        crate::domain::play::commands::ModalSpellMode::TargetPlayerLoseLife => {
            -(lose_amount).cast_signed()
        }
    };

    resolve_targeted_player_life_effect(
        context,
        life_delta,
        "modal choose-one spell resolved without a player target",
    )
}

pub(super) fn resolve_damage_effect(
    context: &mut ResolutionContext<'_>,
    damage: u32,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "damage spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let life_changed = match target {
        SpellTarget::Player(player_id) => {
            let player_index = helpers::find_player_index(context.players, &player_id)?;
            Some(game_effects::adjust_player_life_by_index(
                context.game_id,
                context.players,
                player_index,
                -(damage).cast_signed(),
            )?)
        }
        SpellTarget::Creature(card_id) => {
            apply_damage_to_creature(context.players, context.card_locations, &card_id, damage);
            None
        }
        SpellTarget::Permanent(_) | SpellTarget::GraveyardCard(_) | SpellTarget::StackObject(_) => {
            None
        }
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_pump_target_creature_effect(
    context: &mut ResolutionContext<'_>,
    bonus: (u32, u32),
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "pump spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    if let SpellTarget::Creature(card_id) = target {
        apply_temporary_pump_to_creature(
            context.players,
            context.card_locations,
            &card_id,
            bonus.0,
            bonus.1,
        );
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_put_counter_on_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "counter-placement spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    if let SpellTarget::Creature(card_id) = target {
        if let Some(card) =
            helpers::battlefield_card_mut(context.players, context.card_locations, &card_id)
        {
            card.add_plus_one_plus_one_counters(1);
        }
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
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
    if let SpellTarget::Creature(card_id) = target {
        if let Some(creature_died) = destroy_creature(
            context.game_id,
            context.players,
            context.card_locations,
            &card_id,
        ) {
            creatures_died.push(creature_died);
        }
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
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

    let moved_cards = match target {
        SpellTarget::Permanent(card_id) => {
            destroy_noncreature_permanent(context.players, context.card_locations, &card_id)
                .into_iter()
                .collect()
        }
        SpellTarget::Player(_)
        | SpellTarget::Creature(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => Vec::new(),
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

    let card_exiled = match target {
        SpellTarget::Creature(card_id) => exile_creature_from_battlefield(
            context.game_id,
            context.players,
            context.card_locations,
            &card_id,
        ),
        SpellTarget::Player(_)
        | SpellTarget::Permanent(_)
        | SpellTarget::GraveyardCard(_)
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
