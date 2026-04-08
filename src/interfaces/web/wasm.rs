//! Exposes a playable wasm-facing two-player duel arena adapter.

use {
    rand::random,
    serde::{Deserialize, Serialize},
    wasm_bindgen::prelude::*,
};

use crate::{
    application::{
        choice_requests, game_view, legal_actions, GameService, PublicBattlefieldCardView,
        PublicCardView, PublicChoiceCandidate, PublicChoiceRequest, PublicCommandApplication,
        PublicCommandStatus, PublicEvent, PublicEventLogEntry, PublicGameCommand, PublicGameView,
        PublicLegalAction, PublicPlayableSubsetVersion, PublicPlayerView, PublicSeededGameSetup,
        PublicSeededPlayerSetup, PublicStackObjectView, PublicStackTargetView,
    },
    domain::play::{
        cards::{
            ActivatedAbilityProfile, CardInstance, CardType, CastingRule, KeywordAbility, ManaColor,
        },
        commands::{
            ActivateAbilityCommand, AdvanceTurnCommand, CastSpellCommand, ConcedeCommand,
            DeclareAttackersCommand, DeclareBlockersCommand, DiscardForCleanupCommand, LibraryCard,
            MulliganCommand, PassPriorityCommand, PlayLandCommand, ResolveCombatDamageCommand,
            ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand,
            ResolvePendingScryCommand, ResolvePendingSurveilCommand, TapLandCommand,
        },
        game::{Game, Player},
        ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
        phase::Phase,
    },
    infrastructure::{InMemoryEventBus, InMemoryEventStore},
};

#[wasm_bindgen]
pub struct WebArenaClient {
    service: GameService<InMemoryEventStore, InMemoryEventBus>,
    game: Game,
    duel_setup: PublicSeededGameSetup,
    viewer_ids: Vec<PlayerId>,
    pregame: Option<ArenaPregameController>,
}

#[derive(Serialize)]
struct WebArenaState {
    game: WebGameView,
    pregame: Option<WebPregameState>,
    viewers: Vec<WebViewerState>,
    event_log: Vec<WebTimelineEntry>,
    last_command: Option<WebCommandFeedback>,
}

#[derive(Serialize)]
struct WebViewerState {
    player_id: String,
    is_active: bool,
    is_priority_holder: bool,
    mulligan_used: bool,
    hand: Vec<WebHandCard>,
    legal_actions: Vec<WebLegalAction>,
    choice_requests: Vec<WebChoicePrompt>,
}

#[derive(Serialize)]
struct WebPregameState {
    starting_player_id: String,
    current_decision_player_id: String,
    kept_player_ids: Vec<String>,
}

#[derive(Serialize)]
struct WebGameView {
    game_id: String,
    playable_subset_version: String,
    active_player_id: Option<String>,
    phase: String,
    turn_number: u32,
    priority_holder: Option<String>,
    priority_has_pending_pass: Option<bool>,
    is_over: bool,
    winner_id: Option<String>,
    loser_id: Option<String>,
    end_reason: Option<String>,
    players: Vec<WebPlayerView>,
    stack: Vec<WebStackObject>,
}

#[derive(Serialize)]
struct WebPlayerView {
    player_id: String,
    is_active: bool,
    life: u32,
    mana_total: u32,
    mana_pool: WebManaPoolView,
    hand_count: usize,
    library_count: usize,
    battlefield: Vec<WebBattlefieldCard>,
    graveyard: Vec<WebCardView>,
    exile: Vec<WebCardView>,
}

#[derive(Serialize)]
struct WebManaPoolView {
    colorless: u32,
    white: u32,
    blue: u32,
    black: u32,
    red: u32,
    green: u32,
}

#[derive(Serialize)]
struct WebManaCostView {
    generic: u32,
    white: u32,
    blue: u32,
    black: u32,
    red: u32,
    green: u32,
}

#[derive(Serialize)]
struct WebCardView {
    card_id: String,
    definition_id: String,
    card_type: String,
    mana_cost: WebManaCostView,
}

#[derive(Serialize)]
struct WebHandCard {
    card_id: String,
    definition_id: String,
    card_type: String,
    mana_cost: u32,
    mana_cost_profile: WebManaCostView,
    power: Option<u32>,
    toughness: Option<u32>,
    loyalty: Option<u32>,
    keywords: Vec<String>,
    requires_target: bool,
    requires_choice: bool,
    has_activated_ability: bool,
    can_cast_in_open_priority: bool,
    can_cast_in_open_priority_during_own_turn: bool,
}

#[derive(Serialize)]
struct WebBattlefieldCard {
    card_id: String,
    definition_id: String,
    card_type: String,
    mana_cost: WebManaCostView,
    tapped: bool,
    token: bool,
    attached_to: Option<String>,
    power: Option<u32>,
    toughness: Option<u32>,
    loyalty: Option<u32>,
    summoning_sickness: bool,
    attacking: bool,
    blocking: bool,
    keywords: Vec<String>,
}

#[derive(Serialize)]
struct WebStackObject {
    number: u32,
    kind: String,
    controller_id: Option<String>,
    source_card_id: Option<String>,
    definition_id: Option<String>,
    card_type: Option<String>,
    target: Option<String>,
    requires_choice: bool,
}

#[derive(Serialize)]
struct WebLegalAction {
    kind: String,
    player_id: String,
    summary: String,
    card_ids: Vec<String>,
    blocker_options: Vec<WebBlockerOption>,
}

