//! Exposes a playable wasm-facing two-player duel arena adapter.

use {
    rand::{rngs::StdRng, seq::SliceRandom, RngExt, SeedableRng},
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
            DealOpeningHandsCommand, DeclareAttackersCommand, DeclareBlockersCommand,
            DiscardForCleanupCommand, LibraryCard, MulliganCommand, PassPriorityCommand,
            PlayLandCommand, PlayerDeck, PlayerLibrary, ResolveCombatDamageCommand,
            ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand,
            ResolvePendingScryCommand, ResolvePendingSurveilCommand, StartGameCommand,
            TapLandCommand,
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
    pregame: Option<PregameController>,
}

#[derive(Serialize)]
struct WebArenaState {
    game: WebGameView,
    viewers: Vec<WebViewerState>,
    event_log: Vec<WebTimelineEntry>,
    pregame: Option<WebPregameState>,
    last_command: Option<WebCommandFeedback>,
}

#[derive(Serialize)]
struct WebPregameState {
    starting_player_id: String,
    current_player_id: String,
    players: Vec<WebPregamePlayerState>,
}

#[derive(Serialize)]
struct WebPregamePlayerState {
    player_id: String,
    hand_count: usize,
    is_starting_player: bool,
    is_current: bool,
    mulligan_used: bool,
    kept: bool,
    can_mulligan: bool,
}

#[derive(Serialize)]
struct WebViewerState {
    player_id: String,
    is_active: bool,
    is_priority_holder: bool,
    hand: Vec<WebHandCard>,
    legal_actions: Vec<WebLegalAction>,
    choice_requests: Vec<WebChoicePrompt>,
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
    hand_count: usize,
    library_count: usize,
    battlefield: Vec<WebBattlefieldCard>,
    graveyard: Vec<WebCardView>,
    exile: Vec<WebCardView>,
}

#[derive(Serialize)]
struct WebCardView {
    card_id: String,
    definition_id: String,
    card_type: String,
}

#[derive(Serialize)]
struct WebHandCard {
    card_id: String,
    definition_id: String,
    card_type: String,
    mana_cost: u32,
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

#[derive(Debug, Clone)]
struct PregameController {
    starting_player_id: PlayerId,
    decision_order: Vec<PlayerId>,
    player_states: Vec<PregamePlayerState>,
    current_index: usize,
}

#[derive(Debug, Clone)]
struct PregamePlayerState {
    player_id: PlayerId,
    kept: bool,
    mulligan_used: bool,
}

#[derive(Deserialize)]
struct WebBlockerAssignmentInput {
    blocker_id: String,
    attacker_id: String,
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
        let (game, pregame) =
            start_duel_with_pregame(&service, &duel_setup, &viewer_ids, &initial_viewer)?;

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
        let (game, pregame) = start_duel_with_pregame(
            &self.service,
            &self.duel_setup,
            &self.viewer_ids,
            &initial_viewer,
        )?;
        self.game = game;
        self.pregame = Some(pregame);
        self.project_state(None)
    }

    pub fn keep_opening_hand(&mut self, player_id: &str) -> Result<JsValue, JsValue> {
        let player_id = PlayerId::new(player_id);
        let Some(pregame) = self.pregame.as_mut() else {
            return self.project_state(Some(local_command_feedback(
                false,
                "Opening hand setup is already complete.",
                Vec::new(),
            )));
        };

        if let Err(message) = pregame.keep(&player_id) {
            return self.project_state(Some(local_command_feedback(false, message, Vec::new())));
        }

        if pregame.is_complete() {
            advance_to_first_main(&self.service, &mut self.game)?;
            self.pregame = None;
            return self.project_state(Some(local_command_feedback(
                true,
                format!("{} kept the opening hand. Duel begins.", player_id.as_str()),
                Vec::new(),
            )));
        }

        let next_player = pregame.current_player_id().map_or_else(
            || "unknown".to_string(),
            |current| current.as_str().to_string(),
        );

        self.project_state(Some(local_command_feedback(
            true,
            format!(
                "{} kept the opening hand. {} now decides whether to keep or mulligan.",
                player_id.as_str(),
                next_player,
            ),
            Vec::new(),
        )))
    }

