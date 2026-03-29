//! Supports the public gameplay contract consumed by clients.

mod contract;
mod execution;
mod surface;

pub use contract::*;
pub use surface::{
    choice_requests, game_view, legal_actions, public_command_result, public_event_log,
};