#[derive(Serialize)]
struct WebBlockerOption {
    blocker_id: String,
    attacker_ids: Vec<String>,
}

#[derive(Serialize)]
struct WebChoicePrompt {
    kind: String,
    player_id: String,
    source_card_id: Option<String>,
    summary: String,
    item_ids: Vec<String>,
    options: Vec<String>,
}

#[derive(Serialize)]
struct WebTimelineEntry {
    sequence: u64,
    label: String,
}

#[derive(Serialize)]
struct WebCommandFeedback {
    applied: bool,
    message: String,
    emitted_events: Vec<String>,
}

#[derive(Deserialize)]
struct WebBlockerAssignmentInput {
    blocker_id: String,
    attacker_id: String,
}

struct ArenaPregameController {
    starting_player_id: PlayerId,
    decision_order: Vec<PlayerId>,
    kept_player_ids: Vec<PlayerId>,
    current_decision_index: usize,
}

#[wasm_bindgen]
impl WebArenaClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebArenaClient, JsValue> {
        let service = GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new());
        let duel_setup = duel_setup();
        let viewer_ids = duel_setup
            .players
            .iter()
            .map(|player| player.player_id.clone())
            .collect::<Vec<_>>();
        let initial_viewer = viewer_ids
            .first()
            .cloned()
            .ok_or_else(|| JsValue::from_str("duel setup must include at least one player"))?;
        let (game, pregame) = create_setup_duel(&service, duel_setup.clone(), &initial_viewer)?;

        Ok(Self {
            service,
            game,
            duel_setup,
            viewer_ids,
            pregame: Some(pregame),
        })
    }

    pub fn state(&self) -> Result<JsValue, JsValue> {
        self.project_state(None)
    }

    pub fn reset(&mut self) -> Result<JsValue, JsValue> {
        let initial_viewer = self
            .viewer_ids
            .first()
            .cloned()
            .ok_or_else(|| JsValue::from_str("duel setup must include at least one player"))?;
        let (game, pregame) =
            create_setup_duel(&self.service, self.duel_setup.clone(), &initial_viewer)?;
        self.game = game;
        self.pregame = Some(pregame);
        self.project_state(None)
    }

    pub fn mulligan(&mut self, player_id: &str) -> Result<JsValue, JsValue> {
        self.ensure_pregame_decision_holder(player_id)?;
        let player_id = PlayerId::new(player_id);
        let event = self
            .service
            .mulligan(&mut self.game, MulliganCommand::new(player_id.clone()))
            .map_err(domain_error_to_js)?;

        self.project_state(Some(WebCommandFeedback {
            applied: true,
            message: "Mulligan applied".to_string(),
            emitted_events: vec![format!("{} mulliganed", event.player_id.as_str())],
        }))
    }

    pub fn keep_opening_hand(&mut self, player_id: &str) -> Result<JsValue, JsValue> {
        self.ensure_pregame_decision_holder(player_id)?;
        let player_id = PlayerId::new(player_id);
        let pregame_completed = self
            .pregame
            .as_mut()
            .ok_or_else(|| JsValue::from_str("pregame controller is unavailable"))?
            .keep(&player_id)?;

        if pregame_completed {
            self.pregame = None;
            advance_to_first_main(&self.service, &mut self.game)?;
        }

        self.project_state(Some(WebCommandFeedback {
            applied: true,
            message: "Opening hand kept".to_string(),
            emitted_events: vec![format!("{} kept the opening hand", player_id.as_str())],
        }))
    }

    pub fn pass_priority(&mut self, player_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::PassPriority(PassPriorityCommand::new(
            PlayerId::new(player_id),
        )))
    }

    pub fn advance_turn(&mut self) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::AdvanceTurn(AdvanceTurnCommand::new()))
    }

    pub fn concede(&mut self, player_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::Concede(ConcedeCommand::new(
            PlayerId::new(player_id),
        )))
    }

    pub fn play_land(&mut self, player_id: &str, card_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::PlayLand(PlayLandCommand::new(
            PlayerId::new(player_id),
            CardInstanceId::new(card_id),
        )))
    }

    pub fn tap_mana_source(&mut self, player_id: &str, card_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::TapLand(TapLandCommand::new(
            PlayerId::new(player_id),
            CardInstanceId::new(card_id),
        )))
    }

    pub fn cast_spell(&mut self, player_id: &str, card_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::CastSpell(CastSpellCommand::new(
            PlayerId::new(player_id),
            CardInstanceId::new(card_id),
        )))
    }

    pub fn activate_ability(
        &mut self,
        player_id: &str,
        source_card_id: &str,
    ) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::ActivateAbility(
            ActivateAbilityCommand::new(
                PlayerId::new(player_id),
                CardInstanceId::new(source_card_id),
            ),
        ))
    }

    pub fn declare_attackers(
        &mut self,
        player_id: &str,
        attacker_ids: JsValue,
    ) -> Result<JsValue, JsValue> {
        let attacker_ids = serde_wasm_bindgen::from_value::<Vec<String>>(attacker_ids)
            .map_err(|err| JsValue::from_str(&format!("failed to decode attacker ids: {err}")))?;
        self.apply_command(PublicGameCommand::DeclareAttackers(
            DeclareAttackersCommand::new(
                PlayerId::new(player_id),
                attacker_ids.into_iter().map(CardInstanceId::new).collect(),
            ),
        ))
    }

    pub fn declare_blockers(
        &mut self,
        player_id: &str,
        blocker_assignments: JsValue,
    ) -> Result<JsValue, JsValue> {
        let blocker_assignments =
            serde_wasm_bindgen::from_value::<Vec<WebBlockerAssignmentInput>>(blocker_assignments)
                .map_err(|err| {
                JsValue::from_str(&format!("failed to decode blocker assignments: {err}"))
            })?;
        self.apply_command(PublicGameCommand::DeclareBlockers(
            DeclareBlockersCommand::new(
                PlayerId::new(player_id),
                blocker_assignments
                    .into_iter()
                    .map(|assignment| {
                        (
                            CardInstanceId::new(assignment.blocker_id),
                            CardInstanceId::new(assignment.attacker_id),
                        )
                    })
                    .collect(),
            ),
        ))
    }

    pub fn resolve_combat_damage(&mut self, player_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::ResolveCombatDamage(
            ResolveCombatDamageCommand::new(PlayerId::new(player_id)),
        ))
    }

    pub fn discard_for_cleanup(
        &mut self,
        player_id: &str,
        card_id: &str,
    ) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::DiscardForCleanup(
            DiscardForCleanupCommand::new(PlayerId::new(player_id), CardInstanceId::new(card_id)),
        ))
    }

    pub fn resolve_optional_effect(
        &mut self,
        player_id: &str,
        accept: bool,
    ) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::ResolveOptionalEffect(
            ResolveOptionalEffectCommand::new(PlayerId::new(player_id), accept),
        ))
    }

    pub fn resolve_pending_hand_choice(
        &mut self,
        player_id: &str,
        card_id: &str,
    ) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::ResolvePendingHandChoice(
            ResolvePendingHandChoiceCommand::new(
                PlayerId::new(player_id),
                CardInstanceId::new(card_id),
            ),
        ))
    }

    pub fn resolve_pending_scry(
        &mut self,
        player_id: &str,
        move_to_bottom: bool,
    ) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::ResolvePendingScry(
            ResolvePendingScryCommand::new(PlayerId::new(player_id), move_to_bottom),
        ))
    }

    pub fn resolve_pending_surveil(
        &mut self,
        player_id: &str,
        move_to_graveyard: bool,
    ) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::ResolvePendingSurveil(
            ResolvePendingSurveilCommand::new(PlayerId::new(player_id), move_to_graveyard),
        ))
    }
}

