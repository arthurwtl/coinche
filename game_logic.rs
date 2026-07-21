//! This module contains the definitions of everything you need to play coinche : cards, hands,
//! table, player, and the rules to manipulate them. The server logic dosen't know the rules and
//! obey this module.

// Add .shuffle trait to slices
use rand::seq::SliceRandom;
use std::cmp::Ordering;

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
    TrickWinned(usize),
}

// ------------------ Impl -------------------

impl Card {
    /// Create a new card.
    pub fn new(rank: u8, suit: Suit) -> Card {
        Card { rank, suit }
    }

    /// To be able to compare cards knowing the trump suit.
    /// The rank of a card must be between 0 and 14. (game invariant)
    pub fn strength(self, trump: Suit) -> u8 {
        if self.suit == trump {
            42 + self.rank
        } else {
            self.rank
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
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

/// Examine a table and return the index of the card currently winning or none if the table is empty
fn master(table: &Table, trump: Suit) -> Option<usize> {
    let res = table
        .iter()
        .enumerate()
        .max_by_key(|(_i, c)| c.strength(trump));
    match res {
        None => None,
        Some((i, _c)) => Some(i),
    }
}

/// Contains the rules for a deal. When called on a table and a hand, says if the play was
/// legal and if someone win the trick by returning an enum.
pub fn playing_request(table: Table, hand: Hand, trump: Suit, card: Card) -> PlayingRequestResult {
    // The difference between last to play and second and third is made after determinating weather
    // the move is legal, so we can't return in the big else block.
    let return_value;

    // You must have the card to play in 
    if !hand.contains(&card) {
        return PlayingRequestResult::Illegal;
    }

    // If first to play, play anything
    if table.is_empty() {
        return_value = PlayingRequestResult::Legal;
    }
    // If normal suit required
    else if table[0].suit != trump {
        // Always legal to play the same color as the first one to play
        if card.suit == table[0].suit {
            return_value = PlayingRequestResult::Legal;
        }
        // But illegal to play a different color if I can play what's asked
        else if hand.iter().any(|c: &Card| c.suit == card.suit) {
            return_value = PlayingRequestResult::Illegal;
        }
        // If I play a different color because I don't have the color asked
        else {
            // Check if I'm playing trump, which is always legal here
            if card.suit == trump {
                return_value = PlayingRequestResult::Legal;
            }
            // Else if my partner is winning the trick it's still legal.
            else if table.len() >= 2 && master(&table, trump).unwrap() == table.len() - 3 {
                return_value = PlayingRequestResult::Legal;
            }
            // Else illegal
            else {
                return_value = PlayingRequestResult::Illegal;
            }
        }
    }
    // If a trump is required
    else {
        // And I play trump
        if card.suit == trump {
            // If my card is the stongest among the cards on the table, it's legal
            let current_max = table
                .iter()
                .max_by_key(|c| c.strength(trump))
                .expect("Impossible senario, emptiness check already happend");
            if card.strength(trump) > current_max.strength(trump) {
                return_value = PlayingRequestResult::Legal;
            }
            // Else it's legal only if I can't play on top 
            else {
                let my_max_trick = hand
                    .iter()
                    .max_by_key(|c| c.strength(trump))
                    .expect("Impossible senario once again, emptiness check already happend");
                if my_max_trick < current_max { 
                    return_value = PlayingRequestResult::Legal;
                } else {
                    return_value = PlayingRequestResult::Illegal;
                }
            }
        }
        // If I don't play trump, it's legal only if y don't have any trump, and no other conditions
        else {
            if hand.iter().any(|c: &Card| c.suit == trump) {
                return_value = PlayingRequestResult::Illegal;
            } else {
                return_value = PlayingRequestResult::Legal;
            }
        }
    };

    // Now we just have to check if this close the trick, and if so return the index
    // of the card taking the trick.

    // About to play the fourth card
    if table.len() != 3 {
        return return_value;
    } else {
        return match return_value {
            PlayingRequestResult::Illegal => PlayingRequestResult::Illegal,
            PlayingRequestResult::Legal => {
                let mut _tmp = table.clone();
                _tmp.push(card);
                let (index, _max) = _tmp
                    .iter()
                    .enumerate()
                    .max_by_key(|(i, c)| c.strength(trump))
                    .expect("Impossible senario again, emptiness check already happend");
                PlayingRequestResult::TrickWinned(index)
            }
            _ => unreachable!(),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn card_ordering_test() {
        let c1 = Card::new(7, Suit::Diamonds);
        let c2 = Card::new(8, Suit::Diamonds);
        assert!(c1 < c2);
        assert!(!(c2 < c1));
        assert!(c1 == c1);
    }
}
