use super::{player::Player, Game, Phase};
use crate::domain::{
    commands::StartGameCommand,
    errors::{DomainError, GameError, PlayerError},
    events::GameStarted,
};

pub fn start(cmd: StartGameCommand) -> Result<(Game, GameStarted), DomainError> {
    let player_count = cmd.players.len();

    if player_count < 2 {
        return Err(DomainError::Player(PlayerError::NotEnoughPlayers {
            actual: player_count,
        }));
    }

    if player_count > 2 {
        return Err(DomainError::Player(PlayerError::TooManyPlayers {
            actual: player_count,
        }));
    }

    let mut seen_players = std::collections::HashSet::new();
    let mut players = Vec::new();
    let mut player_ids = Vec::new();

    for pd in &cmd.players {
        if !seen_players.insert(pd.player_id.clone()) {
            return Err(DomainError::Game(GameError::DuplicatePlayer(
                pd.player_id.clone(),
            )));
        }
        players.push(Player::new(pd.player_id.clone(), pd.deck_id.clone()));
        player_ids.push(pd.player_id.clone());
    }

    let game_started = GameStarted::new(cmd.game_id.clone(), player_ids.clone());

    let active_player = player_ids.into_iter().next().ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "player list should not be empty after validation".to_string(),
        ))
    })?;

    let game = Game::new(cmd.game_id, active_player, Phase::Setup, 1, players);

    Ok((game, game_started))
}
