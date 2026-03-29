//! Projects the aggregate into the public gameplay read contract.

mod event_log;
mod game_view;
mod interaction;
mod players;
#[cfg(test)]
mod tests;

pub use event_log::public_event_log;
pub use game_view::game_view;
pub use interaction::{choice_requests, legal_actions, public_command_result};

pub(super) use event_log::{public_event_log_projection, public_events};
pub(super) use interaction::public_surface_state;
