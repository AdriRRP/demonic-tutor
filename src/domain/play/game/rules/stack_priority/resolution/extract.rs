//! Supports stack priority resolution extract.

use crate::domain::play::{
    cards::{ActivatedAbilityProfile, CardInstance, CardType, SupportedSpellRules},
    errors::{DomainError, GameError},
    game::model::{StackObject, StackObjectKind},
    game::SpellTarget,
    ids::{CardInstanceId, GameId, PlayerId, StackObjectId},
};

pub(super) struct ResolvedSpellObject {
    pub source_card_id: CardInstanceId,
    pub controller_id: PlayerId,
    pub stack_object_id: StackObjectId,
    pub card: CardInstance,
    pub card_type: CardType,
    pub mana_cost_paid: u32,
    pub supported_spell_rules: SupportedSpellRules,
    pub target: Option<SpellTarget>,
}

pub(super) fn extract_resolved_spell_object(
    game_id: &GameId,
    stack_object: StackObject,
) -> Result<ResolvedSpellObject, DomainError> {
    let stack_object_id = stack_object.id(game_id);
    let controller_id = stack_object.controller_id().clone();
    let StackObjectKind::Spell(spell) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "spell extraction requires a spell stack object".to_string(),
        )));
    };
    let source_card_id = spell.source_card_id().clone();
    let card_type = *spell.card_type();
    let supported_spell_rules = spell.supported_spell_rules();
    let mana_cost_paid = spell.mana_cost_paid();
    let target = spell.target().cloned();
    let card = spell.into_payload().into_card_instance();

    Ok(ResolvedSpellObject {
        source_card_id,
        controller_id,
        stack_object_id,
        card,
        card_type,
        mana_cost_paid,
        supported_spell_rules,
        target,
    })
}

pub(super) struct ResolvedActivatedAbility {
    pub source_card_id: CardInstanceId,
    pub controller_id: PlayerId,
    pub stack_object_id: StackObjectId,
    pub ability: ActivatedAbilityProfile,
}

pub(super) fn extract_resolved_activated_ability(
    game_id: &GameId,
    stack_object: StackObject,
) -> Result<ResolvedActivatedAbility, DomainError> {
    let stack_object_id = stack_object.id(game_id);
    let controller_id = stack_object.controller_id().clone();
    let StackObjectKind::ActivatedAbility(ability) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "activated-ability extraction requires an activated ability object".to_string(),
        )));
    };

    Ok(ResolvedActivatedAbility {
        source_card_id: ability.source_card_id().clone(),
        controller_id,
        stack_object_id,
        ability: ability.ability(),
    })
}
