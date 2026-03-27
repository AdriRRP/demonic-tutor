//! Supports spell payload conversion and reconstruction.

use super::{
    Arc, CardDefinition, CardDefinitionId, CardFace, CardInstance, CardInstanceId, CardRuntime,
    CardRuntimeKind, CardType, CastingRule, CreatureRuntime, CreatureSpellPayload,
    EffectSpellPayload, PermanentSpellPayload, PlayerId, SpellPayload, SpellPayloadKind,
    SupportedSpellRules,
};

impl CardInstance {
    const fn permanent_payload(definition: &CardDefinition) -> PermanentSpellPayload {
        PermanentSpellPayload {
            supported_spell_rules: definition.supported_spell_rules(),
            activated_ability: definition.activated_ability(),
            triggered_ability: definition.triggered_ability(),
            initial_loyalty: definition.initial_loyalty(),
            attachment_profile: definition.attachment_profile(),
            attached_stat_boost: definition.attached_stat_boost(),
            attached_combat_restriction: definition.attached_combat_restriction(),
            controller_static_effect: definition.controller_static_effect(),
        }
    }

    const fn effect_payload(definition: &CardDefinition) -> EffectSpellPayload {
        EffectSpellPayload {
            casting_permission: definition.casting_permission(),
            supported_spell_rules: definition.supported_spell_rules(),
        }
    }

    #[must_use]
    pub fn into_spell_payload(self) -> SpellPayload {
        let definition_id = self.face.definition.id().clone();
        let owner_id = self.owner_id;
        match &self.runtime.kind {
            CardRuntimeKind::NonCreature => {
                let definition = self.face.definition.as_ref();
                match definition.card_type() {
                    CardType::Artifact => SpellPayload {
                        id: self.id,
                        owner_id,
                        definition_id,
                        exile_on_resolution: false,
                        kind: SpellPayloadKind::Artifact(Self::permanent_payload(definition)),
                    },
                    CardType::Enchantment => SpellPayload {
                        id: self.id,
                        owner_id,
                        definition_id,
                        exile_on_resolution: false,
                        kind: SpellPayloadKind::Enchantment(Self::permanent_payload(definition)),
                    },
                    CardType::Planeswalker => SpellPayload {
                        id: self.id,
                        owner_id,
                        definition_id,
                        exile_on_resolution: false,
                        kind: SpellPayloadKind::Planeswalker(Self::permanent_payload(definition)),
                    },
                    CardType::Land => SpellPayload {
                        id: self.id,
                        owner_id,
                        definition_id,
                        exile_on_resolution: false,
                        kind: SpellPayloadKind::Land(Self::permanent_payload(definition)),
                    },
                    CardType::Instant => SpellPayload {
                        id: self.id,
                        owner_id,
                        definition_id,
                        exile_on_resolution: false,
                        kind: SpellPayloadKind::Instant(Self::effect_payload(definition)),
                    },
                    CardType::Sorcery => SpellPayload {
                        id: self.id,
                        owner_id,
                        definition_id,
                        exile_on_resolution: false,
                        kind: SpellPayloadKind::Sorcery(Self::effect_payload(definition)),
                    },
                    CardType::Creature => {
                        debug_assert!(
                            false,
                            "non-creature spell payloads should never be built from creature definitions"
                        );
                        SpellPayload {
                            id: self.id,
                            owner_id,
                            definition_id,
                            exile_on_resolution: false,
                            kind: SpellPayloadKind::Land(Self::permanent_payload(definition)),
                        }
                    }
                }
            }
            CardRuntimeKind::Creature(creature) => SpellPayload {
                id: self.id,
                owner_id,
                definition_id,
                exile_on_resolution: false,
                kind: SpellPayloadKind::Creature(CreatureSpellPayload {
                    power: creature.power,
                    toughness: creature.toughness,
                    keywords: creature.keywords,
                    activated_ability: self.face.definition.activated_ability(),
                    triggered_ability: self.face.definition.triggered_ability(),
                    controller_static_effect: self.face.definition.controller_static_effect(),
                }),
            },
        }
    }
}

impl SpellPayload {
    #[must_use]
    pub const fn id(&self) -> &CardInstanceId {
        &self.id
    }

    #[must_use]
    pub const fn definition_id(&self) -> &CardDefinitionId {
        &self.definition_id
    }

    #[must_use]
    pub const fn owner_id(&self) -> Option<&PlayerId> {
        self.owner_id.as_ref()
    }

    pub(crate) const fn mark_exile_on_resolution(&mut self) {
        self.exile_on_resolution = true;
    }

    #[must_use]
    pub const fn exile_on_resolution(&self) -> bool {
        self.exile_on_resolution
    }

    #[must_use]
    pub const fn kind(&self) -> &SpellPayloadKind {
        &self.kind
    }

