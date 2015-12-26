use std::fmt;
use std::cmp;
use super::rank::Rank;
use super::suit::Suit;

#[derive(Clone, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn from_str(s: &str) -> Option<Self> {
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
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>{
        match (self.rank.partial_cmp(&other.rank), self.suit.partial_cmp(&other.suit)) {
            (Some(r), Some(s))  => match r {
                cmp::Ordering::Equal    => Some(s),
                _                       => Some(r),
            },
            (_, _)              => None,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            cmp::Ordering::Equal    => self.suit.cmp(&other.suit),
            r                       => r,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
