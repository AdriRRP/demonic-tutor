//! Supports stack priority resolution effects.

mod graveyard_and_library;
mod life_and_damage;
mod misc;
mod modifiers;
mod removal;
pub(super) mod shared;
mod stack_and_hand;
#[cfg(test)]
mod tests;

use self::{
    graveyard_and_library::{
        resolve_exile_target_graveyard_card_effect, resolve_mill_effect,
        resolve_reanimate_target_creature_effect,
        resolve_return_target_creature_from_graveyard_effect,
        resolve_return_target_instant_or_sorcery_from_graveyard_effect,
    },
    life_and_damage::{
        resolve_choose_one_target_player_life_effect, resolve_damage_effect,
        resolve_targeted_player_life_effect,
    },
    misc::{
        resolve_create_keyworded_creature_token_effect,
        resolve_create_multiple_vanilla_creature_tokens_effect,
        resolve_create_vanilla_creature_token_effect,
    },
    modifiers::{
        resolve_cannot_block_target_creature_effect,
        resolve_distribute_two_counters_among_up_to_two_target_creatures_effect,
        resolve_pump_target_creature_effect, resolve_put_counter_on_target_creature_effect,
        resolve_tap_target_creature_effect, resolve_untap_target_creature_effect,
    },
    removal::{
        resolve_destroy_target_artifact_or_enchantment_effect,
        resolve_destroy_target_creature_effect, resolve_exile_target_creature_effect,
    },
    shared::{review_state_based_actions, SpellResolutionSideEffects},
    stack_and_hand::{
        resolve_counter_target_spell_effect, resolve_return_target_permanent_to_hand_effect,
        resolve_target_player_discards_chosen_card_effect,
    },
};
use crate::domain::play::{
    cards::SpellResolutionProfile,
    commands::ModalSpellMode,
    errors::{DomainError, GameError},
    game::model::StackSpellChoice,
};

pub(super) use self::shared::ResolutionContext;

pub(super) fn apply_supported_spell_rules(
    mut context: ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    match context.supported_spell_rules.resolution() {
        SpellResolutionProfile::None
        | SpellResolutionProfile::AttachToTargetCreature
        | SpellResolutionProfile::Scry { .. }
        | SpellResolutionProfile::Surveil { .. }
        | SpellResolutionProfile::LootDrawThenDiscard { .. }
        | SpellResolutionProfile::RummageDiscardThenDraw { .. } => {
            review_state_based_actions(context.game_id, context.players, context.terminal_state)
        }
        SpellResolutionProfile::DealDamage { damage } => {
            resolve_damage_effect(&mut context, damage)
        }
        SpellResolutionProfile::GainLife { amount } => resolve_targeted_player_life_effect(
            &mut context,
            amount.cast_signed(),
            "gain-life spell resolved without a targeting profile",
        ),
        SpellResolutionProfile::LoseLife { amount } => resolve_targeted_player_life_effect(
            &mut context,
            -(amount).cast_signed(),
            "lose-life spell resolved without a targeting profile",
        ),
        SpellResolutionProfile::ChooseOneTargetPlayerGainOrLoseLife {
            gain_amount,
            lose_amount,
        } => resolve_choose_one_target_player_life_effect(&mut context, gain_amount, lose_amount),
        SpellResolutionProfile::CreateVanillaCreatureToken { power, toughness } => {
            resolve_create_vanilla_creature_token_effect(&mut context, power, toughness)
        }
        SpellResolutionProfile::CreateMultipleVanillaCreatureTokens {
            count,
            power,
            toughness,
        } => resolve_create_multiple_vanilla_creature_tokens_effect(
            &mut context,
            count,
            power,
            toughness,
        ),
        SpellResolutionProfile::CreateKeywordedCreatureToken {
            power,
            toughness,
            keywords,
        } => {
            resolve_create_keyworded_creature_token_effect(&mut context, power, toughness, keywords)
        }
        SpellResolutionProfile::PutPlusOnePlusOneCounterOnTargetCreature => {
            resolve_put_counter_on_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::DistributeTwoPlusOnePlusOneCountersAmongUpToTwoTargetCreatures => {
            resolve_distribute_two_counters_among_up_to_two_target_creatures_effect(&mut context)
        }
        SpellResolutionProfile::TapTargetCreature => {
            resolve_tap_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::UntapTargetCreature => {
            resolve_untap_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::CannotBlockTargetCreatureThisTurn => {
            resolve_cannot_block_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::ReturnTargetCreatureCardFromGraveyardToHand => {
            resolve_return_target_creature_from_graveyard_effect(&mut context)
        }
        SpellResolutionProfile::ReturnTargetInstantOrSorceryCardFromGraveyardToHand => {
            resolve_return_target_instant_or_sorcery_from_graveyard_effect(&mut context)
        }
        SpellResolutionProfile::ReanimateTargetCreatureCard => {
            resolve_reanimate_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::MillCards { amount } => resolve_mill_effect(&mut context, amount),
        SpellResolutionProfile::CounterTargetSpell => {
            resolve_counter_target_spell_effect(&mut context)
        }
        SpellResolutionProfile::ReturnTargetPermanentToHand => {
            resolve_return_target_permanent_to_hand_effect(&mut context)
        }
        SpellResolutionProfile::DestroyTargetArtifactOrEnchantment => {
            resolve_destroy_target_artifact_or_enchantment_effect(&mut context)
        }
        SpellResolutionProfile::TargetPlayerDiscardsChosenCard => {
            resolve_target_player_discards_chosen_card_effect(&mut context)
        }
        SpellResolutionProfile::DestroyTargetCreature => {
            resolve_destroy_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::ExileTargetCreature => {
            resolve_exile_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::ExileTargetCardFromGraveyard => {
            resolve_exile_target_graveyard_card_effect(&mut context)
        }
        SpellResolutionProfile::PumpTargetCreatureUntilEndOfTurn { power, toughness } => {
            resolve_pump_target_creature_effect(&mut context, (power, toughness))
        }
    }
}

fn require_modal_choice(mode: Option<StackSpellChoice>) -> Result<ModalSpellMode, DomainError> {
    let Some(StackSpellChoice::ModalMode(mode)) = mode else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "modal choose-one spell resolved without a selected mode".to_string(),
        )));
    };
    Ok(mode)
}
