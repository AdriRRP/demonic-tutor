//! Supports read-only legality queries over the game aggregate.

use super::{helpers, invariants, rules, Game, SpellTarget};
use crate::domain::play::{
    cards::{
        CreatureTargetRule, GraveyardCardTargetRule, KeywordAbility, PlayerTargetRule,
        SingleTargetRule, SpellTargetingProfile,
    },
    game::rules::stack_priority::spell_effects::{
        evaluate_target_legality, supported_spell_rules, SpellTargetLegality, TargetLegalityContext,
    },
    ids::{CardInstanceId, PlayerId, StackObjectId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegalBlockerOption {
    blocker_id: CardInstanceId,
    attacker_ids: Vec<CardInstanceId>,
}

impl LegalBlockerOption {
    #[must_use]
    pub const fn new(blocker_id: CardInstanceId, attacker_ids: Vec<CardInstanceId>) -> Self {
        Self {
            blocker_id,
            attacker_ids,
        }
    }

    #[must_use]
    pub const fn blocker_id(&self) -> &CardInstanceId {
        &self.blocker_id
    }

    #[must_use]
    pub fn attacker_ids(&self) -> &[CardInstanceId] {
        &self.attacker_ids
    }
}

fn legal_targets_for_rule(
    game: &Game,
    actor_index: usize,
    targeting: SpellTargetingProfile,
) -> Vec<SpellTarget> {
    candidate_targets_for_rule(game, actor_index, targeting)
        .into_iter()
        .filter(|candidate| {
            evaluate_target_legality(
                TargetLegalityContext::Cast {
                    players: game.players(),
                    card_locations: &game.card_locations,
                    stack: game.stack(),
                    actor_index,
                },
                targeting,
                Some(candidate),
            ) == SpellTargetLegality::Legal
        })
        .collect()
}

fn candidate_targets_for_rule(
    game: &Game,
    actor_index: usize,
    targeting: SpellTargetingProfile,
) -> Vec<SpellTarget> {
    match targeting {
        SpellTargetingProfile::None => Vec::new(),
        SpellTargetingProfile::ExactlyOne(rule) => {
            candidate_targets_for_single_rule(game, actor_index, rule)
        }
    }
}

fn candidate_targets_for_single_rule(
    game: &Game,
    actor_index: usize,
    rule: SingleTargetRule,
) -> Vec<SpellTarget> {
    match rule {
        SingleTargetRule::Player(rule) => candidate_player_targets(game, actor_index, rule),
        SingleTargetRule::Creature(rule) => candidate_creature_targets(game, actor_index, rule),
        SingleTargetRule::Permanent(rule) => game
            .players()
            .iter()
            .flat_map(|player| {
                player
                    .battlefield_cards()
                    .filter(move |card| rule.allows(*card.card_type()))
                    .map(|card| SpellTarget::Permanent(card.id().clone()))
            })
            .collect(),
        SingleTargetRule::GraveyardCard(rule) => {
            candidate_graveyard_card_targets(game, actor_index, rule)
        }
        SingleTargetRule::StackSpell => candidate_stack_spell_targets(game),
        SingleTargetRule::PlayerOrCreature { player, creature } => {
            let mut candidates = candidate_player_targets(game, actor_index, player);
            candidates.extend(candidate_creature_targets(game, actor_index, creature));
            candidates
        }
    }
}

fn candidate_player_targets(
    game: &Game,
    actor_index: usize,
    rule: PlayerTargetRule,
) -> Vec<SpellTarget> {
    game.players()
        .iter()
        .enumerate()
        .filter(|(target_index, _)| rule.allows(*target_index == actor_index))
        .map(|(_, player)| SpellTarget::Player(player.id().clone()))
        .collect()
}

fn candidate_creature_targets(
    game: &Game,
    actor_index: usize,
    rule: CreatureTargetRule,
) -> Vec<SpellTarget> {
    game.players()
        .iter()
        .enumerate()
        .flat_map(|(player_index, player)| {
            player
                .battlefield_cards()
                .filter(move |card| {
                    card.card_type().is_creature()
                        && rule.allows(
                            player_index == actor_index,
                            card.is_attacking(),
                            card.is_blocking(),
                        )
                })
                .map(|card| SpellTarget::Creature(card.id().clone()))
        })
        .collect()
}

fn candidate_graveyard_card_targets(
    game: &Game,
    actor_index: usize,
    rule: GraveyardCardTargetRule,
) -> Vec<SpellTarget> {
    game.players()
        .iter()
        .enumerate()
        .filter(move |(player_index, _)| rule.allows(*player_index == actor_index))
        .flat_map(|(_, player)| {
            player
                .graveyard()
                .iter()
                .filter_map(|handle| player.card_by_handle(*handle))
                .map(|card| SpellTarget::GraveyardCard(card.id().clone()))
        })
        .collect()
}

fn candidate_stack_spell_targets(game: &Game) -> Vec<SpellTarget> {
    game.stack()
        .objects()
        .iter()
        .map(|object| {
            SpellTarget::StackObject(StackObjectId::for_stack_object(game.id(), object.number()))
        })
        .collect()
}

impl Game {
    #[must_use]
    pub fn can_play_land(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err()
            || invariants::require_no_priority_with_pending_stack(
                self.priority(),
                self.stack.is_empty(),
            )
            .is_err()
        {
            return false;
        }

        let Ok(player_index) = helpers::find_player_index(&self.players, player_id) else {
            return false;
        };

        rules::resource_actions::is_playable_land_candidate(
            &self.players,
            self.active_player_index,
            self.phase,
            player_index,
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn can_tap_mana_source(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err() {
            return false;
        }

        if let Some(priority) = self.priority() {
            if invariants::require_priority_holder(Some(priority), player_id).is_err() {
                return false;
            }
        }

        rules::resource_actions::is_tappable_mana_source_candidate(
            &self.players,
            self.active_player_index,
            self.phase,
            self.priority(),
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn castable_card(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err() {
            return false;
        }

        rules::stack_priority::is_castable_candidate(
            &self.players,
            &self.card_locations,
            self.active_player(),
            self.phase,
            &self.stack,
            self.priority(),
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn activatable_card(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err() {
            return false;
        }

        rules::stack_priority::is_activatable_candidate(
            &self.players,
            &self.card_locations,
            self.active_player(),
            self.phase,
            &self.stack,
            self.priority(),
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn can_attack_with(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err()
            || invariants::require_no_open_priority_window(self.priority()).is_err()
        {
            return false;
        }

        let Ok(player_index) = helpers::find_player_index(&self.players, player_id) else {
            return false;
        };
        if player_index != self.active_player_index {
            return false;
        }

        rules::combat::can_attack_with_candidate(
            &self.players,
            self.active_player_index,
            self.phase,
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn spell_target_candidates(
        &self,
        actor_id: &PlayerId,
        card_id: &CardInstanceId,
    ) -> Vec<SpellTarget> {
        if invariants::require_game_active(self.is_over()).is_err() {
            return Vec::new();
        }
        let Ok(actor_index) = helpers::find_player_index(&self.players, actor_id) else {
            return Vec::new();
        };
        let Some(player) = self.players.get(actor_index) else {
            return Vec::new();
        };
        let Some(card) = player
            .hand_card(card_id)
            .or_else(|| player.graveyard_card(card_id))
        else {
            return Vec::new();
        };

        legal_targets_for_rule(self, actor_index, supported_spell_rules(card).targeting())
    }

    #[must_use]
    pub fn ability_target_candidates(
        &self,
        actor_id: &PlayerId,
        card_id: &CardInstanceId,
    ) -> Vec<SpellTarget> {
        if invariants::require_game_active(self.is_over()).is_err() {
            return Vec::new();
        }
        let Ok(actor_index) = helpers::find_player_index(&self.players, actor_id) else {
            return Vec::new();
        };
        let Some(player) = self.players.get(actor_index) else {
            return Vec::new();
        };
        let Some(card) = player.battlefield_card(card_id) else {
            return Vec::new();
        };
        let Some(ability) = card.activated_ability() else {
            return Vec::new();
        };

        legal_targets_for_rule(self, actor_index, ability.targeting())
    }

    #[must_use]
    pub fn blocker_options(&self, player_id: &PlayerId) -> Vec<LegalBlockerOption> {
        if invariants::require_game_active(self.is_over()).is_err()
            || invariants::require_no_open_priority_window(self.priority()).is_err()
            || !matches!(
                self.phase(),
                crate::domain::play::phase::Phase::DeclareBlockers
            )
        {
            return Vec::new();
        }

        let Ok(defending_player_index) = helpers::find_player_index(&self.players, player_id)
        else {
            return Vec::new();
        };
        if defending_player_index == self.active_player_index {
            return Vec::new();
        }
        let Some(defending_player) = self.players.get(defending_player_index) else {
            return Vec::new();
        };

        let attacker_ids: Vec<_> = self.players[self.active_player_index]
            .battlefield_cards()
            .filter(|card| card.is_attacking())
            .map(|card| card.id().clone())
            .collect();
        let legal_blockers_by_attacker: std::collections::HashMap<_, Vec<CardInstanceId>> =
            attacker_ids
                .iter()
                .map(|attacker_id| {
                    let blocker_ids = defending_player
                        .battlefield_card_ids()
                        .filter(|blocker_id| {
                            rules::combat::can_block_attacker_candidate(
                                &self.players,
                                self.active_player_index,
                                player_id,
                                blocker_id,
                                attacker_id,
                            )
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    (attacker_id.clone(), blocker_ids)
                })
                .collect();
        let public_attacker_ids: Vec<_> = attacker_ids
            .into_iter()
            .filter(|attacker_id| {
                let attacker = self.players[self.active_player_index]
                    .battlefield_card(attacker_id)
                    .and_then(crate::domain::play::cards::CardInstance::keyword_abilities);
                let blocker_count = legal_blockers_by_attacker
                    .get(attacker_id)
                    .map_or(0, Vec::len);
                !attacker.is_some_and(|keywords| {
                    keywords.contains(KeywordAbility::Menace) && blocker_count < 2
                })
            })
            .collect();

        defending_player
            .battlefield_card_ids()
            .filter_map(|blocker_id| {
                let attacker_ids: Vec<_> = public_attacker_ids
                    .iter()
                    .filter(|attacker_id| {
                        legal_blockers_by_attacker
                            .get(*attacker_id)
                            .is_some_and(|blockers| blockers.contains(blocker_id))
                    })
                    .cloned()
                    .collect();
                (!attacker_ids.is_empty())
                    .then(|| LegalBlockerOption::new(blocker_id.clone(), attacker_ids))
            })
            .collect()
    }
}
