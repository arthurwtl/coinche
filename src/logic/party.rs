use core::panic;

use super::playing::*;

/// All the logic for a single round.
#[derive(Debug)]
enum RoundState {
    Bidding(BiddingState),
    // StartingPlayingPhase(Player),
    Playing(PlayingState),
    Over,
}

/// If we are in the play phase, the round is described by this struct.
#[derive(Debug)]
struct PlayingState {
    deck: Deck,
    is_playing: Player,
    first_player: Player,
    history: PlayingHistory,
    table: Vec<Card>,
    trump: Suit,
    // contrat: Bid,
    
}

/// If we are in the bidding phase, the round is described by this struct.
#[derive(Debug)]
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
// struct PlayingHistory {
//     odd: Vec<Trick>,
//     even: Vec<Trick>,
// }

type PlayingHistory = [Vec<Trick>; 2];

/// A player can play or bid, (pass is None)
enum PlayerAction {
    PlayCard { player: Player, card: Card },
    AnounceBid { player: Player, bid: Bid },
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

// impl PlayingHistory {
//     fn new() -> Self {
//         Self {
//             odd: vec![],
//             even: vec![],
//         }
//     }
// }

impl RoundState {
    fn new(first_player: Player) -> Self {
        RoundState::Bidding(BiddingState {
            deck: Deck::random_deck(),
            is_bidding: first_player,
            first_player: first_player,
            bidding_history: vec![],
        })
    }

    /// Dispatch update on the right method.
    fn update(self, action: PlayerAction) -> RoundState {
        match (self, action) {
            (RoundState::Bidding(bidding_state), PlayerAction::AnounceBid { player, bid }) => {
                bidding_state.update(player, bid)
            }
            (RoundState::Playing(playing_state), PlayerAction::PlayCard { player, card }) => {
                playing_state.update(player, card)
            }
            _ => panic!("Illegal action in roundstate.update()"),
        }
    }
}

impl BiddingState {
    fn update(mut self, player: Player, bid: Bid) -> RoundState {
        assert!(
            player == self.is_bidding,
            "Player tries to bid, but not his turn."
        );
        let res = bidding_request(&self.bidding_history, bid);

        match res {
            BiddingRequestResult::Illegal => panic!("Bidding illegal"),

            BiddingRequestResult::Abortion => RoundState::new((self.first_player + 1) % 4),

            BiddingRequestResult::Legal => {
                self.bidding_history.push(bid);
                RoundState::Bidding(BiddingState {
                    is_bidding: (self.is_bidding + 1) % 4,
                    ..self
                })
            }

            BiddingRequestResult::BiddingWinned(index_winner) => {
                RoundState::Playing(PlayingState {
                    deck: self.deck,
                    is_playing: self.first_player,
                    first_player: self.first_player,
                    // history: PlayingHistory::new(),
                    history: [vec![], vec![]],
                    table: vec![],
                    trump: {
                        if let Bid::Value(s, _r) = self.bidding_history[index_winner] {
                            s
                        } else {
                            panic!()
                        }
                    },
                })
            }
        }
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
                first_player: self.first_player,
                history: self.history,
            }),

            PlayingRequestResult::TrickWinned(winner) => {
                if self.deck.iter().flatten().count() == 1 {
                    RoundState::Over
                } else {
                    RoundState::Playing(PlayingState {
                        deck: self.deck.delete_card(player, card),
                        is_playing: winner,
                        table: vec![],
                        history: {
                            let mut tmp_history = self.history;
                            let mut tmp_table = self.table;
                            tmp_table.push(card);
                            tmp_history[winner % 2].push(tmp_table);
                            tmp_history
                        },
                        ..self
                    })
                }
            }
            PlayingRequestResult::Illegal => panic!("requested illegal move"),
        }
    }
}


//yes these test have no assert but at least they don't panic
#[cfg(test)]
mod round_tests {
    use crate::logic::*;

    use super::*;

    #[test]
    fn test_bidding_order() {
        let mut round_state = RoundState::new(0);
        println!("{:#?}", round_state);
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 0,
            bid: Bid::Pass,
        });
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 1,
            bid: Bid::Pass,
        });
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 2,
            bid: Bid::Pass,
        });
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 3,
            bid: Bid::Pass,
        });
        // round_state = round_state.update(PlayerAction::AnounceBid { player: 2, bid: Bid::Pass });
        // round_state = round_state.update(PlayerAction::AnounceBid { player: 3, bid: Bid::Pass });
        println!("{:#?}", round_state);
    }

    #[test]
    #[should_panic]
    fn test_bidding_order_2() {
        let mut round_state = RoundState::new(0);
        println!("{:#?}", round_state);
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 0,
            bid: Bid::Pass,
        });
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 0,
            bid: Bid::Pass,
        });
        println!("{:#?}", round_state);
    }

    #[test]
    #[should_panic]
    fn test_bidding_1() {
        let mut round_state = RoundState::new(0);
        println!("{:#?}", round_state);
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 0,
            bid: Bid::Value(Suit::Hearts, 81),
        });
        println!("{:#?}", round_state);
    }
    
    #[test]
    #[should_panic]
    fn test_bidding_2() {
        let mut round_state = RoundState::new(0);
        println!("{:#?}", round_state);
        round_state = round_state.update(PlayerAction::AnounceBid {
            player: 0,
            bid: Bid::Value(Suit::Hearts, 70),
        });
        println!("{:#?}", round_state);
    }
}
