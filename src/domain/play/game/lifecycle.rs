//! Supports play game lifecycle.

use {
    super::{invariants, rules, Game},
    crate::domain::play::{
        commands::{ConcedeCommand, DealOpeningHandsCommand, MulliganCommand, StartGameCommand},
        errors::DomainError,
        events::{GameEnded, GameStarted, MulliganTaken, OpeningHandDealt},
    },
};

impl Game {
    /// Starts a new game.
    ///
    /// # Errors
    /// See [`rules::lifecycle::start`].
    pub fn start(cmd: StartGameCommand) -> Result<(Self, GameStarted), DomainError> {
        rules::lifecycle::start(cmd)
    }

    /// Deals opening hands to players.
    ///
    /// # Errors
    /// See [`rules::lifecycle::deal_opening_hands`].
    pub fn deal_opening_hands(
        &mut self,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        let result = rules::lifecycle::deal_opening_hands(&mut self.players, cmd, &self.id);
        if result.is_ok() {
            self.rebuild_card_locations_from_players();
        }
        result
    }

    /// Performs a mulligan.
    ///
    /// # Errors
    /// See [`rules::lifecycle::mulligan`].
    pub fn mulligan(&mut self, cmd: MulliganCommand) -> Result<MulliganTaken, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let active_player = self.active_player().clone();
        let result = rules::lifecycle::mulligan(
            &self.id,
            &mut self.players,
            &active_player,
            &self.phase,
            cmd,
        );
        if result.is_ok() {
            self.rebuild_card_locations_from_players();
        }
        result
    }

    /// Concedes the active game for one player.
    ///
    /// # Errors
    /// See [`rules::lifecycle::concede`].
    pub fn concede(&mut self, cmd: ConcedeCommand) -> Result<GameEnded, DomainError> {
        invariants::require_game_active(self.is_over())?;
        rules::lifecycle::concede(&self.id, &self.players, &mut self.terminal_state, cmd)
    }
}

#[cfg(test)]
mod tests {
    //! Verifies lifecycle keeps the aggregate card-location index truthful.

    use super::*;
    use crate::domain::play::{
        commands::{PlayerDeck, PlayerLibrary},
        game::PlayerCardZone,
        ids::{CardDefinitionId, DeckId, GameId, PlayerId},
    };

    fn build_setup_game() -> Game {
        Game::start(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                PlayerDeck::new(PlayerId::new("player-1"), DeckId::new("deck-1")),
                PlayerDeck::new(PlayerId::new("player-2"), DeckId::new("deck-2")),
            ],
        ))
        .expect("game should start")
        .0
    }

    fn opening_hand_command() -> DealOpeningHandsCommand {
        DealOpeningHandsCommand::new(vec![
            PlayerLibrary::new(
                PlayerId::new("player-1"),
                (0..14)
                    .map(|index| {
                        crate::domain::play::commands::LibraryCard::land(
                            CardDefinitionId::new(format!("p1-land-{index}")),
                            crate::domain::play::cards::ManaColor::Green,
                        )
                    })
                    .collect(),
            ),
            PlayerLibrary::new(
                PlayerId::new("player-2"),
                (0..14)
                    .map(|index| {
                        crate::domain::play::commands::LibraryCard::land(
                            CardDefinitionId::new(format!("p2-land-{index}")),
                            crate::domain::play::cards::ManaColor::Blue,
                        )
                    })
                    .collect(),
            ),
        ])
    }

    fn assert_player_locations_match_index(game: &Game, player_index: usize) {
        let card_locations = game.cloned_card_locations();
        for (card_id, _, zone) in game.players()[player_index].owned_card_locations() {
            let indexed = card_locations
                .location(card_id)
                .expect("owned card should remain indexed");
            assert_eq!(indexed.player_index(), player_index);
            assert_eq!(indexed.zone(), zone);
        }
    }

    #[test]
    fn deal_opening_hands_rebuilds_card_locations_for_drawn_cards() {
        let mut game = build_setup_game();

        let events = game
            .deal_opening_hands(&opening_hand_command())
            .expect("opening hands should succeed");

        let first_drawn = &events[0].cards[0];
        let location = game
            .cloned_card_locations()
            .location(first_drawn)
            .expect("drawn card should remain indexed");

        assert_eq!(location.zone(), PlayerCardZone::Hand);
    }

    #[test]
    fn mulligan_rebuilds_card_locations_for_new_hand() {
        let mut game = build_setup_game();
        game.deal_opening_hands(&opening_hand_command())
            .expect("opening hands should succeed");

        game.mulligan(MulliganCommand::new(PlayerId::new("player-1")))
            .expect("mulligan should succeed");

        assert_player_locations_match_index(&game, 0);
    }
}