impl WebArenaClient {
    fn apply_command(&mut self, command: PublicGameCommand) -> Result<JsValue, JsValue> {
        let application = self.service.execute_public_command(&mut self.game, command);
        let feedback = web_command_feedback(&application);
        self.project_state(Some(feedback))
    }

    fn project_state(&self, last_command: Option<WebCommandFeedback>) -> Result<JsValue, JsValue> {
        let game = game_view(&self.game);
        let event_log = self
            .service
            .public_event_log(self.game.id())
            .map_err(domain_error_to_js)?;
        let viewers = self
            .viewer_ids
            .iter()
            .map(|viewer_id| self.web_viewer_state(viewer_id))
            .collect::<Result<Vec<_>, _>>()?;

        serde_wasm_bindgen::to_value(&WebArenaState {
            game: web_game_view(&game),
            pregame: self
                .pregame
                .as_ref()
                .and_then(ArenaPregameController::web_state),
            viewers,
            event_log: event_log.iter().map(web_timeline_entry).collect(),
            last_command,
        })
        .map_err(|err| {
            JsValue::from_str(&format!("failed to serialize web duel arena state: {err}"))
        })
    }

    fn web_viewer_state(&self, viewer_id: &PlayerId) -> Result<WebViewerState, JsValue> {
        let player = player_by_id(&self.game, viewer_id).ok_or_else(|| {
            JsValue::from_str(&format!(
                "viewer {} is missing from the in-memory duel state",
                viewer_id.as_str()
            ))
        })?;

        Ok(WebViewerState {
            player_id: id_string(viewer_id),
            is_active: self.game.active_player() == viewer_id,
            is_priority_holder: self
                .game
                .priority()
                .is_some_and(|priority| priority.current_holder() == viewer_id),
            mulligan_used: player.mulligan_used(),
            hand: player.hand_cards().map(web_hand_card).collect(),
            legal_actions: legal_actions(&self.game, viewer_id)
                .iter()
                .map(web_legal_action)
                .collect(),
            choice_requests: choice_requests(&self.game, viewer_id)
                .iter()
                .map(web_choice_prompt)
                .collect(),
        })
    }

    fn ensure_pregame_decision_holder(&self, player_id: &str) -> Result<(), JsValue> {
        if *self.game.phase() != Phase::Setup {
            return Err(JsValue::from_str(
                "opening hand decisions are only available during Setup",
            ));
        }

        let current_decision_player_id = self
            .pregame
            .as_ref()
            .and_then(ArenaPregameController::current_decision_player_id)
            .ok_or_else(|| JsValue::from_str("pregame decisions are already complete"))?;

        if current_decision_player_id.as_str() == player_id {
            return Ok(());
        }

        Err(JsValue::from_str(&format!(
            "{} is waiting for {} to decide the opening hand",
            player_id,
            current_decision_player_id.as_str()
        )))
    }
}

impl ArenaPregameController {
    fn new(player_ids: &[PlayerId]) -> Result<Self, JsValue> {
        let mut decision_order = player_ids.to_vec();
        if decision_order.is_empty() {
            return Err(JsValue::from_str(
                "pregame controller requires at least one player",
            ));
        }

        if decision_order.len() == 2 && random::<bool>() {
            decision_order.rotate_left(1);
        }

        Ok(Self {
            starting_player_id: decision_order[0].clone(),
            decision_order,
            kept_player_ids: Vec::new(),
            current_decision_index: 0,
        })
    }

