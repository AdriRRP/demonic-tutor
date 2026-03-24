//! Supports rules stack priority spell effects.

use crate::domain::play::{
    cards::{CardInstance, SpellTargetingProfile, SupportedSpellRules},
    game::{helpers, Player, SpellTarget},
    ids::{CardInstanceId, PlayerId},
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
    MissingGraveyardCard(CardInstanceId),
    Legal,
}

#[derive(Debug, Clone, Copy)]
pub enum TargetLegalityContext<'a> {
    Cast {
        players: &'a [Player],
        caster_id: &'a PlayerId,
    },
    Resolution {
        players: &'a [Player],
        controller_id: &'a PlayerId,
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
    pub const fn actor_id(self) -> &'a PlayerId {
        match self {
            Self::Cast { caster_id, .. } => caster_id,
            Self::Resolution { controller_id, .. } => controller_id,
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
    GraveyardCard,
}

fn resolve_target(
    context: TargetLegalityContext<'_>,
    targeting: SpellTargetingProfile,
    target: &SpellTarget,
) -> Result<ResolvedTarget, SpellTargetLegality> {
    let players = context.players();
    let actor_id = context.actor_id();

    if !accepts_target(targeting, target) {
        return Err(SpellTargetLegality::IllegalTargetKind);
    }

    let actor_index = helpers::find_player_index(players, actor_id).ok();

    match target {
        SpellTarget::Player(player_id) => {
            let Some(target_player) = target_player_exists(players, player_id) else {
                return Err(SpellTargetLegality::MissingPlayer(player_id.clone()));
            };

            Ok(ResolvedTarget::Player {
                is_actor: target_player.id() == actor_id,
            })
        }
        SpellTarget::Creature(card_id) => {
            let Some(target_creature) = helpers::battlefield_card_location(players, card_id) else {
                return Err(SpellTargetLegality::MissingCreature(card_id.clone()));
            };

            Ok(ResolvedTarget::Creature {
                is_actor: actor_index.is_some_and(|index| target_creature.owner_index() == index),
                is_attacking: target_creature.card().is_attacking(),
                is_blocking: target_creature.card().is_blocking(),
            })
        }
        SpellTarget::GraveyardCard(card_id) => {
            if helpers::graveyard_card_location(players, card_id).is_none() {
                return Err(SpellTargetLegality::MissingGraveyardCard(card_id.clone()));
            }

            Ok(ResolvedTarget::GraveyardCard)
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
