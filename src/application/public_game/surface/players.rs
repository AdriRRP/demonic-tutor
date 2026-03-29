//! Resolves the player identities needed across a public surface snapshot.

use crate::domain::play::{
    game::{Game, Player},
    ids::PlayerId,
};

#[derive(Clone, Copy)]
pub(super) struct SurfacePlayers<'a> {
    pub(super) active_player: Option<&'a Player>,
    pub(super) defending_player: Option<&'a Player>,
    pub(super) viewer_player: Option<&'a Player>,
    pub(super) viewer_opponent: Option<&'a Player>,
}

impl<'a> SurfacePlayers<'a> {
    pub(super) fn resolve(game: &'a Game, viewer_id: &PlayerId) -> Self {
        let players = game.players();
        let active_player = players.get(game.active_player_index_value());
        let defending_player = active_player.and_then(|player| other_player(players, player.id()));
        let viewer_player = find_player(players, viewer_id);
        let viewer_opponent = viewer_player.and_then(|player| other_player(players, player.id()));

        Self {
            active_player,
            defending_player,
            viewer_player,
            viewer_opponent,
        }
    }
}

fn find_player<'a>(players: &'a [Player], player_id: &PlayerId) -> Option<&'a Player> {
    match players {
        [first, second] => {
            if first.id() == player_id {
                Some(first)
            } else if second.id() == player_id {
                Some(second)
            } else {
                None
            }
        }
        _ => players.iter().find(|player| player.id() == player_id),
    }
}

fn other_player<'a>(players: &'a [Player], player_id: &PlayerId) -> Option<&'a Player> {
    match players {
        [first, second] => {
            if first.id() == player_id {
                Some(second)
            } else if second.id() == player_id {
                Some(first)
            } else {
                None
            }
        }
        _ => players.iter().find(|player| player.id() != player_id),
    }
}
