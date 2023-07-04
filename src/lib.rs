#![allow(missing_docs)]
#![warn(rustdoc::private_doc_tests)]

//! Core library for the `cardgame` crate.
//!
//! Provide the core functionality to play SpotIt!.
//! Auxillary functions to play other card games like French Card game, but it is for testing purposes only.
//!
//! [`Repository`]: https://github.com/anguschiu1/cardgame
//! [`SpotIt!`]: https://www.amazon.com/Asmodee-SP411-Spot-It/dp/B0039S7NO6
//! [`French Card Game`]: https://en.wikipedia.org/wiki/French_playing_cards
//! [`SpotIt! Rules`]: https://www.ultraboardgames.com/spot-it/game-rules.php

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// This defines the suits of the French Card Game.
#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum FrenchSuit {
    /// This is a club, not a clover. The number 1 is not used in the French Card Game, but it is for the ease to compare power of suits in French cards.
    Club = 1,
    /// This is a diamond.
    Diamond,
    /// This is a heart.
    Heart,
    /// This is a spade.
    Spade,
}
/// This defines the ranks of the French Card Game.
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
/// Thie enum defines the suits (or pattern) of the SpotIt! game. One card can has one or more suits.
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
/// This trait defines the common functionality of a deck of cards.
pub trait Deck<T> {
    /// This function creates a new, empty deck of cards.
    fn new() -> Self;
    /// This function creates a new deck of cards with default values, e.g. a deck of French Cards with 52 cards, or a deck of SpotIt Cards with 30 cards.
    fn default() -> Self;
    /// This function mututate original deck and shuffles the deck of cards.
    fn shuffle(&mut self);
    /// This function pops one card from the deck of cards.
    fn pop_card(&mut self) -> Option<T>;
}
/// This tuple struct defines a French Card.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct FrenchCard(FrenchRank, FrenchSuit);

impl FrenchCard {
    /// This function returns the rank of the card.
    pub fn rank(&self) -> FrenchRank {
        self.0
    }
    /// This function returns the suit of the card.
    pub fn suit(&self) -> FrenchSuit {
        self.1
    }
    /// This function returns true if the suit of the card is the same as the suit of the other card.
    pub fn match_suit(&self, card: &Self) -> bool {
        self.1 == card.1
    }
    /// This function returns true if the rank of the card is the same as the rank of the other card.
    pub fn match_rank(&self, card: &Self) -> bool {
        self.0 == card.0
    }
}

/// This tuple struct defines a SpotIt Card.
/// Please notice that a SpotIt Card can have 0, 1, or more than one suits. For example, a card can have both Apple and Banana suits.
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct SpotItCard(HashSet<SpotItSuit>);
impl SpotItCard {
    /// This function returns true if the two cards have exactly one suit in common.
    /// This is the key rule of the SpotIt game.
    pub fn match_exactly_one_suit(&self, card: &Self) -> bool {
        self.0.intersection(&card.0).count() == 1
    }
}
/// This struct defines a deck of French Cards.
#[derive(Debug)]
pub struct FrenchDeck {
    /// This is a vector storing a deck of French Cards.
    pub cards: Vec<FrenchCard>,
}
/// This struct defines a deck of SpotIt Cards.
#[derive(Debug)]
pub struct SpotItDeck {
    /// This is a vector storing a deck of SpotIt Cards.
    pub cards: Vec<SpotItCard>,
}
impl Deck<FrenchCard> for FrenchDeck {
    fn default() -> Self {
        let mut deck = Self::new();
        for suit in FrenchSuit::iter() {
            for rank in FrenchRank::iter() {
                deck.cards.push(FrenchCard(rank, suit));
            }
        }
        deck
    }
    fn new() -> Self {
        let cards = Vec::new();
        FrenchDeck { cards }
    }
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn pop_card(&mut self) -> Option<FrenchCard> {
        self.cards.pop()
    }
}

