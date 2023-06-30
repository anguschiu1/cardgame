use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum FrenchSuit {
    Club = 1,
    Diamond,
    Heart,
    Spade,
}
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum FrenchRank {
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
    Ace, // will be 14
}
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum SpotItSuit {
    Apple,
    Banana,
    Bread,
    Broccoli,
    Carrot,
    Cheese,
    Chicken,
    Corn,
    Egg,
    Fish,
    Grape,
    Hamburger,
    Hotdog,
    Icecream,
    Lettuce,
    Milk,
    Mushroom,
    Onion,
    Orange,
    Pea,
    Pear,
    Pepper,
    Pineapple,
    Pizza,
    Potato,
    Pumpkin,
    Strawberry,
    Tomato,
    Watermelon,
    Yogurt,
}

#[derive(Debug)]
pub struct FrenchCard(FrenchRank, FrenchSuit);
pub struct SpotItCard(SpotItSuit);

pub struct FrenchDeck {
    pub cards: Vec<FrenchCard>,
}

impl Default for FrenchDeck {
    fn default() -> Self {
        Self::new()
    }
}
impl FrenchDeck {
    pub fn new() -> Self {
        let cards = Vec::new();
        let mut deck = FrenchDeck { cards };
        for suit in FrenchSuit::iter() {
            for rank in FrenchRank::iter() {
                deck.cards.push(FrenchCard(rank, suit));
            }
        }
        deck
    }
    pub fn shuffle() {}
    pub fn draw_cards() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_frenchdeck_has_52_cards() {
        let deck: FrenchDeck = FrenchDeck::default();
        assert_eq!(deck.cards.len(), 52);
    }
    #[test]
    fn french_rank_is_cast_as_u8() {
        let ten = FrenchRank::Ten as u8;
        assert_eq!(ten, 10);
        let ace = FrenchRank::Ace as u8;
        assert_eq!(ace, 14);
    }
}
