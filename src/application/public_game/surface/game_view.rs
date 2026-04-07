//! Projects aggregate state into the shared public game snapshot.

use crate::domain::play::{
    cards::{CardInstance, KeywordAbility, ManaCost},
    game::{Game, Player, StackObjectKind},
    ids::{PlayerId, StackObjectId},
};

use super::super::{
    PublicBattlefieldCardView, PublicCardView, PublicCombatStateView, PublicGameView,
    PublicManaCostView, PublicManaPoolView, PublicPermanentStateView,
    PublicPlayableSubsetVersion, PublicPlayerView, PublicPriorityView, PublicStackObjectView,
    PublicStackTargetView,
};

#[must_use]
pub fn game_view(game: &Game) -> PublicGameView {
    let active_player_id = active_player_id_for_public_view(game);
    let players = game
        .players()
        .iter()
        .enumerate()
        .map(|(index, player)| player_view(player, index, active_player_id.as_ref()))
        .collect();
    let stack = game
        .stack()
        .objects()
        .iter()
        .map(|object| stack_object_view(game, object))
        .collect();

    PublicGameView {
        game_id: game.id().clone(),
        playable_subset_version: PublicPlayableSubsetVersion::V1,
        active_player_id,
        phase: *game.phase(),
        turn_number: game.turn_number(),
        priority: game.priority().map(|priority| PublicPriorityView {
            current_holder: priority.current_holder().clone(),
            has_pending_pass: priority.has_pending_pass(),
        }),
        is_over: game.is_over(),
        winner_id: game.winner().cloned(),
        loser_id: game.loser().cloned(),
        end_reason: game.end_reason(),
        players,
        stack,
    }
}

fn player_view(
    player: &Player,
    _index: usize,
    active_player_id: Option<&PlayerId>,
) -> PublicPlayerView {
    PublicPlayerView {
        player_id: player.id().clone(),
        is_active: active_player_id.is_some_and(|active_player_id| player.id() == active_player_id),
        life: player.life(),
        mana_total: player.mana(),
        mana_pool: mana_pool_view(player),
        hand_count: player.hand_size(),
        library_count: player.library_size(),
        battlefield: player
            .battlefield_cards()
            .map(battlefield_card_view)
            .collect(),
        graveyard: player.graveyard_cards().map(card_view).collect(),
        exile: player.exile_cards().map(card_view).collect(),
    }
}

fn mana_pool_view(player: &Player) -> PublicManaPoolView {
    let mana_pool = player.mana_pool();

    PublicManaPoolView {
        colorless: mana_pool.generic(),
        white: mana_pool.white(),
        blue: mana_pool.blue(),
        black: mana_pool.black(),
        red: mana_pool.red(),
        green: mana_pool.green(),
    }
}

fn card_view(card: &CardInstance) -> PublicCardView {
    PublicCardView {
        card_id: card.id().clone(),
        definition_id: card.definition_id().clone(),
        card_type: *card.card_type(),
        mana_cost: mana_cost_view(card.mana_cost_profile()),
    }
}

fn battlefield_card_view(card: &CardInstance) -> PublicBattlefieldCardView {
    PublicBattlefieldCardView {
        card_id: card.id().clone(),
        definition_id: card.definition_id().clone(),
        card_type: *card.card_type(),
        mana_cost: mana_cost_view(card.mana_cost_profile()),
        permanent_state: PublicPermanentStateView {
            tapped: card.is_tapped(),
            token: card.is_token(),
        },
        attached_to: card.attached_to().cloned(),
        power: card.power(),
        toughness: card.toughness(),
        loyalty: card.loyalty(),
        combat_state: PublicCombatStateView {
            summoning_sickness: card.has_summoning_sickness(),
            attacking: card.is_attacking(),
            blocking: card.is_blocking(),
        },
        keywords: keyword_list(card),
    }
}

fn mana_cost_view(mana_cost: ManaCost) -> PublicManaCostView {
    PublicManaCostView {
        generic: mana_cost.generic_requirement(),
        white: mana_cost.white_requirement(),
        blue: mana_cost.blue_requirement(),
        black: mana_cost.black_requirement(),
        red: mana_cost.red_requirement(),
        green: mana_cost.green_requirement(),
    }
}

