//! Supports the public gameplay contract consumed by clients.

use crate::{
    application::{
        game_service::{
            combat::domain_events_for_resolve_combat_damage,
            resource_actions::domain_events_for_adjust_player_life_effect,
            stack::{
                domain_events_for_activate_ability, domain_events_for_cast_spell,
                domain_events_for_pass_priority,
            },
            turn_flow::{domain_events_for_advance_turn, domain_events_for_draw_cards_effect},
            GameService,
        },
        EventBus, EventStore,
    },
    domain::play::{
        cards::{CardInstance, CardType, KeywordAbility, SpellTargetingProfile},
        commands::{
            ActivateAbilityCommand, AdjustPlayerLifeEffectCommand, AdvanceTurnCommand,
            CastSpellCommand, DeclareAttackersCommand, DeclareBlockersCommand,
            DiscardForCleanupCommand, DrawCardsEffectCommand, ExileCardCommand, ModalSpellMode,
            PassPriorityCommand, PlayLandCommand, ResolveCombatDamageCommand, TapLandCommand,
        },
        errors::{DomainError, GameError},
        events::DomainEvent,
        game::{Game, Player, StackObjectKind},
        ids::{CardDefinitionId, CardInstanceId, PlayerId, StackObjectId},
        phase::Phase,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPriorityView {
    pub current_holder: PlayerId,
    pub has_pending_pass: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCardView {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub card_type: CardType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicBattlefieldCardView {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub card_type: CardType,
    pub permanent_state: PublicPermanentStateView,
    pub power: Option<u32>,
    pub toughness: Option<u32>,
    pub loyalty: Option<u32>,
    pub combat_state: PublicCombatStateView,
    pub keywords: Vec<KeywordAbility>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPermanentStateView {
    pub tapped: bool,
    pub token: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCombatStateView {
    pub summoning_sickness: bool,
    pub attacking: bool,
    pub blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicPlayerView {
    pub player_id: PlayerId,
    pub is_active: bool,
    pub life: u32,
    pub mana_total: u32,
    pub hand_count: usize,
    pub library_count: usize,
    pub battlefield: Vec<PublicBattlefieldCardView>,
    pub graveyard: Vec<PublicCardView>,
    pub exile: Vec<PublicCardView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicStackTargetView {
    Player(PlayerId),
    Card(CardInstanceId),
    StackSpell(StackObjectId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicStackObjectView {
    Spell {
        number: u32,
        controller_id: PlayerId,
        source_card_id: CardInstanceId,
        card_type: CardType,
        target: Option<PublicStackTargetView>,
        requires_choice: bool,
    },
    ActivatedAbility {
        number: u32,
        controller_id: PlayerId,
        source_card_id: CardInstanceId,
        target: Option<PublicStackTargetView>,
    },
    TriggeredAbility {
        number: u32,
        controller_id: PlayerId,
        source_card_id: CardInstanceId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicGameView {
    pub game_id: crate::domain::play::ids::GameId,
    pub active_player_id: PlayerId,
    pub phase: Phase,
    pub turn_number: u32,
    pub priority: Option<PublicPriorityView>,
    pub is_over: bool,
    pub winner_id: Option<PlayerId>,
    pub loser_id: Option<PlayerId>,
    pub end_reason: Option<crate::domain::play::events::GameEndReason>,
    pub players: Vec<PublicPlayerView>,
    pub stack: Vec<PublicStackObjectView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCastableCard {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub card_type: CardType,
    pub requires_target: bool,
    pub requires_choice: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicActivatableCard {
    pub card_id: CardInstanceId,
    pub definition_id: CardDefinitionId,
    pub requires_target: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicBlockerOption {
    pub blocker_id: CardInstanceId,
    pub attacker_ids: Vec<CardInstanceId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicLegalAction {
    PassPriority {
        player_id: PlayerId,
    },
    PlayLand {
        player_id: PlayerId,
        playable_land_ids: Vec<CardInstanceId>,
    },
    TapManaSource {
        player_id: PlayerId,
        mana_source_ids: Vec<CardInstanceId>,
    },
    CastSpell {
        player_id: PlayerId,
        castable_cards: Vec<PublicCastableCard>,
    },
    ActivateAbility {
        player_id: PlayerId,
        activatable_cards: Vec<PublicActivatableCard>,
    },
    DeclareAttackers {
        player_id: PlayerId,
        attacker_ids: Vec<CardInstanceId>,
    },
    DeclareBlockers {
        player_id: PlayerId,
        attacker_ids: Vec<CardInstanceId>,
        blocker_options: Vec<PublicBlockerOption>,
    },
    ResolveCombatDamage {
        player_id: PlayerId,
    },
    AdvanceTurn {
        player_id: PlayerId,
    },
    DiscardForCleanup {
        player_id: PlayerId,
        card_ids: Vec<CardInstanceId>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicChoiceCandidate {
    Player(PlayerId),
    Card(CardInstanceId),
    StackSpell(StackObjectId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicChoiceRequest {
    SpellTarget {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        candidates: Vec<PublicChoiceCandidate>,
    },
    SpellChoice {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        hand_card_ids: Vec<CardInstanceId>,
    },
    SpellModalChoice {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        modes: Vec<PublicModalSpellChoice>,
    },
    AbilityTarget {
        player_id: PlayerId,
        source_card_id: CardInstanceId,
        candidates: Vec<PublicChoiceCandidate>,
    },
    CleanupDiscard {
        player_id: PlayerId,
        hand_card_ids: Vec<CardInstanceId>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicModalSpellChoice {
    TargetPlayerGainLife,
    TargetPlayerLoseLife,
}

#[derive(Debug, Clone)]
pub enum PublicGameCommand {
    PlayLand(PlayLandCommand),
    TapLand(TapLandCommand),
    CastSpell(CastSpellCommand),
    ActivateAbility(ActivateAbilityCommand),
    PassPriority(PassPriorityCommand),
    DeclareAttackers(DeclareAttackersCommand),
    DeclareBlockers(DeclareBlockersCommand),
    ResolveCombatDamage(ResolveCombatDamageCommand),
    AdvanceTurn(AdvanceTurnCommand),
    DrawCardsEffect(DrawCardsEffectCommand),
    DiscardForCleanup(DiscardForCleanupCommand),
    AdjustPlayerLifeEffect(AdjustPlayerLifeEffectCommand),
    ExileCard(ExileCardCommand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicCommandRejection {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicCommandStatus {
    Applied,
    Rejected(PublicCommandRejection),
}

#[derive(Debug, Clone)]
pub struct PublicCommandResult {
    pub status: PublicCommandStatus,
    pub emitted_events: Vec<DomainEvent>,
    pub game: PublicGameView,
    pub legal_actions: Vec<PublicLegalAction>,
    pub choice_requests: Vec<PublicChoiceRequest>,
}

#[must_use]
pub fn game_view(game: &Game) -> PublicGameView {
    let players = game
        .players()
        .iter()
        .enumerate()
        .map(|(index, player)| player_view(game, index, player))
        .collect();
    let stack = game
        .stack()
        .objects()
        .iter()
        .map(|object| stack_object_view(game, object))
        .collect();

    PublicGameView {
        game_id: game.id().clone(),
        active_player_id: game.active_player().clone(),
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

#[must_use]
pub fn legal_actions(game: &Game) -> Vec<PublicLegalAction> {
    if game.is_over() {
        return Vec::new();
    }

    let mut actions = Vec::new();

    if let Some(priority) = game.priority() {
        let player_id = priority.current_holder().clone();
        actions.push(PublicLegalAction::PassPriority {
            player_id: player_id.clone(),
        });

        let playable_land_ids = playable_land_ids(game, &player_id);
        if !playable_land_ids.is_empty() {
            actions.push(PublicLegalAction::PlayLand {
                player_id: player_id.clone(),
                playable_land_ids,
            });
        }

        let mana_source_ids = tappable_mana_source_ids(game, &player_id);
        if !mana_source_ids.is_empty() {
            actions.push(PublicLegalAction::TapManaSource {
                player_id: player_id.clone(),
                mana_source_ids,
            });
        }

        let castable_cards = castable_cards(game, &player_id);
        if !castable_cards.is_empty() {
            actions.push(PublicLegalAction::CastSpell {
                player_id: player_id.clone(),
                castable_cards,
            });
        }

        let activatable_cards = activatable_cards(game, &player_id);
        if !activatable_cards.is_empty() {
            actions.push(PublicLegalAction::ActivateAbility {
                player_id,
                activatable_cards,
            });
        }

        return actions;
    }

    match game.phase() {
        Phase::DeclareAttackers => {
            let player_id = game.active_player().clone();
            actions.push(PublicLegalAction::DeclareAttackers {
                player_id: player_id.clone(),
                attacker_ids: attack_candidate_ids(game, &player_id),
            });
        }
        Phase::DeclareBlockers => {
            let player_id =
                defending_player_id(game).unwrap_or_else(|| game.active_player().clone());
            let attacker_ids = attacking_creature_ids(game, game.active_player());
            actions.push(PublicLegalAction::DeclareBlockers {
                player_id: player_id.clone(),
                attacker_ids: attacker_ids.clone(),
                blocker_options: blocker_options(game, &player_id, &attacker_ids),
            });
        }
        Phase::CombatDamage => {
            actions.push(PublicLegalAction::ResolveCombatDamage {
                player_id: game.active_player().clone(),
            });
        }
        Phase::EndStep => {
            let Some(player) = active_player(game) else {
                return actions;
            };
            if player.hand_size() > 7 {
                actions.push(PublicLegalAction::DiscardForCleanup {
                    player_id: player.id().clone(),
                    card_ids: player.hand_card_ids(),
                });
            } else {
                actions.push(PublicLegalAction::AdvanceTurn {
                    player_id: game.active_player().clone(),
                });
            }
        }
        _ => {
            actions.push(PublicLegalAction::AdvanceTurn {
                player_id: game.active_player().clone(),
            });
        }
    }

    actions
}

#[must_use]
pub fn choice_requests(game: &Game) -> Vec<PublicChoiceRequest> {
    if game.is_over() {
        return Vec::new();
    }

    let mut requests = Vec::new();

    if game.priority().is_some() {
        for action in legal_actions(game) {
            match action {
                PublicLegalAction::CastSpell {
                    player_id,
                    castable_cards,
                } => {
                    for castable in castable_cards {
                        if castable.requires_target {
                            requests.push(PublicChoiceRequest::SpellTarget {
                                player_id: player_id.clone(),
                                source_card_id: castable.card_id.clone(),
                                candidates: spell_target_candidates(
                                    game,
                                    &player_id,
                                    &castable.card_id,
                                ),
                            });
                        }
                        if castable.requires_choice {
                            if let Some(request) =
                                spell_choice_request(game, &player_id, &castable.card_id)
                            {
                                requests.push(request);
                            }
                        }
                    }
                }
                PublicLegalAction::ActivateAbility {
                    player_id,
                    activatable_cards,
                } => {
                    for activatable in activatable_cards {
                        if activatable.requires_target {
                            let source_card_id = activatable.card_id.clone();
                            requests.push(PublicChoiceRequest::AbilityTarget {
                                player_id: player_id.clone(),
                                source_card_id: source_card_id.clone(),
                                candidates: ability_target_candidates(
                                    game,
                                    &player_id,
                                    &source_card_id,
                                ),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if matches!(game.phase(), Phase::EndStep) {
        let Some(player) = active_player(game) else {
            return requests;
        };
        if player.hand_size() > 7 {
            requests.push(PublicChoiceRequest::CleanupDiscard {
                player_id: player.id().clone(),
                hand_card_ids: player.hand_card_ids(),
            });
        }
    }

    requests
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Executes a public gameplay command and returns a UI-friendly deterministic envelope.
    pub fn execute_public_command(
        &self,
        game: &mut Game,
        command: PublicGameCommand,
    ) -> PublicCommandResult {
        let result: Result<Vec<DomainEvent>, DomainError> = match command {
            PublicGameCommand::PlayLand(cmd) => {
                self.play_land(game, cmd).map(|event| vec![event.into()])
            }
            PublicGameCommand::TapLand(cmd) => self
                .tap_land(game, cmd)
                .map(|(land_tapped, mana_added)| vec![land_tapped.into(), mana_added.into()]),
            PublicGameCommand::CastSpell(cmd) => self
                .cast_spell(game, cmd)
                .map(|outcome| domain_events_for_cast_spell(&outcome)),
            PublicGameCommand::ActivateAbility(cmd) => self
                .activate_ability(game, cmd)
                .map(|outcome| domain_events_for_activate_ability(&outcome)),
            PublicGameCommand::PassPriority(cmd) => self
                .pass_priority(game, cmd)
                .map(|outcome| domain_events_for_pass_priority(&outcome)),
            PublicGameCommand::DeclareAttackers(cmd) => self
                .declare_attackers(game, cmd)
                .map(|event| vec![event.into()]),
            PublicGameCommand::DeclareBlockers(cmd) => self
                .declare_blockers(game, cmd)
                .map(|event| vec![event.into()]),
            PublicGameCommand::ResolveCombatDamage(cmd) => self
                .resolve_combat_damage(game, cmd)
                .map(|outcome| domain_events_for_resolve_combat_damage(&outcome)),
            PublicGameCommand::AdvanceTurn(cmd) => self
                .advance_turn(game, cmd)
                .map(|outcome| domain_events_for_advance_turn(&outcome)),
            PublicGameCommand::DrawCardsEffect(cmd) => self
                .draw_cards_effect(game, &cmd)
                .map(|outcome| domain_events_for_draw_cards_effect(&outcome)),
            PublicGameCommand::DiscardForCleanup(cmd) => self
                .discard_for_cleanup(game, cmd)
                .map(|event| vec![event.into()]),
            PublicGameCommand::AdjustPlayerLifeEffect(cmd) => self
                .adjust_player_life_effect(game, cmd)
                .map(|outcome| domain_events_for_adjust_player_life_effect(&outcome)),
            PublicGameCommand::ExileCard(cmd) => {
                self.exile_card(game, &cmd).map(|event| vec![event.into()])
            }
        };

        let status = match &result {
            Ok(_) => PublicCommandStatus::Applied,
            Err(err) => PublicCommandStatus::Rejected(PublicCommandRejection {
                message: err.to_string(),
            }),
        };
        let emitted_events = result.unwrap_or_default();

        PublicCommandResult {
            status,
            emitted_events,
            game: game_view(game),
            legal_actions: legal_actions(game),
            choice_requests: choice_requests(game),
        }
    }
}

fn player_view(game: &Game, index: usize, player: &Player) -> PublicPlayerView {
    PublicPlayerView {
        player_id: player.id().clone(),
        is_active: index
            == game
                .players()
                .iter()
                .position(|candidate| candidate.id() == game.active_player())
                .unwrap_or_default(),
        life: player.life(),
        mana_total: player.mana(),
        hand_count: player.hand_size(),
        library_count: player.library_size(),
        battlefield: player
            .battlefield_cards()
            .map(battlefield_card_view)
            .collect(),
        graveyard: player
            .graveyard()
            .iter()
            .filter_map(|handle| player.card_by_handle(*handle))
            .map(card_view)
            .collect(),
        exile: player
            .exile()
            .iter()
            .filter_map(|handle| player.card_by_handle(*handle))
            .map(card_view)
            .collect(),
    }
}

fn card_view(card: &CardInstance) -> PublicCardView {
    PublicCardView {
        card_id: card.id().clone(),
        definition_id: card.definition_id().clone(),
        card_type: *card.card_type(),
    }
}

fn battlefield_card_view(card: &CardInstance) -> PublicBattlefieldCardView {
    PublicBattlefieldCardView {
        card_id: card.id().clone(),
        definition_id: card.definition_id().clone(),
        card_type: *card.card_type(),
        permanent_state: PublicPermanentStateView {
            tapped: card.is_tapped(),
            token: card.is_token(),
        },
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

fn keyword_list(card: &CardInstance) -> Vec<KeywordAbility> {
    const ORDER: [KeywordAbility; 12] = [
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
    let controller_id = game.players()[object.controller_index()].id().clone();
    match object.kind() {
        StackObjectKind::Spell(spell) => PublicStackObjectView::Spell {
            number: object.number(),
            controller_id,
            source_card_id: spell.source_card_id().clone(),
            card_type: *spell.card_type(),
            target: spell
                .target()
                .and_then(|target| stack_target_view(game, *target)),
            requires_choice: spell.choice().is_some(),
        },
        StackObjectKind::ActivatedAbility(ability) => PublicStackObjectView::ActivatedAbility {
            number: object.number(),
            controller_id,
            source_card_id: ability.source_card_id(),
            target: ability
                .target()
                .and_then(|target| stack_target_view(game, *target)),
        },
        StackObjectKind::TriggeredAbility(ability) => PublicStackObjectView::TriggeredAbility {
            number: object.number(),
            controller_id,
            source_card_id: ability.source_card_id(),
        },
    }
}

fn stack_target_view(
    game: &Game,
    target: crate::domain::play::game::model::StackTargetRef,
) -> Option<PublicStackTargetView> {
    match target {
        crate::domain::play::game::model::StackTargetRef::Player(index) => Some(
            PublicStackTargetView::Player(game.players()[index].id().clone()),
        ),
        crate::domain::play::game::model::StackTargetRef::Creature(card_ref)
        | crate::domain::play::game::model::StackTargetRef::Permanent(card_ref)
        | crate::domain::play::game::model::StackTargetRef::GraveyardCard(card_ref) => {
            let card = game.players()[card_ref.owner_index()].card_by_handle(card_ref.handle())?;
            Some(PublicStackTargetView::Card(card.id().clone()))
        }
        crate::domain::play::game::model::StackTargetRef::StackSpell(number) => Some(
            PublicStackTargetView::StackSpell(StackObjectId::for_stack_object(game.id(), number)),
        ),
    }
}

fn playable_land_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .hand_card_ids()
        .into_iter()
        .filter(|card_id| {
            let mut probe = game.clone();
            probe
                .play_land(PlayLandCommand::new(player_id.clone(), card_id.clone()))
                .is_ok()
        })
        .collect()
}

fn tappable_mana_source_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .battlefield_card_ids()
        .filter(|card_id| {
            let mut probe = game.clone();
            probe
                .tap_land(TapLandCommand::new(player_id.clone(), (*card_id).clone()))
                .is_ok()
        })
        .cloned()
        .collect()
}

fn castable_cards(game: &Game, player_id: &PlayerId) -> Vec<PublicCastableCard> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    let mut candidates = player.hand_card_ids();
    candidates.extend(
        player
            .graveyard()
            .iter()
            .filter_map(|handle| player.card_by_handle(*handle))
            .map(|card| card.id().clone()),
    );

    candidates
        .into_iter()
        .filter_map(|card_id| castable_card(game, player_id, &card_id))
        .collect()
}

fn castable_card(
    game: &Game,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Option<PublicCastableCard> {
    let player = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)?;
    let card = player
        .hand_card(card_id)
        .or_else(|| player.graveyard_card(card_id))?;
    let mut probe = game.clone();
    let result = probe.cast_spell(CastSpellCommand::new(player_id.clone(), card_id.clone()));
    if result.is_ok()
        || matches!(
            result,
            Err(DomainError::Game(
                GameError::MissingSpellTarget(_) | GameError::MissingSpellChoice(_),
            ))
        )
    {
        let rules = card.supported_spell_rules();
        Some(PublicCastableCard {
            card_id: card.id().clone(),
            definition_id: card.definition_id().clone(),
            card_type: *card.card_type(),
            requires_target: rules.targeting().requires_target(),
            requires_choice: rules.requires_choice(),
        })
    } else {
        None
    }
}

fn spell_choice_request(
    game: &Game,
    player_id: &PlayerId,
    source_card_id: &CardInstanceId,
) -> Option<PublicChoiceRequest> {
    let player = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)?;
    let card = player
        .hand_card(source_card_id)
        .or_else(|| player.graveyard_card(source_card_id))?;
    let rules = card.supported_spell_rules();

    if rules.requires_explicit_hand_card_choice() {
        return Some(PublicChoiceRequest::SpellChoice {
            player_id: player_id.clone(),
            source_card_id: source_card_id.clone(),
            hand_card_ids: opponent_hand_choice_candidates(game, player_id),
        });
    }

    if rules.requires_explicit_modal_choice() {
        return Some(PublicChoiceRequest::SpellModalChoice {
            player_id: player_id.clone(),
            source_card_id: source_card_id.clone(),
            modes: vec![
                PublicModalSpellChoice::TargetPlayerGainLife,
                PublicModalSpellChoice::TargetPlayerLoseLife,
            ],
        });
    }

    None
}

impl From<ModalSpellMode> for PublicModalSpellChoice {
    fn from(value: ModalSpellMode) -> Self {
        match value {
            ModalSpellMode::TargetPlayerGainLife => Self::TargetPlayerGainLife,
            ModalSpellMode::TargetPlayerLoseLife => Self::TargetPlayerLoseLife,
        }
    }
}

fn activatable_cards(game: &Game, player_id: &PlayerId) -> Vec<PublicActivatableCard> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .battlefield_card_ids()
        .filter_map(|card_id| activatable_card(game, player_id, card_id))
        .collect()
}

fn activatable_card(
    game: &Game,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Option<PublicActivatableCard> {
    let player = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)?;
    let card = player.battlefield_card(card_id)?;
    let ability = card.activated_ability()?;
    let mut probe = game.clone();
    let result = probe.activate_ability(ActivateAbilityCommand::new(
        player_id.clone(),
        card_id.clone(),
    ));
    if result.is_ok()
        || matches!(
            result,
            Err(DomainError::Game(GameError::MissingSpellTarget(_)))
        )
    {
        Some(PublicActivatableCard {
            card_id: card.id().clone(),
            definition_id: card.definition_id().clone(),
            requires_target: ability.targeting().requires_target(),
        })
    } else {
        None
    }
}

fn attack_candidate_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .battlefield_card_ids()
        .filter(|card_id| {
            let mut probe = game.clone();
            probe
                .declare_attackers(DeclareAttackersCommand::new(
                    player_id.clone(),
                    vec![(*card_id).clone()],
                ))
                .is_ok()
        })
        .cloned()
        .collect()
}

fn attacking_creature_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    game.players()
        .iter()
        .find(|player| player.id() == player_id)
        .map(|player| {
            player
                .battlefield_cards()
                .filter(|card| card.is_attacking())
                .map(|card| card.id().clone())
                .collect()
        })
        .unwrap_or_default()
}

fn blocker_options(
    game: &Game,
    player_id: &PlayerId,
    attacker_ids: &[CardInstanceId],
) -> Vec<PublicBlockerOption> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .battlefield_card_ids()
        .map(|blocker_id| PublicBlockerOption {
            attacker_ids: attacker_ids
                .iter()
                .filter(|attacker_id| {
                    blocker_can_target_attacker(game, player_id, blocker_id, attacker_id)
                })
                .cloned()
                .collect(),
            blocker_id: blocker_id.clone(),
        })
        .filter(|option| !option.attacker_ids.is_empty())
        .collect()
}

fn blocker_can_target_attacker(
    game: &Game,
    player_id: &PlayerId,
    blocker_id: &CardInstanceId,
    attacker_id: &CardInstanceId,
) -> bool {
    let Some(defending_player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return false;
    };
    let Some(blocker) = defending_player.battlefield_card(blocker_id) else {
        return false;
    };
    let Some(attacker_owner) = game
        .players()
        .iter()
        .find(|player| player.id() == game.active_player())
    else {
        return false;
    };
    let Some(attacker) = attacker_owner.battlefield_card(attacker_id) else {
        return false;
    };

    matches!(blocker.card_type(), CardType::Creature)
        && !blocker.is_tapped()
        && matches!(attacker.card_type(), CardType::Creature)
        && (!attacker.has_flying() || blocker.has_flying() || blocker.has_reach())
}

fn spell_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    let Some(player) = game.players().iter().find(|player| player.id() == actor_id) else {
        return Vec::new();
    };
    let Some(card) = player
        .hand_card(card_id)
        .or_else(|| player.graveyard_card(card_id))
    else {
        return Vec::new();
    };
    target_candidates_for_rule(game, actor_id, card.supported_spell_rules().targeting())
}

fn ability_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    source_card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    let Some(player) = game.players().iter().find(|player| player.id() == actor_id) else {
        return Vec::new();
    };
    let Some(card) = player.battlefield_card(source_card_id) else {
        return Vec::new();
    };
    let Some(ability) = card.activated_ability() else {
        return Vec::new();
    };
    target_candidates_for_rule(game, actor_id, ability.targeting())
}

fn target_candidates_for_rule(
    game: &Game,
    actor_id: &PlayerId,
    targeting: SpellTargetingProfile,
) -> Vec<PublicChoiceCandidate> {
    let SpellTargetingProfile::ExactlyOne(rule) = targeting else {
        return Vec::new();
    };

    let mut candidates = Vec::new();
    for player in game.players() {
        if rule
            .allows_player_target(player.id() == actor_id)
            .unwrap_or(false)
        {
            candidates.push(PublicChoiceCandidate::Player(player.id().clone()));
        }
    }

    for (owner_index, player) in game.players().iter().enumerate() {
        let controlled_by_actor = player.id() == actor_id;
        for card in player.battlefield_cards() {
            if rule
                .allows_creature_target(
                    controlled_by_actor,
                    card.is_attacking(),
                    card.is_blocking(),
                )
                .unwrap_or(false)
            {
                candidates.push(PublicChoiceCandidate::Card(card.id().clone()));
                continue;
            }
            if rule
                .allows_permanent_target(*card.card_type())
                .unwrap_or(false)
            {
                let _ = owner_index;
                candidates.push(PublicChoiceCandidate::Card(card.id().clone()));
            }
        }
        for card in player
            .graveyard()
            .iter()
            .filter_map(|handle| player.card_by_handle(*handle))
        {
            if rule
                .allows_graveyard_card_target(player.id() == actor_id)
                .unwrap_or(false)
            {
                candidates.push(PublicChoiceCandidate::Card(card.id().clone()));
            }
        }
    }

    if rule.allows_stack_spell_target().unwrap_or(false) {
        candidates.extend(
            game.stack()
                .objects()
                .iter()
                .filter(|object| matches!(object.kind(), StackObjectKind::Spell(_)))
                .map(|object| {
                    PublicChoiceCandidate::StackSpell(StackObjectId::for_stack_object(
                        game.id(),
                        object.number(),
                    ))
                }),
        );
    }

    candidates
}

fn opponent_hand_choice_candidates(game: &Game, actor_id: &PlayerId) -> Vec<CardInstanceId> {
    game.players()
        .iter()
        .find(|player| player.id() != actor_id)
        .map(Player::hand_card_ids)
        .unwrap_or_default()
}

fn active_player(game: &Game) -> Option<&Player> {
    game.players()
        .iter()
        .find(|player| player.id() == game.active_player())
}

fn defending_player_id(game: &Game) -> Option<PlayerId> {
    game.players()
        .iter()
        .find(|player| player.id() != game.active_player())
        .map(|player| player.id().clone())
}
