use coinche::logic::card_logic::*;
// use coinche_logic::server_logic::*;

fn main() {
    let new_deck = Deck::random_deck();
    println!("{new_deck:?}");
}
