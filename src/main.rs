use cardgame::{Deck, FrenchDeck, SpotItDeck};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut french_deck = FrenchDeck::default();
    french_deck.shuffle();

    println!("French deck has these cards: \n");

    dbg!(&french_deck.cards[..]);
    dbg!(&french_deck.cards.len());

    let mut spotit_deck = SpotItDeck::default();
    spotit_deck.shuffle();

    println!("SpotIt deck has these cards: \n");
    dbg!(&spotit_deck.cards);
    dbg!(&spotit_deck.cards.len());
}
