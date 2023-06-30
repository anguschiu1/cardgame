use cardgame::{FrenchDeck, FrenchRank, FrenchSuit};

fn main() {
    let deck = FrenchDeck::new();
    dbg!(&deck.cards[..]);
}
