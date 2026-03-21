use super::{CardType, CastingPermissionProfile, CastingRule, SupportedSpellRules};
use crate::domain::play::ids::CardDefinitionId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    id: CardDefinitionId,
    mana_cost: u32,
    casting_permission: Option<CastingPermissionProfile>,
    supported_spell_rules: SupportedSpellRules,
}

impl CardDefinition {
    #[must_use]
    pub const fn for_card_type(id: CardDefinitionId, mana_cost: u32, card_type: &CardType) -> Self {
        Self {
            id,
            mana_cost,
            casting_permission: CastingPermissionProfile::for_spell_card_type(card_type),
            supported_spell_rules: SupportedSpellRules::none(),
        }
    }

    #[must_use]
    pub const fn with_supported_spell_rules(
        mut self,
        supported_spell_rules: SupportedSpellRules,
    ) -> Self {
        self.supported_spell_rules = supported_spell_rules;
        self
    }

    #[must_use]
    pub const fn with_casting_rule(mut self, casting_rule: CastingRule) -> Self {
        if let Some(permission) = self.casting_permission {
            self.casting_permission = Some(permission.with_rule(casting_rule));
        }
        self
    }

    #[must_use]
    pub const fn id(&self) -> &CardDefinitionId {
        &self.id
    }

    #[must_use]
    pub const fn mana_cost(&self) -> u32 {
        self.mana_cost
    }

    #[must_use]
    pub const fn casting_permission(&self) -> Option<CastingPermissionProfile> {
        self.casting_permission
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        self.supported_spell_rules
    }
}
