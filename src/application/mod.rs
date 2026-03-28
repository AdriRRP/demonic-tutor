//! Supports application.

pub mod game_service;
pub mod ports;
pub mod public_game;

pub use game_service::GameService;
pub use ports::{EventBus, EventStore};
pub use public_game::{
    choice_requests, game_view, legal_actions, public_command_result, public_event_log,
    PublicActivatableCard, PublicBattlefieldCardView, PublicBinaryChoice, PublicCardView,
    PublicCastableCard, PublicChoiceCandidate, PublicChoiceRequest, PublicCommandApplication,
    PublicCommandRejection, PublicCommandResult, PublicCommandStatus, PublicEventLogEntry,
    PublicGameCommand, PublicGameSessionStart, PublicGameView, PublicLegalAction,
    PublicModalSpellChoice, PublicPlayableSubsetVersion, PublicPlayerView, PublicPriorityView,
    PublicRematchCommand, PublicScryChoice, PublicSeededGameSetup, PublicSeededPlayerSetup,
    PublicStackObjectView, PublicStackTargetView, PublicSurveilChoice,
};
