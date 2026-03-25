//! Supports play cards definition.

use {
    super::{
        ActivatedAbilityProfile, ActivatedManaAbilityProfile, CardType, CastingPermissionProfile,
        CastingRule, ManaColor, ManaCost, SupportedSpellRules, TriggeredAbilityProfile,
    },
    crate::domain::play::ids::CardDefinitionId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDefinition {
    id: CardDefinitionId,
    card_type: CardType,
    mana_cost: ManaCost,
    casting_permission: Option<CastingPermissionProfile>,
    supported_spell_rules: SupportedSpellRules,
    activated_mana_ability: Option<ActivatedManaAbilityProfile>,
    activated_ability: Option<ActivatedAbilityProfile>,
    triggered_ability: Option<TriggeredAbilityProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDefinitionParts {
    pub id: CardDefinitionId,
    pub card_type: CardType,
    pub mana_cost: ManaCost,
    pub casting_permission: Option<CastingPermissionProfile>,
    pub supported_spell_rules: SupportedSpellRules,
    pub activated_mana_ability: Option<ActivatedManaAbilityProfile>,
    pub activated_ability: Option<ActivatedAbilityProfile>,
    pub triggered_ability: Option<TriggeredAbilityProfile>,
}

impl CardDefinition {
    #[must_use]
    pub const fn for_card_type(id: CardDefinitionId, mana_cost: u32, card_type: &CardType) -> Self {
        Self {
            id,
            card_type: *card_type,
            mana_cost: ManaCost::generic(mana_cost),
            casting_permission: CastingPermissionProfile::for_spell_card_type(card_type),
            supported_spell_rules: SupportedSpellRules::none(),
            activated_mana_ability: match card_type {
                CardType::Land => Some(ActivatedManaAbilityProfile::tap_for_generic_mana(1)),
                CardType::Creature
                | CardType::Instant
                | CardType::Sorcery
                | CardType::Enchantment
                | CardType::Artifact
                | CardType::Planeswalker => None,
            },
            activated_ability: None,
            triggered_ability: None,
        }
    }

    #[must_use]
    pub const fn land(id: CardDefinitionId, produced_mana: ManaColor) -> Self {
        Self {
            id,
            card_type: CardType::Land,
            mana_cost: ManaCost::generic(0),
            casting_permission: None,
            supported_spell_rules: SupportedSpellRules::none(),
            activated_mana_ability: Some(ActivatedManaAbilityProfile::tap_for_colored_mana(
                produced_mana,
                1,
            )),
            activated_ability: None,
            triggered_ability: None,
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
    pub const fn with_mana_cost(mut self, mana_cost: ManaCost) -> Self {
        self.mana_cost = mana_cost;
        self
    }

    #[must_use]
    pub const fn with_activated_ability(
        mut self,
        activated_ability: ActivatedAbilityProfile,
    ) -> Self {
        self.activated_ability = Some(activated_ability);
        self
    }

    #[must_use]
    pub const fn with_triggered_ability(
        mut self,
        triggered_ability: TriggeredAbilityProfile,
    ) -> Self {
        self.triggered_ability = Some(triggered_ability);
        self
    }

    #[must_use]
    pub fn from_parts(parts: CardDefinitionParts) -> Self {
        Self {
            id: parts.id,
            card_type: parts.card_type,
            mana_cost: parts.mana_cost,
            casting_permission: parts.casting_permission,
            supported_spell_rules: parts.supported_spell_rules,
            activated_mana_ability: parts.activated_mana_ability,
            activated_ability: parts.activated_ability,
            triggered_ability: parts.triggered_ability,
        }
    }

    #[must_use]
    pub const fn id(&self) -> &CardDefinitionId {
        &self.id
    }

    #[must_use]
    pub fn mana_cost(&self) -> u32 {
        self.mana_cost.total()
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        &self.card_type
    }

    #[must_use]
    pub const fn mana_cost_profile(&self) -> ManaCost {
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

    #[must_use]
    pub const fn activated_mana_ability(&self) -> Option<ActivatedManaAbilityProfile> {
        self.activated_mana_ability
    }

    #[must_use]
    pub const fn activated_ability(&self) -> Option<ActivatedAbilityProfile> {
        self.activated_ability
    }

    #[must_use]
    pub const fn triggered_ability(&self) -> Option<TriggeredAbilityProfile> {
        self.triggered_ability
    }
}
