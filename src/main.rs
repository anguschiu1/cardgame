use cardgame::{FrenchDeck, SpotItDeck};

fn main() {
    let french_deck = FrenchDeck::default();

    println!("French deck has these cards: \n");

    dbg!(&french_deck.cards[..]);
    dbg!(&french_deck.cards.len());

    let spotit_deck = SpotItDeck::default();

    println!("SpotIt deck has these cards: \n");
    dbg!(&spotit_deck.cards);
    dbg!(&spotit_deck.cards.len());
}
