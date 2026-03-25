//! Supports application.

pub mod game_service;
pub mod ports;
pub mod public_game;

pub use game_service::GameService;
pub use ports::{EventBus, EventStore};
pub use public_game::{
    choice_requests, game_view, legal_actions, PublicActivatableCard, PublicBattlefieldCardView,
    PublicCardView, PublicCastableCard, PublicChoiceCandidate, PublicChoiceRequest,
    PublicCommandRejection, PublicCommandResult, PublicCommandStatus, PublicGameCommand,
    PublicGameView, PublicLegalAction, PublicModalSpellChoice, PublicPlayerView,
    PublicPriorityView, PublicStackObjectView, PublicStackTargetView,
};
