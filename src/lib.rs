pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    J,
    Q,
    K,
    A,
}
pub struct Deck {
    pub card: (Suit, Rank),
}

impl Deck {
    pub fn shuffle() {}
    pub fn new() {}
    pub fn draw_cards() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let deck = Deck {
            card: (Suit::Club, Rank::Two),
        };
        match deck.card {
            (Suit::Club, Rank::Two) => println!("hi"),
            _ => println!("bye."),
        }
    }
}
