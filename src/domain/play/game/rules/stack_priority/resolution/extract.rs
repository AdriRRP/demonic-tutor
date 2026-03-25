//! Supports stack priority resolution extract.

use crate::domain::play::{
    cards::{ActivatedAbilityProfile, SpellPayload},
    errors::{DomainError, GameError},
    game::model::{StackCardRef, StackObject, StackObjectKind, StackTargetRef},
};

pub(super) struct ResolvedSpellObject {
    pub stack_object_number: u32,
    pub controller_index: usize,
    pub payload: SpellPayload,
    pub mana_cost_paid: u32,
    pub target: Option<StackTargetRef>,
}

pub(super) fn extract_resolved_spell_object(
    stack_object: StackObject,
) -> Result<ResolvedSpellObject, DomainError> {
    let stack_object_number = stack_object.number();
    let controller_index = stack_object.controller_index();
    let StackObjectKind::Spell(spell) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "spell extraction requires a spell stack object".to_string(),
        )));
    };
    let mana_cost_paid = spell.mana_cost_paid();
    let target = spell.target().copied();
    let payload = spell.into_payload();

    Ok(ResolvedSpellObject {
        stack_object_number,
        controller_index,
        payload,
        mana_cost_paid,
        target,
    })
}

pub(super) struct ResolvedActivatedAbility {
    pub stack_object_number: u32,
    pub source_card_ref: StackCardRef,
    pub controller_index: usize,
    pub ability: ActivatedAbilityProfile,
}

pub(super) fn extract_resolved_activated_ability(
    stack_object: StackObject,
) -> Result<ResolvedActivatedAbility, DomainError> {
    let stack_object_number = stack_object.number();
    let controller_index = stack_object.controller_index();
    let StackObjectKind::ActivatedAbility(ability) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "activated-ability extraction requires an activated ability object".to_string(),
        )));
    };

    Ok(ResolvedActivatedAbility {
        stack_object_number,
        source_card_ref: ability.source_card_ref(),
        controller_index,
        ability: ability.ability(),
    })
}
