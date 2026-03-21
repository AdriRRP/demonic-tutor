use crate::domain::play::{
    cards::{CardInstance, SpellEffectProfile},
    game::SpellTarget,
};

#[must_use]
pub fn spell_effect(card: &CardInstance) -> SpellEffectProfile {
    card.spell_effect_profile().clone()
}

#[must_use]
pub const fn accepts_target(effect: &SpellEffectProfile, _target: &SpellTarget) -> bool {
    matches!(effect, SpellEffectProfile::DealDamageToAnyTarget { .. })
}
