use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FrenchSuit {
    Club = 1,
    Diamond,
    Heart,
    Spade,
}
#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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
#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Eq, Hash)]
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
pub trait Deck {
    fn default() -> Self;
    fn new() -> Self;
    fn shuffle(&mut self);
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FrenchCard(FrenchRank, FrenchSuit);

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct SpotItCard(HashSet<SpotItSuit>);
#[derive(Debug)]
pub struct FrenchDeck {
    pub cards: Vec<FrenchCard>,
}
#[derive(Debug)]
pub struct SpotItDeck {
    pub cards: Vec<SpotItCard>,
}
impl Deck for FrenchDeck {
    fn default() -> Self {
        Self::new()
    }
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

impl Deck for SpotItDeck {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_frenchdeck_has_52_cards() {
        let deck: FrenchDeck = FrenchDeck::default();
        assert_eq!(deck.cards.len(), 52);
    }
    #[test]
    fn every_card_on_frenchdeck_is_unique() {
        let deck: FrenchDeck = FrenchDeck::default();
        let mut uniq = HashSet::new();
        assert!(deck.cards.iter().all(move |card| uniq.insert(card)));
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
        assert_eq!(deck.cards.len(), 30);
    }
    #[test]
    fn can_compare_spotitcard() {
        let card1 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        let card2 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        assert_eq!(card1, card2);
    }
    #[test]
    fn can_add_suit_to_spotitcard() {
        let mut card1 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        card1.0.insert(SpotItSuit::Apple);
        let card2 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        assert_ne!(card1, card2);
    }

    #[test]
    fn every_card_on_spotitdeck_is_unique() {
        let mut deck: SpotItDeck = SpotItDeck::default();
        for _ in 1..=deck.cards.len() {
            let card = deck.cards.pop().unwrap();
            assert!(deck.cards.iter().all(move |c| c != &card));
        }
    }
    #[test]
    fn can_add_a_card_on_spotitdeck() {
        let card = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        assert_eq!(card.0.get(&SpotItSuit::Potato), Some(&SpotItSuit::Potato));
    }
    #[test]
    fn each_spotitcard_has_1_suits() {
        let deck: SpotItDeck = SpotItDeck::default();
        for card in deck.cards {
            assert_eq!(card.0.len(), 1);
        }
    }
}
