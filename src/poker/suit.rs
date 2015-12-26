use std::fmt;
use std::str;

#[derive(PartialEq)]
#[derive(Hash)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Suit {
    pub fn from_char(c: char) -> Option<Suit> {
        match c {
            's' => Some(Suit::Spades),
            'c' => Some(Suit::Clubs),
            'h' => Some(Suit::Hearts),
            'd' => Some(Suit::Diamonds),
            _   => None,
        }
    }
}

impl str::FromStr for Suit {
    type Err = ();
    fn from_str(s: &str) -> Result<Suit, ()> {
        match s.chars().next() {
            Some(x) => match Suit::from_char(x) {
                Some(x) => Ok(x),
                _       => Err(()),
            },
            _       => Err(()),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match *self {
                Suit::Spades    => "s".to_string(),
                Suit::Clubs     => "c".to_string(),
                Suit::Hearts    => "h".to_string(),
                Suit::Diamonds  => "d".to_string(),
            }
        )
    }
}
