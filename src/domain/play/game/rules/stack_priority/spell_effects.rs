use crate::domain::play::{
    cards::{
        CardInstance, CreatureTargetRule, PlayerTargetRule, SpellTargetingProfile,
        SupportedSpellRules,
    },
    game::{Player, SpellTarget},
    ids::{CardInstanceId, PlayerId},
};

#[must_use]
pub const fn supported_spell_rules(card: &CardInstance) -> SupportedSpellRules {
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

fn target_creature_on_battlefield<'a>(
    players: &'a [Player],
    card_id: &CardInstanceId,
) -> Option<(&'a Player, &'a CardInstance)> {
    players.iter().find_map(|player| {
        player
            .battlefield()
            .cards()
            .iter()
            .find(|card| card.id() == card_id)
            .map(|card| (player, card))
    })
}

#[must_use]
pub fn evaluate_target_legality(
    context: TargetLegalityContext<'_>,
    targeting: SpellTargetingProfile,
    target: Option<&SpellTarget>,
) -> SpellTargetLegality {
    let players = context.players();
    let _actor_id = context.actor_id();
    match (targeting, target) {
        (SpellTargetingProfile::None, None) => SpellTargetLegality::NoTargetRequired,
        (SpellTargetingProfile::None, Some(_)) => SpellTargetLegality::IllegalTargetKind,
        (SpellTargetingProfile::ExactlyOne(_), None) => SpellTargetLegality::MissingRequiredTarget,
        (targeting, Some(target)) => {
            if !accepts_target(targeting, target) {
                return SpellTargetLegality::IllegalTargetKind;
            }

            let actor_id = context.actor_id();
            match (targeting, target) {
                (SpellTargetingProfile::None, _) => SpellTargetLegality::IllegalTargetKind,
                (SpellTargetingProfile::ExactlyOne(rule), SpellTarget::Player(player_id)) => {
                    let Some(target_player) = target_player_exists(players, player_id) else {
                        return SpellTargetLegality::MissingPlayer(player_id.clone());
                    };

                    match rule.player_rule() {
                        Some(rule) if rule.allows(target_player.id() == actor_id) => {
                            SpellTargetLegality::Legal
                        }
                        Some(PlayerTargetRule::AnyPlayer | PlayerTargetRule::OpponentOfActor) => {
                            SpellTargetLegality::IllegalTargetRule
                        }
                        None => SpellTargetLegality::IllegalTargetKind,
                    }
                }
                (SpellTargetingProfile::ExactlyOne(rule), SpellTarget::Creature(card_id)) => {
                    let Some((controller, target_creature)) =
                        target_creature_on_battlefield(players, card_id)
                    else {
                        return SpellTargetLegality::MissingCreature(card_id.clone());
                    };

                    match rule.creature_rule() {
                        Some(rule)
                            if rule.allows(
                                controller.id() == actor_id,
                                target_creature.is_attacking(),
                                target_creature.is_blocking(),
                            ) =>
                        {
                            SpellTargetLegality::Legal
                        }
                        Some(
                            CreatureTargetRule::AnyCreatureOnBattlefield
                            | CreatureTargetRule::CreatureControlledByActor
                            | CreatureTargetRule::AttackingCreature
                            | CreatureTargetRule::BlockingCreature
                            | CreatureTargetRule::CreatureControlledByActorAndAttacking
                            | CreatureTargetRule::CreatureControlledByActorAndBlocking
                            | CreatureTargetRule::BlockingCreatureControlledByOpponent
                            | CreatureTargetRule::AttackingCreatureControlledByOpponent,
                        ) => SpellTargetLegality::IllegalTargetRule,
                        None => SpellTargetLegality::IllegalTargetKind,
                    }
                }
            }
        }
    }
}
