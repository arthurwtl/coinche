//! This module contains the definitions of everything you need for the playing phase : cards, hands,
//! table, player, and the rules to play this phase.

// Add .shuffle trait to slices
use rand::seq::SliceRandom;
use std::ops::{Index, IndexMut, Deref};


// ========= Cards and deck definitions ==========

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Diamonds,
    Hearts,
    Clubs,
}

use Suit::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rank {
    Seven,
    Eigth,
    Nine,
    Jack,
    Queen,
    King,
    Ten,
    Ace,
}

use Rank::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Tricks are made of four cards.
pub type Trick = [Card; 4];

pub type Hand = Vec<Card>;

pub type Table = Vec<Card>;

/// Deck(tab[player][card_index])
#[derive(Clone, Debug, PartialEq)]
pub struct Deck(Vec<Vec<Card>>);

pub type Player = usize;

/// See [`playing_request`], the only fontion returning this output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayingRequestResult {
    Legal,
    Illegal,
    TrickWinned(usize),
}

/// A bid by a player
pub struct Bid {
    pub suit: Suit,
    pub val: u32,
}
// ------------------ Impl -------------------

impl Card {
    /// Create a new card.
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    fn normal_strenght(self) -> u32 {
        match self.rank {
            Seven => 0,
            Eigth => 1,
            Nine => 2,
            Jack => 3,
            Queen => 4,
            King => 5,
            Ten => 6,
            Ace => 7,
        }
    }

    fn trump_strenght(self) -> u32 {
        match self.rank {
            Seven => 10,
            Eigth => 11,
            Queen => 12,
            King => 13,
            Ten => 14,
            Ace => 15,
            Nine => 16,
            Jack => 17,
        }
    }

    /// To be able to compare cards knowing the trump suit.
    pub fn strength(self, trump: Suit, asked: Suit) -> u32 {
        if self.suit == trump {
            self.trump_strenght()
        } else if self.suit == asked {
            self.normal_strenght()
        } else {
            0
        }
    }
}

impl Deck {
    /// Create a new random deck.
    pub fn random_deck() -> Deck {
        let mut rng = rand::rng();
        let mut tab = [Spades, Diamonds, Hearts, Clubs]
            .into_iter()
            .flat_map(|s| {
                vec![Seven, Eigth, Nine, Ten, Jack, King, Queen, Ace]
                    .into_iter()
                    .map(|r| Card::new(r, s))
                    .collect::<Vec<Card>>()
            })
            .collect::<Vec<Card>>();
        tab.shuffle(&mut rng);
        // here tab is a flat shuffled set of all the cards.
        let tab = tab
            .chunks_exact(8)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<Card>>>();
        Deck(tab)
    }

    /// Deleate a card when called on a deck, and given a player and a card.
    ///
    /// # Panic
    /// If the card is not in the player's hand.
    pub fn delete_card(self, player: Player, card: Card) -> Self {
        let Deck(mut tab) = self;
        if tab[player].contains(&card) {
            println!("carte détectée");
            tab[player].retain(|c| *c != card);
            Deck(tab)
        } else {
            panic!("Tring to delete a card, but the card dosen't exists");
        }
    }
}