    fn current_decision_player_id(&self) -> Option<&PlayerId> {
        self.decision_order.get(self.current_decision_index)
    }

    fn keep(&mut self, player_id: &PlayerId) -> Result<bool, JsValue> {
        self.ensure_current_decision_holder(player_id)?;
        if self.kept_player_ids.contains(player_id) {
            return Err(JsValue::from_str(&format!(
                "{} already kept the opening hand",
                player_id.as_str()
            )));
        }

        self.kept_player_ids.push(player_id.clone());
        self.current_decision_index += 1;
        Ok(self.current_decision_player_id().is_none())
    }

    fn web_state(&self) -> Option<WebPregameState> {
        self.current_decision_player_id()
            .map(|current_decision_player_id| WebPregameState {
                starting_player_id: id_string(&self.starting_player_id),
                current_decision_player_id: id_string(current_decision_player_id),
                kept_player_ids: self.kept_player_ids.iter().map(id_string).collect(),
            })
    }

    fn ensure_current_decision_holder(&self, player_id: &PlayerId) -> Result<(), JsValue> {
        let current = self
            .current_decision_player_id()
            .ok_or_else(|| JsValue::from_str("pregame decisions are already complete"))?;

        if current == player_id {
            return Ok(());
        }

        Err(JsValue::from_str(&format!(
            "{} is waiting for {} to decide the opening hand",
            player_id.as_str(),
            current.as_str()
        )))
    }
}

fn advance_to_first_main(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    game: &mut Game,
) -> Result<(), JsValue> {
    for _ in 0..32 {
        if *game.phase() == Phase::FirstMain && game.priority().is_some() {
            return Ok(());
        }

        let application = if let Some(priority) = game.priority() {
            service.execute_public_command(
                game,
                PublicGameCommand::PassPriority(PassPriorityCommand::new(
                    priority.current_holder().clone(),
                )),
            )
        } else {
            service.execute_public_command(
                game,
                PublicGameCommand::AdvanceTurn(AdvanceTurnCommand::new()),
            )
        };

        if let PublicCommandStatus::Rejected(rejection) = application.status {
            return Err(JsValue::from_str(&format!(
                "failed to prepare duel arena state: {}",
                rejection.message
            )));
        }
    }

    Err(JsValue::from_str(
        "failed to prepare duel arena state before reaching FirstMain",
    ))
}

fn create_setup_duel(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    duel_setup: PublicSeededGameSetup,
    initial_viewer: &PlayerId,
) -> Result<(Game, ArenaPregameController), JsValue> {
    let (mut game, _) = service
        .start_seeded_public_game(duel_setup, initial_viewer)
        .map_err(domain_error_to_js)?;
    let player_ids = game
        .players()
        .iter()
        .map(|player| player.id().clone())
        .collect::<Vec<_>>();
    let pregame = ArenaPregameController::new(&player_ids)?;
    set_active_player(&mut game, &pregame.starting_player_id)?;
    Ok((game, pregame))
}

fn set_active_player(game: &mut Game, player_id: &PlayerId) -> Result<(), JsValue> {
    let active_player_index = game
        .players()
        .iter()
        .position(|player| player.id() == player_id)
        .ok_or_else(|| {
            JsValue::from_str(&format!(
                "failed to prepare duel arena state for {}",
                player_id.as_str()
            ))
        })?;
    game.replace_active_player_index(active_player_index);
    Ok(())
}

fn duel_setup() -> PublicSeededGameSetup {
    PublicSeededGameSetup::new(
        GameId::new("web-duel-arena"),
        vec![
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-1"),
                DeckId::new("deck-1"),
                duel_library("p1"),
            ),
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-2"),
                DeckId::new("deck-2"),
                duel_library("p2"),
            ),
        ],
        11,
    )
}

fn duel_library(prefix: &str) -> Vec<LibraryCard> {
    vec![
        basic_forest(),
        basic_forest(),
        basic_forest(),
        basic_forest(),
        basic_forest(),
        basic_forest(),
        LibraryCard::creature(CardDefinitionId::new("duel-scout"), 1, 2, 1),
        LibraryCard::creature(CardDefinitionId::new("duel-bear"), 2, 2, 2),
        LibraryCard::creature(CardDefinitionId::new("duel-warden"), 2, 1, 4),
        LibraryCard::creature(CardDefinitionId::new("duel-brute"), 3, 3, 3),
        LibraryCard::creature(
            CardDefinitionId::new(format!("{prefix}-duel-flash-sprinter")),
            2,
            2,
            1,
        )
        .with_casting_rule(CastingRule::OpenPriorityWindow),
        LibraryCard::new(
            CardDefinitionId::new(format!("{prefix}-duel-relic")),
            CardType::Artifact,
            1,
        )
        .with_activated_ability(ActivatedAbilityProfile::tap_to_gain_life_to_controller(1)),
    ]
}

fn basic_forest() -> LibraryCard {
    LibraryCard::land(CardDefinitionId::new("forest"), ManaColor::Green)
}

fn player_by_id<'a>(game: &'a Game, player_id: &PlayerId) -> Option<&'a Player> {
    game.players()
        .iter()
        .find(|player| player.id() == player_id)
}

