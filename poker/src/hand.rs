use crate::card::{Card, Deck, Rank};
use thiserror::Error;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HandRank {
    HighCard(u32),
    Pair(u32),
    TwoPair(u32),
    ThreeOfAKind(u32),
    Straight(u32),
    Flush(u32),
    FullHouse(u32),
    FourOfAKind(u32),
    StraightFlush(u32),
}

impl From<u16> for HandRank {
    fn from(value: u16) -> Self {
        match value {
            
            0..=1277    => HandRank::HighCard(value as u32),
            1278..=4137 => HandRank::Pair(value as u32 - 1277),
            4138..=4995 => HandRank::TwoPair(value as u32 - 4137),
            4996..=5852 => HandRank::ThreeOfAKind(value as u32 - 4995),
            5853..=5863 => HandRank::Straight(value as u32 - 5852),
            5864..=7140 => HandRank::Flush(value as u32 - 5863),
            7141..=7296 => HandRank::FullHouse(value as u32 - 7140),
            7297..=7452 => HandRank::FourOfAKind(value as u32 - 7296),
            7453..=7462 => HandRank::StraightFlush(value as u32 - 7452),

            _ => panic!("Invalid hand rank value: {}", value),
        }
    }
}

#[derive(Error, Debug)]
pub enum HandParseError {
    #[error("Invalid number of cards in hand. expected 2")]
    InvalidNumberOfCards,
}

pub struct Hand(pub Card, pub Card);

impl Hand {
    pub fn deal(deck: &mut Deck) -> Hand {
        Hand(deck.draw().unwrap(), deck.draw().unwrap())
    }

    pub fn from_str(s: &str) -> Result<Hand, HandParseError> {
        
        let re = Regex::new(r"(?i)[2-9TJQKA][c|s|h|d]").unwrap();

        let a = re.find_iter(s)
            .nth(0)
            .ok_or(HandParseError::InvalidNumberOfCards)?
            .as_str()
            .into();
        let b = re.find_iter(s)
            .nth(1)
            .ok_or(HandParseError::InvalidNumberOfCards)?
            .as_str()
            .into();

        Ok(Hand(a, b))
    }   

    pub fn pocket_pair(&self) -> bool {
        self.0.rank() == self.1.rank()
    }

    pub fn suited(&self) -> bool {
        self.0.suit() == self.1.suit()
    }

    pub fn high_card(&self) -> Rank {
        self.0.rank().max(self.1.rank())
    } 

    // Returns index of hand in the range array.
    pub fn range_idx(&self) -> usize {
        let (high, low): (usize, usize) = if self.0.rank() >= self.1.rank() {
            (self.0.rank() as u8 as usize , self.1.rank() as u8 as usize)
        } else {
            (self.1.rank() as u8 as usize, self.0.rank() as u8 as usize)
        };
        
        high * (101 - high) / 2 + low - 1
    }

    pub fn chen_strength(&self) -> u8 {
        
        let mut base = self.0.chen_score().max(self.1.chen_score());
        let gap = (self.0.rank() as i8 - self.1.rank() as i8).abs() as u8;

        if self.pocket_pair() {
            base = 5.max(base * 2);
        }
        if self.suited() {
            base += 2;
        }

        match gap {
            0 => {},
            1 => base += 1,
            2 => base -= 1,
            3 => base -= 2,
            4 => base -= 4,
            _ => base -= 5,
        }

        base - gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn test_from_str() {
        // With space.
        let hand = Hand::from_str("Ah Kd").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::King, Suit::Diamonds));
        // Without space.
        let hand = Hand::from_str("Th4c").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ten, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::Four, Suit::Clubs));
        // Same rank.
        let hand = Hand::from_str("2h2d").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Two, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::Two, Suit::Diamonds));
        // With separator.
        let hand = Hand::from_str("-Ah//Kd-").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::King, Suit::Diamonds));
        // Upper/Lower case.
        let hand = Hand::from_str("aH kD").unwrap();
        assert_eq!(hand.0, Card::new(Rank::Ace, Suit::Hearts));
        assert_eq!(hand.1, Card::new(Rank::King, Suit::Diamonds));
    }

    #[test]
    fn test_hand_rank_ord() {
        assert!(HandRank::HighCard(1) > HandRank::HighCard(0));
        assert!(HandRank::StraightFlush(5) > HandRank::HighCard(7));
        assert!(HandRank::Pair(2) == HandRank::Pair(2));
    }

    #[test]
    fn test_chen_strength() {
        let hand = Hand::from_str("AhAc").unwrap();
        assert_eq!(hand.chen_strength(), 20);
    }

    #[test]
    fn test_range_idx() {
        let hand = Hand::from_str("AsAc").unwrap();
        println!("{}", hand.range_idx());
    }
}