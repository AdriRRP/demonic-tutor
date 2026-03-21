use crate::domain::play::{
    cards::{CardInstance, SpellTargetingProfile, SupportedSpellRules},
    game::SpellTarget,
};

#[must_use]
pub const fn supported_spell_rules(card: &CardInstance) -> SupportedSpellRules {
    card.supported_spell_rules()
}

#[must_use]
pub const fn accepts_target(targeting: SpellTargetingProfile, target: &SpellTarget) -> bool {
    targeting.permits(target.kind())
}
