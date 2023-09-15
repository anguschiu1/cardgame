#![allow(missing_docs)]
#![warn(rustdoc::private_doc_tests)]

//! Support library to define French cards in `cardgame` crate.
//!
//! Auxillary functions to define French cards, but it is for testing purposes only.
//!
//! [`Repository`]: https://github.com/anguschiu1/cardgame
//! [`SpotIt!`]: https://www.amazon.com/Asmodee-SP411-Spot-It/dp/B0039S7NO6
//! [`French Card Game`]: https://en.wikipedia.org/wiki/French_playing_cards
//! [`SpotIt! Rules`]: https://www.ultraboardgames.com/spot-it/game-rules.php

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

/// This tuple struct defines a French Card.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct FrenchCard(pub FrenchRank, pub FrenchSuit);

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

#[cfg(test)]
mod tests {
    use super::*;
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
}
