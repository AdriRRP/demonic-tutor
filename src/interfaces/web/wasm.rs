//! Exposes a minimal wasm-facing public gameplay demo adapter.

use {serde::Serialize, wasm_bindgen::prelude::*};

use crate::{
    application::{
        choice_requests, game_view, legal_actions, GameService, PublicBattlefieldCardView,
        PublicCardView, PublicChoiceRequest, PublicCommandApplication, PublicCommandStatus,
        PublicEvent, PublicEventLogEntry, PublicGameCommand, PublicGameView, PublicLegalAction,
        PublicPlayableSubsetVersion, PublicPlayerView, PublicSeededGameSetup,
        PublicSeededPlayerSetup, PublicStackObjectView, PublicStackTargetView,
    },
    domain::play::{
        commands::{
            AdvanceTurnCommand, LibraryCard, PassPriorityCommand, PlayLandCommand, TapLandCommand,
        },
        ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
        phase::Phase,
    },
    infrastructure::{InMemoryEventBus, InMemoryEventStore},
};

#[wasm_bindgen]
pub struct WebDemoClient {
    service: GameService<InMemoryEventStore, InMemoryEventBus>,
    game: crate::domain::play::game::Game,
    viewer_id: PlayerId,
    demo_setup: PublicSeededGameSetup,
}

#[derive(Serialize)]
struct WebDemoState {
    game: WebGameView,
    legal_actions: Vec<WebLegalAction>,
    choice_requests: Vec<WebChoicePrompt>,
    event_log: Vec<WebTimelineEntry>,
    last_command: Option<WebCommandFeedback>,
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
}

#[derive(Serialize)]
struct WebChoicePrompt {
    kind: String,
    player_id: String,
    source_card_id: Option<String>,
    summary: String,
    item_ids: Vec<String>,
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

#[wasm_bindgen]
impl WebDemoClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebDemoClient, JsValue> {
        let service = GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new());
        let viewer_id = PlayerId::new("player-1");
        let demo_setup = demo_setup();
        let (mut game, _) = service
            .start_seeded_public_game(demo_setup.clone(), &viewer_id)
            .map_err(domain_error_to_js)?;
        advance_demo_to_first_main(&service, &mut game)?;

        Ok(Self {
            service,
            game,
            viewer_id,
            demo_setup,
        })
    }

    pub fn state(&self) -> Result<JsValue, JsValue> {
        self.project_state(None)
    }

    pub fn reset(&mut self) -> Result<JsValue, JsValue> {
        let (mut game, _) = self
            .service
            .start_seeded_public_game(self.demo_setup.clone(), &self.viewer_id)
            .map_err(domain_error_to_js)?;
        advance_demo_to_first_main(&self.service, &mut game)?;
        self.game = game;
        self.project_state(None)
    }

    pub fn step_demo(&mut self) -> Result<JsValue, JsValue> {
        let command = if let Some(priority) = self.game.priority() {
            PublicGameCommand::PassPriority(PassPriorityCommand::new(
                priority.current_holder().clone(),
            ))
        } else {
            PublicGameCommand::AdvanceTurn(AdvanceTurnCommand::new())
        };
        self.apply_command(command)
    }

    pub fn play_land(&mut self, card_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::PlayLand(PlayLandCommand::new(
            self.viewer_id.clone(),
            CardInstanceId::new(card_id),
        )))
    }

    pub fn tap_mana_source(&mut self, card_id: &str) -> Result<JsValue, JsValue> {
        self.apply_command(PublicGameCommand::TapLand(TapLandCommand::new(
            self.viewer_id.clone(),
            CardInstanceId::new(card_id),
        )))
    }
}

impl WebDemoClient {
    fn apply_command(&mut self, command: PublicGameCommand) -> Result<JsValue, JsValue> {
        let application = self.service.execute_public_command(&mut self.game, command);
        let feedback = web_command_feedback(&application);
        self.project_state(Some(feedback))
    }

    fn project_state(&self, last_command: Option<WebCommandFeedback>) -> Result<JsValue, JsValue> {
        let game = game_view(&self.game);
        let legal_actions = legal_actions(&self.game, &self.viewer_id);
        let choice_requests = choice_requests(&self.game, &self.viewer_id);
        let event_log = self
            .service
            .public_event_log(self.game.id())
            .map_err(domain_error_to_js)?;

        serde_wasm_bindgen::to_value(&WebDemoState {
            game: web_game_view(&game),
            legal_actions: legal_actions.iter().map(web_legal_action).collect(),
            choice_requests: choice_requests.iter().map(web_choice_prompt).collect(),
            event_log: event_log.iter().map(web_timeline_entry).collect(),
            last_command,
        })
        .map_err(|err| JsValue::from_str(&format!("failed to serialize web demo state: {err}")))
    }
}

fn advance_demo_to_first_main(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    game: &mut crate::domain::play::game::Game,
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
                "failed to prepare wasm demo state: {}",
                rejection.message
            )));
        }
    }

    Err(JsValue::from_str(
        "failed to prepare wasm demo state before reaching FirstMain",
    ))
}

