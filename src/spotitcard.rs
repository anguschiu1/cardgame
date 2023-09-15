#![allow(missing_docs)]
#![warn(rustdoc::private_doc_tests)]

//! Support library to define SpotIt! cards in `cardgame` crate.
//!
//! Provide structures and enums to define SpotIt! cards in the `cardgame` crate.
//!
//! [`Repository`]: https://github.com/anguschiu1/cardgame
//! [`SpotIt!`]: https://www.amazon.com/Asmodee-SP411-Spot-It/dp/B0039S7NO6
//! [`French Card Game`]: https://en.wikipedia.org/wiki/French_playing_cards
//! [`SpotIt! Rules`]: https://www.ultraboardgames.com/spot-it/game-rules.php

use std::collections::HashSet;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Eq, Hash)]
/// Thie enum defines the suits (or pattern) of the SpotIt! game. One card can has one or more suits.
pub enum SpotItSymbol {
    Apple,
    Apricot,
    Avocado,
    Banana,
    Bilberry,
    Blackberry,
    Blackcurrant,
    Blueberry,
    Boysenberry,
    Currant,
    Cherry,
    Cherimoya,
    ChicoFruit,
    Cloudberry,
    Coconut,
    Cranberry,
    Cucumber,
    CustardApple,
    Damson,
    Date,
    Dragonfruit,
    Durian,
    Elderberry,
    Feijoa,
    Fig,
    GojiBerry,
    Gooseberry,
    Grape,
    Raisin,
    Grapefruit,
    Guava,
    Honeyberry,
    Huckleberry,
    Jabuticaba,
    Jackfruit,
    Jambul,
    Jujube,
    JuniperBerry,
    Kiwano,
    Kiwifruit,
    Kumquat,
    Lemon,
    Lime,
    Loquat,
    Longan,
    Lychee,
    Mango,
    Mangosteen,
    Marionberry,
    Melon,
    Cantaloupe,
    Honeydew,
    Watermelon,
    MiracleFruit,
    Mulberry,
    Nectarine,
    Nance,
    Olive,
    Orange,
    BloodOrange,
    Clementine,
    Mandarine,
    Tangerine,
    Papaya,
    Passionfruit,
    Peach,
    Pear,
    Persimmon,
    Physalis,
    Plantain,
    Plum,
    Prune,
    Pineapple,
    Plumcot,
    Pomegranate,
    Pomelo,
    PurpleMangosteen,
    Quince,
    Raspberry,
    Salmonberry,
    Rambutan,
    Redcurrant,
    SalalBerry,
    Salak,
    Satsuma,
    Soursop,
    StarFruit,
    SolanumQuitoense,
    Strawberry,
    Tamarillo,
    Tamarind,
    UgliFruit,
    Yuzu,
}

/// This tuple struct defines a SpotIt Card.
/// Please notice that a SpotIt Card can have 0, 1, or more than one suits. For example, a card can have both Apple and Banana suits.
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct SpotItCard(pub HashSet<SpotItSymbol>);
impl SpotItCard {
    /// This function returns true if the two cards have exactly one suit in common.
    /// This is the key rule of the SpotIt game.
    pub fn match_exactly_one_symbol(&self, card: &Self) -> bool {
        self.0.intersection(&card.0).count() == 1
    }
    /// This function returns the symbols of the card.
    pub fn symbols(&self) -> Vec<SpotItSymbol> {
        self.0.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_compare_spotitcard() {
        let card1 = SpotItCard(HashSet::from([SpotItSymbol::Banana]));
        let card2 = SpotItCard(HashSet::from([SpotItSymbol::Banana]));
        let card3 = SpotItCard(HashSet::from([SpotItSymbol::Apple]));
        let card4: SpotItCard =
            SpotItCard(HashSet::from([SpotItSymbol::Banana, SpotItSymbol::Apple]));
        let card5 = SpotItCard(HashSet::from([SpotItSymbol::Banana, SpotItSymbol::Apple]));
        let card6 = SpotItCard(HashSet::from([SpotItSymbol::Apple, SpotItSymbol::Apple]));
        assert_eq!(card1, card2); // two cards with same one suit
        assert_ne!(card2, card3); // two cards with different suits
        assert_ne!(card3, card4); // two cards with non-identical set of suit
        assert_ne!(card1, card4); // two cards with non-identical set of suit
        assert_eq!(card4, card5); // two cards with same set of suit
        assert_eq!(card3, card6); // two cards with same suit, even one suit is added twice
    }

    #[test]
    fn can_match_exactly_one_suit_on_two_spotitcard() {
        let card1 = SpotItCard(HashSet::from([SpotItSymbol::Banana]));
        let card2: SpotItCard =
            SpotItCard(HashSet::from([SpotItSymbol::Banana, SpotItSymbol::Apple]));
        let card3: SpotItCard =
            SpotItCard(HashSet::from([SpotItSymbol::Banana, SpotItSymbol::Apple]));
        let card4: SpotItCard = SpotItCard(HashSet::from([SpotItSymbol::ChicoFruit]));
        let card5 = SpotItCard(HashSet::new());
        let card6 = SpotItCard(HashSet::new());
        assert!(card1.match_exactly_one_symbol(&card2));
        assert!(card2.match_exactly_one_symbol(&card1));
        assert!(!card3.match_exactly_one_symbol(&card2)); // cards that have more than one suit in common
        assert!(!card4.match_exactly_one_symbol(&card3)); // cards that have no suit in common
        assert!(!card5.match_exactly_one_symbol(&card6)); // cards that have no suit in common as they are both empty
    }
    #[test]
    fn can_add_symbol_to_spotitcard() {
        let mut card1 = SpotItCard(HashSet::from([SpotItSymbol::Apple]));
        let card2 = SpotItCard(HashSet::from([SpotItSymbol::Banana]));
        card1.0.insert(SpotItSymbol::Banana);
        assert_ne!(card1, card2);
        assert!(card1.match_exactly_one_symbol(&card2))
    }
    #[test]
    fn can_show_symbols_on_spotitcard() {
        let card1 = SpotItCard(HashSet::from([SpotItSymbol::Yuzu]));
        assert_eq!(card1.symbols(), vec![SpotItSymbol::Yuzu]);
        //TODO how to test the order of the symbols?
    }
}
