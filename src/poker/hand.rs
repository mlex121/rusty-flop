use std::collections::HashSet;
use super::card::Card;

pub struct Hand {
    cards: HashSet<Card>,
}

impl Hand {
    pub fn is_high_card(&self) -> bool {
        false
    }

    pub fn is_pair(&self) -> bool {
        false
    }

    pub fn is_two_pair(&self) -> bool {
        false
    }

    pub fn is_three_of_a_kind(&self) -> bool {
        false
    }

    pub fn is_straight(&self) -> bool {
        false
    }

    pub fn is_flush(&self) -> bool {
        false
    }

    pub fn is_full_house(&self) -> bool {
        false
    }

    pub fn is_four_of_a_kind(&self) -> bool {
        false
    }

    pub fn is_straight_flush(&self) -> bool {
        false
    }
}