    pub fn take_mulligan(&mut self, player_id: &str) -> Result<JsValue, JsValue> {
        let player_id = PlayerId::new(player_id);
        let Some(pregame) = self.pregame.as_mut() else {
            return self.project_state(Some(local_command_feedback(
                false,
                "Opening hand setup is already complete.",
                Vec::new(),
            )));
        };

        if let Err(message) = pregame.record_mulligan(&player_id) {
            return self.project_state(Some(local_command_feedback(false, message, Vec::new())));
        }

        match self
            .service
            .mulligan(&mut self.game, MulliganCommand::new(player_id.clone()))
        {
            Ok(_) => self.project_state(Some(local_command_feedback(
                true,
                format!(
                    "{} took a mulligan. Review the new opening hand and keep to continue.",
                    player_id.as_str(),
                ),
                vec!["MulliganTaken".to_string()],
            ))),
            Err(error) => {
                pregame.revert_mulligan(&player_id);
                self.project_state(Some(local_command_feedback(
                    false,
                    error.to_string(),
                    Vec::new(),
                )))
            }
        }
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
        if self.pregame.is_some() {
            return self.project_state(Some(local_command_feedback(
                false,
                "Complete the opening hand setup before using battlefield commands.",
                Vec::new(),
            )));
        }

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
            viewers,
            event_log: event_log.iter().map(web_timeline_entry).collect(),
            pregame: self.web_pregame_state()?,
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

    fn web_pregame_state(&self) -> Result<Option<WebPregameState>, JsValue> {
        let Some(pregame) = &self.pregame else {
            return Ok(None);
        };

        let current_player_id = pregame.current_player_id().ok_or_else(|| {
            JsValue::from_str("pregame controller is active without a current player")
        })?;
        let players = pregame
            .player_states
            .iter()
            .map(|player_state| {
                let player =
                    player_by_id(&self.game, &player_state.player_id).ok_or_else(|| {
                        JsValue::from_str(&format!(
                            "pregame player {} is missing from the in-memory duel state",
                            player_state.player_id.as_str()
                        ))
                    })?;

                Ok(WebPregamePlayerState {
                    player_id: id_string(&player_state.player_id),
                    hand_count: player.hand_size(),
                    is_starting_player: player_state.player_id == pregame.starting_player_id,
                    is_current: player_state.player_id == *current_player_id,
                    mulligan_used: player_state.mulligan_used,
                    kept: player_state.kept,
                    can_mulligan: pregame.can_mulligan(&player_state.player_id),
                })
            })
            .collect::<Result<Vec<_>, JsValue>>()?;

        Ok(Some(WebPregameState {
            starting_player_id: id_string(&pregame.starting_player_id),
            current_player_id: id_string(current_player_id),
            players,
        }))
    }
}

impl PregameController {
    fn new(starting_player_id: PlayerId, viewer_ids: &[PlayerId]) -> Self {
        let mut decision_order = viewer_ids.to_vec();
        if let Some(starting_index) = decision_order
            .iter()
            .position(|player_id| player_id == &starting_player_id)
        {
            decision_order.rotate_left(starting_index);
        }

        Self {
            starting_player_id,
            decision_order,
            player_states: viewer_ids
                .iter()
                .cloned()
                .map(|player_id| PregamePlayerState {
                    player_id,
                    kept: false,
                    mulligan_used: false,
                })
                .collect(),
            current_index: 0,
        }
    }

    fn is_complete(&self) -> bool {
        self.current_index >= self.decision_order.len()
    }

    fn current_player_id(&self) -> Option<&PlayerId> {
        self.decision_order.get(self.current_index)
    }

    fn can_mulligan(&self, player_id: &PlayerId) -> bool {
        self.current_player_id() == Some(player_id)
            && self
                .player_state(player_id)
                .is_some_and(|player_state| !player_state.mulligan_used)
    }

