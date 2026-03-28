//! Supports player-life and direct-damage spell-resolution effects.

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
    game::{helpers, rules::game_effects, SpellTarget},
    ids::CardInstanceId,
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
            zone_changes: Vec::new(),
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
            zone_changes: Vec::new(),
            life_changed,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}