fn demo_setup() -> PublicSeededGameSetup {
    PublicSeededGameSetup::new(
        GameId::new("web-demo-game"),
        vec![
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-1"),
                DeckId::new("deck-1"),
                land_library("p1"),
            ),
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-2"),
                DeckId::new("deck-2"),
                land_library("p2"),
            ),
        ],
        7,
    )
}

fn land_library(prefix: &str) -> Vec<LibraryCard> {
    ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"]
        .into_iter()
        .map(|suffix| {
            LibraryCard::land(
                CardDefinitionId::new(format!("{prefix}-forest-{suffix}")),
                crate::domain::play::cards::ManaColor::Green,
            )
        })
        .collect()
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
        },
        PublicLegalAction::ResolvePendingScry { player_id } => WebLegalAction {
            kind: "ResolvePendingScry".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve pending scry".to_string(),
            card_ids: Vec::new(),
        },
        PublicLegalAction::ResolvePendingSurveil { player_id } => WebLegalAction {
            kind: "ResolvePendingSurveil".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve pending surveil".to_string(),
            card_ids: Vec::new(),
        },
        PublicLegalAction::ResolvePendingHandChoice { player_id } => WebLegalAction {
            kind: "ResolvePendingHandChoice".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve pending hand choice".to_string(),
            card_ids: Vec::new(),
        },
        PublicLegalAction::ResolveOptionalEffect { player_id } => WebLegalAction {
            kind: "ResolveOptionalEffect".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve optional effect".to_string(),
            card_ids: Vec::new(),
        },
        PublicLegalAction::PassPriority { player_id } => WebLegalAction {
            kind: "PassPriority".to_string(),
            player_id: id_string(player_id),
            summary: "Pass priority".to_string(),
            card_ids: Vec::new(),
        },
        PublicLegalAction::PlayLand {
            player_id,
            playable_land_ids,
        } => WebLegalAction {
            kind: "PlayLand".to_string(),
            player_id: id_string(player_id),
            summary: "Play a land".to_string(),
            card_ids: playable_land_ids.iter().map(id_string).collect(),
        },
        PublicLegalAction::TapManaSource {
            player_id,
            mana_source_ids,
        } => WebLegalAction {
            kind: "TapManaSource".to_string(),
            player_id: id_string(player_id),
            summary: "Tap a mana source".to_string(),
            card_ids: mana_source_ids.iter().map(id_string).collect(),
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
        },
        PublicLegalAction::DeclareAttackers {
            player_id,
            attacker_ids,
        } => WebLegalAction {
            kind: "DeclareAttackers".to_string(),
            player_id: id_string(player_id),
            summary: "Declare attackers".to_string(),
            card_ids: attacker_ids.iter().map(id_string).collect(),
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
        },
        PublicLegalAction::ResolveCombatDamage { player_id } => WebLegalAction {
            kind: "ResolveCombatDamage".to_string(),
            player_id: id_string(player_id),
            summary: "Resolve combat damage".to_string(),
            card_ids: Vec::new(),
        },
        PublicLegalAction::AdvanceTurn { player_id } => WebLegalAction {
            kind: "AdvanceTurn".to_string(),
            player_id: id_string(player_id),
            summary: "Advance turn".to_string(),
            card_ids: Vec::new(),
        },
        PublicLegalAction::DiscardForCleanup {
            player_id,
            card_ids,
        } => WebLegalAction {
            kind: "DiscardForCleanup".to_string(),
            player_id: id_string(player_id),
            summary: "Discard for cleanup".to_string(),
            card_ids: card_ids.iter().map(id_string).collect(),
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
        },
        PublicChoiceRequest::PriorityUnavailable { player_id } => WebChoicePrompt {
            kind: "PriorityUnavailable".to_string(),
            player_id: id_string(player_id),
            source_card_id: None,
            summary: "Priority surface unavailable".to_string(),
            item_ids: Vec::new(),
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
            item_ids: candidates.iter().map(web_choice_candidate_label).collect(),
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
        },
        PublicChoiceRequest::SpellSecondaryCreatureChoice {
            player_id,
            source_card_id,
            creature_ids,
            ..
        } => WebChoicePrompt {
            kind: "SpellSecondaryCreatureChoice".to_string(),
            player_id: id_string(player_id),
            source_card_id: Some(id_string(source_card_id)),
            summary: "Choose secondary creature targets".to_string(),
            item_ids: creature_ids.iter().map(id_string).collect(),
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
            item_ids: modes.iter().map(|mode| format!("{mode:?}")).collect(),
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
            item_ids: candidates.iter().map(web_choice_candidate_label).collect(),
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
        },
    }
}

fn web_choice_candidate_label(candidate: &crate::application::PublicChoiceCandidate) -> String {
    match candidate {
        crate::application::PublicChoiceCandidate::Player(player_id) => {
            format!("Player {}", player_id.as_str())
        }
        crate::application::PublicChoiceCandidate::Card(card_id) => {
            format!("Card {}", card_id.as_str())
        }
        crate::application::PublicChoiceCandidate::StackSpell(stack_object_id) => {
            format!("Stack {}", stack_object_id.as_str())
        }
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
