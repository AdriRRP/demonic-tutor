//! Supports spell-resolution effects that modify battlefield stats or counters.

use super::shared::{
    resolve_target_legality_for_effect, review_state_based_actions,
    review_state_based_actions_after_effect, EffectOutcomeSeed, ResolutionContext,
    SpellResolutionSideEffects,
};
use crate::domain::play::{
    cards::SpellTargetingProfile,
    errors::DomainError,
    game::{
        helpers,
        rules::stack_priority::spell_effects::{
            evaluate_target_legality, SpellTargetLegality, TargetLegalityContext,
        },
        SpellTarget,
    },
    ids::CardInstanceId,
};

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

fn tap_target_creature(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) {
    if let Some(card) = helpers::battlefield_card_mut(players, card_locations, target_id) {
        card.tap();
    }
}

fn untap_target_creature(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) {
    if let Some(card) = helpers::battlefield_card_mut(players, card_locations, target_id) {
        card.untap();
    }
}

fn prevent_target_creature_from_blocking_this_turn(
    players: &mut [crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) {
    if let Some(card) = helpers::battlefield_card_mut(players, card_locations, target_id) {
        card.add_temporary_cant_block();
    }
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
            card_discarded: None,
            zone_changes: Vec::new(),
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
            card_discarded: None,
            zone_changes: Vec::new(),
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

fn legal_secondary_creature_target(context: &ResolutionContext<'_>) -> Option<CardInstanceId> {
    let Some(crate::domain::play::game::model::StackSpellChoice::SecondaryCreatureTarget(
        target_ref,
    )) = context.choice
    else {
        return None;
    };

    let target_ref = target_ref?;

    let card = context.players[target_ref.player_index()].card_by_handle(target_ref.handle())?;
    let target_id = card.id().clone();

    let legality = evaluate_target_legality(
        TargetLegalityContext::Resolution {
            players: context.players,
            card_locations: context.card_locations,
            stack: context.stack,
            actor_index: context.controller_index,
        },
        SpellTargetingProfile::ExactlyOne(
            crate::domain::play::cards::SingleTargetRule::any_creature_on_battlefield(),
        ),
        Some(&SpellTarget::Creature(target_id.clone())),
    );

    match legality {
        SpellTargetLegality::Legal => Some(target_id),
        SpellTargetLegality::IllegalTargetKind
        | SpellTargetLegality::IllegalTargetRule
        | SpellTargetLegality::MissingCreature(_)
        | SpellTargetLegality::MissingPermanent(_)
        | SpellTargetLegality::MissingPlayer(_)
        | SpellTargetLegality::MissingGraveyardCard(_)
        | SpellTargetLegality::MissingRequiredTarget
        | SpellTargetLegality::MissingStackSpell(_)
        | SpellTargetLegality::NoTargetRequired => None,
    }
}

pub(super) fn resolve_distribute_two_counters_among_up_to_two_target_creatures_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let second_target_was_selected = matches!(
        context.choice,
        Some(crate::domain::play::game::model::StackSpellChoice::SecondaryCreatureTarget(Some(_),))
    );
    let primary_target = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "distributed counter spell resolved without a targeting profile",
    )?;
    let secondary_target = legal_secondary_creature_target(context);

    if let Some(SpellTarget::Creature(card_id)) = primary_target.as_ref() {
        let counters = if second_target_was_selected { 1 } else { 2 };
        if let Some(card) =
            helpers::battlefield_card_mut(context.players, context.card_locations, card_id)
        {
            card.add_plus_one_plus_one_counters(counters);
        }
    }

    if let Some(card_id) = secondary_target.as_ref() {
        if let Some(card) =
            helpers::battlefield_card_mut(context.players, context.card_locations, card_id)
        {
            card.add_plus_one_plus_one_counters(1);
        }
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_discarded: None,
            zone_changes: Vec::new(),
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_tap_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "tap-creature spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    if let SpellTarget::Creature(card_id) = target {
        tap_target_creature(context.players, context.card_locations, &card_id);
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_discarded: None,
            zone_changes: Vec::new(),
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_untap_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "untap-creature spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    if let SpellTarget::Creature(card_id) = target {
        untap_target_creature(context.players, context.card_locations, &card_id);
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_discarded: None,
            zone_changes: Vec::new(),
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

pub(super) fn resolve_cannot_block_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "cannot-block spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    if let SpellTarget::Creature(card_id) = target {
        prevent_target_creature_from_blocking_this_turn(
            context.players,
            context.card_locations,
            &card_id,
        );
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_discarded: None,
            zone_changes: Vec::new(),
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}