impl Index<usize> for Deck {
    type Output = Vec<Card>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Deck {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Deref for Deck {
    type Target = Vec<Vec<Card>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Examine a table and return the index of the card currently winning
///
/// # Requirement
/// the table must not be empty
fn master(table: &Table, trump: Suit) -> usize {
    table
        .iter()
        .enumerate()
        .max_by_key(|(_i, c)| c.strength(trump, table[0].suit))
        .unwrap()
        .0
}

/// Contains the rules for a deal. When called on a table and a hand, says if the play was
/// legal and if someone win the trick by returning an enum.
pub fn playing_request(
    table: &Table,
    hand: &Hand,
    trump: Suit,
    card: Card,
) -> PlayingRequestResult {
    // The difference between last to play and second and third is made after determinating weather
    // the move is legal, so we can't return in the big else block.
    let return_value;
    println!("====== coucou de la part de playing_request");

    // You must have the card to play in
    if !hand.contains(&card) {
        println!("You must have the card to play in");
        return PlayingRequestResult::Illegal;
    }

    // If first to play, play anything
    if table.is_empty() {
        println!("If first to play, play anything");
        return_value = PlayingRequestResult::Legal;
    }
    // If normal suit required
    else if table[0].suit != trump {
        println!("If normal suit required");
        // Always legal to play the same color as the first one to play
        if card.suit == table[0].suit {
            println!("Always legal to play the same color as the first one to play");
            return_value = PlayingRequestResult::Legal;
        }
        // But illegal to play a different color if I can play what's asked
        else if hand
            .iter()
            .inspect(|c| println!("     {:?} ?= {:?}", c.suit, table[0].suit))
            .any(|c: &Card| c.suit == table[0].suit)
        {
            println!("But illegal to play a different color if I can play what's asked");
            println!("asked : {:?}, you have some of that", table[0].suit);
            return_value = PlayingRequestResult::Illegal;
        }
        // If I play a different color because I don't have the color asked
        else {
            println!("If I play a different color because I don't have the color asked");
            // Check if I'm playing trump, which is always legal here
            if card.suit == trump {
                println!("Check if I'm playing trump, which is always legal here");
                return_value = PlayingRequestResult::Legal;
            }
            // Else if my partner is winning the trick it's still legal.
            else if table.len() >= 2 && master(table, trump) == table.len() - 2 {
                println!("Else if my partner is winning the trick it's still legal.");
                return_value = PlayingRequestResult::Legal;
            }
            // Else illegal
            else {
                println!("Else illegal");
                return_value = PlayingRequestResult::Illegal;
            }
        }
    }
    // If a trump is required
    else {
        println!("If a trump is required");
        // And I play trump
        if card.suit == trump {
            println!("And I play trump");
            // If my card is the stongest among the cards on the table, it's legal
            let current_max = table
                .iter()
                .max_by_key(|c| c.strength(trump, trump))
                .expect("Impossible senario, emptiness check already happend");
            if card.strength(trump, trump) > current_max.strength(trump, trump) {
                return_value = PlayingRequestResult::Legal;
            }
            // Else it's legal only if I can't play on top
            else {
                println!("Else it's legal only if I can't play on top");
                let my_max_trump = hand
                    .iter()
                    .max_by_key(|c| c.strength(trump, trump))
                    .expect("Impossible senario once again, emptiness check already happend");
                if my_max_trump.strength(trump, trump) < current_max.strength(trump, trump) {
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
        return_value
    } else {
        match return_value {
            PlayingRequestResult::Illegal => PlayingRequestResult::Illegal,
            PlayingRequestResult::Legal => {
                let mut _tmp = table.clone();
                _tmp.push(card);
                let (index, _max) = _tmp
                    .iter()
                    .enumerate()
                    .inspect(|truc| {
                        println!("    {truc:?}, {}", truc.1.strength(trump, table[0].suit))
                    })
                    .max_by_key(|(_i, c)| c.strength(trump, table[0].suit))
                    .expect("Impossible senario again, emptiness check already happend");
                PlayingRequestResult::TrickWinned(index)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    // use rand::seq::index;
    use super::*;
    use PlayingRequestResult::*;
    use std::collections::HashSet;

    #[test]
    fn card_ordering_test() {
        let c1 = Card::new(Seven, Diamonds);
        let c2 = Card::new(Eigth, Diamonds);
        assert!(c1.normal_strenght() < c2.normal_strenght());
        assert!(!(c2.normal_strenght() < c1.normal_strenght()));
        assert!(c1 == c1);
    }

    #[test]
    fn random_deck_test() {
        // Correct partition
        let deck1 = Deck::random_deck();
        assert!(deck1.0.len() == 4);
        for i in 0..=3 {
            assert!(deck1[i].len() == 8);
        }
        // Not twice the same card in a deck
        let mut seen = HashSet::new();
        let unicity = deck1.0.iter().all(|x| seen.insert(x));
        assert!(unicity);
        // Not twice the same deck
        let deck2 = Deck::random_deck();
        assert!(deck1 != deck2);
    }

    #[test]
    fn delete_card_test() {
        let deck = Deck::random_deck();
        let first_card = deck[0][0];
        println!("La première carte est : {:?}\n", first_card);
        println!("{:?}\n len = {}\n", deck, deck.0.len());
        let deck = deck.delete_card(0, first_card);
        println!("{:?}\n len = {}\n", deck, deck.0.len());
        assert!(first_card != deck[0][0]);
        assert!(deck.0[0].len() == 7);
    }

    #[test]
    fn strenth_and_master_test() {
        let trump = Diamonds;
        let asked = Spades;
        let strong = Card::new(Ace, trump);
        let weak = Card::new(Seven, asked);
        let pretty_strong = Card::new(Seven, trump);
        let pretty_weak = Card::new(Ten, asked);

        assert!(strong.strength(trump, asked) > weak.strength(trump, asked));
        assert!(pretty_strong.strength(trump, asked) > pretty_weak.strength(trump, asked));
        assert!(pretty_strong.strength(trump, asked) < strong.strength(trump, asked));
        assert!(weak.strength(trump, asked) < pretty_weak.strength(trump, asked));
        println!(
            "{} > {}",
            strong.strength(trump, asked),
            weak.strength(trump, asked)
        );
        println!(
            "{} > {}",
            pretty_strong.strength(trump, asked),
            pretty_weak.strength(trump, asked)
        );
        println!(
            "{} < {}",
            pretty_strong.strength(trump, asked),
            strong.strength(trump, asked)
        );
        println!(
            "{} < {}",
            strong.strength(trump, asked),
            pretty_strong.strength(trump, asked)
        );

        let table = vec![strong, pretty_strong, pretty_weak, weak];
        assert!(master(&table, trump) == 0);
        let table = vec![pretty_strong, pretty_weak, weak, strong];
        assert!(master(&table, trump) == 3);
    }

    #[test]
    #[rustfmt::skip] // Comments indentation is meanigful
    fn playing_request_test() {
        // You must have the card to play in
        let table: Table = vec![];
        let hand = vec![
            Card::new(Seven, Diamonds),
            Card::new(Nine, Spades),
            Card::new(Ten, Hearts),
            Card::new(Seven, Spades),
        ];
        let trump = Hearts;
        let dont_have_it = Card::new(Eigth, Diamonds);
        assert!(playing_request(&table, &hand, trump, dont_have_it) == Illegal);
        // println!("coucou 0 {:?}", playing_request(&table, &hand, trump, dont_have_it));
        // If first to play, play anything
        // println!("coucou 1 {:?}", playing_request(&table, &hand, trump, hand[0]));
        // println!("coucou 2 {:?}", playing_request(&table, &hand, trump, hand[1]));
        // println!("coucou 3 {:?}", playing_request(&table, &hand, trump, hand[2]));
        // println!("coucou 4 {:?}", playing_request(&table, &hand, trump, hand[3]));
        assert!(playing_request(&table, &hand, trump, hand[0]) == Legal);
        assert!(playing_request(&table, &hand, trump, hand[1]) == Legal);
        assert!(playing_request(&table, &hand, trump, hand[2]) == Legal);
        assert!(playing_request(&table, &hand, trump, hand[3]) == Legal);
        // If normal suit required
        let table = vec![Card::new(Eigth, Spades), Card::new(Jack, Clubs)];
            // Always legal to play the same color as the first one to play
        assert!(playing_request(&table, &hand, trump, hand[3]) == Legal);
        assert!(playing_request(&table, &hand, trump, hand[1]) == Legal);
            // But illegal to play a different color if I can play what's asked
        assert!(playing_request(&table, &hand, trump, hand[0]) == Illegal);
        assert!(playing_request(&table, &hand, trump, hand[2]) == Illegal);
            // If I play a different color because I don't have the color asked
        let table = vec![Card::new(Eigth, Clubs), Card::new(Seven, Clubs)];
                // Check if I'm playing trump, which is always legal here
        assert!(playing_request(&table, &hand, trump, hand[2]) == Legal);
                // Else if my partner is winning the trick it's still legal.
        let table = vec![Card::new(Jack, Clubs), Card::new(Eigth, Clubs)];
        assert!(playing_request(&table, &hand, trump, hand[0]) == Legal);
                // Else illegal
        let table = vec![Card::new(Eigth, Clubs), Card::new(Jack, Clubs)];
        assert!(playing_request(&table, &hand, trump, hand[0]) == Illegal);
        // If a trump is required
        let table = vec![Card::new(Queen, Hearts), Card::new(Jack, Clubs)];
            // And I play trump
                // If my card is the stongest among the cards on the table, it's legal
        assert!(playing_request(&table, &hand, trump, hand[2]) == Legal);
                // Else it's legal only if I can't play on top
        let table = vec![Card::new(Jack, Hearts), Card::new(Jack, Clubs)];
        assert!(playing_request(&table, &hand, trump, hand[2]) == Legal);
        let table = vec![Card::new(Queen, Hearts), Card::new(Jack, Clubs)];
        let hand = vec![
            Card::new(Jack, Hearts),
            Card::new(Nine, Spades),
            Card::new(Eigth, Hearts),
            Card::new(Seven, Spades),
        ];
        assert!(playing_request(&table, &hand, trump, hand[2]) == Illegal);
            // If I don't play trump, it's legal only if y don't have any trump, and no other conditions
        assert!(playing_request(&table, &hand, trump, hand[1]) == Illegal);
        let hand = vec![
            Card::new(Jack, Diamonds),
            Card::new(Nine, Spades),
            Card::new(Eigth, Diamonds),
            Card::new(Seven, Diamonds),
        ];
        assert!(playing_request(&table, &hand, trump, hand[1]) == Legal);

    }

    #[test]
    fn test_playing_request_winner() {
        // Now we just have to check if this close the trick, and if so return the index
        // of the card taking the trick.
        // About to play the fourth card
        let trump = Diamonds;
        let hand = vec![
            Card::new(Jack, Diamonds),
            Card::new(Nine, Spades),
            Card::new(Eigth, Diamonds),
            Card::new(Seven, Diamonds),
        ];
        // No trump
        let table = vec![
            Card::new(Eigth, Spades),
            Card::new(Jack, Clubs),
            Card::new(Queen, Spades),
        ];
        assert!(playing_request(&table, &hand, trump, hand[1]) == TrickWinned(2));
        let table = vec![
            Card::new(Eigth, Spades),
            Card::new(Jack, Clubs),
            Card::new(Seven, Spades),
        ];
        assert!(playing_request(&table, &hand, trump, hand[1]) == TrickWinned(3));
        // Trump
        let table = vec![
            Card::new(Eigth, Spades),
            Card::new(Nine, Diamonds),
            Card::new(Seven, Spades),
        ];
        assert!(playing_request(&table, &hand, trump, hand[1]) == TrickWinned(1));
        let table = vec![
            Card::new(Eigth, Hearts),
            Card::new(Nine, Diamonds),
            Card::new(Seven, Spades),
        ];
        assert!(playing_request(&table, &hand, trump, hand[0]) == TrickWinned(3));
        let table = vec![
            Card::new(Eigth, Hearts),
            Card::new(Nine, Diamonds),
            Card::new(Seven, Spades),
        ];
        let hand = vec![
            Card::new(Ten, Diamonds),
            Card::new(Nine, Spades),
            Card::new(Eigth, Diamonds),
            Card::new(Seven, Diamonds),
        ];
        // 1 is my partner
        assert!(playing_request(&table, &hand, trump, hand[0]) == TrickWinned(1));
        assert!(playing_request(&table, &hand, trump, hand[1]) == TrickWinned(1));

        // 2 is not my partner
        let table = vec![
            Card::new(Eigth, Hearts),
            Card::new(Seven, Spades),
            Card::new(Nine, Diamonds),
        ];
        assert!(playing_request(&table, &hand, trump, hand[0]) == TrickWinned(2));
        assert!(playing_request(&table, &hand, trump, hand[1]) == Illegal);
    }
}
