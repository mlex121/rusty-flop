use std::fmt;
use std::str;

#[derive(PartialEq)]
#[derive(Hash)]
pub enum Rank {
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
    Ace,
}

impl Rank {
    pub fn from_char(c: char) -> Option<Rank> {
        match c {
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            '9' => Some(Rank::Nine),
            'T' => Some(Rank::Ten),
            'J' => Some(Rank::Jack),
            'Q' => Some(Rank::Queen),
            'K' => Some(Rank::King),
            'A' => Some(Rank::Ace),
            _ => None,
        }
    }
}

impl str::FromStr for Rank {
    type Err = ();
    fn from_str(s: &str) -> Result<Rank, ()> {
        match s.chars().next() {
            Some(x) => match Rank::from_char(x) {
                Some(x) => Ok(x),
                _       => Err(()),
            },
            _       => Err(())
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match *self {
                Rank::Two   => "2".to_string(),
                Rank::Three => "3".to_string(),
                Rank::Four  => "4".to_string(),
                Rank::Five  => "5".to_string(),
                Rank::Six   => "6".to_string(),
                Rank::Seven => "7".to_string(),
                Rank::Eight => "8".to_string(),
                Rank::Nine  => "9".to_string(),
                Rank::Ten   => "T".to_string(),
                Rank::Jack  => "J".to_string(),
                Rank::Queen => "Q".to_string(),
                Rank::King  => "K".to_string(),
                Rank::Ace   => "A".to_string(),
                // Currently fails, need to fix.
                // For now, we explicitly handle each case.
                // r       => (r as i32).to_string(),
            }
        )
    }
}
