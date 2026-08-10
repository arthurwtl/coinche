use super::playing::*;

/// All the logic for a single round.
enum RoundState {
    Bidding(BiddingState),
    // StartingPlayingPhase(Player),
    Playing(PlayingState),
    Over,
}

/// If we are in the play phase, the round is described by this struct.
struct PlayingState {
    deck: Deck,
    is_playing: Player,
    first_player: Player,
    history: PlayingHistory,
    table: Vec<Card>,
    trump: Suit,
}

/// If we are in the bidding phase, the round is described by this struct.
struct BiddingState {
    deck: Deck,
    is_bidding: Player,
    bidding_history: Vec<Bid>,
    first_player: Player,
}

enum Team {
    Odd,
    Even,
}

/// Contains the history on the current round by listing the triks won by
/// Odd and Even, the two opposed teams.
struct PlayingHistory {
    odd: Vec<Trick>,
    even: Vec<Trick>,
}

/// A player can play or bid, (pass is None)
enum PlayerAction {
    PlayCard { player: Player, card: Card },
    AnounceBid { player: Player, bid: Option<Bid> },
}

// Current score of the party
struct Score {
    odd: u8,
    even: u8,
}

impl Score {
    fn new() -> Self {
        Score { odd: 0, even: 0 }
    }
}

// ------------ Impl --------------

impl PlayingHistory {
    fn new() -> Self {
        Self {
            odd: vec![],
            even: vec![],
        }
    }
}

impl RoundState {
    fn new(first_player: Player) -> Self {
        RoundState::Bidding(BiddingState {
            deck: Deck::random_deck(),
            is_bidding: first_player,
            first_player: first_player,
            bidding_history: vec![],
        })
    }

    /// Simply see in which phase we are (among initial, bidding, playing) and call the right method.
    fn update(round_state: RoundState, action: PlayerAction) -> RoundState {
        match (round_state, action) {
            (RoundState::Bidding(bidding_state), PlayerAction::AnounceBid { player, bid }) => {
                bidding_state.update(player, bid.unwrap())
            }
            (RoundState::Playing(playing_state), PlayerAction::PlayCard { player, card }) => {
                playing_state.update(player, card)
            }
            _ => panic!("Illegal action in roundstate.update()"),
        }
    }
}

impl BiddingState {
    fn update(self, player: Player, bid: Bid) -> RoundState {
        todo!();
    }
}

impl PlayingState {
    fn update(self, player: Player, card: Card) -> RoundState {
        assert!(
            player == self.is_playing,
            "Player tries to play, but not his turn."
        );
        let res = playing_request(&self.table, &self.deck[player], self.trump, card);

        match res {
            PlayingRequestResult::Legal => RoundState::Playing(PlayingState {
                deck: self.deck.delete_card(player, card),
                is_playing: (self.is_playing + 1) % 4,
                table: {
                    let mut tmp = self.table;
                    tmp.push(card);
                    tmp
                },
                trump: self.trump,
                first_player : self.first_player,
                history: self.history,
            }),
            PlayingRequestResult::TrickWinned(winner) => {
                if self.deck.iter().flatten().count() == 1 {
                    RoundState::Over
                } else {
                    RoundState::Playing(PlayingState {
                        deck: self.deck.delete_card(player, card),
                        first_player: (self.first_player + 1) % 4,
                        is_playing: (self.first_player + 1) % 4, // is_playing: (self.first_player + 1) % 4
                        table: {
                            let mut tmp = self.table;
                            tmp.push(card);
                            tmp
                        },
                        trump: self.trump,
                        history: self.history,
                    })
                }
            }
            PlayingRequestResult::Illegal => panic!("requested illegal move"),
        }
    }
}
