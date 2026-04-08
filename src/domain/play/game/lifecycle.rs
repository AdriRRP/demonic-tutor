//! Supports play game lifecycle.

use {
    super::{invariants, rules, Game},
    crate::domain::play::{
        commands::{ConcedeCommand, DealOpeningHandsCommand, MulliganCommand, StartGameCommand},
        errors::{DomainError, GameError, PhaseError},
        events::{
            CardMovedZone, GameEnded, GameStarted, MulliganTaken, OpeningHandDealt, ZoneType,
        },
        ids::{CardInstanceId, PlayerId},
        phase::Phase,
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

    /// Puts selected opening-hand cards on the bottom of the library during setup.
    ///
    /// # Errors
    /// Returns an error when the game is not in `Setup`, when the selection count
    /// does not match the current mulligan count, or when any chosen card is not
    /// currently in that player's hand.
    pub fn bottom_opening_hand_cards(
        &mut self,
        player_id: PlayerId,
        card_ids: &[CardInstanceId],
    ) -> Result<Vec<CardMovedZone>, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        if !matches!(self.phase, Phase::Setup) {
            return Err(DomainError::Phase(PhaseError::InvalidForMulligan));
        }

        let player_index =
            crate::domain::play::game::helpers::find_player_index(&self.players, &player_id)?;
        let player = &self.players[player_index];
        let expected = usize::try_from(player.mulligan_count()).map_err(|_| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "opening-hand bottom count overflowed usize".to_string(),
            ))
        })?;

        if card_ids.len() != expected {
            return Err(DomainError::Game(
                GameError::InvalidOpeningHandBottomCount {
                    player: player_id,
                    expected,
                    actual: card_ids.len(),
                },
            ));
        }

        let moved_cards = card_ids
            .iter()
            .map(|card_id| {
                self.players[player_index]
                    .move_hand_card_to_library_bottom(card_id)
                    .ok_or_else(|| {
                        DomainError::Game(GameError::InvalidHandCardChoice(card_id.clone()))
                    })?;

                Ok(CardMovedZone::new(
                    self.id.clone(),
                    self.players[player_index].id().clone(),
                    card_id.clone(),
                    ZoneType::Hand,
                    ZoneType::Library,
                ))
            })
            .collect::<Result<Vec<_>, DomainError>>()?;

        self.rebuild_card_locations_from_players();
        Ok(moved_cards)
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

    #![allow(clippy::expect_used)]

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

    #[test]
    fn mulligan_can_be_taken_multiple_times_during_setup() {
        let mut game = build_setup_game();
        game.deal_opening_hands(&opening_hand_command())
            .expect("opening hands should succeed");

        game.mulligan(MulliganCommand::new(PlayerId::new("player-1")))
            .expect("first mulligan should succeed");
        game.mulligan(MulliganCommand::new(PlayerId::new("player-1")))
            .expect("second mulligan should succeed");

        assert_eq!(game.players()[0].mulligan_count(), 2);
        assert_eq!(game.players()[0].hand_size(), 7);
    }

    #[test]
    fn bottom_opening_hand_cards_moves_selected_cards_to_library_bottom() {
        let mut game = build_setup_game();
        game.deal_opening_hands(&opening_hand_command())
            .expect("opening hands should succeed");
        game.mulligan(MulliganCommand::new(PlayerId::new("player-1")))
            .expect("mulligan should succeed");

        let selected = game.players()[0]
            .hand_card_ids()
            .into_iter()
            .take(1)
            .collect::<Vec<_>>();

        let zone_changes = game
            .bottom_opening_hand_cards(PlayerId::new("player-1"), &selected)
            .expect("bottoming opening-hand cards should succeed");

        assert_eq!(zone_changes.len(), 1);
        assert_eq!(game.players()[0].hand_size(), 6);
        assert_eq!(game.players()[0].library_size(), 8);
        assert_player_locations_match_index(&game, 0);
        assert!(
            game.players()[0]
                .hand_card_ids()
                .iter()
                .all(|card_id| !selected.contains(card_id)),
            "bottomed cards should leave the hand"
        );
    }
}
