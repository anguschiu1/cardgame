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

pub mod frenchcard;
use frenchcard::{FrenchCard, FrenchRank, FrenchSuit};

pub mod spotitcard;
use spotitcard::{SpotItCard, SpotItSymbol};

use fraction::Fraction;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use strum::IntoEnumIterator;
type F = fraction::Fraction;

/// This trait defines the common functionality of a deck of cards.
pub trait Deck<T> {
    /// This function creates a new, empty deck of cards.
    fn new() -> Self;

    /// This function creates a new deck of cards with default values, e.g. a deck of French Cards with 52 cards, or a deck of SpotIt Cards with 57 cards.
    fn default() -> Self;

    /// This function mututate original deck and shuffles the deck of cards.
    fn shuffle(&mut self);

    /// This function pops one card (latest) from the deck of cards.
    fn pop_card(&mut self) -> Option<T>;

    /// This function pops one card (by index) from the deck of cards.
    fn pop_card_by_index(&mut self, index: usize) -> Option<T>;

    /// This function pushes one card to the deck of cards.
    fn push_card(&mut self, card: T);

    /// This function returns the number of cards in the deck.
    fn len(&self) -> usize;

    /// This function returns false if the deck is empty.
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

    fn pop_card_by_index(&mut self, index: usize) -> Option<FrenchCard> {
        match self.len() {
            n if n > index => Some(self.cards.remove(index)),
            _ => None,
        }
    }
}

impl Deck<SpotItCard> for SpotItDeck {
    /// This function pushes one SpotIt! card to the deck of cards.
    fn push_card(&mut self, card: SpotItCard) {
        self.cards.push(card);
    }
    /// This function creates a new deck of SpotIt! Cards usually played, with 57 cards, 8 symbols each.
    fn default() -> Self {
        Self::generate_by_prime(7).unwrap()
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

    fn pop_card_by_index(&mut self, index: usize) -> Option<SpotItCard> {
        match self.cards.len() {
            n if n > index => Some(self.cards.remove(index)),
            _ => None,
        }
    }
}

impl SpotItDeck {
    /// This function generates a deck of SpotIt Cards by a prime number n.
    pub fn generate_by_prime(n: u8) -> Result<SpotItDeck, String> {
        // Check if the n is too large for defaul symbols to generate deck
        // n^2 + n + 1 <= default symbols
        if (n * n + n + 1) as usize > SpotItSymbol::iter().count() {
            Err("n is too large, hence not enough symobls to generate deck.".to_string())
        // Check if the n is prime
        } else if let (false, _) = prime_checker::is_prime(n as u64) {
            Err("{} is not prime".to_string())

        // handle n = 1 edge case
        } else if n == 1 {
            let mut deck = SpotItDeck::new();
            let (plane, line) = SpotItDeck::gen_projective_plane(n);
            let line_symbols = line.clone();

            for line_symbol in line {
                let mut symbol_on_plane = HashSet::new();
                symbol_on_plane.insert(plane[0][0]);
                symbol_on_plane.insert(line_symbol);
                let mut card = SpotItCard(HashSet::new());
                card.0 = symbol_on_plane.clone();
                deck.push_card(card);
            }
            deck.push_card(SpotItCard(line_symbols.into_iter().collect()));

            Ok(deck)
        }
        // other cases
        else {
            let mut deck = SpotItDeck::new();
            // deck_size = n^2 + n + 1
            // symbols_per_card = n + 1

            // Generate a projective plane of n^2 + n + 1 symbols
            let (plane, line) = SpotItDeck::gen_projective_plane(n);

            let mut line_iter = line.iter();

            // Calculate the set of slope of the plane using fractions
            let slopes = SpotItDeck::cal_slope(n);

            for slope in slopes.iter() {
                println!("slope: {:?}", slope);
                let line_symbol = *(line_iter.next().unwrap());
                for c in 0..n {
                    let mut symbol_on_plane = HashSet::new();
                    // loop around an enlarged plane to find the symbols on the line
                    // FIXME: this is a hacky way to loop around the plane, find a better way to do this
                    for x in 0..n * n {
                        for y in 0..n * n {
                            match slope {
                                fraction::GenericFraction::NaN => {}
                                fraction::GenericFraction::Infinity(_) => {
                                    if x as u64 == c as u64 {
                                        println!("x: {}, y: {}, a: inf, c: {}", x, y, c);
                                        symbol_on_plane
                                            .insert(plane[(y % n) as usize][(x % n) as usize]);
                                    }
                                }
                                fraction::GenericFraction::Rational(_, slope) => {
                                    if y as u64
                                        == slope.numer() / slope.denom() * x as u64 + c as u64
                                    {
                                        println!(
                                            "x: {}, y: {}, a: {:.2}, c: {}",
                                            x,
                                            y,
                                            (slope.numer() / slope.denom()),
                                            c
                                        );
                                        symbol_on_plane
                                            .insert(plane[(y % n) as usize][(x % n) as usize]);
                                    }
                                }
                            }
                        }
                    }
                    symbol_on_plane.insert(line_symbol);
                    deck.push_card(SpotItCard(symbol_on_plane.clone()));
                    dbg!(&symbol_on_plane);
                }
            }
            deck.push_card(SpotItCard(line.into_iter().collect()));

            Ok(deck)
        }
    }