    fn keep(&mut self, player_id: &PlayerId) -> Result<(), String> {
        self.require_current_player(player_id)?;
        let player_state = self.player_state_mut(player_id)?;
        player_state.kept = true;
        self.current_index += 1;
        Ok(())
    }

    fn record_mulligan(&mut self, player_id: &PlayerId) -> Result<(), String> {
        self.require_current_player(player_id)?;
        let player_state = self.player_state_mut(player_id)?;
        if player_state.mulligan_used {
            return Err(format!(
                "{} has already used the available mulligan in this slice.",
                player_id.as_str()
            ));
        }

        player_state.mulligan_used = true;
        Ok(())
    }

    fn revert_mulligan(&mut self, player_id: &PlayerId) {
        if let Ok(player_state) = self.player_state_mut(player_id) {
            player_state.mulligan_used = false;
        }
    }

    fn require_current_player(&self, player_id: &PlayerId) -> Result<(), String> {
        let Some(current_player_id) = self.current_player_id() else {
            return Err("Opening hand setup is already complete.".to_string());
        };

        if current_player_id != player_id {
            return Err(format!(
                "{} cannot act during setup. {} must decide first.",
                player_id.as_str(),
                current_player_id.as_str()
            ));
        }

        Ok(())
    }

    fn player_state(&self, player_id: &PlayerId) -> Option<&PregamePlayerState> {
        self.player_states
            .iter()
            .find(|player_state| player_state.player_id == *player_id)
    }