    fn effect_into_card_instance(
        id: CardInstanceId,
        owner_id: Option<PlayerId>,
        definition_id: CardDefinitionId,
        payload: &EffectSpellPayload,
        card_type: CardType,
    ) -> CardInstance {
        let mut definition = CardDefinition::for_card_type(definition_id, 0, &card_type)
            .with_supported_spell_rules(payload.supported_spell_rules);
        if let Some(permission) = payload.casting_permission {
            if permission.supports(CastingRule::OpenPriorityWindowDuringOwnTurn) {
                definition =
                    definition.with_casting_rule(CastingRule::OpenPriorityWindowDuringOwnTurn);
            }
            if permission.supports(CastingRule::CastFromOwnGraveyard) {
                definition = definition.with_casting_rule(CastingRule::CastFromOwnGraveyard);
            }
            if permission.supports(CastingRule::ExileOnResolutionWhenCastFromOwnGraveyard) {
                definition = definition
                    .with_casting_rule(CastingRule::ExileOnResolutionWhenCastFromOwnGraveyard);
            }
        }
        CardInstance {
            id,
            owner_id,
            face: CardFace {
                definition: Arc::new(definition),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: 0,
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                attached_to: None,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    fn permanent_into_card_instance(
        id: CardInstanceId,
        owner_id: Option<PlayerId>,
        definition_id: CardDefinitionId,
        payload: &PermanentSpellPayload,
        card_type: CardType,
    ) -> CardInstance {
        let mut definition = CardDefinition::for_card_type(definition_id, 0, &card_type);
        definition = definition.with_supported_spell_rules(payload.supported_spell_rules);
        if let Some(activated_ability) = payload.activated_ability {
            definition = definition.with_activated_ability(activated_ability);
        }
        if let Some(triggered_ability) = payload.triggered_ability {
            definition = definition.with_triggered_ability(triggered_ability);
        }
        if let Some(initial_loyalty) = payload.initial_loyalty {
            definition = definition.with_initial_loyalty(initial_loyalty);
        }
        if let Some(attachment_profile) = payload.attachment_profile {
            definition = definition.with_attachment_profile(attachment_profile);
        }
        if let Some(attached_stat_boost) = payload.attached_stat_boost {
            definition = definition.with_attached_stat_boost(attached_stat_boost);
        }
        if let Some(attached_combat_restriction) = payload.attached_combat_restriction {
            definition = definition.with_attached_combat_restriction(attached_combat_restriction);
        }
        if let Some(controller_static_effect) = payload.controller_static_effect {
            definition = definition.with_controller_static_effect(controller_static_effect);
        }
        CardInstance {
            id,
            owner_id,
            face: CardFace {
                definition: Arc::new(definition),
            },
            runtime: CardRuntime {
                tapped: false,
                loyalty: payload.initial_loyalty.unwrap_or(0),
                loyalty_ability_activated_this_turn: false,
                is_token: false,
                attached_to: None,
                kind: CardRuntimeKind::NonCreature,
            },
        }
    }

    #[must_use]
    pub const fn card_type(&self) -> &CardType {
        match self.kind() {
            SpellPayloadKind::Instant(_) => &CardType::Instant,
            SpellPayloadKind::Sorcery(_) => &CardType::Sorcery,
            SpellPayloadKind::Artifact(_) => &CardType::Artifact,
            SpellPayloadKind::Enchantment(_) => &CardType::Enchantment,
            SpellPayloadKind::Planeswalker(_) => &CardType::Planeswalker,
            SpellPayloadKind::Land(_) => &CardType::Land,
            SpellPayloadKind::Creature(_) => &CardType::Creature,
        }
    }

    #[must_use]
    pub const fn supported_spell_rules(&self) -> SupportedSpellRules {
        match self.kind() {
            SpellPayloadKind::Instant(payload) | SpellPayloadKind::Sorcery(payload) => {
                payload.supported_spell_rules
            }
            SpellPayloadKind::Artifact(payload)
            | SpellPayloadKind::Enchantment(payload)
            | SpellPayloadKind::Planeswalker(payload)
            | SpellPayloadKind::Land(payload) => payload.supported_spell_rules,
            SpellPayloadKind::Creature(_) => SupportedSpellRules::none(),
        }
    }

    #[must_use]
    pub fn into_card_instance(self) -> CardInstance {
        let Self {
            id,
            owner_id,
            definition_id,
            exile_on_resolution: _,
            kind,
        } = self;

        match kind {
            SpellPayloadKind::Instant(payload) => Self::effect_into_card_instance(
                id,
                owner_id,
                definition_id,
                &payload,
                CardType::Instant,
            ),
            SpellPayloadKind::Sorcery(payload) => Self::effect_into_card_instance(
                id,
                owner_id,
                definition_id,
                &payload,
                CardType::Sorcery,
            ),
            SpellPayloadKind::Artifact(payload) => Self::permanent_into_card_instance(
                id,
                owner_id,
                definition_id,
                &payload,
                CardType::Artifact,
            ),
            SpellPayloadKind::Enchantment(payload) => Self::permanent_into_card_instance(
                id,
                owner_id,
                definition_id,
                &payload,
                CardType::Enchantment,
            ),
            SpellPayloadKind::Planeswalker(payload) => Self::permanent_into_card_instance(
                id,
                owner_id,
                definition_id,
                &payload,
                CardType::Planeswalker,
            ),
            SpellPayloadKind::Land(payload) => Self::permanent_into_card_instance(
                id,
                owner_id,
                definition_id,
                &payload,
                CardType::Land,
            ),
            SpellPayloadKind::Creature(payload) => CardInstance {
                id,
                owner_id,
                face: CardFace {
                    definition: Arc::new({
                        let mut definition =
                            CardDefinition::for_card_type(definition_id, 0, &CardType::Creature);
                        if let Some(activated_ability) = payload.activated_ability {
                            definition = definition.with_activated_ability(activated_ability);
                        }
                        if let Some(triggered_ability) = payload.triggered_ability {
                            definition = definition.with_triggered_ability(triggered_ability);
                        }
                        if let Some(controller_static_effect) = payload.controller_static_effect {
                            definition =
                                definition.with_controller_static_effect(controller_static_effect);
                        }
                        definition
                    }),
                },
                runtime: CardRuntime {
                    tapped: false,
                    loyalty: 0,
                    loyalty_ability_activated_this_turn: false,
                    is_token: false,
                    attached_to: None,
                    kind: CardRuntimeKind::Creature(CreatureRuntime::new_with_keywords(
                        payload.power,
                        payload.toughness,
                        payload.keywords,
                    )),
                },
            },
        }
    }
}