fn web_game_view(game: &PublicGameView) -> WebGameView {
    WebGameView {
        game_id: game.game_id.as_str().to_string(),
        playable_subset_version: match game.playable_subset_version {
            PublicPlayableSubsetVersion::V1 => "v1".to_string(),
        },
        active_player_id: game.active_player_id.as_ref().map(id_string),
        phase: format!("{:?}", game.phase),
        turn_number: game.turn_number,
        priority_holder: game
            .priority
            .as_ref()
            .map(|priority| id_string(&priority.current_holder)),
        priority_has_pending_pass: game
            .priority
            .as_ref()
            .map(|priority| priority.has_pending_pass),
        is_over: game.is_over,
        winner_id: game.winner_id.as_ref().map(id_string),
        loser_id: game.loser_id.as_ref().map(id_string),
        end_reason: game.end_reason.map(|reason| format!("{reason:?}")),
        players: game.players.iter().map(web_player_view).collect(),
        stack: game.stack.iter().map(web_stack_object).collect(),
    }
}

fn web_player_view(player: &PublicPlayerView) -> WebPlayerView {
    WebPlayerView {
        player_id: id_string(&player.player_id),
        is_active: player.is_active,
        life: player.life,
        mana_total: player.mana_total,
        mana_pool: WebManaPoolView {
            colorless: player.mana_pool.colorless,
            white: player.mana_pool.white,
            blue: player.mana_pool.blue,
            black: player.mana_pool.black,
            red: player.mana_pool.red,
            green: player.mana_pool.green,
        },
        hand_count: player.hand_count,
        library_count: player.library_count,
        battlefield: player
            .battlefield
            .iter()
            .map(web_battlefield_card)
            .collect(),
        graveyard: player.graveyard.iter().map(web_card_view).collect(),
        exile: player.exile.iter().map(web_card_view).collect(),
    }
}

fn web_card_view(card: &PublicCardView) -> WebCardView {
    WebCardView {
        card_id: id_string(&card.card_id),
        definition_id: id_string(&card.definition_id),
        card_type: format!("{:?}", card.card_type),
        mana_cost: web_public_mana_cost_view(&card.mana_cost),
    }
}

fn web_hand_card(card: &CardInstance) -> WebHandCard {
    let casting_permission = card.casting_permission_profile();
    let supported_spell_rules = card.supported_spell_rules();
    let (power, toughness) = card
        .creature_stats()
        .map_or((None, None), |(power, toughness)| {
            (Some(power), Some(toughness))
        });

    WebHandCard {
        card_id: id_string(card.id()),
        definition_id: id_string(card.definition_id()),
        card_type: format!("{:?}", card.card_type()),
        mana_cost: card.mana_cost(),
        mana_cost_profile: web_runtime_mana_cost_view(card.mana_cost_profile()),
        power,
        toughness,
        loyalty: card.loyalty(),
        keywords: hand_card_keywords(card),
        requires_target: supported_spell_rules.targeting().requires_target(),
        requires_choice: supported_spell_rules.requires_choice(),
        has_activated_ability: card.activated_ability().is_some(),
        can_cast_in_open_priority: casting_permission
            .is_some_and(|permission| permission.supports(CastingRule::OpenPriorityWindow)),
        can_cast_in_open_priority_during_own_turn: casting_permission.is_some_and(|permission| {
            permission.supports(CastingRule::OpenPriorityWindowDuringOwnTurn)
        }),
    }
}

fn hand_card_keywords(card: &CardInstance) -> Vec<String> {
    all_keyword_abilities()
        .into_iter()
        .filter(|ability| card.has_keyword(*ability))
        .map(|ability| format!("{ability:?}"))
        .collect()
}

fn all_keyword_abilities() -> [KeywordAbility; 13] {
    [
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
    ]
}

fn web_battlefield_card(card: &PublicBattlefieldCardView) -> WebBattlefieldCard {
    WebBattlefieldCard {
        card_id: id_string(&card.card_id),
        definition_id: id_string(&card.definition_id),
        card_type: format!("{:?}", card.card_type),
        mana_cost: web_public_mana_cost_view(&card.mana_cost),
        tapped: card.permanent_state.tapped,
        token: card.permanent_state.token,
        attached_to: card.attached_to.as_ref().map(id_string),
        power: card.power,
        toughness: card.toughness,
        loyalty: card.loyalty,
        summoning_sickness: card.combat_state.summoning_sickness,
        attacking: card.combat_state.attacking,
        blocking: card.combat_state.blocking,
        keywords: card
            .keywords
            .iter()
            .map(|keyword| format!("{keyword:?}"))
            .collect(),
    }
}

fn web_stack_object(object: &PublicStackObjectView) -> WebStackObject {
    match object {
        PublicStackObjectView::Unavailable { number } => WebStackObject {
            number: *number,
            kind: "Unavailable".to_string(),
            controller_id: None,
            source_card_id: None,
            definition_id: None,
            card_type: None,
            target: None,
            requires_choice: false,
        },
        PublicStackObjectView::Spell {
            number,
            controller_id,
            source_card_id,
            definition_id,
            card_type,
            target,
            requires_choice,
        } => WebStackObject {
            number: *number,
            kind: "Spell".to_string(),
            controller_id: Some(id_string(controller_id)),
            source_card_id: Some(id_string(source_card_id)),
            definition_id: Some(id_string(definition_id)),
            card_type: Some(format!("{card_type:?}")),
            target: target.as_ref().map(web_stack_target_label),
            requires_choice: *requires_choice,
        },
        PublicStackObjectView::ActivatedAbility {
            number,
            controller_id,
            source_card_id,
            definition_id,
            card_type,
            target,
        } => WebStackObject {
            number: *number,
            kind: "ActivatedAbility".to_string(),
            controller_id: Some(id_string(controller_id)),
            source_card_id: Some(id_string(source_card_id)),
            definition_id: definition_id.as_ref().map(id_string),
            card_type: card_type.map(|card_type| format!("{card_type:?}")),
            target: target.as_ref().map(web_stack_target_label),
            requires_choice: false,
        },
        PublicStackObjectView::TriggeredAbility {
            number,
            controller_id,
            source_card_id,
            definition_id,
            card_type,
        } => WebStackObject {
            number: *number,
            kind: "TriggeredAbility".to_string(),
            controller_id: Some(id_string(controller_id)),
            source_card_id: Some(id_string(source_card_id)),
            definition_id: definition_id.as_ref().map(id_string),
            card_type: card_type.map(|card_type| format!("{card_type:?}")),
            target: None,
            requires_choice: false,
        },
    }
}

