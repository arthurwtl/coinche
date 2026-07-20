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

pub type Trick = [Card; 4];

pub type Hand = Vec<Card>;

pub type Table = Vec<Card>;

#[derive(Clone, Debug, PartialEq)]
pub struct Deck(Vec<Vec<Card>>);

pub type Player = usize;

pub enum PlayingRequestResult {
    Legal,
    Illegal,
    TrickWinned(Player),
}

// ------------------ Impl -------------------

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Card {
        Card { rank, suit }
    }
}


impl Deck {
    // Pas encore aléatoire
    pub fn random_deck() -> Deck {
        let mut rng = rand::rng();
        let mut tab = [Suit::Spades, Suit::Diamonds, Suit::Hearts, Suit::Clubs]
            .into_iter()
            .map(|c| (7..=14).map(|i| Card::new(i, c)).collect::<Vec<Card>>())
            .flatten()
            .collect::<Vec<Card>>();
        tab.shuffle(&mut rng);
        // here tab is a flat shuffled set of all the cards
        let tab = tab.chunks_exact(8)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<Card>>>();
        return Deck(tab)
    }

    pub fn delete_card(&mut self, player: Player, card: Card) {
        let Deck(tab) = self;
        if tab[player].contains(&card) {
            tab[player].retain(|c| *c == card);
        } else { 
            panic!("Tring to delete a card, but the card dosen't exists"); 
        } 
    }
}

/// Contains the rules of a deal. When called on a table and a hand, says if the play was
/// legal and if someone win the trick by returning an enum
pub fn playing_request(table: Table, hand: Hand) -> PlayingRequestResult {
    PlayingRequestResult::Illegal
}


