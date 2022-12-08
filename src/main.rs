use cardgame::{Deck, Rank, Suit};

fn main() {
    let deck = Deck {
        card: (Suit::Club, Rank::Two),
    };
    match deck.card {
        (Suit::Club, Rank::Two) => println!("♣️2"),
        _ => println!("bye."),
    }
}
