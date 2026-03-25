//! Supports rules stack priority spell effects.

use crate::domain::play::{
    cards::{CardInstance, SpellTargetingProfile, SupportedSpellRules},
    game::{helpers, model::StackZone, AggregateCardLocationIndex, Player, SpellTarget},
    ids::{CardInstanceId, PlayerId, StackObjectId},
};

#[must_use]
pub fn supported_spell_rules(card: &CardInstance) -> SupportedSpellRules {
    card.supported_spell_rules()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellTargetLegality {
    NoTargetRequired,
    MissingRequiredTarget,
    IllegalTargetKind,
    IllegalTargetRule,
    MissingPlayer(PlayerId),
    MissingCreature(CardInstanceId),
    MissingPermanent(CardInstanceId),
    MissingGraveyardCard(CardInstanceId),
    MissingStackSpell(StackObjectId),
    Legal,
}

#[derive(Debug, Clone, Copy)]
pub enum TargetLegalityContext<'a> {
    Cast {
        players: &'a [Player],
        card_locations: &'a AggregateCardLocationIndex,
        stack: &'a StackZone,
        actor_index: usize,
    },
    Resolution {
        players: &'a [Player],
        card_locations: &'a AggregateCardLocationIndex,
        stack: &'a StackZone,
        actor_index: usize,
    },
}

impl<'a> TargetLegalityContext<'a> {
    #[must_use]
    pub const fn players(self) -> &'a [Player] {
        match self {
            Self::Cast { players, .. } | Self::Resolution { players, .. } => players,
        }
    }

    #[must_use]
    pub const fn card_locations(self) -> &'a AggregateCardLocationIndex {
        match self {
            Self::Cast { card_locations, .. } | Self::Resolution { card_locations, .. } => {
                card_locations
            }
        }
    }

    #[must_use]
    pub const fn stack(self) -> &'a StackZone {
        match self {
            Self::Cast { stack, .. } | Self::Resolution { stack, .. } => stack,
        }
    }

    #[must_use]
    pub const fn actor_index(self) -> usize {
        match self {
            Self::Cast { actor_index, .. } | Self::Resolution { actor_index, .. } => actor_index,
        }
    }
}

#[must_use]
pub const fn accepts_target(targeting: SpellTargetingProfile, target: &SpellTarget) -> bool {
    targeting.allows_target_kind(target.kind())
}

fn target_player_exists<'a>(players: &'a [Player], player_id: &PlayerId) -> Option<&'a Player> {
    players.iter().find(|player| player.id() == player_id)
}

enum ResolvedTarget {
    Player {
        is_actor: bool,
    },
    Creature {
        is_actor: bool,
        is_attacking: bool,
        is_blocking: bool,
    },
    Permanent {
        card_type: crate::domain::play::cards::CardType,
    },
    GraveyardCard,
    StackSpell,
}

fn resolve_target(
    context: TargetLegalityContext<'_>,
    targeting: SpellTargetingProfile,
    target: &SpellTarget,
) -> Result<ResolvedTarget, SpellTargetLegality> {
    let players = context.players();
    let card_locations = context.card_locations();
    let stack = context.stack();
    let actor_index = context.actor_index();

    if !accepts_target(targeting, target) {
        return Err(SpellTargetLegality::IllegalTargetKind);
    }

    match target {
        SpellTarget::Player(player_id) => {
            let Some(target_player) = target_player_exists(players, player_id) else {
                return Err(SpellTargetLegality::MissingPlayer(player_id.clone()));
            };

            Ok(ResolvedTarget::Player {
                is_actor: players
                    .get(actor_index)
                    .is_some_and(|actor| target_player.id() == actor.id()),
            })
        }
        SpellTarget::Creature(card_id) => {
            let Some(target_creature) =
                helpers::battlefield_card_location(players, card_locations, card_id)
            else {
                return Err(SpellTargetLegality::MissingCreature(card_id.clone()));
            };

            Ok(ResolvedTarget::Creature {
                is_actor: target_creature.owner_index() == actor_index,
                is_attacking: target_creature.card().is_attacking(),
                is_blocking: target_creature.card().is_blocking(),
            })
        }
        SpellTarget::Permanent(card_id) => {
            let Some(target_permanent) =
                helpers::battlefield_card_location(players, card_locations, card_id)
            else {
                return Err(SpellTargetLegality::MissingPermanent(card_id.clone()));
            };

            Ok(ResolvedTarget::Permanent {
                card_type: *target_permanent.card().card_type(),
            })
        }
        SpellTarget::GraveyardCard(card_id) => {
            if !helpers::graveyard_card_exists(players, card_locations, card_id) {
                return Err(SpellTargetLegality::MissingGraveyardCard(card_id.clone()));
            }

            Ok(ResolvedTarget::GraveyardCard)
        }
        SpellTarget::StackObject(stack_object_id) => {
            let Some(object_number) = stack_object_id.object_number() else {
                return Err(SpellTargetLegality::MissingStackSpell(stack_object_id.clone()));
            };
            let Some(stack_object) = stack.object(object_number) else {
                return Err(SpellTargetLegality::MissingStackSpell(stack_object_id.clone()));
            };
            if !matches!(
                stack_object.kind(),
                crate::domain::play::game::model::StackObjectKind::Spell(_)
            ) {
                return Err(SpellTargetLegality::IllegalTargetRule);
            }

            Ok(ResolvedTarget::StackSpell)
        }
    }
}

