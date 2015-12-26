use std::fmt;
use super::rank::Rank;
use super::suit::Suit;

#[derive(Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn from_str(s: &str) -> Option<Card> {
        let rank = match s.chars().nth(0) {
            Some(x) => Rank::from_char(x),
            _       => None,
        };
        let suit = match s.chars().nth(1) {
            Some(x) => Suit::from_char(x),
            _       => None,
        };
        match (rank, suit) {
            (Some(r), Some(s)) => Some(Card {rank: r, suit: s}),
            (_, _) => None
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

impl Eq for Card {}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
