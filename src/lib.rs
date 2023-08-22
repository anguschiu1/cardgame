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

pub mod card;
use card::{FrenchCard, FrenchRank, FrenchSuit, SpotItCard, SpotItSymbol};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use strum::IntoEnumIterator;

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
    /// This function pushes one card to the deck of cards.
    fn push_card(&mut self, card: T);

    /// This function returns the number of cards in the deck.
    fn len(&self) -> usize;
    /// This function returns the number of cards in the deck.
    fn is_empty(&self) -> bool;
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
    fn push_card(&mut self, card: FrenchCard) {
        self.cards.push(card);
    }

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
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    fn len(&self) -> usize {
        self.cards.len()
    }
}

impl Deck<SpotItCard> for SpotItDeck {
    fn push_card(&mut self, card: SpotItCard) {
        self.cards.push(card);
    }
    fn default() -> Self {
        let mut deck = Self::new();
        for suit in SpotItSymbol::iter() {
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
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    fn len(&self) -> usize {
        self.cards.len()
    }
}

impl SpotItDeck {
    fn generate_by_prime(n: u8) -> Result<SpotItDeck, String> {
        if SpotItSymbol::iter().count() < (n * n + n + 1) as usize {
            Err("n is too large, hence not enough symobls to generate deck.".to_string())
        } else {
            let mut deck = SpotItDeck::new();
            let count = (n * n + 1) as usize;
            let symbols_per_card: usize = (n + 1).into();

            for i in 0..count {
                let mut card = SpotItCard(HashSet::new());
                for j in 0..symbols_per_card {
                    card.0
                        .insert(SpotItSymbol::iter().nth(i + j * n as usize).unwrap());
                }
                deck.cards.push(card);
            }
            Ok(deck)
        }
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
        let card = SpotItCard(HashSet::from([SpotItSymbol::Potato]));
        deck.cards.push(card.clone());
        assert_eq!(deck.cards.pop(), Some(card));
    }
    #[test]
    fn can_use_fn_to_pop_card_from_spotitdeck() {
        let mut deck: SpotItDeck = SpotItDeck::new();
        let card = SpotItCard(HashSet::from([SpotItSymbol::Potato]));
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
    #[test]
    fn can_generate_by_prime() {
        let deck = SpotItDeck::generate_by_prime(1).unwrap();
        assert_eq!(deck.cards.len(), 1 * 1 + 1);
        let deck = SpotItDeck::generate_by_prime(2).unwrap();
        assert_eq!(deck.cards.len(), 2 * 2 + 1);
        let deck = SpotItDeck::generate_by_prime(3).unwrap();
        assert_eq!(deck.cards.len(), 3 * 3 + 1);
        let deck = SpotItDeck::generate_by_prime(4).unwrap();
        assert_eq!(deck.cards.len(), 4 * 4 + 1);
        let deck = SpotItDeck::generate_by_prime(5).unwrap();
        assert_eq!(deck.cards.len(), 5 * 5 + 1);
        // return error if n is too large
        assert!(SpotItDeck::generate_by_prime(6).is_err());
    }
}
