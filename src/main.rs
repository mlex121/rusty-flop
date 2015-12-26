mod poker;

fn main() {
    let rank = poker::Rank::Ace;
    let suit = poker::Suit::Spades;
    println!("{}{}", rank, suit);

    let card = poker::Card {rank: rank, suit: suit};
    println!("{}", card);

    if let Some(card2) = poker::Card::from_str("As") {
        println!("{}", card2);
    }
}
