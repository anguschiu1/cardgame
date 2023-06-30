use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Debug)]
pub struct Card(Rank, Suit);

pub struct Deck {}

impl Deck {
    pub fn new() -> Vec<Card> {
        let mut deck = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                deck.push(Card(rank.clone(), suit.clone()));
            }
        }
        return deck;
    }
    pub fn shuffle() {}
    pub fn draw_cards() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let deck = Deck {
    //         card: (Suit::Club, Rank::Two),
    //     };
    //     match deck.card {
    //         (Suit::Club, Rank::Two) => println!("hi"),
    //         _ => println!("bye."),
    //     }
    // }
    #[test]
    fn new_deck_has_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.len(), 52);
    }
    #[test]
    fn rank_is_cast_as_u8() {
        let ten = Rank::Ten as u8;
        assert_eq!(ten, 10);
    }
}
