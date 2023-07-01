use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::thread_rng;
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
#[derive(EnumIter, Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
pub struct SpotItCard(HashSet<SpotItSuit>);

pub struct FrenchDeck {
    pub cards: Vec<FrenchCard>,
}

impl Default for FrenchDeck {
    fn default() -> Self {
        Self::new()
    }
}
impl Deck<FrenchCard> for FrenchDeck {
    fn new() -> Self {
        let cards = Vec::new();
        let mut deck = FrenchDeck { cards };
        for suit in FrenchSuit::iter() {
            for rank in FrenchRank::iter() {
                deck.cards.push(FrenchCard(rank, suit));
            }
        }
        deck
    }
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}
#[derive(Debug)]
pub struct SpotItDeck {
    pub cards: Vec<SpotItCard>,
}

impl Default for SpotItDeck {
    fn default() -> Self {
        Self::new()
    }
}
impl Deck<SpotItCard> for SpotItDeck {
    fn new() -> Self {
        let cards: Vec<SpotItCard> = Vec::new();
        let mut deck = SpotItDeck { cards };
        for suit in SpotItSuit::iter() {
            let mut card = SpotItCard(HashSet::new());
            card.0.insert(suit);
            deck.cards.push(card);
        }
        deck
    }
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

pub trait Deck<T> {
    fn new() -> Self;
    fn shuffle(&mut self) {}
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

    #[test]
    fn new_spotitdeck_has_31_cards() {
        let deck: SpotItDeck = SpotItDeck::default();
        assert_eq!(deck.cards.len(), 31);
    }
    #[test]
    fn each_spotitcard_has_1_suits() {
        let deck: SpotItDeck = SpotItDeck::default();
        for card in deck.cards {
            assert_eq!(card.0.len(), 1);
        }
    }
}
