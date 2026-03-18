use crate::domain::play::{cards::CardInstance, game::SpellTarget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellEffect {
    None,
    DealDamageToAnyTarget { damage: u32 },
}

impl SpellEffect {
    #[must_use]
    pub const fn requires_target(&self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    pub const fn accepts_target(&self, _target: &SpellTarget) -> bool {
        matches!(self, Self::DealDamageToAnyTarget { .. })
    }
}

#[must_use]
pub fn spell_effect(card: &CardInstance) -> SpellEffect {
    match card.definition_id().as_str() {
        "shock" | "bdd-shock" | "player-shock" | "creature-shock" => {
            SpellEffect::DealDamageToAnyTarget { damage: 2 }
        }
        _ => SpellEffect::None,
    }
}