    /// This function fills symbols onto a plane [y][x] (note: inverted x and y) where x and y are both in range of 0..n.
    fn gen_projective_plane(n: u8) -> (Vec<Vec<SpotItSymbol>>, Vec<SpotItSymbol>) {
        let mut symbol = SpotItSymbol::iter();
        let mut plane: Vec<Vec<SpotItSymbol>> = Vec::new();
        let mut line: Vec<SpotItSymbol> = Vec::new();
        for _ in 0..n {
            let mut row: Vec<SpotItSymbol> = Vec::new();
            for _ in 0..n {
                row.push(symbol.next().unwrap());
            }
            plane.push(row);
        }
        for _ in 0..=n {
            line.push(symbol.next().unwrap());
        }
        (plane, line)
    }

    fn cal_slope(n: u8) -> Vec<Fraction> {
        let mut slope: Vec<Fraction> = Vec::new();
        slope.push(F::new(0u8, n - 1));
        for i in 0..n {
            slope.push(F::new(1u8, i));
        }
        slope
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn default_frenchdeck_has_52_cards() {
        let deck: FrenchDeck = FrenchDeck::default();
        assert_eq!(deck.cards.len(), 52);
    }
    #[test]
    fn card_on_default_frenchdeck_is_unique() {
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
        deck.push_card(card.clone());
        assert_eq!(deck.pop_card(), Some(card));
    }
    #[test]
    fn default_spotitdeck_has_57_cards() {
        let deck: SpotItDeck = SpotItDeck::default();
        assert_eq!(deck.cards.len(), 57);
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
        let card = SpotItCard(HashSet::from([SpotItSymbol::Banana]));
        deck.cards.push(card.clone());
        assert_eq!(deck.cards.pop(), Some(card));
    }
    #[test]
    fn can_use_fn_to_pop_card_from_spotitdeck() {
        let mut deck: SpotItDeck = SpotItDeck::new();
        let card = SpotItCard(HashSet::from([SpotItSymbol::Banana]));
        deck.cards.push(card.clone());
        assert_eq!(deck.pop_card(), Some(card));
        assert_eq!(deck.pop_card(), None);
    }

    #[test]
    fn can_generate_right_cards_number_by_prime() {
        let deck = SpotItDeck::generate_by_prime(1).unwrap();
        assert_eq!(deck.cards.len(), 1 + 1 + 1);
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
    fn can_generate_right_deck_of_card_by_prime_3() {
        let deck = SpotItDeck::generate_by_prime(3).unwrap();
        assert_eq!(deck.cards.len(), 3 * 3 + 3 + 1);
        let first_card = SpotItCard(HashSet::from([
            SpotItSymbol::Apple,
            SpotItSymbol::Apricot,
            SpotItSymbol::Avocado,
            SpotItSymbol::Currant,
        ]));
        let card = deck.cards.first().unwrap();

        // The first card is [0,1,2,9], which is [Apple, Banana, Bread, Fish]
        assert_eq!(*card, first_card);
    }
    #[test]
    fn can_generate_projective_plane_of_5_with_symbols() {
        let (plane, line) = SpotItDeck::gen_projective_plane(5);
        assert_eq!(plane.len(), 5);
        assert_eq!(plane[0].len(), 5);
        assert_eq!(plane[1].len(), 5);
        assert_eq!(plane[2].len(), 5);
        assert_eq!(plane[3].len(), 5);
        assert_eq!(plane[4].len(), 5);
        assert_eq!(plane[0][0], SpotItSymbol::Apple);
        assert_eq!(plane[0][1], SpotItSymbol::Apricot);
        assert_eq!(plane[0][2], SpotItSymbol::Avocado);
        assert_eq!(plane[0][3], SpotItSymbol::Banana);
        assert_eq!(plane[0][4], SpotItSymbol::Bilberry);
        assert_eq!(plane[1][0], SpotItSymbol::Blackberry);
        assert_eq!(plane[1][1], SpotItSymbol::Blackcurrant);
        assert_eq!(line.len(), 6);

        let mut symbols = SpotItSymbol::iter();
        for x in plane.iter() {
            for y in x.iter() {
                assert_eq!(*y, symbols.next().unwrap());
            }
        }
    }
    #[test]
    fn can_calculate_slope() {
        let slope = SpotItDeck::cal_slope(3);
        assert_eq!(slope.len(), 4);
        assert_eq!(slope[0], F::new(0u8, 2u8));
        assert_eq!(slope[1], F::new(1u8, 0u8));
        assert_eq!(slope[2], F::new(1u8, 1u8));
        assert_eq!(slope[3], F::new(1u8, 2u8));
    }
    #[test]
    fn can_pop_french_card_by_index() {
        let mut deck: FrenchDeck = FrenchDeck::default();
        let first_card = FrenchCard(FrenchRank::Ace, FrenchSuit::Spade);
        let last_card = FrenchCard(FrenchRank::Two, FrenchSuit::Club);
        assert_eq!(deck.len(), 52);
        assert_eq!(deck.pop_card_by_index(52), None);
        assert_eq!(deck.pop_card_by_index(0), Some(last_card));
        assert_eq!(deck.pop_card_by_index(50), Some(first_card));
        assert_eq!(deck.len(), 50);
    }
    #[test]
    fn can_pop_spotit_card_by_index() {
        let mut deck: SpotItDeck = SpotItDeck::default();
        assert_eq!(deck.len(), 57);
        assert_eq!(deck.pop_card_by_index(57), None);
        let last_card = deck.pop_card();
        for _ in 1..=deck.len() - 1 {
            deck.pop_card();
        }
        let first_card = deck.pop_card();

        let mut deck2: SpotItDeck = SpotItDeck::default();
        assert_eq!(deck2.pop_card_by_index(56), last_card);
        assert_eq!(deck2.pop_card_by_index(0), first_card);
        assert_eq!(deck2.len(), 55);
    }
}
