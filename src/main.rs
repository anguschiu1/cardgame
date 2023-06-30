use cardgame::{Deck, Rank, Suit};

fn main() {
    let deck = Deck::new();
    dbg!(&deck[..]);
}
