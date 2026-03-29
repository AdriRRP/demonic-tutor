//! Supports the public gameplay contract consumed by clients.

mod contract;
mod execution;
mod surface;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use contract::*;
pub use surface::{
    choice_requests, game_view, legal_actions, public_command_result, public_event_log,
};
#[cfg(target_arch = "wasm32")]
pub use wasm::WebDemoClient;