fn web_public_mana_cost_view(
    mana_cost: &crate::application::PublicManaCostView,
) -> WebManaCostView {
    WebManaCostView {
        generic: mana_cost.generic,
        white: mana_cost.white,
        blue: mana_cost.blue,
        black: mana_cost.black,
        red: mana_cost.red,
        green: mana_cost.green,
    }
}

fn web_runtime_mana_cost_view(mana_cost: crate::domain::play::cards::ManaCost) -> WebManaCostView {
    WebManaCostView {
        generic: mana_cost.generic_requirement(),
        white: mana_cost.white_requirement(),
        blue: mana_cost.blue_requirement(),
        black: mana_cost.black_requirement(),
        red: mana_cost.red_requirement(),
        green: mana_cost.green_requirement(),
    }
}

fn web_stack_target_label(target: &PublicStackTargetView) -> String {
    match target {
        PublicStackTargetView::Unavailable => "Unavailable".to_string(),
        PublicStackTargetView::Player(player_id) => format!("Player {}", player_id.as_str()),
        PublicStackTargetView::Card(card_id) => format!("Card {}", card_id.as_str()),
        PublicStackTargetView::StackSpell(stack_object_id) => {
            format!("Stack {}", stack_object_id.as_str())
        }
    }
}

fn web_legal_action(action: &PublicLegalAction) -> WebLegalAction {
    match action {
        PublicLegalAction::Concede { player_id } => WebLegalAction {
            kind: "Concede".to_string(),
            player_id: id_string(player_id),
            summary: "Concede the game".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::ResolvePendingScry { player_id } => WebLegalAction {
            kind: "ResolvePendingScry".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve pending scry".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::ResolvePendingSurveil { player_id } => WebLegalAction {
            kind: "ResolvePendingSurveil".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve pending surveil".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::ResolvePendingHandChoice { player_id } => WebLegalAction {
            kind: "ResolvePendingHandChoice".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve pending hand choice".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::ResolveOptionalEffect { player_id } => WebLegalAction {
            kind: "ResolveOptionalEffect".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve optional effect".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::PassPriority { player_id } => WebLegalAction {
            kind: "PassPriority".to_string(),
            player_id: id_string(player_id),
            summary: "Pass priority".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::PlayLand {
            player_id,
            playable_land_ids,
        } => WebLegalAction {
            kind: "PlayLand".to_string(),
            player_id: id_string(player_id),
            summary: "Play a land".to_string(),
            card_ids: playable_land_ids.iter().map(id_string).collect(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::TapManaSource {
            player_id,
            mana_source_ids,
        } => WebLegalAction {
            kind: "TapManaSource".to_string(),
            player_id: id_string(player_id),
            summary: "Tap a mana source".to_string(),
            card_ids: mana_source_ids.iter().map(id_string).collect(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::CastSpell {
            player_id,
            castable_cards,
        } => WebLegalAction {
            kind: "CastSpell".to_string(),
            player_id: id_string(player_id),
            summary: "Cast a spell".to_string(),
            card_ids: castable_cards
                .iter()
                .map(|card| id_string(&card.card_id))
                .collect(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::ActivateAbility {
            player_id,
            activatable_cards,
        } => WebLegalAction {
            kind: "ActivateAbility".to_string(),
            player_id: id_string(player_id),
            summary: "Activate an ability".to_string(),
            card_ids: activatable_cards
                .iter()
                .map(|card| id_string(&card.card_id))
                .collect(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::DeclareAttackers {
            player_id,
            attacker_ids,
        } => WebLegalAction {
            kind: "DeclareAttackers".to_string(),
            player_id: id_string(player_id),
            summary: "Declare attackers".to_string(),
            card_ids: attacker_ids.iter().map(id_string).collect(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::DeclareBlockers {
            player_id,
            attacker_ids,
            blocker_options,
        } => WebLegalAction {
            kind: "DeclareBlockers".to_string(),
            player_id: id_string(player_id),
            summary: format!("Declare blockers across {} options", blocker_options.len()),
            card_ids: attacker_ids.iter().map(id_string).collect(),
            blocker_options: blocker_options
                .iter()
                .map(|option| WebBlockerOption {
                    blocker_id: id_string(&option.blocker_id),
                    attacker_ids: option.attacker_ids.iter().map(id_string).collect(),
                })
                .collect(),
        },
        PublicLegalAction::ResolveCombatDamage { player_id } => WebLegalAction {
            kind: "ResolveCombatDamage".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve combat damage".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::AdvanceTurn { player_id } => WebLegalAction {
            kind: "AdvanceTurn".to_string(),
            player_id: id_string(player_id),
            summary: "Advance turn".to_string(),
            card_ids: Vec::new(),
            blocker_options: Vec::new(),
        },
        PublicLegalAction::DiscardForCleanup {
            player_id,
            card_ids,
        } => WebLegalAction {
            kind: "DiscardForCleanup".to_string(),
            player_id: id_string(player_id),
            summary: "Discard for cleanup".to_string(),
            card_ids: card_ids.iter().map(id_string).collect(),
            blocker_options: Vec::new(),
        },
    }
}

fn web_choice_prompt(request: &PublicChoiceRequest) -> WebChoicePrompt {
    match request {
        PublicChoiceRequest::PhaseUnavailable { player_id, phase } => WebChoicePrompt {
            kind: "PhaseUnavailable".to_string(),
            player_id: id_string(player_id),
            source_card_id: None,
            summary: format!("Phase {phase:?} is unavailable"),
            item_ids: Vec::new(),
            options: Vec::new(),
        },
        PublicChoiceRequest::PriorityUnavailable { player_id } => WebChoicePrompt {
            kind: "PriorityUnavailable".to_string(),
            player_id: id_string(player_id),
            source_card_id: None,
            summary: "Priority surface unavailable".to_string(),
            item_ids: Vec::new(),
            options: Vec::new(),
        },
        PublicChoiceRequest::PendingDecisionUnavailable {
            player_id,
            decision,
        } => WebChoicePrompt {
            kind: "PendingDecisionUnavailable".to_string(),
            player_id: id_string(player_id),
            source_card_id: None,
            summary: format!("Pending decision {decision:?} unavailable"),
            item_ids: Vec::new(),
            options: Vec::new(),
        },
        PublicChoiceRequest::PendingScry {
            player_id,
            source_card_id,
            looked_at_card_ids,
            ..
        } => WebChoicePrompt {
            kind: "PendingScry".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Resolve scry".to_string(),
            item_ids: looked_at_card_ids.iter().map(id_string).collect(),
            options: vec!["KeepOnTop".to_string(), "MoveToBottom".to_string()],
        },
        PublicChoiceRequest::PendingSurveil {
            player_id,
            source_card_id,
            looked_at_card_ids,
            ..
        } => WebChoicePrompt {
            kind: "PendingSurveil".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Resolve surveil".to_string(),
            item_ids: looked_at_card_ids.iter().map(id_string).collect(),
            options: vec!["KeepOnTop".to_string(), "MoveToGraveyard".to_string()],
        },
        PublicChoiceRequest::PendingHandChoice {
            player_id,
            source_card_id,
            hand_card_ids,
        } => WebChoicePrompt {
            kind: "PendingHandChoice".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose a hand card".to_string(),
            item_ids: hand_card_ids.iter().map(id_string).collect(),
            options: Vec::new(),
        },
        PublicChoiceRequest::OptionalEffectDecision {
            player_id,
            source_card_id,
            ..
        } => WebChoicePrompt {
            kind: "OptionalEffectDecision".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose yes or no".to_string(),
            item_ids: Vec::new(),
            options: vec!["Yes".to_string(), "No".to_string()],
        },
        PublicChoiceRequest::SpellTarget {
            player_id,
            source_card_id,
            candidates,
        } => WebChoicePrompt {
            kind: "SpellTarget".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose a spell target".to_string(),
            item_ids: candidates.iter().map(web_choice_candidate_id).collect(),
            options: Vec::new(),
        },
        PublicChoiceRequest::SpellChoiceInvariantViolation {
            player_id,
            source_card_id,
        } => WebChoicePrompt {
            kind: "SpellChoiceInvariantViolation".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Spell choice lookup failed".to_string(),
            item_ids: Vec::new(),
            options: Vec::new(),
        },
        PublicChoiceRequest::SpellChoice {
            player_id,
            source_card_id,
            hand_card_ids,
        } => WebChoicePrompt {
            kind: "SpellChoice".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose a card from hand".to_string(),
            item_ids: hand_card_ids.iter().map(id_string).collect(),
            options: Vec::new(),
        },
        PublicChoiceRequest::SpellSecondaryCreatureChoiceUnavailable {
            player_id,
            source_card_id,
        } => WebChoicePrompt {
            kind: "SpellSecondaryCreatureChoiceUnavailable".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Secondary creature choice unavailable".to_string(),
            item_ids: Vec::new(),
            options: Vec::new(),
        },
        PublicChoiceRequest::SpellSecondaryCreatureChoice {
            player_id,
            source_card_id,
            creature_ids,
            allows_skipping,
        } => WebChoicePrompt {
            kind: "SpellSecondaryCreatureChoice".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose secondary creature targets".to_string(),
            item_ids: creature_ids.iter().map(id_string).collect(),
            options: allows_skipping
                .then_some(vec!["Skip".to_string()])
                .unwrap_or_default(),
        },
        PublicChoiceRequest::SpellModalChoice {
            player_id,
            source_card_id,
            modes,
        } => WebChoicePrompt {
            kind: "SpellModalChoice".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose a spell mode".to_string(),
            item_ids: Vec::new(),
            options: modes.iter().map(|mode| format!("{mode:?}")).collect(),
        },
        PublicChoiceRequest::AbilityTarget {
            player_id,
            source_card_id,
            candidates,
        } => WebChoicePrompt {
            kind: "AbilityTarget".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose an ability target".to_string(),
            item_ids: candidates.iter().map(web_choice_candidate_id).collect(),
            options: Vec::new(),
        },
        PublicChoiceRequest::CleanupDiscard {
            player_id,
            hand_card_ids,
        } => WebChoicePrompt {
            kind: "CleanupDiscard".to_string(),
            player_id: id_string(player_id),
            source_card_id: None,
            summary: "Discard down to hand size".to_string(),
            item_ids: hand_card_ids.iter().map(id_string).collect(),
            options: Vec::new(),
        },
    }
}

fn web_choice_candidate_id(candidate: &PublicChoiceCandidate) -> String {
    match candidate {
        PublicChoiceCandidate::Player(player_id) => id_string(player_id),
        PublicChoiceCandidate::Card(card_id) => id_string(card_id),
        PublicChoiceCandidate::StackSpell(stack_object_id) => id_string(stack_object_id),
    }
}

fn web_timeline_entry(entry: &PublicEventLogEntry) -> WebTimelineEntry {
    WebTimelineEntry {
        sequence: entry.sequence,
        label: web_event_label(&entry.event),
    }
}

fn web_event_label(event: &PublicEvent) -> String {
    match event {
        PublicEvent::GameStarted(event) => {
            format!("Game started for {} players", event.players.len())
        }
        PublicEvent::OpeningHandDealt(event) => {
            format!(
                "{} drew {} opening cards",
                event.player_id.as_str(),
                event.card_count
            )
        }
        PublicEvent::GameEnded(event) => format!("Game ended: {:?}", event.reason),
        PublicEvent::LandPlayed(event) => {
            format!(
                "{} played {}",
                event.player_id.as_str(),
                event.card_id.as_str()
            )
        }
        PublicEvent::TurnProgressed(event) => {
            format!(
                "Turn {} {:?} -> {} {:?} ({})",
                event.from_turn,
                event.from_phase,
                event.to_turn,
                event.to_phase,
                event.active_player.as_str()
            )
        }
        PublicEvent::CardDrawn(event) => {
            format!(
                "{} drew a card ({:?})",
                event.player_id.as_str(),
                event.draw_kind
            )
        }
        PublicEvent::CardDiscarded(event) => {
            format!(
                "{} discarded {}",
                event.player_id.as_str(),
                event.card_id.as_str()
            )
        }
        PublicEvent::MulliganTaken(event) => {
            format!("{} took a mulligan", event.player_id.as_str())
        }
        PublicEvent::LifeChanged(event) => {
            format!(
                "{} now has {} life",
                event.player_id.as_str(),
                event.to_life
            )
        }
        PublicEvent::LandTapped(event) => {
            format!(
                "{} tapped {}",
                event.player_id.as_str(),
                event.card_id.as_str()
            )
        }
        PublicEvent::ManaAdded(event) => {
            format!(
                "{} now has {} mana",
                event.player_id.as_str(),
                event.new_mana_total
            )
        }
        PublicEvent::ActivatedAbilityPutOnStack(event) => format!(
            "{} put activated ability {} on stack",
            event.player_id.as_str(),
            event.source_card_id.as_str()
        ),
        PublicEvent::TriggeredAbilityPutOnStack(event) => format!(
            "{} put triggered ability {} on stack",
            event.player_id.as_str(),
            event.source_card_id.as_str()
        ),
        PublicEvent::SpellPutOnStack(event) => {
            format!(
                "{} cast {}",
                event.player_id.as_str(),
                event.card_id.as_str()
            )
        }
        PublicEvent::PriorityPassed(event) => {
            format!("{} passed priority", event.player_id.as_str())
        }
        PublicEvent::StackTopResolved(event) => format!(
            "Stack resolved {} from {}",
            event.source_card_id.as_str(),
            event.player_id.as_str()
        ),
        PublicEvent::SpellCast(event) => format!(
            "{} resolved {} as {:?}",
            event.player_id.as_str(),
            event.card_id.as_str(),
            event.outcome
        ),
        PublicEvent::AttackersDeclared(event) => {
            format!(
                "{} declared {} attackers",
                event.player_id.as_str(),
                event.attackers.len()
            )
        }
        PublicEvent::BlockersDeclared(event) => {
            format!(
                "{} declared {} blockers",
                event.player_id.as_str(),
                event.assignments.len()
            )
        }
        PublicEvent::CombatDamageResolved(event) => format!(
            "{} resolved combat damage with {} hits",
            event.player_id.as_str(),
            event.damage_events.len()
        ),
        PublicEvent::CreatureDied(event) => {
            format!(
                "{} died for {}",
                event.card_id.as_str(),
                event.player_id.as_str()
            )
        }
        PublicEvent::CardMovedZone(event) => format!(
            "{} moved {} -> {}",
            event.card_id.as_str(),
            event.origin_zone.as_str(),
            event.destination_zone.as_str()
        ),
    }
}

fn web_command_feedback(application: &PublicCommandApplication) -> WebCommandFeedback {
    match &application.status {
        PublicCommandStatus::Applied => WebCommandFeedback {
            applied: true,
            message: "Command applied".to_string(),
            emitted_events: application
                .emitted_events
                .iter()
                .map(web_event_label)
                .collect(),
        },
        PublicCommandStatus::Rejected(rejection) => WebCommandFeedback {
            applied: false,
            message: rejection.message.clone(),
            emitted_events: Vec::new(),
        },
    }
}

fn id_string(id: &impl core::fmt::Display) -> String {
    id.to_string()
}

fn domain_error_to_js(error: crate::domain::play::errors::DomainError) -> JsValue {
    JsValue::from_str(&error.to_string())
}