impl Deck<SpotItCard> for SpotItDeck {
    fn default() -> Self {
        let mut deck = Self::new();
        for suit in SpotItSuit::iter() {
            let mut card = SpotItCard(HashSet::new());
            card.0.insert(suit);
            deck.cards.push(card);
        }
        deck
    }
    fn new() -> Self {
        let cards: Vec<SpotItCard> = Vec::new();
        SpotItDeck { cards }
    }
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    fn pop_card(&mut self) -> Option<SpotItCard> {
        self.cards.pop()
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
    fn can_compare_frenchcard() {
        let card1 = FrenchCard(FrenchRank::Ace, FrenchSuit::Spade);
        let card2 = FrenchCard(FrenchRank::Ace, FrenchSuit::Spade);
        let card3 = FrenchCard(FrenchRank::Ace, FrenchSuit::Heart);
        let card4 = FrenchCard(FrenchRank::King, FrenchSuit::Heart);
        assert_eq!(card1, card2); // two cards with same rank and suit
        assert_ne!(card2, card3); // two cards with same rank but different suit
        assert_ne!(card3, card4); // two cards with same suit but different rank
    }
    #[test]
    fn can_compare_frenchcard_by_rank_and_suit() {
        let card1 = FrenchCard(FrenchRank::Ace, FrenchSuit::Spade);
        let card2 = FrenchCard(FrenchRank::Ace, FrenchSuit::Spade);
        let card3 = FrenchCard(FrenchRank::Ace, FrenchSuit::Heart);
        let card4 = FrenchCard(FrenchRank::King, FrenchSuit::Heart);
        assert!(card1.match_rank(&card2)); // two cards with same rank and suit
        assert!(card1.match_rank(&card3)); // two cards with same rank but different suit
        assert!(!card1.match_rank(&card4)); // two cards with different rank and suit
        assert!(!card3.match_rank(&card4)); // two cards with different rank but same suit

        assert!(card1.match_suit(&card2)); // two cards with same rank and suit
        assert!(card3.match_suit(&card4)); // two cards with same suit but different rank
        assert!(!card1.match_suit(&card3)); // two cards with same rank but different suit
        assert!(!card1.match_suit(&card4)); // two cards with different suit and rank
    }
    #[test]
    fn can_push_pop_same_card_on_frenchdeck() {
        let mut deck: FrenchDeck = FrenchDeck::new();
        let card = FrenchCard(FrenchRank::Ace, FrenchSuit::Spade);
        deck.cards.push(card.clone());
        assert_eq!(deck.cards.pop(), Some(card));
    }
    #[test]

    fn can_use_fn_to_pop_card_from_frenchdeck() {
        let mut deck: FrenchDeck = FrenchDeck::new();
        let card = FrenchCard(FrenchRank::Ace, FrenchSuit::Spade);
        deck.cards.push(card.clone());
        assert_eq!(deck.pop_card(), Some(card));
    }
    #[test]
    fn new_spotitdeck_has_30_cards() {
        let deck: SpotItDeck = SpotItDeck::default();
        assert_eq!(deck.cards.len(), 30);
    }
    #[test]
    fn can_compare_spotitcard() {
        let card1 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        let card2 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        let card3 = SpotItCard(HashSet::from([SpotItSuit::Apple]));
        let card4: SpotItCard = SpotItCard(HashSet::from([SpotItSuit::Potato, SpotItSuit::Apple]));
        let card5 = SpotItCard(HashSet::from([SpotItSuit::Potato, SpotItSuit::Apple]));
        let card6 = SpotItCard(HashSet::from([SpotItSuit::Apple, SpotItSuit::Apple]));
        assert_eq!(card1, card2); // two cards with same one suit
        assert_ne!(card2, card3); // two cards with different suits
        assert_ne!(card3, card4); // two cards with non-identical set of suit
        assert_ne!(card1, card4); // two cards with non-identical set of suit
        assert_eq!(card4, card5); // two cards with same set of suit
        assert_eq!(card3, card6); // two cards with same suit, even one suit is added twice
    }

    #[test]
    fn can_match_exactly_one_suit_on_two_spotitcard() {
        let card1 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        let card2: SpotItCard = SpotItCard(HashSet::from([SpotItSuit::Potato, SpotItSuit::Apple]));
        let card3: SpotItCard = SpotItCard(HashSet::from([SpotItSuit::Potato, SpotItSuit::Apple]));
        let card4: SpotItCard = SpotItCard(HashSet::from([SpotItSuit::Yogurt]));
        let card5 = SpotItCard(HashSet::new());
        let card6 = SpotItCard(HashSet::new());
        assert!(card1.match_exactly_one_suit(&card2));
        assert!(card2.match_exactly_one_suit(&card1));
        assert!(!card3.match_exactly_one_suit(&card2)); // cards that have more than one suit in common
        assert!(!card4.match_exactly_one_suit(&card3)); // cards that have no suit in common
        assert!(!card5.match_exactly_one_suit(&card6)); // cards that have no suit in common as they are both empty
    }
    #[test]
    fn can_add_suit_to_spotitcard() {
        let mut card1 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        card1.0.insert(SpotItSuit::Apple);
        let card2 = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        assert_ne!(card1, card2);
    }

    #[test]
    fn every_card_on_default_spotitdeck_is_unique() {
        let mut deck: SpotItDeck = SpotItDeck::default();
        for _ in 1..=deck.cards.len() {
            let card = deck.cards.pop().unwrap();
            assert!(deck.cards.iter().all(move |c| c != &card));
        }
    }
    #[test]
    fn can_push_pop_same_card_on_spotitdeck() {
        let mut deck: SpotItDeck = SpotItDeck::new();
        let card = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        deck.cards.push(card.clone());
        assert_eq!(deck.cards.pop(), Some(card));
    }
    #[test]
    fn can_use_fn_to_pop_card_from_spotitdeck() {
        let mut deck: SpotItDeck = SpotItDeck::new();
        let card = SpotItCard(HashSet::from([SpotItSuit::Potato]));
        deck.cards.push(card.clone());
        assert_eq!(deck.pop_card(), Some(card));
        assert_eq!(deck.pop_card(), None);
    }
    #[test]
    fn each_spotitcard_has_1_suits() {
        let deck: SpotItDeck = SpotItDeck::default();
        for card in deck.cards {
            assert_eq!(card.0.len(), 1);
        }
    }
}
