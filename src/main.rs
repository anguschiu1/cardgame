use cardgame::{Deck, FrenchDeck, SpotItDeck};

fn main() {
    // let mut french_deck = FrenchDeck::default();
    // french_deck.shuffle();

    // println!("French deck has these cards: \n");

    // dbg!(&french_deck.cards[..]);
    // dbg!(&french_deck.cards.len());

    // let mut spotit_deck = SpotItDeck::default();
    // spotit_deck.shuffle();

    // println!("SpotIt default deck has these cards: \n");
    // dbg!(&spotit_deck.cards);
    // dbg!(&spotit_deck.cards.len());

    // let mut spotit_deck = SpotItDeck::generate_by_prime(2).unwrap();

    // println!("SpotIt prime of 2 has a deck of these cards: \n");
    // dbg!(&spotit_deck.cards);
    // dbg!(&spotit_deck.cards.len());

    let mut spotit_deck = SpotItDeck::generate_by_prime(3).unwrap();
    // dbg!(&spotit_deck.cards);
}
