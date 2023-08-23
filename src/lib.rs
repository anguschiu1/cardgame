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
//! [`Maths behind SpotIt! card game]: https://www.smithsonianmag.com/science-nature/math-card-game-spot-it-180970873/
//! [`More Maths behind SpotIt! card game]: https://science.mom/images/Worksheets/ScienceWorksheets/SpotIt.pdf

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
    /// This function generates a deck of SpotIt Cards by a prime number n.
    pub fn generate_by_prime(n: u8) -> Result<SpotItDeck, String> {
        if (n * n + n + 1) as usize > SpotItSymbol::iter().count() {
            println!("{} is too large", n);
            return Err("n is too large, hence not enough symobls to generate deck.".to_string());
        } else {
            let (check, _) = prime_checker::is_prime(n as u64);
            if check {
                println!("{} is prime", n);
            } else {
                println!("{} is NOT prime", n);
                return Err("{} is not prime".to_string());
            }
        }
        println!(
            "SpotItSymbol::iter().count() is {}",
            SpotItSymbol::iter().count()
        );
        let mut deck = SpotItDeck::new();
        // deck_size = n^2 + n + 1
        let deck_size = (n * n + n + 1).into();
        // symbols_used = n^2 + n + 1
        let symbols_used: usize = (n * n + n + 1).into();
        // symbols_per_card = n + 1
        let symbols_per_card: usize = (n + 1).into();

        // 1. Create a deck of cards
        for i in 0..deck_size {
            // 2. Create a card
            let mut card = SpotItCard(HashSet::new());
            // 3. Add n + 1 symbols to the card
            for j in 0..symbols_per_card {
                // 4. Add symbol to the card using the formula
                card.0.insert(SpotItSymbol::iter().nth(1).unwrap());
            }
            deck.cards.push(card);
        }
        Ok(deck)
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
        assert_eq!(deck.cards.len(), 31);
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
        assert_eq!(deck.cards.len(), 1 * 1 + 1 + 1);
        let deck = SpotItDeck::generate_by_prime(2).unwrap();
        assert_eq!(deck.cards.len(), 2 * 2 + 2 + 1);
        let deck = SpotItDeck::generate_by_prime(3).unwrap();
        assert_eq!(deck.cards.len(), 3 * 3 + 3 + 1);

        // // return error if n is not prime
        assert!(SpotItDeck::generate_by_prime(4).is_err());

        let deck = SpotItDeck::generate_by_prime(5).unwrap();
        assert_eq!(deck.cards.len(), 5 * 5 + 5 + 1);

        // // return error if n is too large
        assert!(SpotItDeck::generate_by_prime(6).is_err());
    }
    #[test]
    fn can_generate_by_prime_3() {
        let mut deck = SpotItDeck::generate_by_prime(3).unwrap();
        assert_eq!(deck.cards.len(), 3 * 3 + 3 + 1);
        let first_card = SpotItCard(HashSet::from([
            SpotItSymbol::Apple,
            SpotItSymbol::Banana,
            SpotItSymbol::Bread,
            SpotItSymbol::Fish,
        ]));
        let card = deck.cards.pop().unwrap();

        // The first card is [0,1,2,9], which is [Apple, Banana, Bread, Fish]
        assert_eq!(card, first_card);
    }
}