    fn player_state_mut(
        &mut self,
        player_id: &PlayerId,
    ) -> Result<&mut PregamePlayerState, String> {
        self.player_states
            .iter_mut()
            .find(|player_state| player_state.player_id == *player_id)
            .ok_or_else(|| {
                format!(
                    "{} is missing from the opening hand setup.",
                    player_id.as_str()
                )
            })
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

fn start_duel_with_pregame(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    setup: &PublicSeededGameSetup,
    viewer_ids: &[PlayerId],
    _initial_viewer: &PlayerId,
) -> Result<(Game, PregameController), JsValue> {
    let starting_player_id = choose_random_starting_player(setup)?;
    let (player_decks, player_libraries) = seeded_start_inputs(setup.clone());
    let player_decks = rotate_player_decks_to_start(player_decks, &starting_player_id)?;
    let opening_hands = DealOpeningHandsCommand::new(player_libraries);
    let (game, _, _) = service
        .start_game_with_opening_hands(
            StartGameCommand::new(setup.game_id.clone(), player_decks),
            &opening_hands,
        )
        .map_err(domain_error_to_js)?;

    Ok((game, PregameController::new(starting_player_id, viewer_ids)))
}

fn choose_random_starting_player(setup: &PublicSeededGameSetup) -> Result<PlayerId, JsValue> {
    let player_count = setup.players.len();
    if player_count == 0 {
        return Err(JsValue::from_str(
            "duel setup must include at least one player",
        ));
    }

    let mut rng = rand::rng();
    let starting_index = rng.random_range(0..player_count);

    setup
        .players
        .get(starting_index)
        .map(|player| player.player_id.clone())
        .ok_or_else(|| JsValue::from_str("failed to choose a starting player"))
}

fn seeded_start_inputs(setup: PublicSeededGameSetup) -> (Vec<PlayerDeck>, Vec<PlayerLibrary>) {
    let mut rng = StdRng::seed_from_u64(setup.shuffle_seed);

    setup
        .players
        .into_iter()
        .map(|player| {
            let PublicSeededPlayerSetup {
                player_id,
                deck_id,
                mut cards,
            } = player;
            cards.shuffle(&mut rng);
            (
                PlayerDeck::new(player_id.clone(), deck_id),
                PlayerLibrary::new(player_id, cards),
            )
        })
        .unzip()
}

fn rotate_player_decks_to_start(
    mut player_decks: Vec<PlayerDeck>,
    starting_player_id: &PlayerId,
) -> Result<Vec<PlayerDeck>, JsValue> {
    let starting_index = player_decks
        .iter()
        .position(|player_deck| player_deck.player_id == *starting_player_id)
        .ok_or_else(|| {
            JsValue::from_str(&format!(
                "starting player {} is missing from the duel setup",
                starting_player_id.as_str()
            ))
        })?;
    player_decks.rotate_left(starting_index);

    Ok(player_decks)
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
            card_type: None,
            target: None,
            requires_choice: false,
        },
        PublicStackObjectView::Spell {
            number,
            controller_id,
            source_card_id,
            card_type,
            target,
            requires_choice,
        } => WebStackObject {
            number: *number,
            kind: "Spell".to_string(),
            controller_id: Some(id_string(controller_id)),
            source_card_id: Some(id_string(source_card_id)),
            card_type: Some(format!("{card_type:?}")),
            target: target.as_ref().map(web_stack_target_label),
            requires_choice: *requires_choice,
        },
        PublicStackObjectView::ActivatedAbility {
            number,
            controller_id,
            source_card_id,
            target,
        } => WebStackObject {
            number: *number,
            kind: "ActivatedAbility".to_string(),
            controller_id: Some(id_string(controller_id)),
            source_card_id: Some(id_string(source_card_id)),
            card_type: None,
            target: target.as_ref().map(web_stack_target_label),
            requires_choice: false,
        },
        PublicStackObjectView::TriggeredAbility {
            number,
            controller_id,
            source_card_id,
        } => WebStackObject {
            number: *number,
            kind: "TriggeredAbility".to_string(),
            controller_id: Some(id_string(controller_id)),
            source_card_id: Some(id_string(source_card_id)),
            card_type: None,
            target: None,
            requires_choice: false,
        },
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

fn local_command_feedback(
    applied: bool,
    message: impl Into<String>,
    emitted_events: Vec<String>,
) -> WebCommandFeedback {
    WebCommandFeedback {
        applied,
        message: message.into(),
        emitted_events,
    }
}

fn id_string(id: &impl core::fmt::Display) -> String {
    id.to_string()
}

fn domain_error_to_js(error: crate::domain::play::errors::DomainError) -> JsValue {
    JsValue::from_str(&error.to_string())
}

#[cfg(test)]
mod tests {
    //! Verifies web duel pregame progression stays deterministic and seat-scoped.

    #![allow(clippy::expect_used)]

    use super::PregameController;
    use crate::domain::play::ids::PlayerId;

    fn viewer_ids() -> Vec<PlayerId> {
        vec![PlayerId::new("player-1"), PlayerId::new("player-2")]
    }

    #[test]
    fn pregame_starts_from_the_randomized_starting_player() {
        let controller = PregameController::new(PlayerId::new("player-2"), &viewer_ids());

        assert_eq!(
            controller
                .current_player_id()
                .expect("current player should exist")
                .as_str(),
            "player-2"
        );
    }

    #[test]
    fn mulligan_keeps_the_same_player_until_they_keep() {
        let mut controller = PregameController::new(PlayerId::new("player-1"), &viewer_ids());

        controller
            .record_mulligan(&PlayerId::new("player-1"))
            .expect("starting player should mulligan once");

        assert_eq!(
            controller
                .current_player_id()
                .expect("same player should remain current")
                .as_str(),
            "player-1"
        );
    }

    #[test]
    fn keep_advances_pregame_and_completes_after_both_players_keep() {
        let mut controller = PregameController::new(PlayerId::new("player-2"), &viewer_ids());

        controller
            .keep(&PlayerId::new("player-2"))
            .expect("starting player should keep");
        assert_eq!(
            controller
                .current_player_id()
                .expect("second player should now be current")
                .as_str(),
            "player-1"
        );

        controller
            .keep(&PlayerId::new("player-1"))
            .expect("second player should keep");
        assert!(controller.is_complete());
    }
}