const fn evaluate_resolved_target_rule(
    targeting: SpellTargetingProfile,
    resolved_target: &ResolvedTarget,
) -> SpellTargetLegality {
    match (targeting, resolved_target) {
        (SpellTargetingProfile::None, _) => SpellTargetLegality::IllegalTargetKind,
        (SpellTargetingProfile::ExactlyOne(rule), ResolvedTarget::Player { is_actor }) => {
            match rule.allows_player_target(*is_actor) {
                Some(true) => SpellTargetLegality::Legal,
                Some(false) => SpellTargetLegality::IllegalTargetRule,
                None => SpellTargetLegality::IllegalTargetKind,
            }
        }
        (
            SpellTargetingProfile::ExactlyOne(rule),
            ResolvedTarget::Creature {
                is_actor,
                is_attacking,
                is_blocking,
            },
        ) => match rule.allows_creature_target(*is_actor, *is_attacking, *is_blocking) {
            Some(true) => SpellTargetLegality::Legal,
            Some(false) => SpellTargetLegality::IllegalTargetRule,
            None => SpellTargetLegality::IllegalTargetKind,
        },
        (SpellTargetingProfile::ExactlyOne(rule), ResolvedTarget::GraveyardCard) => {
            match rule.allows_graveyard_card_target() {
                Some(true) => SpellTargetLegality::Legal,
                Some(false) => SpellTargetLegality::IllegalTargetRule,
                None => SpellTargetLegality::IllegalTargetKind,
            }
        }
        (SpellTargetingProfile::ExactlyOne(rule), ResolvedTarget::Permanent { card_type }) => {
            match rule.allows_permanent_target(*card_type) {
                Some(true) => SpellTargetLegality::Legal,
                Some(false) => SpellTargetLegality::IllegalTargetRule,
                None => SpellTargetLegality::IllegalTargetKind,
            }
        }
        (SpellTargetingProfile::ExactlyOne(rule), ResolvedTarget::StackSpell) => {
            match rule.allows_stack_spell_target() {
                Some(true) => SpellTargetLegality::Legal,
                Some(false) => SpellTargetLegality::IllegalTargetRule,
                None => SpellTargetLegality::IllegalTargetKind,
            }
        }
    }
}

#[must_use]
pub fn evaluate_target_legality(
    context: TargetLegalityContext<'_>,
    targeting: SpellTargetingProfile,
    target: Option<&SpellTarget>,
) -> SpellTargetLegality {
    match (targeting, target) {
        (SpellTargetingProfile::None, None) => SpellTargetLegality::NoTargetRequired,
        (SpellTargetingProfile::None, Some(_)) => SpellTargetLegality::IllegalTargetKind,
        (SpellTargetingProfile::ExactlyOne(_), None) => SpellTargetLegality::MissingRequiredTarget,
        (targeting, Some(target)) => match resolve_target(context, targeting, target) {
            Ok(resolved_target) => evaluate_resolved_target_rule(targeting, &resolved_target),
            Err(illegal) => illegal,
        },
    }
}
