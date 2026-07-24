//! Contains the logic used by the server to update the state of the game.

use super::bidding::*;
use super::playing::*;
use super::round::*;
use super::party::*;


// ======= State definitions =======

/// Contains the history on the current round by listing the triks won by
/// Odd and Even, the two opposed teams.
struct PlayingHistory {
    odd: Vec<Trick>,
    even: Vec<Trick>,
}


// Current score of the party
struct Score {
    odd: u8,
    even: u8,
}

/// All the logic for a single round.
enum RoundState {
    Initial,
    Over,
    Playing(PlayingState),
    Bidding(BiddingState),
}

/// If we are in the play phase, the round is described by this struct.
struct PlayingState {
    deck: Deck,
    is_playing: Player,
    history: PlayingHistory,
    table: Vec<Card>,
    trump: Suit,
}

/// If we are in the bidding phase, the round is described by this struct.
struct BiddingState {
    deck: Deck,
    is_bidding: Player,
    bidding_history: Vec<Bid>,
}

/// A player can play or bid, (pass is None)
enum PlayerAction {
    Nil,
    PlayCard { player: Player, card: Card },
    Bid { player: Player, bid: Option<Bid> },
}

// ------------- Impl --------------

impl PlayingHistory {
    fn new() -> Self {
        Self {
            odd: vec![],
            even: vec![],
        }
    }
}

impl Score {
    fn new() -> Self {
        Score { odd: 0, even: 0 }
    }
}

impl RoundState {
    /// Simply see in which phase we are (among initial, biddin, playing) and call the right method.
    fn update(round_state: RoundState, action: PlayerAction) -> RoundState {
        match (round_state, action) {
            (RoundState::Initial, PlayerAction::Nil) => RoundState::Bidding(BiddingState {
                deck: Deck::random_deck(),
                is_bidding: 0,
                bidding_history: vec![],
            }),
            (RoundState::Playing(playing_state), PlayerAction::PlayCard { player, card }) => {
                playing_state.update(player, card)
            }
            _ => todo!(),
        }
    }
}

impl PlayingState {
    /// Ask the game logic what to do and update the state accordingly.
    fn update(self, player: Player, card: Card) -> RoundState {

        // At this position we are shure the play is correct (not implemented) (attention anglais
        // pas tip top)
        //
        // Code à réorganiser avec un match sur le resultat de l'appel à la logique du jeu
        let mut deck;
        let mut is_playing;
        let mut history;
        let mut table: Vec<_>;
        let trump = self.trump;

        if self.table.len() < 3 {
            deck = self.deck;
            deck.delete_card(player, card);
            is_playing = (player + 1) % 4;
            history = self.history;
            table = self.table;
            table.push(card);
        } else if self.table.len() == 3 {
            deck = self.deck;
            deck.delete_card(player, card);
            is_playing = (player + 1) % 4;
            history = self.history;
            table = self.table;
            table.push(card);
        } else {
            panic!("wtf not possible in table.len() matching")
        }

        return RoundState::Playing(PlayingState {
            deck,
            is_playing,
            history,
            table,
            trump,
        });
    }
}