fn keyword_list(card: &CardInstance) -> Vec<KeywordAbility> {
    const ORDER: [KeywordAbility; 13] = [
        KeywordAbility::Flying,
        KeywordAbility::Reach,
        KeywordAbility::Haste,
        KeywordAbility::Vigilance,
        KeywordAbility::Trample,
        KeywordAbility::FirstStrike,
        KeywordAbility::Deathtouch,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Lifelink,
        KeywordAbility::Menace,
        KeywordAbility::Hexproof,
        KeywordAbility::Indestructible,
        KeywordAbility::Defender,
    ];

    let Some(keywords) = card.keyword_abilities() else {
        return Vec::new();
    };

    ORDER
        .into_iter()
        .filter(|ability| keywords.contains(*ability))
        .collect()
}

fn stack_object_view(
    game: &Game,
    object: &crate::domain::play::game::StackObject,
) -> PublicStackObjectView {
    let Some(controller_id) = game
        .players()
        .get(object.controller_index())
        .map(crate::domain::play::game::Player::id)
        .cloned()
    else {
        return PublicStackObjectView::Unavailable {
            number: object.number(),
        };
    };

    match object.kind() {
        StackObjectKind::Spell(spell) => PublicStackObjectView::Spell {
            number: object.number(),
            controller_id,
            source_card_id: spell.source_card_id().clone(),
            definition_id: spell.payload().definition_id().clone(),
            card_type: *spell.card_type(),
            target: spell
                .target()
                .map(|target| stack_target_view(game, *target)),
            requires_choice: spell.choice().is_some(),
        },
        StackObjectKind::ActivatedAbility(ability) => {
            let source_card_id = ability.source_card_id();
            let source_card = stack_source_card(game, &source_card_id);

            PublicStackObjectView::ActivatedAbility {
                number: object.number(),
                controller_id,
                source_card_id,
                definition_id: source_card.map(|card| card.definition_id().clone()),
                card_type: source_card.map(|card| *card.card_type()),
                target: ability
                    .target()
                    .map(|target| stack_target_view(game, *target)),
            }
        }
        StackObjectKind::TriggeredAbility(ability) => {
            let source_card_id = ability.source_card_id();
            let source_card = stack_source_card(game, &source_card_id);

            PublicStackObjectView::TriggeredAbility {
                number: object.number(),
                controller_id,
                source_card_id,
                definition_id: source_card.map(|card| card.definition_id().clone()),
                card_type: source_card.map(|card| *card.card_type()),
            }
        }
    }
}

fn stack_source_card<'a>(
    game: &'a Game,
    source_card_id: &crate::domain::play::ids::CardInstanceId,
) -> Option<&'a CardInstance> {
    game.players().iter().find_map(|player| {
        let handle = player.resolve_public_card_handle(source_card_id)?;
        player.card_by_handle(handle)
    })
}

fn stack_target_view(
    game: &Game,
    target: crate::domain::play::game::model::StackTargetRef,
) -> PublicStackTargetView {
    match target {
        crate::domain::play::game::model::StackTargetRef::Player(index) => game
            .players()
            .get(index)
            .map_or(PublicStackTargetView::Unavailable, |player| {
                PublicStackTargetView::Player(player.id().clone())
            }),
        crate::domain::play::game::model::StackTargetRef::Creature(card_ref)
        | crate::domain::play::game::model::StackTargetRef::Permanent(card_ref)
        | crate::domain::play::game::model::StackTargetRef::GraveyardCard(card_ref) => game
            .players()
            .get(card_ref.player_index())
            .and_then(|player| player.card_by_handle(card_ref.handle()))
            .map_or(PublicStackTargetView::Unavailable, |card| {
                PublicStackTargetView::Card(card.id().clone())
            }),
        crate::domain::play::game::model::StackTargetRef::StackSpell(number) => {
            PublicStackTargetView::StackSpell(StackObjectId::for_stack_object(game.id(), number))
        }
    }
}

fn active_player_id_for_public_view(game: &Game) -> Option<PlayerId> {
    game.players()
        .get(game.active_player_index_value())
        .map(|player| player.id().clone())
}
