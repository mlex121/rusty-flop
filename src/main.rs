mod poker;

use std::collections::BTreeSet;

fn main() {
    let rank = poker::Rank::Ace;
    let suit = poker::Suit::Spades;
    let card1 = poker::Card {rank: rank, suit: suit};

    let card2_option = poker::Card::from_str("Ac");

    if card2_option.is_none() {
        panic!();
    }

    let card2 = card2_option.unwrap();

    let vec = [card1, card2];

    for c in vec.iter() {
        println!("{}", c);
    }

    let set: BTreeSet<poker::Card> = vec.iter().cloned().collect();

    for c in set.iter() {
        println!("{}", c);
    }

    let hand: poker::Hand = vec.iter().cloned().collect();

    for c in hand.cards.iter() {
        println!("{}", c);
    }
}
