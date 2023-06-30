use cardgame::FrenchDeck;

fn main() {
    let deck = FrenchDeck::new();
    dbg!(&deck.cards[..]);
    dbg!(&deck.cards.len());
}
