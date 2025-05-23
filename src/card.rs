use std::fmt::{Display, Debug};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CardParseError {
    #[error("Invalid rank: {0}. Expected one of 2-9, T, J, Q, K, A.")]
    InvalidRank(char),
    #[error("Invalid suit: {0}. Expected one of h, s, d, c.")]
    InvalidSuit(char),
    #[error("Invalid length: {0}. Expected 2.")]
    InvalidLength(usize),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Suit { 
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

impl From<u8> for Suit {
    fn from(value: u8) -> Self {
        match value {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => panic!("Invalid suit value: {}", value),
        }
    }
}

impl Suit {
    pub fn from_str(s: char) -> Result<Suit, CardParseError> {
        match s {
            'h' | 'H' => Ok(Suit::Hearts),
            's' | 'S' => Ok(Suit::Spades),
            'd' | 'D' => Ok(Suit::Diamonds),
            'c' | 'C' => Ok(Suit::Clubs),
            _ => Err(CardParseError::InvalidSuit(s)),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Hearts   => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs    => write!(f, "♣"),
            Suit::Spades   => write!(f, "♠"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
pub enum Rank {
    Two,
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

pub const RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        match value {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => panic!("Invalid rank value: {}", value),
        }
    }
}

impl Rank {
    pub fn prime(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 5,
            Rank::Five => 7,
            Rank::Six => 11,
            Rank::Seven => 13,
            Rank::Eight => 17,
            Rank::Nine => 19,
            Rank::Ten => 23,
            Rank::Jack => 29,
            Rank::Queen => 31,
            Rank::King => 37,
            Rank::Ace => 41,
        }
    }

    pub fn from_str(s: char) -> Result<Rank, CardParseError> {
        match s {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            't' | 'T' => Ok(Rank::Ten),
            'j' | 'J' => Ok(Rank::Jack),
            'q' | 'Q' => Ok(Rank::Queen),
            'k' | 'K' => Ok(Rank::King),
            'a' | 'A' => Ok(Rank::Ace),            
            _ => Err(CardParseError::InvalidRank(s)),
        }
    }

    pub fn chen_score(&self) -> f32 {
        match &self {
            Rank::Ace => 10.0,
            Rank::King => 8.0,
            Rank::Queen => 7.0,
            Rank::Jack => 6.0,
            _ => (*self as u8 as f32 + 2.0) / 2.0,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "T"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(pub u8);

impl Default for Card {
    fn default() -> Self {
        Card(0xFF)
    }
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        Card(value)
    }
}

impl Card {
    
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card(4 * (rank as u8) + (suit as u8))
    }

    pub fn random() -> Card {
        Card(fastrand::u8(0..52))
    }

    pub fn is_valid(&self) -> bool {
        self.0 < 52
    }

    pub fn is_dealt(&self) -> bool {
        self.0 != 0xFF
    }

    #[inline]
    pub fn mask(&self) -> u64 {
        1 << self.0
    }

    pub fn from_str(s: &str) -> Result<Card, CardParseError> {

        if s.len() != 2 {
            return Err(CardParseError::InvalidLength(s.len()));
        }
        let mut chars = s.chars();
        
        let first = chars.next().unwrap();
        let rank = match first {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' | 't'  => Rank::Ten,
            'J' | 'j'  => Rank::Jack,
            'Q' | 'q'  => Rank::Queen,
            'K' | 'k'  => Rank::King,
            'A' | 'a'  => Rank::Ace,
            _ => return Err(CardParseError::InvalidRank(first)),
        };

        let second = chars.next().unwrap();
        let suit = match second{
            'd' | 'D'  => Suit::Diamonds,
            'h' | 'H'  => Suit::Hearts,
            'c' | 'C'  => Suit::Clubs,
            's' | 'S'  => Suit::Spades,
            _ => return Err(CardParseError::InvalidSuit(second)),
        };

        Ok(Card::new(rank, suit))
    }

    pub fn vec_from_str(s: &str) -> Result<Vec<Card>, CardParseError> {
        let mut cards = Vec::new();
        let stripped = s.replace(" ", "");
        // While 2 chars remain, parse a card.
        let mut i = 0;
        for _ in 0..stripped.len() / 2 {
            let card = Card::from_str(&stripped[i..i+2])?;
            cards.push(card);
            i += 2;
        }
        Ok(cards)
    }

    #[inline]
    pub fn suit(&self) -> Suit {
        (self.0 & 3).into()
    }

    #[inline]
    pub fn suit_u8(&self) -> u8 {
        self.0 & 3
    }

    #[inline]
    pub fn swap_suit(&self, suit: Suit) -> Card {
        Card((self.0 & !3) | suit as u8)
    } 

    #[inline]
    pub fn rank(&self) -> Rank {
        (self.0 >> 2).into()
    }

    #[inline]
    pub fn rank_u8(&self) -> u8 {
        self.0 >> 2
    }

    pub fn chen_score(&self) -> f32 {
        self.rank().chen_score()
    }

    //   For use in two-plus-two hand evaluator.
    //   An integer is made up of four bytes.  The high-order
    //   bytes are used to hold the rank bit pattern, whereas
    //   the low-order bytes hold the suit/rank/prime value
    //   of the card.
    //
    //   +--------+--------+--------+--------+
    //   |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
    //   +--------+--------+--------+--------+
    pub fn bit_mask(&self) -> u32 {

        let p = self.rank().prime();
        let r = (self.rank() as u32) << 8;
        let suit : u32  = match self.suit() {
            Suit::Hearts   => 0x1000,
            Suit::Spades   => 0x2000,
            Suit::Diamonds => 0x4000,
            Suit::Clubs    => 0x8000,
        };
        let b = 1 << (self.rank() as i32 + 16);
        
        b | suit | r | p        
   }

   pub fn from_bit_mask(mask: u32) -> Card {
        let rank = ((mask & 0x00_00_0F_00) >> 8) as u8;
        let suit = match mask & 0x00_00_F0_00 {
            0x1000 => Suit::Hearts,
            0x2000 => Suit::Spades,
            0x4000 => Suit::Diamonds,
            0x8000 => Suit::Clubs,
            _ => panic!("Invalid suit value: {}", mask),
        };
        Card::new(rank.into(), suit)
   }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_dealt() {
            write!(f, "Not dealt")
        } else {
            write!(f, "{}{}", self.rank(), self.suit())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_size() {
        assert!(std::mem::size_of::<Card>() == 1);
    }

    #[test]
    fn test_from_str() {
        let card = Card::from_str("Ah").unwrap();
        assert!(card.rank() == Rank::Ace);
        assert!(card.suit() == Suit::Hearts);

        let card = Card::from_str("2d").unwrap();
        assert!(card.rank() == Rank::Two);
        assert!(card.suit() == Suit::Diamonds);

        let card = Card::from_str("Ac").unwrap();
        assert!(card.rank() == Rank::Ace);
        assert!(card.suit() == Suit::Clubs);
    }

    #[test]
    fn test_vec_from_str() {

        let s = "Ah 2d Kc 9d";
        let cards = Card::vec_from_str(s).unwrap();
        assert!(cards.len() == 4);

        let s_2 = "Ah2dKc9d3c";
        let cards_2 = Card::vec_from_str(s_2).unwrap();
        assert!(cards_2.len() == 5);
    }   

    #[test]
    fn test_suit_u8() {
        let card = Card::new(Rank::Ace, Suit::Hearts);
        assert!(card.suit() as u8 == 2);

        let card = Card::new(Rank::Ace, Suit::Clubs);
        assert!(card.suit() as u8 == 0);
    }

    #[test]
    fn test_mask_u32() {
        assert_eq!(Card::from_str("5c").unwrap().bit_mask(), 0b00000000_00001000_10000011_00000111);
        assert_eq!(Card::from_str("Ah").unwrap().bit_mask(), 0b00010000_00000000_00011100_00101001);

        assert_eq!(Card::from_bit_mask(0b00000000_00001000_10000011_00000111), Card::from_str("5c").unwrap());
        // assert_eq!(Card::from_bit_mask(0b00010000_00000000_00101100_00101001), Card::from_str("Ah").unwrap());
    }
}