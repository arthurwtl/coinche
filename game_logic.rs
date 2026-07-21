//! This module contains the definitions of everything you need to play coinche : cards, hands,
//! table, player, and the rules to manipulate them. The server logic dosen't know the rules and
//! obey this module.

// Un trait qui rajoute .shuffle au slices
use rand::seq::SliceRandom;

// ========= Cards and deck definitions ==========

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

/// Tricks are always made of four cards.
pub type Trick = [Card; 4];

pub type Hand = Vec<Card>;

pub type Table = Vec<Card>;

/// Deck(tab[player][card_index])
#[derive(Clone, Debug, PartialEq)]
pub struct Deck(Vec<Vec<Card>>);

pub type Player = usize;

/// See [`playing_request`], the only fontion returning this output.
pub enum PlayingRequestResult {
    Legal,
    Illegal,
    TrickWinned(Player),
}

// ------------------ Impl -------------------

impl Card {
    /// Create a new card.
    pub fn new(rank: u8, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

impl Deck {
    /// Create a new random deck.
    pub fn random_deck() -> Deck {
        let mut rng = rand::rng();
        let mut tab = [Suit::Spades, Suit::Diamonds, Suit::Hearts, Suit::Clubs]
            .into_iter()
            .map(|c| (7..=14).map(|i| Card::new(i, c)).collect::<Vec<Card>>())
            .flatten()
            .collect::<Vec<Card>>();
        tab.shuffle(&mut rng);
        // here tab is a flat shuffled set of all the cards.
        let tab = tab
            .chunks_exact(8)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<Card>>>();
        return Deck(tab);
    }

    /// Deleate a card when called on a deck, and given a player and a card.
    ///
    /// # Panic
    /// If the card is not in the player's hand.
    pub fn delete_card(&mut self, player: Player, card: Card) {
        let Deck(tab) = self;
        if tab[player].contains(&card) {
            tab[player].retain(|c| *c == card);
        } else {
            panic!("Tring to delete a card, but the card dosen't exists");
        }
    }
}

/// Contains the rules for a deal. When called on a table and a hand, says if the play was
/// legal and if someone win the trick by returning an enum
pub fn playing_request(table: Table, hand: Hand, trump: Suit, card: Card) -> PlayingRequestResult {
    // Premier à jouer 
    if table.is_empty() {
        return PlayingRequestResult::Legal

    // Tour normal
    } else if table[0].suit != trump {
        if card.suit == table[0].suit {
           return PlayingRequestResult::Legal;
        } else if table.iter().any(|c: &Card| c.suit == card.suit) {
            return PlayingRequestResult::Illegal;
        } else {
            return PlayingRequestResult::Legal;
        }

    // Tour d'atout
    } else {
       todo!(); 
    } 

    // unreachable!();
}

